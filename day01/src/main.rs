// use std::collections::HashSet;
use std::error::Error;
use std::fs;

// const INPUT: &str = "./input/test.txt";
const INPUT: &str = "./input/input.txt";

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;

fn main() -> Result<()> {
	let input = fs::read_to_string(INPUT).expect("Something went wrong reading the file");

	println!("\npart 1:");
	part1(&input)?;
	println!("\npart 2:");
	part2(&input)?;
	Ok(())
}

fn part1(input: &str) -> Result<()> {
	let mut b = 0;
	for elf in input.split("\n\n")
	{
		let mut sum = 0;
		for snack in elf.lines()
		{
			sum += snack.parse::<i32>().unwrap();
		}
		if sum > b
		{
			b = sum;
		}
	}
	println!("Biggest elf: {}", b);

	Ok(())
}

fn part2(input: &str) -> Result<()> {
		let mut e:  Vec<u32> = vec![];
		for elf in input.split("\n\n")
		{
			let mut sum: u32 = 0;
			for snack in elf.lines()
			{
				sum += snack.parse::<u32>().unwrap();
			}
			e.push(sum);
		}
		e.sort();
		e.reverse();
		println!("Biggest elf: {}", e[0] + e[1] + e[2]);
	
		Ok(())
	}