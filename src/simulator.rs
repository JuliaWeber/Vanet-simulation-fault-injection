use crate::grid::{Coordinate, Grid};
use crate::obu_manager::OnBoardUnitManager;
use crate::rsu_manager::RoadSideUnitManager;
use rand::seq::SliceRandom;
use rand::thread_rng;

pub struct Simulator {
    obu_manager: OnBoardUnitManager,
    rsu_manager: RoadSideUnitManager,
    grid: Grid,
    round: usize,
}

impl Simulator {
    /**
     * Create a new Simulator
     */
    pub fn new(blocks_per_street: usize, block_size: usize, rsu_comms_range: usize) -> Simulator {
        Simulator {
            obu_manager: OnBoardUnitManager::new(),
            rsu_manager: RoadSideUnitManager::new(rsu_comms_range),
            grid: Grid::new(blocks_per_street, block_size),
            round: 0,
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
     * Add RoadSideUnits to the grid.
     */
    pub fn add_road_side_units(&mut self) {
        assert_eq!(
            self.rsu_manager.rsus.len(),
            0,
            "RSUs already added to the grid."
        );

        let comms_range = self.rsu_manager.get_comms_range();
        let mut next_coordinate = Coordinate {
            x: comms_range - 1,
            y: comms_range - 1,
        };
        let mut last_added_id = 0;

        // add rsus to the grid
        while next_coordinate.x < self.grid.get_dimension() {
            while next_coordinate.y < self.grid.get_dimension() {
                last_added_id = self.rsu_manager.create_rsu(next_coordinate.clone());
                next_coordinate.y += (comms_range * 2) - 1;

                // if the next coordinate is out of bounds
                if next_coordinate.y >= self.grid.get_dimension() {
                    let last_added = self.rsu_manager.rsus.get(&last_added_id).unwrap();

                    // check if the last added rsu range do not cover the grid border
                    if last_added.get_coordinate().y + comms_range < self.grid.get_dimension() {
                        next_coordinate.y = self.grid.get_dimension() - 1;
                    }
                }
            }

            next_coordinate.y = comms_range - 1;
            next_coordinate.x += (comms_range * 2) - 1;

            // if the next coordinate is out of bounds
            if next_coordinate.x >= self.grid.get_dimension() {
                let last_added = self.rsu_manager.rsus.get(&last_added_id).unwrap();

                // check if the last added rsu range do not cover the grid border
                if last_added.get_coordinate().x + comms_range < self.grid.get_dimension() {
                    next_coordinate.x = self.grid.get_dimension() - 1;
                }
            }
        }
    }

    /**
     * Run the simulation
     */
    pub fn run(&mut self, steps: usize) {
        // update the grid cells
        self.grid.update_next_coordinates();

        for _ in 0..steps {
            self.round += 1;
            self.move_on_board_units();
        }
    }

    /**
     * Move OnBoardUnits.
     */
    pub fn move_on_board_units(&mut self) {
        for obu in self.obu_manager.obus.values_mut() {
            // get the next possible coordinates for the obu
            let possible_moves = self.grid.get_possible_moves(obu.get_coordinate());

            // randomly select a coordinate
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
mod tests {

    use super::*;

    /**
     * Test the creation of a Simulator.
     */
    #[test]
    fn test_create_simulator() {
        let simulator = Simulator::new(3, 2, 5);

        assert_eq!(simulator.obu_manager.obus.len(), 0);
        assert_eq!(simulator.rsu_manager.rsus.len(), 0);
        assert_eq!(simulator.grid.get_dimension(), 10);
        assert_eq!(simulator.round, 0);
    }

    /**
     * Test RSU addition to the grid.
     */
    #[test]
    fn test_add_road_side_units() {
        let mut simulator = Simulator::new(3, 2, 3);

        simulator.add_road_side_units();

        assert_eq!(simulator.rsu_manager.rsus.len(), 4);

        let rsu = simulator.rsu_manager.rsus.get(&0).unwrap();
        assert_eq!(rsu.get_coordinate().x, 2);
        assert_eq!(rsu.get_coordinate().y, 2);

        let rsu = simulator.rsu_manager.rsus.get(&1).unwrap();
        assert_eq!(rsu.get_coordinate().x, 2);
        assert_eq!(rsu.get_coordinate().y, 7);

        let rsu = simulator.rsu_manager.rsus.get(&2).unwrap();
        assert_eq!(rsu.get_coordinate().x, 7);
        assert_eq!(rsu.get_coordinate().y, 2);

        let rsu = simulator.rsu_manager.rsus.get(&3).unwrap();
        assert_eq!(rsu.get_coordinate().x, 7);
        assert_eq!(rsu.get_coordinate().y, 7);
    }

    /**
     * Test RSU addition to the grid (2).
     */
    #[test]
    fn test_add_road_side_units_2() {
        let mut simulator = Simulator::new(3, 2, 4);

        simulator.add_road_side_units();

        assert_eq!(simulator.rsu_manager.rsus.len(), 4);

        let rsu = simulator.rsu_manager.rsus.get(&0).unwrap();
        assert_eq!(rsu.get_coordinate().x, 3);
        assert_eq!(rsu.get_coordinate().y, 3);

        let rsu = simulator.rsu_manager.rsus.get(&1).unwrap();
        assert_eq!(rsu.get_coordinate().x, 3);
        assert_eq!(rsu.get_coordinate().y, 9);

        let rsu = simulator.rsu_manager.rsus.get(&2).unwrap();
        assert_eq!(rsu.get_coordinate().x, 9);
        assert_eq!(rsu.get_coordinate().y, 3);

        let rsu = simulator.rsu_manager.rsus.get(&3).unwrap();
        assert_eq!(rsu.get_coordinate().x, 9);
        assert_eq!(rsu.get_coordinate().y, 9);
    }

    /**
     * Test RSU addition to the grid (3).
     */
    #[test]
    fn test_add_road_side_units_3() {
        let mut simulator = Simulator::new(3, 2,6);

        simulator.add_road_side_units();

        assert_eq!(simulator.rsu_manager.rsus.len(), 1);

        let rsu = simulator.rsu_manager.rsus.get(&0).unwrap();
        assert_eq!(rsu.get_coordinate().x, 5);
        assert_eq!(rsu.get_coordinate().y, 5);
    }
}
