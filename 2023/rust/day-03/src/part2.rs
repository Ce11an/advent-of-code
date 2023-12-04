use std::collections::HashMap;

use crate::custom_error::AocError;
use glam::IVec2;
use itertools::Itertools;
use nom::{
	branch::alt,
	bytes::complete::{tag, take_till1},
	character::complete::digit1,
	combinator::iterator,
	IResult, Parser,
};
use nom_locate::LocatedSpan;

type Span<'a> = LocatedSpan<&'a str>;
type SpanIVec2<'a> = LocatedSpan<&'a str, IVec2>;

#[derive(Debug, PartialEq)]
enum Value<'a> {
	Empty,
	Symbol(SpanIVec2<'a>),
	Number(SpanIVec2<'a>),
}

fn with_xy(span: Span) -> SpanIVec2 {
	// column/location are 1-indexed
	let x = span.get_column() as i32 - 1;
	let y = span.location_line() as i32 - 1;

	span.map_extra(|_| IVec2::new(x, y))
}
fn parse_grid(input: Span) -> IResult<Span, Vec<Value>> {
	let mut it = iterator(
		input,
		alt((
			digit1.map(with_xy).map(Value::Number),
			tag("*").map(with_xy).map(Value::Symbol),
			take_till1(|c: char| c.is_ascii_digit() || c == '*').map(|_| Value::Empty),
		)),
	);

	let parsed = it.filter(|value| value != &Value::Empty).collect::<Vec<Value>>();
	let res: IResult<_, _> = it.finish();

	res.map(|(input, _)| (input, parsed))
}

const POSITIONS: [IVec2; 8] = [
	// north border
	IVec2::new(0, 1),
	// northeast
	IVec2::new(1, 1),
	// east border
	IVec2::new(1, 0),
	// south east
	IVec2::new(1, -1),
	// south
	IVec2::new(0, -1),
	// south west
	IVec2::new(-1, -1),
	// west
	IVec2::new(-1, 0),
	// northwest
	IVec2::new(-1, 1),
];

pub fn process(input: &str) -> miette::Result<String, AocError> {
	let objects = parse_grid(Span::new(input)).unwrap().1;

	let number_map = objects
		.iter()
		.filter_map(|value| match value {
			Value::Empty => None,
			Value::Symbol(_) => None,
			Value::Number(num) => Some((
				num.extra,
				num.fragment(),
				// used as ID
				num.location_offset(),
			)),
		})
		.flat_map(|(ivec, fragment, id)| {
			(ivec.x..(ivec.x + fragment.len() as i32))
				.map(move |x| (IVec2::new(x, ivec.y), (id, fragment)))
		})
		.collect::<HashMap<IVec2, (usize, &&str)>>();

	let result = objects
		.iter()
		.filter_map(|value| {
			let Value::Symbol(sym) = value else {
				return None;
			};
			let matching_numbers = POSITIONS
				.iter()
				.map(|pos| *pos + sym.extra)
				.filter_map(|surrounding_symbol_position| {
					number_map.get(&surrounding_symbol_position)
				})
				.unique()
				.map(|(_, fragment)| fragment.parse::<i32>().expect("should be a valid number"))
				.collect::<Vec<i32>>();

			(matching_numbers.len() == 2).then_some(matching_numbers.iter().product::<i32>())
		})
		.sum::<i32>();

	Ok(result.to_string())
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_process() -> miette::Result<()> {
		let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
		assert_eq!("467835", process(input)?);
		Ok(())
	}
}
