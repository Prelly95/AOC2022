use std::collections::HashMap;
use std::error::Error;
use std::fs;

// const INPUT: &str = "./input/test.txt";
const INPUT: &str = "./input/input.txt";

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let input = fs::read_to_string(INPUT).expect("Something went wrong reading the file");

    println!("\npart 1:");
    let (dock, ins) = get_dock(&input);
    part1(dock.clone(), ins)?;
    println!("\npart 2:");
    part2(dock.clone(), ins)?;
    Ok(())
}

fn get_dock(input: &str) -> (HashMap<usize, Vec<String>>, &str) {
    let (dock, instructions) = input.split_once("\n\n").unwrap();
    let dock: Vec<&str> = dock.split("\n").collect();
    let rows: Vec<&str> = dock[0..dock.len() - 1].to_vec();
    let l = dock[dock.len() - 1];
    let mut stacks: HashMap<usize, Vec<String>> = HashMap::new();

    let col_names: Vec<usize> = l
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap() - 1)
        .collect::<Vec<usize>>();
    for boxes in rows {
        let boxes = boxes
            .chars()
            .collect::<Vec<char>>()
            .chunks(4)
            .map(|c| c.iter().collect::<String>())
            .map(|x| String::from(x.trim().replace("[", "").replace("]", "")))
            .collect::<Vec<String>>();

        for ii in &col_names {
            if boxes[*ii] != "" {
                match stacks.get_mut(&(ii + 1)) {
                    Some(x) => x.push(boxes[*ii].clone()),
                    None => {
                        stacks.insert((*ii) + 1, vec![boxes[*ii].clone()]);
                    }
                }
            }
        }
    }
    return (stacks, instructions);
}

fn part1(mut dock: HashMap<usize, Vec<String>>, instructions: &str) -> Result<()> {
    for ii in instructions.lines() {
        let vals = ii
            .split_whitespace()
            .filter(|x| x.parse::<usize>().is_ok())
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
        let m = vals[0];
        let f = vals[1];
        let t = vals[2];

        for _ in 0..m {
            let from_stack = dock.get_mut(&f).unwrap().remove(0);
            let to_stack = dock.get_mut(&t).unwrap();
            to_stack.insert(0, from_stack);
        }
    }
    let mut res = vec![String::new(); dock.len()];
    for (k, v) in dock {
        res[k - 1] = v.first().unwrap_or(&String::from("")).clone();
    }
    println!("{:?}", res.into_iter().collect::<String>());
    Ok(())
}

fn part2(mut dock: HashMap<usize, Vec<String>>, instructions: &str) -> Result<()> {
    for ii in instructions.lines() {
        let vals = ii
            .split_whitespace()
            .filter(|x| x.parse::<usize>().is_ok())
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
        let m = vals[0];
        let f = vals[1];
        let t = vals[2];
		let mut bx = vec![];
        for _ in 0..m {
            bx.push(dock.get_mut(&f).unwrap().remove(0));
        }
		bx.reverse();
		for b in bx
		{
			let to_stack = dock.get_mut(&t).unwrap();
			to_stack.insert(0, b);
		}
    }
    let mut res = vec![String::new(); dock.len()];
    for (k, v) in dock {
        res[k - 1] = v.first().unwrap_or(&String::from("")).clone();
    }
    println!("{:?}", res.into_iter().collect::<String>());
    Ok(())
}
