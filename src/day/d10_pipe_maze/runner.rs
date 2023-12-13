use crate::d10_pipe_maze::maze::*;

pub struct MazeRunner<'a> { 
    maze: &'a mut Maze,
    pos: Tile,
    starting_dir: Direction,
    walking_dir: Direction,
    started: bool,
}

impl<'a> MazeRunner<'a> {
    pub fn new(maze: &'a mut Maze, start: Tile, direction: Direction) -> Self {
        Self {
            maze,
            pos: start,
            starting_dir: direction,
            walking_dir: direction,
            started: false,
        }
    }

    // TODO move to maze
    fn cal_start_pipe(&self) -> Pipe {
        // HACK do this with notoperator somehow
        match (self.starting_dir, self.walking_dir) {
            (Direction::North, Direction::East) => Pipe::NorthWest,
            (Direction::North, Direction::South) => Pipe::Vertical,
            (Direction::North, Direction::West) => Pipe::NorthEast,

            (Direction::East, Direction::North) => Pipe::SouthEast,
            (Direction::East, Direction::South) => Pipe::NorthEast,
            (Direction::East, Direction::West) => Pipe::Horizontal,

            (Direction::South, Direction::North) => Pipe::Vertical,
            (Direction::South, Direction::East) => Pipe::SouthWest,
            (Direction::South, Direction::West) => Pipe::SouthEast,

            (Direction::West, Direction::North) => Pipe::SouthWest,
            (Direction::West, Direction::East) => Pipe::Horizontal,
            (Direction::West, Direction::South) => Pipe::NorthWest,
        
            _ => panic!("Invalid start pipe"),
        }
    }
}

impl Iterator for MazeRunner<'_> {
    type Item = (Tile, Direction);

    fn next(&mut self) -> Option<(Tile, Direction)> {
        // println!("pos: {:?} ({:?})", self.pos, self.walking_dir);

        if self.started && self.pos.typ.is_start() {

            let start_pipe = self.cal_start_pipe();
            let start_type = TileType::Start(Some(start_pipe));

            self.maze.start.typ = start_type;
            self.maze.get_tile(&self.pos.pos).unwrap().typ = start_type;

            return None;
        }
        self.started = true;

        let new_step = self.pos.walk(&self.walking_dir).map(|(new_dir, new_pos)| {
            let new_tile = self.maze.get_tile(&new_pos);
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

