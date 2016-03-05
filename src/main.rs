use std::fmt;
use std::thread;
use std::time::Duration;

#[derive(Copy,Clone,PartialEq,Debug)]
enum CellState { Dead, Alive }

type CellMap = Vec<Vec<CellState>>;
type Coord = (usize, usize);

struct GameState {
    cycle: i64,
    cells: CellMap
}

struct GameOfLife {
    game_state: GameState
}

impl GameOfLife {
    fn new(game_size: GameSize, initially_alive_cells: Vec<Coord>, origin: Coord) -> GameOfLife {
        GameOfLife { game_state: GameState::new(game_size, initially_alive_cells, origin) }
    }
}

impl Iterator for GameOfLife {
    type Item = GameState;
    
    fn next(&mut self) -> Option<GameState> {
        self.game_state = GameState::next_from(&self.game_state);
        Some(self.game_state.clone())
    }
}

impl Clone for GameState {
    fn clone(&self) -> GameState {
        GameState { cycle: self.cycle, cells: self.cells.clone() }
    }   
}

impl GameState {
    fn new(game_size: GameSize, initially_alive_cells: Vec<Coord>, origin: Coord) -> GameState {
        let GameSize(size) = game_size;
        let cells = vec![vec![CellState::Dead; size]; size];
        let mut new_game = GameState { cycle: 0,  cells: cells };
        for alive_cell in &initially_alive_cells {
            new_game.cells[alive_cell.0 + origin.0][alive_cell.1 + origin.1] = CellState::Alive;
        }
        new_game
    }
    
    fn next_from(from_state: &GameState) -> GameState {
        let mut next_state = GameState { cycle: from_state.cycle + 1, cells: from_state.cells.clone() };
        
        for (r, row) in from_state.cells.iter().enumerate() {
            for (c, cell) in row.iter().enumerate() {
                let alive_neighbour_count = count_neighbours_alive(r, c, &from_state.cells);
                let new_cell_state = match alive_neighbour_count {
                  0...1 | 4...8 => CellState::Dead,
                  3 if *cell == CellState::Dead => CellState::Alive,
                  _ => *cell
                }; 

                next_state.cells[r][c] = new_cell_state;
            }
        }
        
        next_state
    }
}

fn count_neighbours_alive(origin_row: usize, origin_col: usize, cells: &CellMap) -> usize {
    let size = cells.len();
    let valid_indices = get_valid_indices(origin_row, origin_col, size);
    let alive_neighbours = valid_indices.into_iter()
        .map(|(row, col)| cells[row][col])
        .filter(|&state| state == CellState::Alive)
        .collect::<Vec<_>>();
    
    alive_neighbours.len()
}

fn get_valid_indices(origin_row: usize, origin_col: usize, size: usize) -> Vec<Coord> {
    let left = origin_col.checked_sub(1);
    let right = checked_increment(origin_col, size);
    let up = origin_row.checked_sub(1);
    let down = checked_increment(origin_row, size);
    let origin_col = Some(origin_col);
    let origin_row = Some(origin_row);
    let indices = [
        (origin_row, right), (origin_row, left), (up, origin_col), (down, origin_col),
        (up, left), (down, left), (up, right), (down, right)
    ];
    
    indices.iter()
        .filter(|&&(row, col)| row.is_some() && col.is_some())
        .map(|&(row, col)| (row.unwrap(), col.unwrap()))
        .collect::<Vec<_>>()
}

fn checked_increment(index: usize, size: usize) -> Option<usize> {
    if index + 1 == size {
        None
    } else { 
        Some(index + 1)
    }
}

impl fmt::Display for GameState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut printable = String::from(format!("cycle: {}\n", self.cycle));

        for row in self.cells.iter() {
            for cell in row.iter() {
                let cell_icon = match *cell {
                    CellState::Alive => 'o',
                    CellState::Dead => ' ',
                };
                printable.push(cell_icon);
            }
            printable.push('\n');
        }
        write!(f, "{}", printable)
    }
}

struct GameSize(usize);

fn main() {
    let glider = vec![(5,3), (6,4), (6,5), (5,5), (4,5)];
    let blinker = vec![(0,1), (1,1), (2,1)];
    let game_of_life = GameOfLife::new(GameSize(40), glider, (15, 15));
    for iteration in game_of_life {
        thread::sleep(Duration::from_millis(500));
        println!("{}", iteration);
    }
}

#[cfg(test)]
mod maintest;