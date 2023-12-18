use std::collections::HashMap;

use crate::prelude::*;

use num::integer::lcm;

#[test]
fn test_steps() {
    let input = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";

    assert_eq!(2, cal_steps(input).unwrap());
}

#[test]
fn test_steps2() {
    let input = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

    assert_eq!(6, cal_steps(input).unwrap());
}

#[test]
fn test_simultanious() {
    let input = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

    assert_eq!(6, cal_steps_simultanious(input).unwrap());
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Node([char; 3]);

impl Node {

    fn start() -> Self {
        const START: [char; 3] = ['A', 'A', 'A'];
        Self(START)
    }

    fn is_zzz(&self) -> bool {
        const DESTINATION: [char; 3] = ['Z', 'Z', 'Z'];
        self.0 == DESTINATION
    } 

    fn is_destination(&self) -> bool {
        self.0[2] == 'Z'
    }

    fn is_start(&self) -> bool {
        self.0[2] == 'A'
    }
}

impl From<&str> for Node {
    fn from(s: &str) -> Self {
        let vec = s.chars().collect::<Vec<_>>();
        Self(vec.as_slice().try_into().unwrap())
    }
}

struct Network { 
    left: Node,
    right: Node,
}

impl Network {
    fn next(&self, instruction: Instruction) -> Node {
        match instruction {
            Instruction::Left => self.left,
            Instruction::Right => self.right,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Left,
    Right,
}

impl From<char> for Instruction {
    fn from(c: char) -> Self {
        match c {
            'L' => Instruction::Left,
            'R' => Instruction::Right,
            _ => panic!("Invalid instruction: {}", c),
        }
    }
}

pub fn cal_steps(input: &str) -> Result<u64> {
    let (instructions, networks) = parse_network(input);
    let mut node = Node::start();

    let needed_steps = instructions.into_iter()
    .cycle()
    .enumerate()
    .find_map(|(step, instruction)| {
        node = networks.get(&node).unwrap().next(instruction);
        node.is_zzz().then_some(step+1)
    });

    Ok(needed_steps.unwrap() as u64)
}

fn cal_steps_needed(instructions: &[Instruction], networks: &HashMap<Node, Network>, start: Node) -> u64 {
    let mut node = start;

    instructions.iter()
    .cycle()
    .enumerate()
    .find_map(|(step, instruction)| {
        node = networks.get(&node).unwrap().next(*instruction);
        // if node.is_destination() {
        //     println!("end node {:?} step {}", node.0, step+1);
        // };
        node.is_destination().then_some(step+1)
        // let endless: Option<usize> = None; 
        // endless
    }).unwrap() as u64
}

pub fn cal_steps_simultanious(input: &str) -> Result<u64> {
    let (instructions, networks) = parse_network(input);

    let nodes = networks.keys()
    .filter(|node| node.is_start())
    .copied()
    .collect::<Vec<_>>();

    // println!("start: {:#?}", nodes);

    let steps_to_finish = nodes.iter().map(|start| {
        cal_steps_needed(&instructions, &networks, *start)
    }).collect::<Vec<_>>();

    // println!("steps needed: {:?}", steps_to_finish);
    let lcm = steps_to_finish.iter().fold(steps_to_finish[0], |acc, &x| {
        lcm(acc, x)
    });
    // println!("lcm: {:?}", lcm);
    
    Ok(lcm)
}

fn parse_network(input: &str) -> (Vec<Instruction>, HashMap<Node, Network>) {
    let double_line = LINE_ENDING.to_owned() + LINE_ENDING;
    let (instructions, networks) = input.split_once(&double_line).unwrap();    
    let instructions = instructions.chars().map(Instruction::from).collect::<Vec<_>>();

    let networks = networks.lines()
    .map(|line| {
        let (src, dest) = line.split_once(" = ").unwrap();
        let src = Node::from(src);
        let dest = dest.replace(['(', ')'], "");
        let (left, right) = dest.split_once(", ").unwrap();
        let left = Node::from(left);
        let right = Node::from(right);
        (src, Network { left, right })
    });

    let networks = networks.collect::<HashMap<_, _>>();

    (instructions, networks)
}

