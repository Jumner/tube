use std::{
	collections::{HashMap, HashSet},
	fmt,
	hash::Hash,
};

#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Color {
	Empty,
	Full(u8),
}

impl Color {
	pub fn new(number: u8) -> Color {
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

#[derive(Clone, Copy, Eq, PartialOrd, Ord)]
pub struct Tube {
	colors: [Color; 4],
	id: usize,
}

impl Hash for Tube {
	fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
		self.colors.hash(state);
	}
}
impl PartialEq for Tube {
	fn eq(&self, other: &Self) -> bool {
		self.colors.eq(&other.colors)
	}
}

impl Tube {
	pub fn new(number_list: [u8; 4], id: usize) -> Tube {
		let color_list: [Color; 4] = number_list
			.iter()
			.map(|num| Color::new(*num))
			.collect::<Vec<Color>>()
			.as_slice()
			.try_into()
			.expect("Unable to create tube");
		Tube {
			colors: color_list,
			id,
		}
	}

	fn pour(&mut self, tube: &mut Tube) -> bool {
		// println!("{:?}\n{:?}", self, tube);
		let mut output = false;
		let mut color = Color::Empty;
		'outer: for i in 0..4 {
			if color.is_empty() && self.colors[i].is_value() {
				color = self.colors[i];
				// println!("color is {:?}", color);
			}
			if color.is_value() {
				if color != self.colors[i] {
					// println!("color ({:?}) != self ({:?})", color, self.colors[i]);
					return output;
				}
				for o in 0..4 {
					if tube.colors[o].is_empty() && (tube[o + 1] == color || o == 3) {
						output = true;
						// println!("Swap {i} {o}");
						tube.colors[o] = self[i];
						self.colors[i] = Color::Empty;
						continue 'outer;
					} else if tube[o + 1].is_value() {
						return output;
					}
				}
			}
		}
		output
	}

	fn is_uniform(&self) -> bool {
		for color in &self.colors[1..] {
			if color != &self.colors[0] {
				return false;
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

#[derive(Clone)]
pub struct Game {
	size: usize,
	pub state: Vec<Tube>,
	pub moves: Vec<(usize, usize)>,
}

impl Game {
	pub fn new(number_state: Vec<[u8; 4]>) -> Game {
		let size: usize = number_state.len();
		let game_state: Vec<Tube> = number_state
			.iter()
			.enumerate()
			.map(|(index, nums)| Tube::new(*nums, index + 1))
			.collect();
		Game {
			state: game_state,
			size,
			moves: vec![],
		}
	}

	fn check_valid(&self) {
		let mut color_count: HashMap<&Color, usize> = HashMap::new();
		for tube in &self.state {
			for color in &tube.colors {
				let count = color_count.entry(color).or_insert(0);
				*count += 1;
			}
		}
		for (&&color, &count) in &color_count {
			match color {
				Color::Empty if count % 4 != 0 => panic!("Invalid Number of Empty colors: {count}"),
				Color::Full(_) if count != 4 => panic!("Invalid Number of Colors: {:#?}", color_count),
				_ => (),
			}
		}
	}

	pub fn pour(&mut self, a: usize, b: usize) -> bool {
		if a == b {
			return false;
		}
		let mut tube = self.state[b];
		let output = self.state[a].pour(&mut tube);
		self.state[b] = tube;
		output
	}
	fn try_pour(&self, a: usize, b: usize) -> Option<Game> {
		let mut game = self.clone();
		if game.pour(a, b) {
			return Some(game);
		}
		None
	}

	fn moves(&self) -> Vec<Game> {
		let mut games = HashSet::new();
		for i in 0..self.size {
			for o in 0..self.size {
				if let Some(mut game) = self.try_pour(i, o) {
					game.moves.push((game.state[i].id, game.state[o].id));
					game.sort();
					games.insert(game);
				}
			}
		}
		games.into_iter().collect()
	}

	fn sort(&mut self) {
		self.state.sort_unstable();
	}

	pub fn is_solved(&self) -> bool {
		for tube in self.state.iter() {
			if !tube.is_uniform() {
				return false;
			}
		}
		true
	}

	pub fn tube(&self, id: usize) -> usize {
		for (index, tube) in self.state.iter().enumerate() {
			if tube.id == id {
				return index;
			}
		}
		panic!("Id Not Found {}", id);
	}
}

impl fmt::Debug for Game {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(
			f,
			"Game Size {}:\n{:?}\nMoves: {:?}",
			self.size,
			self.state.iter(),
			self.moves
		)
	}
}

impl Eq for Game {}
impl PartialEq for Game {
	fn eq(&self, other: &Self) -> bool {
		self.size == other.size && self.state == other.state
	}
}

impl Hash for Game {
	fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
		self.size.hash(state);
		self.state.hash(state);
		// No moves bc we want to duplicate moves avoid games states
	}
}
#[derive(Debug)]
pub struct Solver {
	states: HashSet<Game>,
	queue: Vec<Game>,
	solutions: Vec<Game>,
}

impl Solver {
	pub fn new(game: Game) -> Self {
		let mut solver = Solver {
			states: HashSet::new(),
			queue: vec![],
			solutions: vec![],
		};
		game.check_valid();
		solver.states.insert(game.clone());
		solver.queue.insert(0, game);
		solver
	}
	fn next_move(&mut self) -> bool {
		let mut queue = vec![];
		while let Some(game) = self.queue.pop() {
			for new_move in game.moves() {
				if new_move.is_solved() {
					self.solutions.push(new_move);
				} else if self.states.insert(new_move.clone()) {
					queue.insert(0, new_move);
				}
			}
		}
		self.queue = queue;
		// !self.queue.is_empty() // True if not done
		self.solutions.is_empty() // Search until a solution is found
	}

	pub fn get_solutions(&mut self) -> Vec<Vec<(usize, usize)>> {
		while self.next_move() {
			println!("Games to check: {}", self.queue.len());
		}
		self.solutions.sort_by(|a, b| b.moves.cmp(&a.moves));
		self
			.solutions
			.iter()
			.map(|solution| solution.moves.clone())
			.collect()
	}

	pub fn solve(&mut self) {
		let solutions = self.get_solutions();
		let moves = solutions[0].len();
		let solutions = solutions
			.iter()
			.map(|solution| format!("{:?}", solution))
			.collect::<Vec<String>>();
		println!(
			"Done! {} Solution(s) Found with {} moves:\n{:#?}",
			solutions.len(),
			moves,
			solutions
		);
	}
}

// --- Tests ---

#[cfg(test)]
mod tests {
	use super::*;

	fn test_many_tubes<F>(closure: F)
	where
		F: Fn([u8; 4]),
	{
		for a in 0..10 {
			for b in 64..74 {
				for c in 128..138 {
					for d in 196..206 {
						let colors = [a, b, c, d]; // Test a bunch of possibilities
						closure(colors);
					}
				}
			}
		}
	}

	#[test]
	fn color_constructor() {
		assert_eq!(Color::new(0), Color::Empty);
		assert_eq!(Color::new(255), Color::Full(255));
	}

	#[test]
	fn empty_color_is_empty() {
		assert!(Color::Empty.is_empty());
		assert!(!Color::Empty.is_value());
	}

	#[test]
	fn full_color_is_full() {
		assert!(Color::new(3).is_value());
		assert!(!Color::new(67).is_empty());
	}

	#[test]
	fn tube_constructor() {
		assert_eq!(
			Tube::new([0; 4], 0),
			Tube {
				colors: [Color::Empty; 4],
				id: 0
			}
		);
		test_many_tubes(|colors| {
			assert_eq!(
				Tube::new(colors, 0),
				Tube {
					colors: [
						Color::new(colors[0]),
						Color::new(colors[1]),
						Color::new(colors[2]),
						Color::new(colors[3])
					],
					id: 0
				}
			);
		});
	}

	#[test]
	fn can_pour_into_empty() {
		let mut tube = Tube::new([1, 2, 3, 4], 0);
		let mut empty = Tube::new([0, 0, 0, 0], 0);
		assert!(tube.pour(&mut empty));
		assert_eq!(tube, Tube::new([0, 2, 3, 4], 0));
		empty = Tube::new([0, 0, 0, 0], 0);
		assert!(tube.pour(&mut empty));
		assert_eq!(tube, Tube::new([0, 0, 3, 4], 0));
	}

	#[test]
	fn can_pour_into_partialy_full() {
		let mut tube = Tube::new([1, 2, 3, 4], 0);
		let mut partial = Tube::new([0, 1, 1, 1], 0);
		assert!(tube.pour(&mut partial));
		assert_eq!(tube, Tube::new([0, 2, 3, 4], 0));
		assert_eq!(partial, Tube::new([1, 1, 1, 1], 0));
	}

	#[test]
	fn can_pour_multiple() {
		let mut tube = Tube::new([1, 1, 2, 3], 0);
		let mut into = Tube::new([0, 0, 1, 1], 0);
		assert!(tube.pour(&mut into));
		assert_eq!(tube, Tube::new([0, 0, 2, 3], 0));
		assert_eq!(into, Tube::new([1, 1, 1, 1], 0));
	}

	#[test]
	fn cant_pour_underneath() {
		let mut tube = Tube::new([1, 1, 2, 1], 0);
		let mut into = Tube::new([0, 0, 0, 0], 0);
		assert!(tube.pour(&mut into));
		assert_eq!(tube, Tube::new([0, 0, 2, 1], 0));
		assert_eq!(into, Tube::new([0, 0, 1, 1], 0));
	}

	#[test]
	fn pours_partial() {
		let mut tube = Tube::new([1, 1, 2, 3], 0);
		let mut into = Tube::new([0, 1, 2, 3], 0);
		assert!(tube.pour(&mut into));
		assert_eq!(tube, Tube::new([0, 1, 2, 3], 0));
		assert_eq!(into, Tube::new([1, 1, 2, 3], 0));
	}

	#[test]
	fn cant_pour_into_full() {
		let mut tube = Tube::new([0, 0, 1, 2], 0);
		let mut into = Tube::new([1, 2, 3, 4], 0);
		assert!(!tube.pour(&mut into));
		assert_eq!(tube, Tube::new([0, 0, 1, 2], 0));
		assert_eq!(into, Tube::new([1, 2, 3, 4], 0));
	}

	#[test]
	fn cant_pour_wrong_color() {
		let mut tube = Tube::new([1, 2, 3, 4], 0);
		let mut into = Tube::new([0, 0, 2, 3], 0);
		assert!(!tube.pour(&mut into));
		assert_eq!(tube, Tube::new([1, 2, 3, 4], 0));
		assert_eq!(into, Tube::new([0, 0, 2, 3], 0));
	}

	#[test]
	fn cant_pour_nothing() {
		let mut tube = Tube::new([0, 0, 0, 0], 0);
		let mut into = Tube::new([0, 0, 0, 0], 0);
		assert!(!tube.pour(&mut into));
		into = Tube::new([0, 0, 1, 2], 0);
		assert!(!tube.pour(&mut into));
	}

	#[test]
	fn tube_index() {
		test_many_tubes(|colors| {
			let tube = Tube::new(colors, 0);
			assert_eq!(tube[0], Color::new(colors[0]));
			assert_eq!(tube[1], Color::new(colors[1]));
			assert_eq!(tube[2], Color::new(colors[2]));
			assert_eq!(tube[3], Color::new(colors[3]));
			assert_eq!(tube[4], Color::Empty);
		});
	}

	#[test]
	fn game_constructor() {
		let game = Game::new(vec![[1, 2, 3, 4], [0; 4]]);
		assert_eq!(
			game,
			Game {
				size: 2,
				state: vec![Tube::new([1, 2, 3, 4], 1), Tube::new([0, 0, 0, 0], 2)],
				moves: vec![]
			}
		);
	}

	#[test]
	fn cant_pour_into_self() {
		let mut game = Game::new(vec![[1, 2, 3, 4]]);
		assert!(!game.pour(0, 0));
	}

	#[test]
	fn basic_moves() {
		let game = Game::new(vec![[1, 2, 3, 4], [0; 4]]);
		let moves = game.moves();
		assert_eq!(moves, vec![Game::new(vec![[0, 0, 0, 1], [0, 2, 3, 4]])]);
	}
	#[test]
	fn simple_moves() {
		let game = Game::new(vec![[1, 1, 1, 3], [1, 2, 3, 4], [0; 4]]);
		let moves = game.moves();
		println!("{:?}", moves);
		assert!(moves.contains(&Game::new(vec![[0, 0, 0, 3], [0, 1, 1, 1], [1, 2, 3, 4]])));
		assert!(moves.contains(&Game::new(vec![[0, 0, 0, 1], [0, 2, 3, 4], [1, 1, 1, 3]])));
	}
	#[test]
	fn duplicate_moves() {
		let game = Game::new(vec![[1, 1, 3, 4], [0; 4], [0; 4]]);
		let moves = game.moves();
		assert_eq!(
			moves,
			vec![Game::new(vec![[0; 4], [0, 0, 1, 1], [0, 0, 3, 4]])]
		);
	}
}
