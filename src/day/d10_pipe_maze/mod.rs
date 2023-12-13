use std::collections::HashMap;

use crate::prelude::*;
mod runner;
mod maze;

use maze::*;
use runner::*;

#[allow(dead_code)]
#[test]
fn test_maze_distance() {
    let input = "-L|F7
7S-7|
L|7||
-L-J|
L|-JF";
    assert_eq!(4, cal_maze_distance(input).unwrap());
}

#[test]
fn test_maze_distance2() {
    let input = "7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ";
    assert_eq!(8, cal_maze_distance(input).unwrap());
}

#[test]
fn test_enclosed_tiles() {
    let input = "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";

    assert_eq!(4, cal_enclosed_tiles(input).unwrap());
}

#[test]
fn test_enclosed_tiles2() {
    let input = ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...";

    assert_eq!(8, cal_enclosed_tiles(input).unwrap());
}

#[test]
fn test_enclosed_tiles3() {
    let input = "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";

    assert_eq!(10, cal_enclosed_tiles(input).unwrap());
}


fn get_main_pipe(maze: &Maze) -> HashMap<Position, Tile> {
    let runner =
        enum_iterator::all::<Direction>().map(|dir| MazeRunner::new(maze.clone(), maze.start, dir));

    runner
        .map(|runner| {
            runner
                .map(|(tile, _dir)| (tile.pos, tile))
                .collect::<HashMap<_, _>>()
        })
        .max_by(|a, b| a.len().cmp(&b.len()))
        .unwrap()
}

pub fn cal_maze_distance(input: &str) -> Result<usize> {
    let maze = Maze::from(input);

    let len = get_main_pipe(&maze).len();
    Ok(len / 2)
}

pub fn cal_enclosed_tiles(input: &str) -> Result<usize> {
    let mut maze = Maze::from(input);
    let main_pipe = get_main_pipe(&maze);
    maze.cleanup(&main_pipe);

    todo!()
}
