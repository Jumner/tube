use tube::{Game, Solver};
fn main() {
	let game = Game::new(vec![
		[1, 1, 2, 3],
		[3, 2, 3, 4],
		[1, 5, 4, 3],
		[2, 5, 1, 4],
		[2, 5, 4, 5],
		[0; 4],
		[0; 4],
	]);
	let mut solver = Solver::new(game);
	solver.solve();
	// println!("{:?}", game);
}

#[cfg(test)]
mod test {
	use tube::{Game, Solver};
	// --- Solutions ---

	fn test_game(mut game: Game) {
		let solutions = Solver::new(game.clone()).get_solutions();
		let solution = solutions.first().unwrap();
		for (a, b) in solution {
			let i = game.tube(*a);
			let o = game.tube(*b);
			game.pour(i, o);
		}
		assert!(game.is_solved());
	}

	#[test]
	fn simple_game() {
		let mut solver = Solver::new(Game::new(vec![[1, 2, 1, 2], [2, 1, 2, 1], [0, 0, 0, 0]]));
		assert!(solver.get_solutions().contains(&vec![
			(1, 3),
			(2, 1),
			(2, 3),
			(1, 2),
			(1, 3),
			(2, 1),
			(2, 3)
		]));
	}
	#[test]
	fn game_2() {
		test_game(Game::new(vec![[1, 2, 1, 2], [2, 1, 2, 1], [0; 4]]));
	}
	#[test]
	fn game_3() {
		test_game(Game::new(vec![
			[1, 2, 3, 1],
			[1, 2, 3, 3],
			[2, 3, 1, 2],
			[0; 4],
			[0; 4],
		]));
	}
	#[test]
	fn game_4() {
		test_game(Game::new(vec![
			[1, 1, 2, 3],
			[2, 3, 2, 3],
			[1, 2, 3, 1],
			[0; 4],
			[0; 4],
		]));
	}
	#[test]
	fn game_6() {
		test_game(Game::new(vec![
			[1, 1, 1, 2],
			[1, 3, 2, 4],
			[4, 2, 4, 3],
			[3, 4, 3, 5],
			[2, 5, 5, 5],
			[0; 4],
			[0; 4],
		]))
	}
	#[test]
	fn game_7() {
		test_game(Game::new(vec![
			[1, 2, 1, 3],
			[1, 2, 4, 4],
			[1, 4, 5, 5],
			[4, 2, 5, 3],
			[5, 3, 2, 3],
			[0; 4],
			[0; 4],
		]));
	}
	#[test]
	fn game_9() {
		test_game(Game::new(vec![
			[1, 2, 3, 1],
			[4, 5, 5, 3],
			[1, 4, 4, 3],
			[2, 5, 3, 1],
			[2, 2, 4, 5],
			[0; 4],
			[0; 4],
		]));
	}
	#[test]
	fn game_10() {
		test_game(Game::new(vec![
			[1, 2, 1, 3],
			[4, 3, 5, 6],
			[2, 7, 7, 1],
			[2, 6, 6, 3],
			[4, 2, 5, 5],
			[7, 7, 4, 1],
			[5, 6, 3, 4],
			[0; 4],
			[0; 4],
		]));
	}
	#[test]
	fn game_12() {
		test_game(Game::new(vec![
			[1, 1, 2, 3],
			[3, 2, 3, 4],
			[1, 5, 4, 3],
			[2, 5, 1, 4],
			[2, 5, 4, 5],
			[0; 4],
			[0; 4],
		]))
	}
}
