use std::fmt;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Color {
	Empty,
	Full(u8),
}

impl Color {
	fn new(number: u8) -> Color {
		if number == 0 {
			Color::Empty
		} else {
			Color::Full(number)
		}
	}

	fn is_value(&self) -> bool {
		match self {
			Color::Empty => false,
			Color::Full(_) => true,
		}
	}
	fn is_empty(&self) -> bool {
		!self.is_value()
	}
}

impl fmt::Debug for Color {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Color::Empty => write!(f, "Empty"),
			Color::Full(n) => write!(f, "{}", n),
		}
	}
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Tube {
	colors: [Color; 4],
}

impl Tube {
	fn new(number_list: [u8; 4]) -> Tube {
		let color_list: [Color; 4] = number_list
			.iter()
			.map(|num| Color::new(*num))
			.collect::<Vec<Color>>()
			.as_slice()
			.try_into()
			.expect("Unable to create tube");
		Tube { colors: color_list }
	}

	fn pour(&mut self, tube: &mut Tube) -> bool {
		println!("{:?}\n{:?}", self, tube);
		let mut color = Color::Empty;
		'outer: for i in 0..4 {
			if color.is_empty() && self.colors[i].is_value() {
				color = self.colors[i];
				println!("color is {:?}", color);
			}
			if color.is_value() {
				if color != self.colors[i] {
					println!("color ({:?}) != self ({:?})", color, self.colors[i]);
					return false;
				}
				for o in 0..4 {
					if tube.colors[o].is_empty() && (tube[o + 1] == color || o == 3) {
						println!("Swap {i} {o}");
						tube.colors[o] = self[i];
						self.colors[i] = Color::Empty;
						continue 'outer;
					} else if tube[o + 1].is_value() {
						return false;
					}
				}
			}
		}
		true
	}
}

impl fmt::Debug for Tube {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.debug_list().entries(self.colors).finish()
	}
}

impl core::ops::Index<usize> for Tube {
	type Output = Color;
	fn index(&self, index: usize) -> &Self::Output {
		if index < 4 {
			return &self.colors[index];
		}
		&Color::Empty
	}
}

struct Game {
	size: u8,
	state: Vec<Tube>,
}

impl Game {
	fn new(number_state: Vec<[u8; 4]>) -> Game {
		let size: u8 = number_state
			.len()
			.try_into()
			.expect("Unable to get game size");
		let game_state: Vec<Tube> = number_state.iter().map(|nums| Tube::new(*nums)).collect();
		Game {
			state: game_state,
			size,
		}
	}
	fn pour(&mut self, a: usize, b: usize) {
		let mut tube = self.state[b];
		self.state[a].pour(&mut tube);
		self.state[b] = tube;
	}
}

impl fmt::Debug for Game {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "Game Size {}:\n{:?}", self.size, self.state.iter())
	}
}

fn main() {
	println!("Hello, world!");
	let mut game = Game::new(vec![[1, 2, 1, 2], [2, 1, 2, 1], [0, 0, 0, 0]]);
	game.pour(0, 2);
	game.pour(1, 0);
	game.pour(1, 2);
	game.pour(0, 1);
	game.pour(0, 2);
	game.pour(1, 0);
	game.pour(1, 2);
	println!("{:?}", game);
}
