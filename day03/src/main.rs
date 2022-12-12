use std::collections::HashSet;
use std::error::Error;
use std::fs;

// const INPUT: &str = "./input/test.txt";
const INPUT: &str = "./input/input.txt";

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;

fn main() -> Result<()> {
	let input = fs::read_to_string(INPUT).expect("Something went wrong reading the file");

	// println!("\npart 1:");
	// part1(&input)?;
	println!("\npart 2:");
	part2(&input)?;
	Ok(())
}

fn part1(input: &str) -> Result<()> {
	let mut score: u32 = 0;
	for l in input.lines() {
		let (upper, lower) = l.split_at(l.len() / 2);
		let mut common: HashSet<char> = HashSet::new();
		for u in upper.chars() {
			for l in lower.chars() {
				if l == u {
					common.insert(l);
				}
			}
		}
		for c in common {
			println!("common - {}", c);

			if c.is_uppercase() {
				score += (c as u32) - 'A' as u32 + 27;
			} else {
				score += (c as u32) - 'a' as u32 + 1;
			}
		}
	}

	println!("{}", score);
	Ok(())
}

fn part2(input: &str) -> Result<()> {
	let mut score: u32 = 0;
	let l: Vec<&str> = input.split("\n").collect();
	for ii in (0..l.len()).step_by(3) {
		let mut badge: HashSet<char> = HashSet::new();
		let e1 = l[ii];
		let e2 = l[ii + 1];
		let e3 = l[ii + 2];
		let mut common: Vec<char> = vec![];
		for a in e1.chars() {
			for b in e2.chars() {
				if a == b {
					common.push(a);
				}
			}
		}
		for c in e3.chars() {
			for d in &common {
				if &c == d {
					badge.insert(c);
				}
			}
		}
		for b in badge.clone() {
			if b.is_uppercase() {
				score += (b as u32) - 'A' as u32 + 27;
			} else {
				score += (b as u32) - 'a' as u32 + 1;
			}
		}
	}

	println!("{}", score);
	Ok(())
}
