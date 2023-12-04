use day_02::part2::process;
use miette::Context;

fn main() -> miette::Result<()> {
	let file = include_str!("../../input2.txt");
	let result = process(file).context("process part 2")?;
	println!("{}", result);
	Ok(())
}
