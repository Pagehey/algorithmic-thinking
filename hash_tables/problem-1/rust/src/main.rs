use std::{fs::{self, File}, env, io::{BufReader, BufRead}};

const MAX_SIZE: usize = 100_000;

type SnowflakeNodePtr = Option<Box<SnowflakeNode>>;
type Snowflake = [usize;6];

#[derive(Debug)]
struct SnowflakeNode {
    snowflake: Snowflake,
    next: SnowflakeNodePtr
}

impl SnowflakeNode {
    fn new(snowflake: Snowflake) -> Self {
        SnowflakeNode { snowflake, next: None }
    }
}

fn main() {
    let current_dir = env::current_dir().unwrap();
    let data_path = format!("{}/../data/", current_dir.to_string_lossy());
    let dir = fs::read_dir(data_path);

    if let Ok(entries) = dir {
        for entry in entries {
            let entry = entry.unwrap();
            let file = fs::File::open(entry.path()).unwrap();
            let snowflakes = build_snowflakes_table(file);
            identify_identical(snowflakes);
        }
    }
}

fn build_snowflakes_table(file: File) -> Vec<SnowflakeNodePtr> {
    // let mut snowflakes: [SnowflakeNodePtr;MAX_SIZE] = array_init(|_| None);
    // let mut snowflakes: [SnowflakeNodePtr;MAX_SIZE] = array![None; 100_000];
    // let mut snowflakes: Vec<SnowflakeNodePtr> = vec![None;MAX_SIZE];
    let mut snowflakes: Vec<SnowflakeNodePtr> = Vec::with_capacity(MAX_SIZE);
    for _ in 0..MAX_SIZE { snowflakes.push(None) }

    let mut reader = BufReader::new(file);
    let mut first_line = String::new();
    reader.read_line(&mut first_line).expect("error when parsing first line.");

    for line in reader.lines() {
        let line = line.unwrap();

        let snowflake: Snowflake = parse_line_to_snowflake(line);
        let index = code(&snowflake);


        let mut node = SnowflakeNode::new(snowflake);
        if let Some(_) = snowflakes.get(index) {
            let previous_node = snowflakes[index].take();
            node.next = previous_node;
        }
        snowflakes[index] = SnowflakeNodePtr::Some(Box::new(node));
    }
    snowflakes
}

fn parse_line_to_snowflake(line: String) -> Snowflake {
    let snow: Vec<usize> = line
        .split_whitespace()
        .map(|n| n.parse::<usize>().unwrap())
        .collect();

    core::array::from_fn(|i| snow[i])
}

fn code(snowflake: &[usize;6]) -> usize {
    snowflake.iter().sum::<usize>() % MAX_SIZE
}

fn identify_identical(snowflakes: Vec<SnowflakeNodePtr>) {
    for i in 0..MAX_SIZE {
        let mut node1 = &snowflakes[i];
        while let Some(snowflake_node_1) = node1 {
            let mut node2 = &snowflake_node_1.next;
            while let Some(snowflake_node_2) = node2 {
                if are_identical(&snowflake_node_1.snowflake, &snowflake_node_2.snowflake) {
                    println!("Twin snowflakes found");
                    return;
                }
                node2 = &snowflake_node_2.next;
            }
            node1 = &snowflake_node_1.next;
        }
    }
    println!("No two snowflakes are alike.")
}

fn are_identical(snowflake1: &Snowflake, snowflake2: &Snowflake) -> bool {
    for start in 0..6 {
        if identical_right(&snowflake1, &snowflake2, start) ||
            identical_left(&snowflake1, &snowflake2, start) {
            return true
        }
    }
    false
}

fn identical_right(snowflake1: &Snowflake, snowflake2: &Snowflake, start: usize) -> bool {
    for offset in 0..6 {
        if snowflake1[offset]  != snowflake2[(start + offset) % 6] {
            return false;
        }
    }
    true
}

fn identical_left(snowflake1: &Snowflake, snowflake2: &Snowflake, start: usize) -> bool {
    for offset in 0..6 {
        let snow_index: i8 = (start - offset) as i8;
        let snow_index = if snow_index < 0 { start - offset + 6 } else { snow_index as usize };
        if snowflake1[offset] != snowflake2[snow_index] {
            return false;
        }
    }
    true
}
