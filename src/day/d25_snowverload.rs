// TODO implementd Day 25
#![allow(dead_code, unused)]

use std::collections::{HashMap, HashSet};

use crate::prelude::*;

#[test]
fn test_group_size() {
    let input = "jqt: rhn xhk nvd
rsh: frs pzl lsr
xhk: hfx
cmg: qnr nvd lhk bvb
rhn: xhk bvb hfx
bvb: xhk hfx
pzl: lsr hfx nvd
qnr: nvd
ntq: jqt hfx bvb xhk
nvd: lhk
lsr: lhk
rzs: qnr cmg lsr rsh
frs: qnr lhk lsr";

    assert_eq!(cal_group_size(input).unwrap(), 54);
}

type Name = String;
type Connecton = (Name, Name);
type Path = HashSet<Name>;

#[derive(Clone)]
struct System {
    components: HashMap<Name, HashSet<Name>>,
    path: Path,
}

impl System {
    fn new(input: &str) -> Self {
        todo!()
    }

    fn seperate_by3(&self) -> (HashSet<Name>, HashSet<Name>) {
        //TODO skip doubles (end, start)
        for start in self.components.keys() {
            for end in self.components.get(start).unwrap() {
                let con = (start.to_owned(), end.to_owned());
                let mut new_components = self.clone();
                new_components.remove_connection(&con);
                if let Some((con1, con2)) = new_components.seperate_by2(con) {
                    new_components.remove_connection(&con1);
                    new_components.remove_connection(&con2); 
                    return new_components.split();
                }
            } 
        }
        panic!("No solution found")
    }

    fn seperate_by2(&self, (start, end): Connecton) -> Option<(Connecton, Connecton)> {

        todo!()
    }

    fn find_paths(&self, (start, end): Connecton) -> Vec<Path> {
        todo!()
    }

    fn walk(self, node: Name, end: &Name, paths: &mut HashSet<Path>) {
        if node == *end {
            todo!()
        } 
    }

    fn remove_component(&mut self, component: &Name) -> HashSet<Name> {
        todo!()
    }


    fn remove_connection(&mut self, connection: &Connecton) {
        todo!()
    }

    fn split(self) -> (HashSet<Name>, HashSet<Name>) {
        todo!()
    }


}

pub fn cal_group_size(input: &str) -> Result<usize> {
    let system = System::new(input);
    let (group1, group2) = system.seperate_by3();
    let mult = group1.len() * group2.len();
    Ok(mult)
}