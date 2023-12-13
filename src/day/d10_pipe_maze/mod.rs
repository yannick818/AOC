use std::collections::HashMap;

use crate::prelude::*;
mod maze;
mod runner;

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

fn get_main_pipe(maze: &mut Maze) -> HashMap<Position, Tile> {
    for dir in enum_iterator::all::<Direction>() {
        let runner = MazeRunner::new(maze, maze.start, dir);
        let (path, is_finished) = runner
            .fold((HashMap::new(), false), |(mut map, _), (tile, _)| {
                map.insert(tile.pos, tile);
                let finished = tile.typ.is_start();

                (map, finished)
            });

        if is_finished {
            return path;
        }
    }
    panic!("No main pipe found");
}

pub fn cal_maze_distance(input: &str) -> Result<usize> {
    let mut maze = Maze::from(input);
    let len = get_main_pipe(&mut maze).len();
    println!("{:#?}", maze);
    Ok(len / 2)
}

pub fn cal_enclosed_tiles(input: &str) -> Result<usize> {
    let mut maze = Maze::from(input);
    let main_pipe = get_main_pipe(&mut maze);
    maze.cleanup(&main_pipe);
    println!("{:#?}", maze);
    let inner_tiles = maze.define_ground();
    Ok(inner_tiles)
}
