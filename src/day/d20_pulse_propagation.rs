use std::{
    collections::{HashMap, VecDeque},
    ops::Not,
};

use crate::prelude::*;

#[test]
fn test_pulses() {
    let input = "broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a";
    assert_eq!(cal_pulses(input).unwrap(), 32000000);
}

#[test]
fn test_pulses2() {
    let input = "broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output";
    assert_eq!(cal_pulses(input).unwrap(), 11687500);
}

enum ModuleTyp {
    Broadcast,
    FlipFlop,
    Conjunction(Vec<String>),
}

struct Module {
    typ: ModuleTyp,
    name: String,
    targets: Vec<String>,
}

impl Module {
    fn send_pulse(&mut self, pulse: Pulse, queue: &mut Queue, states: &mut State) {
        let output = match &mut self.typ {
            ModuleTyp::Broadcast => Some(pulse),
            ModuleTyp::FlipFlop => {
                let state = states.get_mut(&self.name).unwrap();
                if pulse == Pulse::Low {
                    *state = !*state;
                    Some(*state)
                } else {
                    None
                }
            }
            ModuleTyp::Conjunction(inputs) => {
                let all_high = inputs
                    .iter()
                    .map(|name| states.get(name).unwrap())
                    .all(|&pulse| pulse == Pulse::High);

                let state = states.get_mut(&self.name).unwrap();
                if all_high {
                    *state = Pulse::Low;
                    Some(Pulse::Low)
                } else {
                    *state = Pulse::High;
                    Some(Pulse::High)
                }
            }
        };

        if let Some(pulse) = output {
            for target in self.targets.iter() {
                queue.push_back((target.clone(), pulse));
            }
        }
    }

    fn parse(input: &str) -> (String, Module) {
        let (typ, targets) = input.split_once(" -> ").unwrap();
        let (typ, name) = match typ {
            "broadcaster" => {
                let typ = ModuleTyp::Broadcast;
                let name = "broadcaster".to_owned();
                (typ, name)
            }
            label => {
                if let Some(name) = label.strip_prefix('%') {
                    let typ = ModuleTyp::FlipFlop;
                    let name = name.to_owned();
                    (typ, name)
                } else if let Some(name) = label.strip_prefix('&') {
                    let typ = ModuleTyp::Conjunction(Vec::new());
                    let name = name.to_owned();
                    (typ, name)
                } else {
                    panic!("Invalid module type: {}", typ)
                }
            }
        };

        let targets = targets
            .split(", ")
            .map(|s| s.to_owned())
            .collect::<Vec<_>>();
        let module = Self {
            typ,
            name: name.clone(),
            targets,
        };

        (name, module)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Pulse {
    High,
    Low,
}

impl Not for Pulse {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Pulse::High => Pulse::Low,
            Pulse::Low => Pulse::High,
        }
    }
}

type Queue = VecDeque<(String, Pulse)>;
type Modules = HashMap<String, Module>;
type State = HashMap<String, Pulse>;

struct Machine {
    modules: Modules,
    low_pulses: usize,
    high_pulses: usize,
}

impl Machine {
    fn push_button(&mut self, pushes: usize) -> Option<usize> {
        let mut queue = Queue::new();
        // let mut seen = Vec::new();
        // let mut counts = Vec::new();
        let mut rec_rx = None;
        let mut state = self
            .modules
            .keys()
            .map(|name| (name.clone(), Pulse::Low))
            .collect::<State>();
        for round in 0..pushes {
            // if let Some(start) = seen.iter().position(|seen| *seen == state) {
                // println!("Loop detected at round {}", round);
                // let loop_len = round - start;
                // let loop_start: (usize, usize) = counts[start];
                // let loop_delta = (
                    // self.low_pulses - loop_start.0,
                    // self.high_pulses - loop_start.1,
                // );
                // let loop_count = (pushes - start) / loop_len;

                // let remaining = (pushes - start) % loop_len;
                // let remaining_end = counts[start + remaining];
                // let remaining_delta = (
                    // remaining_end.0 - loop_start.0,
                    // remaining_end.1 - loop_start.1,
                // );

                // self.low_pulses = loop_start.0 + loop_delta.0 * loop_count + remaining_delta.0;
                // self.high_pulses = loop_start.1 + loop_delta.1 * loop_count + remaining_delta.1;

                // break;
            // }

            // seen.push(state.clone());
            // // println!("State: {:#?}", state);
            // counts.push(self.get_pulses());

            queue.push_back(("broadcaster".to_owned(), Pulse::Low));
            while let Some((name, pulse)) = queue.pop_front() {
                match pulse {
                    Pulse::High => self.high_pulses += 1,
                    Pulse::Low => self.low_pulses += 1,
                }
                if let Some(module) = self.modules.get_mut(&name) {
                    module.send_pulse(pulse, &mut queue, &mut state);
                } else if "rx" == name  {
                    if pulse == Pulse::Low && rec_rx.is_none() {
                        println!("RX LOW at round {}", round);
                        rec_rx = Some(round);
                    } else {
                        // println!("RX HIGH at round {}", round);
                    }
                }
            }
        }
        rec_rx
    }

    fn get_pulses(&self) -> (usize, usize) {
        (self.low_pulses, self.high_pulses)
    }
}

impl From<&str> for Machine {
    fn from(input: &str) -> Self {
        let mut modules = input.lines().map(Module::parse).collect::<HashMap<_, _>>();

        let conjunctions = modules
            .iter()
            .filter(|(_, module)| matches!(module.typ, ModuleTyp::Conjunction(_)))
            .map(|(name, _)| name.clone())
            .collect::<Vec<_>>();

        for con_name in conjunctions {
            let new_inputs = modules
                .iter()
                .filter(|(_, module)| module.targets.contains(&con_name))
                .map(|(name, _)| name.clone())
                .collect::<Vec<_>>();

            let con = modules.get_mut(&con_name).unwrap();
            if let ModuleTyp::Conjunction(inputs) = &mut con.typ {
                *inputs = new_inputs;
            } else {
                panic!("Invalid module type");
            }
        }

        Self {
            modules,
            low_pulses: 0,
            high_pulses: 0,
        }
    }
}

pub fn cal_pulses(input: &str) -> Result<usize> {
    let mut machine = Machine::from(input);
    machine.push_button(1000);
    let (lows, highs) = machine.get_pulses();
    Ok(lows * highs)
}

//TODO too slow
#[allow(dead_code)]
pub fn cal_rx_pushes(input: &str) -> Result<usize> {
    let mut machine = Machine::from(input);
    let rx = machine.push_button(1_000_000);
    Ok(rx.unwrap())
}
