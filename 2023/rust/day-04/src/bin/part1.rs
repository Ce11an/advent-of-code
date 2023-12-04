use day_04::part1::process;
use miette::Context;

fn main() -> miette::Result<()> {
	let file = include_str!("../../input1.txt");
	let result = process(file).context("process part 1")?;
	println!("{}", result);
	Ok(())
}
