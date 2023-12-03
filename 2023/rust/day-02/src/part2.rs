use std::collections::BTreeMap;
use crate::custom_error::AocError;

#[derive(Debug)]
struct Cube<'a> {
	colour: &'a str,
	count: u32
}
#[derive(Debug)]
struct Game<'a> {
	rounds: Vec<Vec<Cube<'a>>>
}

impl<'a> Game<'a> {
	fn minimum_cube_set(&self) -> u32 {
		let map: BTreeMap<&str, u32> = BTreeMap::new();
		self.rounds
			.iter()
			.fold(map, |mut acc, round| {
				for cube in round.iter() {
					acc.entry(cube. colour)
						.and_modify(|v| {
							*v = (*v).max(cube.count);
						})
						.or_insert(cube.count);
				}
				acc
			})
			.values()
			.product()
	}
}

pub fn process(input: &str) -> miette::Result<String, AocError> {
	let games = parse_games(input).expect("should parse games");
	Ok(games
		   .iter()
		   .map(|game| game.minimum_cube_set())
		   .sum::<u32>()
		   .to_string())
}

fn parse_games(input: &str) -> Result<Vec<Game>, AocError> {
	input.lines().map(|line| {
		let (_, game_rounds) = line.split_once(": ").expect("should have a game");
		let rounds = parse_rounds(game_rounds)?;
		Ok(Game { rounds })
	}).collect()
}

fn parse_rounds(rounds: &str) -> Result<Vec<Vec<Cube>>, AocError> {
	rounds.split("; ").map(|round| {
		let round = parse_round(round)?;
		Ok(round)
	}).collect()
}

fn parse_round(round: &str) -> Result<Vec<Cube>, AocError> {
	round.split(", ").map(|round| {
		let cube = parse_cube(round)?;
		Ok(cube)
	}).collect()
}

fn parse_cube(cube: &str) -> Result<Cube, AocError> {
	let (count, colour) = cube.split_once(' ').expect("should have a cube");
	let count = count.parse::<u32>().expect("should be a number");
	Ok(Cube { colour, count })
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_process() -> miette::Result<()> {
		let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
		assert_eq!("2286", process(input)?);
		Ok(())
	}
}
