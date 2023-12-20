use std::collections::HashMap;

use crate::prelude::*;

#[allow(dead_code)]
const INPUT: &str = "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}";

#[test]
fn test_sum_accepted() {
    assert_eq!(19114, cal_sum_accepted(INPUT).unwrap());
}

#[test]
fn test_sum_accepted2() {
    todo!("too slow to test");
    // assert_eq!(167409079868000, cal_all_possibilities(INPUT).unwrap());
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
struct Part {
    component: [usize; 4],
}

impl Part {
    fn sum(&self) -> usize {
        self.component.iter().sum()
    }

    fn get_rating(&self, categorie: Category) -> usize {
        let index = match categorie {
            Category::XtreamCool => 0,
            Category::Musical => 1,
            Category::Aerodynamic => 2,
            Category::Shiny => 3,
        };
        self.component[index]
    }
}

impl From<&str> for Part {
    fn from(input: &str) -> Self {
        let input = input.replace(['{', '}'], "");
        let component = input
            .split(',')
            .map(|element| {
                let (_, value) = element.split_once('=').unwrap();
                value.parse().unwrap()
            })
            .enumerate()
            .fold([0; 4], |mut components, (index, value)| {
                components[index] = value;
                components
            });

        Self { component }
    }
}

#[derive(Copy, Clone)]
enum Category {
    XtreamCool,
    Musical,
    Aerodynamic,
    Shiny,
}

impl From<&str> for Category {
    fn from(input: &str) -> Self {
        match input {
            "x" => Category::XtreamCool,
            "m" => Category::Musical,
            "a" => Category::Aerodynamic,
            "s" => Category::Shiny,
            _ => panic!("unknown category {}", input),
        }
    }
}

#[derive(Clone)]
enum RuleResult {
    Approved,
    Rejected,
    Rule(String),
}

impl From<&str> for RuleResult {
    fn from(input: &str) -> Self {
        match input {
            "A" => RuleResult::Approved,
            "R" => RuleResult::Rejected,
            name => RuleResult::Rule(name.to_owned()),
        }
    }
}

struct Rule {
    result: RuleResult,
    check: Option<(Category, bool, usize)>,
}

impl Rule {
    fn apply(&self, part: &Part) -> Option<RuleResult> {
        if let Some((category, bigger_then, value)) = self.check {
            let val = part.get_rating(category);
            let check = if bigger_then {
                val > value
            } else {
                val < value
            };
            if check {
                Some(self.result.clone())
            } else {
                None
            }
        } else {
            Some(self.result.clone())
        }
    }

    fn apply_all(rules: &[Self], part: &Part) -> RuleResult {
        for rule in rules {
            if let Some(res) = rule.apply(part) {
                return res;
            }
        }
        panic!("no rule applied")
    }
}

impl From<&str> for Rule {
    fn from(input: &str) -> Self {
        if let Some((check, result)) = input.split_once(':') {
            let result = RuleResult::from(result);
            let bigger_then = check.contains('>');
            let splitter = if bigger_then { '>' } else { '<' };
            let (category, value) = check.split_once(splitter).unwrap();
            let category = Category::from(category);
            let value = value.parse().unwrap();

            Self {
                result,
                check: Some((category, bigger_then, value)),
            }
        } else {
            Self {
                check: None,
                result: RuleResult::from(input),
            }
        }
    }
}

type Name = String;

struct Workflow {
    rules: HashMap<Name, Vec<Rule>>,
    parts: Vec<Part>,
}

impl Workflow {
    fn parse(input: &str) -> Self {
        let (rules, parts) = input.split_once("\n\n").unwrap();
        let rules = rules
            .lines()
            .map(|line| {
                let (name, workflow) = line.split_once('{').unwrap();
                let workflow = workflow.replace('}', "");
                let rules = workflow.split(',').map(Rule::from).collect();
                (name.to_owned(), rules)
            })
            .collect();

        let parts = parts.lines().map(Part::from).collect();

        Self { rules, parts }
    }

    fn get_accepted(&self) -> Vec<Part> {
        let mut accepted = Vec::new();
        for part in self.parts.iter() {
            let mut rules = self.rules.get("in").unwrap();
            loop {
                match Rule::apply_all(rules, part) {
                    RuleResult::Approved => {
                        accepted.push(*part);
                        break;
                    }
                    RuleResult::Rejected => break,
                    RuleResult::Rule(name) => rules = self.rules.get(&name).unwrap(),
                }
            }
        }
        accepted
    }

    fn get_combinations(&self) -> usize {
        let mut accepted = 0;
        let all_parts = (1..=4000).flat_map(|x| {
            (1..=4000).flat_map(move |m| {
                (1..=4000).flat_map(move |a| {
                    (1..=4000).map(move |s| {
                        let component = [x, m, a, s];
                        Part { component }
                    })
                })
            })
        });
        for part in all_parts {
            println!("{:?}", part);
            let mut rules = self.rules.get("in").unwrap();
            loop {
                match Rule::apply_all(rules, &part) {
                    RuleResult::Approved => {
                        accepted += 1;
                        break;
                    }
                    RuleResult::Rejected => break,
                    RuleResult::Rule(name) => rules = self.rules.get(&name).unwrap(),
                }
            }
        }
        accepted
    }
}

pub fn cal_sum_accepted(input: &str) -> Result<usize> {
    let workflow = Workflow::parse(input);
    let accepted = workflow.get_accepted();
    let sum = accepted.into_iter().map(|p| p.sum()).sum();
    Ok(sum)
}

#[allow(dead_code)]
pub fn cal_all_possibilities(input: &str) -> Result<usize> {
    let workflow = Workflow::parse(input);
    let comb = workflow.get_combinations();
    Ok(comb)
}
