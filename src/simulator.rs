use crate::grid::Grid;
use crate::obu::OnBoardUnitManager;
use rand::seq::SliceRandom;
use rand::thread_rng;

pub struct Simulator {
    obu_manager: OnBoardUnitManager,
    grid: Grid,
}

impl Simulator {
    /**
     * Create a new Simulator
     */
    pub fn new(blocks_per_street: usize, block_size: usize) -> Simulator {
        Simulator {
            obu_manager: OnBoardUnitManager::new(),
            grid: Grid::new(blocks_per_street, block_size),
        }
    }

    /**
     * Run the simulation
     */
    pub fn run(&mut self) {
        // update the grid cells
        self.grid.update_next_coordinates();

        for _ in 1..30 {
            self.move_on_board_units();
        }
    }

    /**
     * Add a new OnBoardUnit to the grid.
     */
    pub fn add_on_board_unit(&mut self) {
        // add a new obu to the grid
        match self.grid.insert_obu(self.obu_manager.get_next_id()) {
            Some(coordinate) => {
                self.obu_manager.create_obu(coordinate);
            }
            None => println!("No space available for a new OBU"),
        }
    }

    /**
     * Move an OnBoardUnits.
     */
    pub fn move_on_board_units(&mut self) {
        for obu in self.obu_manager.obus.values_mut() {
            // get the next possible coordinates for the obu
            let possible_moves = self.grid.get_possible_moves(obu.get_coordinate());

            // randomly select a coordinatef
            match possible_moves.choose(&mut thread_rng()) {
                Some(coordinate) => {
                    let current_coordinate = obu.get_coordinate();

                    println!(
                        "obu {} from {},{} to {},{}",
                        obu.get_id(),
                        current_coordinate.x,
                        current_coordinate.y,
                        coordinate.x,
                        coordinate.y
                    );
                    
                    obu.set_coordinate(
                        self.grid.move_obu(obu.get_coordinate(), coordinate.clone()),
                    );
                }
                None => (),
            };
        }
    }
}

/***
 * TESTS MODULE
 */
#[cfg(test)]
mod tests {}
