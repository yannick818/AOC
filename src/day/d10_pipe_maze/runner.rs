use crate::d10_pipe_maze::maze::*;

pub struct MazeRunner<'a> { maze: &'a Maze,
    pos: Tile,
    walking_dir: Direction,
    started: bool,
}

impl<'a> MazeRunner<'a> {
    pub fn new(maze: &'a Maze, start: Tile, direction: Direction) -> Self {
        Self {
            maze,
            pos: start,
            walking_dir: direction,
            started: false,
        }
    }
}

impl Iterator for MazeRunner<'_> {
    type Item = (Tile, Direction);

    fn next(&mut self) -> Option<(Tile, Direction)> {
        // println!("pos: {:?} ({:?})", self.pos, self.walking_dir);

        if self.started && self.pos.typ == TileType::Start {
            return None;
        }
        self.started = true;

        let new_step = self.pos.walk(&self.walking_dir).map(|(new_dir, new_pos)| {
            let new_tile = self.maze.maze.get(&new_pos);
            (new_tile, new_dir)
        });

        match new_step {
            Ok((Some(new_tile), new_dir)) => {
                self.pos = *new_tile;
                self.walking_dir = new_dir;
                Some((*new_tile, new_dir))
            }
            //walked out of maze
            Ok((None, _)) => None,
            //hit obstacle
            Err(_) => None,
        }
    }
}

struct LineRunner<'a> {
    maze: &'a mut Maze,
    pos: Tile,
    walking_dir: Direction,
    started: bool,
}
