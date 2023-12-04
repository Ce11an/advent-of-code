use crate::custom_error::AocError;
use nom::bytes::complete::tag;
use nom::character::complete;
use nom::multi::{fold_many1, separated_list1};
use nom::sequence::{delimited, separated_pair, terminated, tuple};
use nom::{IResult, Parser};
use std::collections::{BTreeMap, HashSet};

#[derive(Debug)]
struct ScratchCard {
	winning_numbers: HashSet<u32>,
	my_numbers: HashSet<u32>,
}

impl ScratchCard {
	fn num_matches(&self) -> usize {
		self.winning_numbers.intersection(&self.my_numbers).count()
	}
}

fn parse_numbers(input: &str) -> IResult<&str, HashSet<u32>> {
	fold_many1(
		terminated(complete::u32, complete::space0),
		HashSet::new,
		|mut acc: HashSet<u32>, item| {
			acc.insert(item);
			acc
		},
	)(input)
}

fn parse_card(input: &str) -> IResult<&str, ScratchCard> {
	let (input, _) = delimited(
		tuple((tag("Card"), complete::space1)),
		complete::digit1,
		tuple((tag(":"), complete::space1)),
	)(input)?;

	separated_pair(parse_numbers, tuple((tag("|"), complete::space1)), parse_numbers)
		.map(|(winning_numbers, my_numbers)| ScratchCard {
			winning_numbers,
			my_numbers,
		})
		.parse(input)
}

fn parse_cards(input: &str) -> IResult<&str, Vec<ScratchCard>> {
	separated_list1(complete::line_ending, parse_card)(input)
}

pub fn process(input: &str) -> miette::Result<String, AocError> {
	let (_, card_data) = parse_cards(input).expect("a valid parse");
	let data = card_data.iter().map(|card| card.num_matches()).collect::<Vec<_>>();

	let store = (0..card_data.len()).map(|index| (index, 1)).collect::<BTreeMap<usize, u32>>();
	let result = data
		.iter()
		.enumerate()
		.fold(store, |mut acc, (index, card_score)| {
			let to_add = *acc.get(&index).unwrap();

			for i in (index + 1)..(index + 1 + *card_score) {
				acc.entry(i).and_modify(|value| {
					*value += to_add;
				});
			}
			acc
		})
		.values()
		.sum::<u32>();

	Ok(result.to_string())
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_process() -> miette::Result<()> {
		let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
		assert_eq!("30", process(input)?);
		Ok(())
	}
}
