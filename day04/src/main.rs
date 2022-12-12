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
    let mut count = 0;
    for l in input.lines() {
        let (e1, e2) = l.split_once(",").unwrap();

        let (e1_start, e1_end) = e1.split_once("-").unwrap();
        let e1_start: u32 = e1_start.parse().unwrap();
        let e1_end: u32 = e1_end.parse().unwrap();

        let (e2_start, e2_end) = e2.split_once("-").unwrap();
        let e2_start: u32 = e2_start.parse().unwrap();
        let e2_end: u32 = e2_end.parse().unwrap();
        if e1_start <= e2_start && e1_end >= e2_end {
            count += 1;
        } else if e1_start >= e2_start && e1_end <= e2_end {
            count += 1;
        }
    }
    println!("Duplicates: {}", count);
    Ok(())
}

fn part2(input: &str) -> Result<()> {
    let mut count = 0;
    for l in input.lines() {
        let (e1, e2) = l.split_once(",").unwrap();

        let (e1_start, e1_end) = e1.split_once("-").unwrap();
        let e1_start: u32 = e1_start.parse().unwrap();
        let e1_end: u32 = e1_end.parse().unwrap();

        let (e2_start, e2_end) = e2.split_once("-").unwrap();
        let e2_start: u32 = e2_start.parse().unwrap();
        let e2_end: u32 = e2_end.parse().unwrap();
        // println!("{}", l);
        if e1_start <= e2_start && e1_end >= e2_start {
            count += 1;
        } else if e2_start <= e1_start && e2_end >= e1_start {
            count += 1;
        }
    }
    println!("Overlap: {}", count);
    Ok(())
}
