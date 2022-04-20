use tube::{Game, Solver};
fn main() {
	let mut solver = Solver::new(Game::new(vec![[1, 2, 1, 2], [2, 1, 2, 1], [0, 0, 0, 0]]));
	solver.solve();
	// println!("{:?}", game);
}

#[cfg(test)]
mod test {
	use tube::{Game, Solver};
	// --- Solutions ---

	#[test]
	fn simple_game() {
		let mut solver = Solver::new(Game::new(vec![[1, 2, 1, 2], [2, 1, 2, 1], [0, 0, 0, 0]]));
		assert!(solver.get_solutions().contains(&vec![
			(0, 2),
			(2, 1),
			(1, 0),
			(2, 1),
			(1, 0),
			(2, 0),
			(0, 1)
		]));
		assert!(solver.get_solutions().contains(&vec![
			(1, 2),
			(2, 1),
			(1, 0),
			(2, 0),
			(0, 1),
			(2, 0),
			(0, 1)
		]));
	}
}
