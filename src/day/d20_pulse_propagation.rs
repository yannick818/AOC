use std::{
    collections::{HashMap, VecDeque},
    ops::{BitXor, Not},
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
    FlipFlop(Pulse),
    Conjunction(Vec<String>),
}

struct Module {
    typ: ModuleTyp,
    name: String,
    targets: Vec<String>,
}

impl Module {
    fn send_pulse(&mut self, pulse: Pulse, queue: &mut Queue, modules: &Modules) {
        let output = match &mut self.typ {
            ModuleTyp::Broadcast => Some(pulse),
            ModuleTyp::FlipFlop(state) => {
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
                    .map(|name| {
                        let input = modules.get(name).unwrap();
                        input.last_pulse()
                    })
                    .all(|pulse| pulse == Pulse::High);

                if all_high {
                    Some(Pulse::Low)
                } else {
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

    fn last_pulse(&self) -> Pulse {
        match self.typ {
            ModuleTyp::Broadcast => Pulse::Low,
            ModuleTyp::FlipFlop(state) => state,
            ModuleTyp::Conjunction(_) => todo!(),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
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

struct Machine {
    modules: Modules,
    low_pulses: usize,
    high_pulses: usize,
}

impl Machine {
    fn push_button(&mut self, cnt: usize) {
        let mut queue = Queue::new();
        for _ in 0..cnt {
            // TODO check duplicate state here
            queue.push_back(("broadcaster".to_owned(), Pulse::Low));
            while let Some((name, pulse)) = queue.pop_front() {
                match pulse {
                    Pulse::High => self.high_pulses += 1,
                    Pulse::Low => self.low_pulses += 1,
                }
                let module = self.modules.get_mut(&name).unwrap();
                module.send_pulse(pulse, &mut queue, &self.modules);
            }
        }
        todo!()
    }
    fn get_pulses(&self) -> (usize, usize) {
        (self.low_pulses, self.high_pulses)
    }
}

impl From<&str> for Machine {
    fn from(input: &str) -> Self {
        todo!()
    }
}

pub fn cal_pulses(input: &str) -> Result<usize> {
    let mut machine = Machine::from(input);
    machine.push_button(1000);
    let (lows, highs) = machine.get_pulses();
    Ok(lows * highs)
}
