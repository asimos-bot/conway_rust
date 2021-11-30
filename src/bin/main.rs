use std::env;
use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use std::collections::HashMap;
use std::io::stdin;

#[derive(Debug, PartialEq, Eq, Clone)]
struct Grid {
    x: usize,
    y: usize,
    cells: Vec<Cell>
}

#[derive(Debug, PartialEq, Eq)]
struct Cell {
    state: State,
    neighbors: Vec<usize>,
    next_state: Option<State>
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
enum State {
    Dead,
    Alive,
    Zombie
}

impl Cell {

    fn new(state: State, neighbors: Vec<usize>) -> Cell {

        return Cell {
            state,
            neighbors,
            next_state: None
        };
    }

    fn update(&mut self, neighbors: &Vec<Cell>) -> bool {
        self.next_state(neighbors);
        if let Some(next_state) = self.next_state {
            self.state = next_state;
            true
        } else {
            false
        }
    }

    fn nearby_states(&self, neighbors : &Vec<Cell>) -> HashMap<State, u64> {
        let mut states : HashMap<State, u64> = HashMap::new();
        states.insert(State::Dead, 0);
        states.insert(State::Alive, 0);
        states.insert(State::Zombie, 0);

        for &cell_idx in &self.neighbors {
            states.insert(neighbors[cell_idx].state, states.get(&neighbors[cell_idx].state).unwrap() + 1);
        }
        states
    }

    fn next_state(&mut self, neighbors : &Vec<Cell>) {

        let states = self.nearby_states(neighbors);

        self.next_state = match self.state {
            State::Dead => {
                if *states.get(&State::Alive).unwrap() == 3 as u64 {
                    Some(State::Alive)
                } else {
                    None
                }
            },
            State::Alive => {
                if *states.get(&State::Zombie).unwrap() > 0 as u64 {
                    Some(State::Zombie)
                } else if *states.get(&State::Alive).unwrap() < 2 as u64 {
                    Some(State::Dead)
                } else if *states.get(&State::Alive).unwrap() > 3 as u64 {
                    Some(State::Dead)
                } else {
                    None
                }
            },
            State::Zombie => {
                if *states.get(&State::Alive).unwrap() == 0 as u64 {
                    Some(State::Dead)
                } else {
                    None
                }
            }
        };
    }

    fn to_char(&self, neighbors : &Vec<Cell>) -> String {

        //let states = self.nearby_states(neighbors);
        let string : String =format!("");//A:{},D:{},Z:{}", *states.get(&State::Alive).unwrap(), *states.get(&State::Dead).unwrap(), *states.get(&State::Zombie).unwrap());

        match self.state {
            State::Alive => format!("O{}", string),
            State::Dead => format!("#{}", string),
            State::Zombie => format!("%{}", string)
        }
    }
}

impl Clone for Cell {

    fn clone(&self) -> Cell {
        Cell {
            state: self.state,
            neighbors: self.neighbors.clone(),
            next_state: self.next_state
        }
    }
}

impl Grid {

    fn new(raw_matrix: Vec<Vec<u8>>) -> Grid {

        let mut cells = Vec::<Cell>::new();

        let max_x = raw_matrix[0].len();
        let max_y = raw_matrix.len();

        // iterate over cells
        for y in 0..max_y {

            for x in 0..max_x {

                let mut neighbors : Vec<usize> = Vec::<usize>::new();

                if y > 0 {
                    if x > 0 {
                        neighbors.push( (y-1) * max_x + x - 1);
                    }
                    neighbors.push( (y-1) * max_x + x );
                    if x < max_x - 1 {
                        neighbors.push( (y-1) * max_x + x + 1 );
                    }
                }
                if x < max_x - 1 {
                    neighbors.push( y * max_x + x + 1 );
                }
                if x > 0 {
                    neighbors.push( y * max_x + x - 1);
                }
                if y < max_y - 1 {
                     if x > 0 {
                        neighbors.push( (y+1) * max_x + x - 1);
                    }
                    neighbors.push( (y+1) * max_x + x );
                    if x < max_x - 1 {
                        neighbors.push( (y+1) * max_x + x + 1 );
                    }                   
                }

                let state = match raw_matrix[y][x] as char {
                    '#' => State::Dead,
                    'O' => State::Alive,
                    '%' => State::Zombie,
                    c => panic!("Invalid Symbol: {}", c)
                };

                cells.push(Cell::new(state, neighbors));
            }
        }

        Grid {
            x: raw_matrix[0].len(),
            y: raw_matrix.len(),
            cells,
        }
    }

    fn next_matrix(&mut self) -> Grid {

        let mut next : Grid = self.clone();
        for cell in &mut next.cells {
            cell.update(&self.cells);
        }
        next
    }
}

impl Iterator for Grid {

    type Item = Grid;
    fn next(&mut self) -> Option<Self::Item> {
        let next_matrix : Grid = self.next_matrix();
        if next_matrix != *self {
            self.cells = next_matrix.cells.clone();
            Some(next_matrix)
        } else {
            None
        }
    }
}

fn print_char_vec(v: &Vec<String>) {

    for c in v {
        print!("{}", c);
    }
    print!("\n")
}

fn main() {

    let args: Vec<String> = env::args().collect();

    let filename = if args.len() > 1 {
        &args[1]
    } else {
        panic!("Filename not given");
    };

    let mut f = BufReader::new(
          File::open(filename).expect("couldn't open file")
        );

    let mut lines = Vec::<Vec<u8>>::new();
    let mut buf = Vec::<u8>::new();

    while f.read_until(b'\n', &mut buf).expect("read_until failed") != 0 {

        // remove newline
        if buf[buf.len() - 1] == b'\n' {
            buf.remove(buf.len() - 1);
        }
        lines.push(
            buf.to_vec()
        );

        buf.clear();
    }

    let initial_grid = Grid::new(lines);

    for y in 0..initial_grid.y {

        let mut v = Vec::new();
        for x in 0..initial_grid.x {

            v.push(initial_grid.cells[y * initial_grid.x + x].to_char(&initial_grid.cells));
        }
        //println!("{:?}", v);
        print_char_vec(&v);
    }

    get_input();

    for grid in initial_grid {

        for y in 0..grid.y {

            let mut v = Vec::new();
            for x in 0..grid.x {

                v.push(grid.cells[y * grid.x + x].to_char(&grid.cells));
            }
            //println!("{:?}", v);
            print_char_vec(&v);
        }
        get_input();
    }
}

fn get_input() -> String {

    let mut line = String::new();
    stdin().read_line(&mut line).unwrap();
    line
}
