/**
 * GRID
 *
 * The grid is a 2d array of cells, each cell is a position on the simulated
 * environment. The grid is used to store the state of the environment.
 */

#[derive(Clone, Copy, Debug)]
pub struct Coordinate {
    pub x: usize,
    pub y: usize,
}

#[derive(Debug, PartialEq)]
enum FlowDirection {
    FromZero, // left -> right / top -> bottom
    ToZero,   // right -> left / bottom -> top
}

#[derive(Clone, Debug)]
pub struct CellState {
    pub id: usize,                     // unique id
    is_street: bool,                   // Is this cell a street?
    pub obu_id: Option<u32>,           // If this cell is occupied by an OBU, what is its ID?
    next_coordinates: Vec<Coordinate>, // If this cell is a street, what are the next possible coordinates?
}

pub struct Grid {
    block_size: usize,
    cells: Vec<Vec<CellState>>, // The grid itself
    dimension: usize,           // The dimension of the grid
}

impl Grid {
    /**
     * Create a new grid with the given number of blocks per street and the size of
     * each block. The blocks and the grid are always a square. And the blocks have
     * streets on all sides.
     */
    pub fn new(blocks_per_street: usize, block_size: usize) -> Grid {
        // calculate grid dimension considering the blocks and the streets
        let dimension = blocks_per_street * block_size + blocks_per_street + 1;

        // counter for cell unique id
        let mut id = 0;

        // create the grid vector of cells, marking the streets
        let mut cells = Vec::new();
        for i in 0..dimension {
            let mut row = Vec::new();
            for j in 0..dimension {
                let is_street = i % (block_size + 1) == 0 || j % (block_size + 1) == 0;
                row.push(CellState {
                    id,
                    is_street,
                    obu_id: None,
                    next_coordinates: Vec::new(),
                });
                // increment the id counter
                id += 1;
            }
            cells.push(row);
        }

        Grid {
            block_size,
            cells,
            dimension,
        }        
    }

    /**
     * Check if the given row or column is a street.
     */
    fn is_street(&self, i: usize) -> bool {
        i % (self.block_size + 1) == 0
    }

    /**
     * Get flow direction for a given street id.
     */
    fn get_flow_direction(&self, street_id: usize, is_column: bool) -> FlowDirection {
        if street_id % 2 == 0 {
            if is_column {
                FlowDirection::ToZero
            } else {
                FlowDirection::FromZero
            }
        } else {
            if is_column {
                FlowDirection::FromZero
            } else {
                FlowDirection::ToZero
            }
        }
    }

    /**
     * Get the next possible coordinates from a given coordinate.
     */
    pub fn get_next_coordinates(&mut self, coordinate: Coordinate) -> Vec<Coordinate> {
        // check if the next coordinates were already calculated for all cells
        if self.cells[0][0].next_coordinates.len() == 0 {
            self.update_next_coordinates();
        }

        // get the next coordinates from the cell state
        self.cells[coordinate.x][coordinate.y]
            .next_coordinates
            .clone()
    }

    /**
     * Calculate the next possible coordinates from a given coordinate.
     * Do not call this method directly, use grid.get_next_coordinates() instead.
     */
    fn calculate_next_coordinates(&self, coordinate: Coordinate) -> Vec<Coordinate> {
        let mut next_coordinates = Vec::new();

        // check if column is a street
        if self.is_street(coordinate.x) {
            match self.get_flow_direction(coordinate.x, true) {
                FlowDirection::FromZero => {
                    // check if there is at least one cell ahead
                    if coordinate.y < self.dimension - 1 {
                        next_coordinates.push(Coordinate {
                            x: coordinate.x,
                            y: coordinate.y + 1,
                        });
                    }
                }
                FlowDirection::ToZero => {
                    // check if there is at least one cell ahead
                    if coordinate.y > 0 {
                        next_coordinates.push(Coordinate {
                            x: coordinate.x,
                            y: coordinate.y - 1,
                        });
                    }
                }
            }
        }

        // check if row is a street
        if self.is_street(coordinate.y) {
            match self.get_flow_direction(coordinate.y, false) {
                FlowDirection::FromZero => {
                    // check if there is at least one cell ahead
                    if coordinate.x < self.dimension - 1 {
                        next_coordinates.push(Coordinate {
                            x: coordinate.x + 1,
                            y: coordinate.y,
                        });
                    }
                }
                FlowDirection::ToZero => {
                    // check if there is at least one cell ahead
                    if coordinate.x > 0 {
                        next_coordinates.push(Coordinate {
                            x: coordinate.x - 1,
                            y: coordinate.y,
                        });
                    }
                }
            }
        }

        next_coordinates
    }

    /**
     * Update the next coordinates of each cell in the grid.
     */
    pub fn update_next_coordinates(&mut self) {
        // iterate over the grid
        for i in 0..self.dimension {
            for j in 0..self.dimension {
                // get the current cell
                let cell = &self.cells[i][j];

                // if the cell is a street, update its next coordinates
                if cell.is_street {
                    // get the current cell coordinates
                    let current_coordinates = Coordinate { x: i, y: j };

                    // get the next coordinates
                    let next_coordinates = self.calculate_next_coordinates(current_coordinates);

                    // update the next coordinates of the current cell
                    self.cells[i][j].next_coordinates = next_coordinates;
                }
            }
        }
    }

    /**
     * Get the cell state for a given coordinate.
     */
    pub fn get_cell_state(&self, coordinate: Coordinate) -> CellState {
        self.cells[coordinate.x][coordinate.y].clone()
    }

    /**
     * Get possible moves for a given coordinate.
     */
    pub fn get_possible_moves(&mut self, coordinate: Coordinate) -> Vec<Coordinate> {
        // get the next coordinates
        let possible_moves = self.get_next_coordinates(coordinate);

        // filter the next coordinates to get only the possible moves
        possible_moves
            .into_iter()
            .filter(|next_coordinate| {
                // get the next cell state
                let next_cell_state = self.get_cell_state(next_coordinate.clone());

                // check if the next cell is not occupied
                next_cell_state.obu_id.is_none()
            })
            .collect()
    }

    /**
     * Move obu_id from the current coordinate to the next coordinate.
     */
    pub fn move_obu(&mut self, current_coordinate: Coordinate, next_coordinate: Coordinate) -> Coordinate {
        // get the current cell state
        let mut current_cell_state = self.get_cell_state(current_coordinate);

        // get the next cell state
        let mut next_cell_state = self.get_cell_state(next_coordinate);

        // move the obu_id from the current cell to the next cell
        next_cell_state.obu_id = current_cell_state.obu_id;
        current_cell_state.obu_id = None;

        // update the current cell state
        self.cells[current_coordinate.x][current_coordinate.y] = current_cell_state;

        // update the next cell state
        self.cells[next_coordinate.x][next_coordinate.y] = next_cell_state;

        next_coordinate.clone()
    }

    /**
     * Print the grid to the console, showing the cells' ids and the streets.
     */
    pub fn print_grid_with_cells_ids(&self) {
        for i in 0..self.dimension {
            for j in 0..self.dimension {
                let cell = &self.cells[i as usize][j as usize];
                if cell.is_street {
                    print!("|");
                } else {
                    print!(" ");
                }
                print!("{:02}", cell.id);
            }
            println!();
        }
    }

    /**
     * Print the grid to the console, showing the cells' coordinates and the id.
     */
    pub fn print_grid_with_cells_coordinates(&self) {
        for i in 0..self.dimension {
            for j in 0..self.dimension {
                let cell = &self.cells[i as usize][j as usize];
                if cell.is_street {
                    print!("|");
                } else {
                    print!(" ");
                }
                print!("({:02},{:02})", i, j);
            }
            println!();
        }
    }

    /**
     * Print next possible coordinates for a cell
     */
    pub fn print_next_coordinates(&self, coordinate: Coordinate) {
        let cell = &self.cells[coordinate.x][coordinate.y];
        println!("Cell id: {}", cell.id);
        println!("Next coordinates:");
        let next_coordinates = self.calculate_next_coordinates(coordinate);
        for coordinate in next_coordinates {
            println!("({},{})", coordinate.x, coordinate.y);
        }
    }

    /**
     * Insert a new obu in the grid. Choose first street with first cell available,
     * considering the flow direction.
     */
    pub fn insert_obu(&mut self, obu_id: u32) -> Option<Coordinate> {
        let mut street_id = 0;

        // iterate over the streets
        while street_id < self.dimension {
            // if this id is a street (row/column)
            if self.is_street(street_id) {
                // check column first
                let start = match self.get_flow_direction(street_id, true) {
                    FlowDirection::FromZero => 0,
                    FlowDirection::ToZero => self.dimension - 1,
                };

                // check if cell is empty
                if self.cells[street_id][start].obu_id.is_none() {
                    // insert obu
                    self.cells[street_id][start].obu_id = Some(obu_id);
                    return Some(Coordinate {
                        x: street_id,
                        y: start,
                    });
                }

                // then check row
                let start = match self.get_flow_direction(street_id, false) {
                    FlowDirection::FromZero => 0,
                    FlowDirection::ToZero => self.dimension - 1,
                };

                // check if cell is empty
                if self.cells[start][street_id].obu_id.is_none() {
                    // insert obu
                    self.cells[start][street_id].obu_id = Some(obu_id);
                    return Some(Coordinate {
                        x: start,
                        y: street_id,
                    });
                }
            }

            street_id += self.block_size + 1;
        }

        None
    }
}

/***
 * TESTS MODULE
 */
#[cfg(test)]
mod tests {
    use crate::grid::{Coordinate, FlowDirection, Grid};
    use crate::obu::OnBoardUnit;

    /**
     * Test grid creation
     */
    #[test]
    fn test_grid_creation() {
        let grid = Grid::new(3, 2);

        // check grid dimension
        assert_eq!(grid.dimension, 10);

        // check all the cells of the first street
        for i in 0..10 {
            assert_eq!(grid.cells[0][i].is_street, true);
        }

        // check all the cells of the first block
        for i in 1..=2 {
            for j in 1..=2 {
                assert_eq!(grid.cells[i][j].is_street, false);
            }
        }

        // check all the cells of the last block
        for i in 7..=8 {
            for j in 7..=8 {
                assert_eq!(grid.cells[i][j].is_street, false);
            }
        }

        // check all the cells of the last vertical street
        for i in 1..=8 {
            assert_eq!(grid.cells[i][9].is_street, true);
        }
    }

    /**
     * Test street identification
     */
    #[test]
    fn test_street_identification() {
        let grid = Grid::new(3, 2);

        // check if row/column 0 is a street
        assert_eq!(grid.is_street(0), true);

        // check if row/column 1 is a street
        assert_eq!(grid.is_street(1), false);

        // check if row/column 2 is a street
        assert_eq!(grid.is_street(2), false);

        // check if row/column 3 is a street
        assert_eq!(grid.is_street(3), true);
    }

    /**
     * Test flow direction
     */
    #[test]
    fn test_flow_direction() {
        let grid = Grid::new(3, 2);

        assert_eq!(grid.get_flow_direction(0, false), FlowDirection::FromZero);
        assert_eq!(grid.get_flow_direction(0, true), FlowDirection::ToZero);

        assert_eq!(grid.get_flow_direction(3, false), FlowDirection::ToZero);
        assert_eq!(grid.get_flow_direction(3, true), FlowDirection::FromZero);

        assert_eq!(grid.get_flow_direction(6, false), FlowDirection::FromZero);
        assert_eq!(grid.get_flow_direction(6, true), FlowDirection::ToZero);

        assert_eq!(grid.get_flow_direction(9, false), FlowDirection::ToZero);
        assert_eq!(grid.get_flow_direction(9, true), FlowDirection::FromZero);
    }

    /**
     * Test next coordinates calculation
     */
    #[test]
    fn test_next_coordinates() {
        let grid = Grid::new(3, 2);

        // from 0,0
        let next_coordinates = grid.calculate_next_coordinates(Coordinate { x: 0, y: 0 });
        assert_eq!(next_coordinates.len(), 1);
        assert_eq!(next_coordinates[0].x, 1);
        assert_eq!(next_coordinates[0].y, 0);

        // from 3,6
        let next_coordinates = grid.calculate_next_coordinates(Coordinate { x: 3, y: 6 });
        assert_eq!(next_coordinates.len(), 2);
        assert_eq!(next_coordinates[0].x, 3);
        assert_eq!(next_coordinates[0].y, 7);
        assert_eq!(next_coordinates[1].x, 4);
        assert_eq!(next_coordinates[1].y, 6);

        // from 6,9
        let next_coordinates = grid.calculate_next_coordinates(Coordinate { x: 6, y: 9 });
        assert_eq!(next_coordinates.len(), 2);
        assert_eq!(next_coordinates[0].x, 6);
        assert_eq!(next_coordinates[0].y, 8);
        assert_eq!(next_coordinates[1].x, 5);
        assert_eq!(next_coordinates[1].y, 9);

        // from 3,0
        let next_coordinates = grid.calculate_next_coordinates(Coordinate { x: 3, y: 0 });
        assert_eq!(next_coordinates.len(), 2);
        assert_eq!(next_coordinates[0].x, 3);
        assert_eq!(next_coordinates[0].y, 1);
        assert_eq!(next_coordinates[1].x, 4);
        assert_eq!(next_coordinates[1].y, 0);
    }

    /**
     * Test stored next coordinates
     */
    #[test]
    fn test_stored_next_coordinates() {
        let mut grid = Grid::new(3, 2);

        grid.update_next_coordinates();

        // from 0,0
        let next_coordinates = &grid.cells[0][0].next_coordinates;
        assert_eq!(next_coordinates.len(), 1);
        assert_eq!(next_coordinates[0].x, 1);
        assert_eq!(next_coordinates[0].y, 0);

        // from 3,6
        let next_coordinates = &grid.cells[3][6].next_coordinates;
        assert_eq!(next_coordinates.len(), 2);
        assert_eq!(next_coordinates[0].x, 3);
        assert_eq!(next_coordinates[0].y, 7);
        assert_eq!(next_coordinates[1].x, 4);
        assert_eq!(next_coordinates[1].y, 6);

        // from 6,9
        let next_coordinates = &grid.cells[6][9].next_coordinates;
        assert_eq!(next_coordinates.len(), 2);
        assert_eq!(next_coordinates[0].x, 6);
        assert_eq!(next_coordinates[0].y, 8);
        assert_eq!(next_coordinates[1].x, 5);
        assert_eq!(next_coordinates[1].y, 9);

        // from 3,0
        let next_coordinates = &grid.cells[3][0].next_coordinates;
        assert_eq!(next_coordinates.len(), 2);
        assert_eq!(next_coordinates[0].x, 3);
        assert_eq!(next_coordinates[0].y, 1);
        assert_eq!(next_coordinates[1].x, 4);
        assert_eq!(next_coordinates[1].y, 0);
    }

    /**
     * Test get next coordinates
     */
    #[test]
    fn test_get_next_coordinates() {
        let mut grid = Grid::new(3, 2);

        // from 0,0
        let next_coordinates = grid.get_next_coordinates(Coordinate { x: 0, y: 0 });
        assert_eq!(next_coordinates.len(), 1);
        assert_eq!(next_coordinates[0].x, 1);
        assert_eq!(next_coordinates[0].y, 0);

        // from 3,6
        let next_coordinates = grid.get_next_coordinates(Coordinate { x: 3, y: 6 });
        assert_eq!(next_coordinates.len(), 2);
        assert_eq!(next_coordinates[0].x, 3);
        assert_eq!(next_coordinates[0].y, 7);
        assert_eq!(next_coordinates[1].x, 4);
        assert_eq!(next_coordinates[1].y, 6);

        // from 6,9
        let next_coordinates = grid.get_next_coordinates(Coordinate { x: 6, y: 9 });
        assert_eq!(next_coordinates.len(), 2);
        assert_eq!(next_coordinates[0].x, 6);
        assert_eq!(next_coordinates[0].y, 8);
        assert_eq!(next_coordinates[1].x, 5);
        assert_eq!(next_coordinates[1].y, 9);

        // from 3,0
        let next_coordinates = grid.get_next_coordinates(Coordinate { x: 3, y: 0 });
        assert_eq!(next_coordinates.len(), 2);
        assert_eq!(next_coordinates[0].x, 3);
        assert_eq!(next_coordinates[0].y, 1);
        assert_eq!(next_coordinates[1].x, 4);
        assert_eq!(next_coordinates[1].y, 0);
    }

    /**
     * Test obu insertion
     */
    #[test]
    fn test_obu_insertion() {
        let mut grid = Grid::new(3, 2);

        let mut obus = Vec::new();
        let mut obu_id = 0;

        // insert first 7 OnBoardUnits
        for _i in 0..8 {
            match grid.insert_obu(obu_id) {
                Some(coordinate) => {
                    obus.push(OnBoardUnit::new(obu_id, coordinate));
                    obu_id += 1;
                }
                None => {
                    panic!("No more space available");
                }
            }
        }

        // check the coordinates of the first obu
        let coordinate = obus[0].get_coordinate();
        assert_eq!(coordinate.x, 0);
        assert_eq!(coordinate.y, 9);

        // check the coordinates of the second obu
        let coordinate = obus[1].get_coordinate();
        assert_eq!(coordinate.x, 0);
        assert_eq!(coordinate.y, 0);

        // check the coordinates of the third obu
        let coordinate = obus[2].get_coordinate();
        assert_eq!(coordinate.x, 3);
        assert_eq!(coordinate.y, 0);

        // check the coordinates of the fourth obu
        let coordinate = obus[3].get_coordinate();
        assert_eq!(coordinate.x, 9);
        assert_eq!(coordinate.y, 3);

        // check the coordinates of the fifth obu
        let coordinate = obus[4].get_coordinate();
        assert_eq!(coordinate.x, 6);
        assert_eq!(coordinate.y, 9);

        // check the coordinates of the sixth obu
        let coordinate = obus[5].get_coordinate();
        assert_eq!(coordinate.x, 0);
        assert_eq!(coordinate.y, 6);

        // check the coordinates of the seventh obu
        let coordinate = obus[6].get_coordinate();
        assert_eq!(coordinate.x, 9);
        assert_eq!(coordinate.y, 0);

        // check the coordinates of the eighth obu
        let coordinate = obus[7].get_coordinate();
        assert_eq!(coordinate.x, 9);
        assert_eq!(coordinate.y, 9);
    }

    /**
     * Test get possible moves
     */
    #[test]
    fn test_get_possible_moves() {
        let mut grid = Grid::new(3, 2);

        let possible_moves = grid.get_possible_moves(Coordinate { x: 3, y: 0 });
        assert_eq!(possible_moves.len(), 2);
        assert_eq!(possible_moves[0].x, 3);
        assert_eq!(possible_moves[0].y, 1);
        assert_eq!(possible_moves[1].x, 4);
        assert_eq!(possible_moves[1].y, 0);

        grid.cells[3][1].obu_id = Some(0);
        let possible_moves = grid.get_possible_moves(Coordinate { x: 3, y: 0 });
        assert_eq!(possible_moves.len(), 1);
        assert_eq!(possible_moves[0].x, 4);
        assert_eq!(possible_moves[0].y, 0);

        grid.cells[1][0].obu_id = Some(1);
        let possible_moves = grid.get_possible_moves(Coordinate { x: 0, y: 0 });
        assert_eq!(possible_moves.len(), 0);
    }
}
