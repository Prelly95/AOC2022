use std::collections::HashSet;
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
	// let bit = input.chars().map(|x| x as u32).collect::<Vec<u32>>();
	let bit = input.chars().collect::<Vec<char>>();
	for ii in 0..bit.len() {
		let mut marker = vec![];
		for jj in 0..4 {
			if !marker.contains(&bit[ii + jj]) {
				marker.push(bit[ii + jj]);
			}
		}
		if marker.len() >= 4 {
			println!("First packet marker = {:?}, {}", marker, ii + 4);
			break;
		}
	}
	Ok(())
}

fn part2(input: &str) -> Result<()> {
	// let bit = input.chars().map(|x| x as u32).collect::<Vec<u32>>();
	let bit = input.chars().collect::<Vec<char>>();
	for ii in 0..bit.len() {
		let mut marker = vec![];
		for jj in 0..14 {
			if !marker.contains(&bit[ii + jj]) {
				marker.push(bit[ii + jj]);
			}
		}
		if marker.len() >= 14 {
			println!("First packet marker = {:?}, {}", marker, ii + 14);
			break;
		}
	}
	Ok(())
}
