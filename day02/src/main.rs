// use std::collections::HashSet;
use std::error::Error;
use std::fs;
const ROCK: u8 = 1;
const PAPER: u8 = 2;
const SCISSORS: u8 = 3;
const WIN: u8 = 6;
const DRAW: u8 = 3;
const LOSE: u8 = 0;

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
	let mut total_score: u32 = 0;
	for l in input.lines() {
		let (a, b) = l.split_once(" ").unwrap();
		match a {
			"A" => match b {
				"X" => {
					let score = ROCK + DRAW;
					total_score += score as u32;
				}
				"Y" => {
					let score = PAPER + WIN;
					total_score += score as u32;
				}
				"Z" => {
					let score = SCISSORS + LOSE;
					total_score += score as u32;
				}
				_ => {}
			},
			"B" => match b {
				"X" => {
					let score = ROCK + LOSE;
					total_score += score as u32;
				}
				"Y" => {
					let score = PAPER + DRAW;
					total_score += score as u32;
				}
				"Z" => {
					let score = SCISSORS + WIN;
					total_score += score as u32;
				}
				_ => {}
			},
			"C" => match b {
				"X" => {
					let score = ROCK + WIN;
					total_score += score as u32;
				}
				"Y" => {
					let score = PAPER + LOSE;
					total_score += score as u32;
				}
				"Z" => {
					let score = SCISSORS + DRAW;
					total_score += score as u32;
				}
				_ => {}
			},
			_ => {}
		}
	}
	println!("{}", total_score);
	Ok(())
}

fn part2(input: &str) -> Result<()> {
	let mut total_score: u32 = 0;
	for l in input.lines() {
		let (a, b) = l.split_once(" ").unwrap();
		match a {
			"A" => match b {
				"X" => {
					let score = SCISSORS + LOSE;
					total_score += score as u32;
				}
				"Y" => {
					let score = ROCK + DRAW;
					total_score += score as u32;
				}
				"Z" => {
					let score = PAPER + WIN;
					total_score += score as u32;
				}
				_ => {}
			},
			"B" => match b {
				"X" => {
					let score = ROCK + LOSE;
					total_score += score as u32;
				}
				"Y" => {
					let score = PAPER + DRAW;
					total_score += score as u32;
				}
				"Z" => {
					let score = SCISSORS + WIN;
					total_score += score as u32;
				}
				_ => {}
			},
			"C" => match b {
				"X" => {
					let score = PAPER + LOSE;
					total_score += score as u32;
				}
				"Y" => {
					let score = SCISSORS + DRAW;
					total_score += score as u32;
				}
				"Z" => {
					let score = ROCK + WIN;
					total_score += score as u32;
				}
				_ => {}
			},
			_ => {}
		}
	}
	println!("{}", total_score);
	Ok(())
}
