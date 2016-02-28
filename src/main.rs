use std::fmt;

#[derive(Copy,Clone,PartialEq)]
enum CellState { Dead, Alive }

type CellMap = Vec<Vec<CellState>>;

struct GameState {
    cycle: i64,
    cells: CellMap
}

struct GameOfLife {
    game_state: GameState
}

impl GameOfLife {
    fn new(game_size: GameSize, initially_alive_cells: Vec<(usize, usize)>) -> GameOfLife {
        GameOfLife { game_state: GameState::new(game_size, initially_alive_cells) }
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
    fn new(game_size: GameSize, initially_alive_cells: Vec<(usize, usize)>) -> GameState {
        let GameSize(size) = game_size;
        let cells = vec![vec![CellState::Dead; size]; size];
        let mut new_game = GameState { cycle: 0,  cells: cells };
        for alive_cell in &initially_alive_cells {
            new_game.cells[alive_cell.0][alive_cell.1] = CellState::Alive;
        }
        new_game
    }
    
    fn next_from(from_state: &GameState) -> GameState {
        let mut next_state = GameState { cycle: from_state.cycle + 1, cells: from_state.cells.clone() };
        
        for (r, row) in from_state.cells.iter().enumerate() {
            for (c, cell) in row.iter().enumerate() {
                let alive_neighbour_count = count_neighbours_alive(r, c, &next_state.cells);
                
                let new_cell_state = match alive_neighbour_count {
                  0...1 | 4...8 => CellState::Dead,
                //   2 | 3 if *cell == CellState::Alive => *cell,
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
        .map(|i| cells[i.0][i.1])
        .filter(|state| *state == CellState::Alive)
        .collect::<Vec<CellState>>();
    
    alive_neighbours.len()
}

fn get_valid_indices(origin_row: usize, origin_col: usize, size: usize) -> Vec<(usize, usize)> {
    let left = origin_col.checked_sub(1);
    let right = checked_increment(origin_col, size);
    let up = origin_row.checked_sub(1);
    let down = checked_increment(origin_row, size);
    let origin_col = Some(origin_col);
    let origin_row = Some(origin_row);
    let indices = [
        (origin_row, right), (origin_row, left), (origin_col, up), (origin_col, down),
        (up, left), (down, left), (up, right), (down, right)
    ];
    
    indices.iter()
        .filter(|t| !t.0.is_none() && !t.1.is_none())
        .map(|t| (t.0.unwrap(), t.1.unwrap()))
        .collect::<Vec<(usize, usize)>>()
}

fn checked_increment(index: usize, size: usize) -> Option<usize> {
    if index + 1 == size {
        return None
    }    

    Some(index + 1)
}

impl fmt::Display for GameState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut printable = String::from(format!("cycle: {}\n", self.cycle));

        for row in self.cells.iter() {
            for cell in row.iter() {
                let cell_icon = match *cell {
                    CellState::Alive => 'x',
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
    let initially_alive_cells = vec![(3,5), (4,6), (5,6), (5,5), (5,4)];
    let game_of_life = GameOfLife::new(GameSize(30), initially_alive_cells);
    for iteration in game_of_life {
        println!("{}", iteration);
    }
}
