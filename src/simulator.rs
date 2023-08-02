use crate::comms::Ether;
use crate::grid::{Coordinate, Grid, GridParams};
use crate::obu_manager::{ObuManagerParams, OnBoardUnitManager};
use crate::rsu_manager::{RoadSideUnitManager, RsuManagerParams};
use rand::distributions::{Distribution, Uniform};
use rand::seq::SliceRandom;
use rand::thread_rng;

#[derive(Clone, Debug)]
pub enum NodeType {
    OBU,
    RSU,
}

pub struct Simulator {
    obu_manager: OnBoardUnitManager,
    rsu_manager: RoadSideUnitManager,
    grid: Grid,
    round: u32,
    ether: Ether,
}

impl Simulator {
    /**
     * Create a new Simulator
     */
    pub fn new(
        grid_params: GridParams,
        rsu_manager_params: RsuManagerParams,
        obu_manager_params: ObuManagerParams,
    ) -> Simulator {
        let grid = Grid::new(grid_params);
        let obu_manager = OnBoardUnitManager::new(obu_manager_params, grid.get_dimension());
        let rsu_manager = RoadSideUnitManager::new(rsu_manager_params);

        Simulator {
            obu_manager,
            rsu_manager,
            grid,
            round: 0,
            ether: Ether::new(),
        }
    }

    /**
     * Add a new OnBoardUnit to the grid.
     */
    pub fn add_on_board_unit(&mut self) -> Option<u32> {
        // try to find an empty entry in the grid
        match self.grid.insert_obu(self.obu_manager.get_next_id()) {
            // if an empty entry was found, try to create a new OnBoardUnit
            Some(coordinate) => self.obu_manager.create_obu(coordinate),
            None => None,
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

        let rx_range: u32 = self.rsu_manager.get_rx_range();
        let mut next_coordinate = Coordinate {
            x: rx_range - 1,
            y: rx_range - 1,
        };
        let mut last_added_id = 0;

        // add rsus to the grid
        while next_coordinate.x < self.grid.get_dimension() {
            while next_coordinate.y < self.grid.get_dimension() {
                last_added_id = self.rsu_manager.create_rsu(next_coordinate.clone());
                next_coordinate.y += (rx_range * 2) - 1;

                // if the next coordinate is out of bounds
                if next_coordinate.y >= self.grid.get_dimension() {
                    let last_added = self.rsu_manager.rsus.get(&last_added_id).unwrap();

                    // check if the last added rsu range do not cover the grid border
                    if last_added.get_coordinate().y + rx_range < self.grid.get_dimension() {
                        next_coordinate.y = self.grid.get_dimension() - 1;
                    }
                }
            }

            next_coordinate.y = rx_range - 1;
            next_coordinate.x += (rx_range * 2) - 1;

            // if the next coordinate is out of bounds
            if next_coordinate.x >= self.grid.get_dimension() {
                let last_added = self.rsu_manager.rsus.get(&last_added_id).unwrap();

                // check if the last added rsu range do not cover the grid border
                if last_added.get_coordinate().x + rx_range < self.grid.get_dimension() {
                    next_coordinate.x = self.grid.get_dimension() - 1;
                }
            }
        }
    }

    /**
     * Initialize the simulation
     */
    pub fn init(&mut self) {
        self.grid.update_next_coordinates();
        self.add_road_side_units();

        // set the current round for the managers
        self.rsu_manager.set_current_round(0);
        self.obu_manager.set_current_round(0);

        println!("--- SIMULATION INITIALIZED ---");
        println!("Number of RSUs: {}", self.rsu_manager.rsus.len());
        println!("Number of OBUs: {}", self.obu_manager.get_max_obus());

        self.grid.print_stats(Some(self.obu_manager.get_max_obus()));
    }

    /**
     * Run the simulation
     */
    pub fn run(&mut self, rounds: usize) {
        println!("--- SIMULATION RUNNING ---");

        // Collect messages for the first round
        if self.round == 0 {
            self.collect_messages();
        }

        // Run the simulation for the given number of rounds
        for _ in 0..rounds {
            // Deliver messages from the previous round
            self.deliver_messages();

            // Move obus
            self.do_obus_moves();

            // Add new obus if needed and possible
            let mut added_obus = 0;
            while self.obu_manager.obus.len() < self.obu_manager.get_max_obus() as usize {
                match self.add_on_board_unit() {
                    Some(_) => added_obus += 1,
                    None => break,
                }
            }

            // Print the number of added obus
            if added_obus > 0 {
                println!("Added {} new OBUs in round {}.", added_obus, self.round);
            }

            // Update the round
            self.round += 1;

            // Update the current round for the managers
            self.rsu_manager.set_current_round(self.round);
            self.obu_manager.set_current_round(self.round);

            // Collect messages for the next round delivery
            self.collect_messages();
        }

        println!("--- SIMULATION FINISHED ---");
        self.obu_manager.print_stats();

        let mut true_positive = 0;
        let mut false_positive = 0;
        let mut true_negative = 0;
        let mut false_negative = 0;

        // Get the faulty obus ids from the rsu manager
        let rsu_faulty_obs = self.rsu_manager.find_faulty_obus();

        // Check RSU predictions
        for obu in self.obu_manager.obus.values() {
            let is_faulty = obu.is_faulty();
            let is_in_rsu_faulty = rsu_faulty_obs.contains(&obu.get_id());

            match (is_faulty, is_in_rsu_faulty) {
                (true, true) => true_positive += 1,
                (true, false) => false_negative += 1,
                (false, true) => false_positive += 1,
                (false, false) => true_negative += 1,
            }
        }

        // Calculate the total
        let total = (true_positive + true_negative + false_positive + false_negative) as f32;
        let total_positive = (false_positive + true_negative) as f32;
        let total_negative = (false_negative + true_positive) as f32;

        // Calculate the detection rate
        let detection_rate = (true_positive + true_negative) as f32 / total;

        // Calculate the false positive rate
        let false_positive_rate = false_positive as f32 / total_positive;

        // Calculate the false negative rate
        let false_negative_rate = false_negative as f32 / total_negative;

        println!("--- FINAL STATS ---");
        println!("True Positive: {}", true_positive);
        println!("False Positive: {}", false_positive);
        println!("True Negative: {}", true_negative);
        println!("False Negative: {}", false_negative);
        println!("Detection Rate: {}", detection_rate);
        println!("False Positive Rate: {}", false_positive_rate);
        println!("False Negative Rate: {}", false_negative_rate);
    }

    /**
     * Move OnBoardUnits.
     */
    fn do_obus_moves(&mut self) {
        for obu in self.obu_manager.obus.values_mut() {
            // get the next possible coordinates for the obu
            let possible_moves = self.grid.get_possible_moves(obu.get_coordinate());

            // randomly select a coordinate
            match possible_moves.choose(&mut thread_rng()) {
                Some(coordinate) => {
                    obu.set_coordinate(
                        self.grid.move_obu(obu.get_coordinate(), coordinate.clone()),
                    );
                }
                None => (),
            };
        }
    }

    /**
     * Collect messages from OBUs and RSUs and send them to the Ether.
     */
    fn collect_messages(&mut self) {
        // Clear the ether
        self.ether.clear();

        // Collect messages from OBUs
        let messages = self.obu_manager.collect_messages();
        for mut message in messages {
            // Calculate the physical area of the message
            message.phy_area = self
                .grid
                .get_square_coords(message.phy_coord, message.phy_range);

            // Send the message to the ether
            self.ether.send_message(message);
        }

        // Collect messages from RSUs
        let comms_range = self.rsu_manager.get_tx_range();
        for rsu in self.rsu_manager.rsus.values() {
            match rsu.get_message() {
                Some(mut message) => {
                    message.phy_area = self.grid.get_square_coords(message.phy_coord, comms_range);

                    self.ether.send_message(message);
                }
                None => (),
            }
        }
    }

    /**
     * Deliver messages from the ether to the OBUs and RSUs.
     */
    fn deliver_messages(&mut self) {
        // deliver messages to OBUs
        self.obu_manager
            .deliver_messages(&self.ether.get_messages());

        // deliver messages to RSUs
        self.rsu_manager
            .deliver_messages(&self.ether.get_messages());
    }

    /**
     * Use uniform distribution to randomly select a number between 1 and 100,
     * and return true if the number is less than or equal to the given probability.
     */
    pub fn random_event(probability: f32) -> bool {
        let between = Uniform::from(1..=100);
        let mut rng = rand::thread_rng();
        let random_number = between.sample(&mut rng) as f32;
        random_number / 100.0 <= probability
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
        let grid_params = GridParams {
            blocks_per_street: 3,
            block_size: 2,
        };

        let rsu_manager_params = RsuManagerParams {
            tx_range: 5,
            rx_range: 5,
            detect_obu_gps_failure: false,
            detect_obu_tx_failure: false,
        };

        let obu_manager_params = ObuManagerParams {
            max_obus: 2,
            comms_range: 2,
            tx_base_failure_rate: 0.0,
            tx_faulty_obu_failure_rate: 0.0,
            gps_failure_rate: 0.0,
            gps_faulty_obu_failure_rate: 0.0,
            faulty_obus: 0,
        };

        let simulator = Simulator::new(grid_params, rsu_manager_params, obu_manager_params);

        assert_eq!(simulator.obu_manager.obus.len(), 0);
        assert_eq!(simulator.rsu_manager.rsus.len(), 0);
        assert_eq!(simulator.grid.get_dimension(), 10);
        assert_eq!(simulator.round, 0);
    }

    /**
     * Test Simulator initialization.
     */
    #[test]
    fn test_add_road_side_units() {
        let grid_params = GridParams {
            blocks_per_street: 3,
            block_size: 2,
        };

        let rsu_manager_params = RsuManagerParams {
            tx_range: 3,
            rx_range: 3,
            detect_obu_gps_failure: false,
            detect_obu_tx_failure: false,
        };

        let obu_manager_params = ObuManagerParams {
            max_obus: 2,
            comms_range: 2,
            tx_base_failure_rate: 0.0,
            tx_faulty_obu_failure_rate: 0.0,
            gps_failure_rate: 0.0,
            gps_faulty_obu_failure_rate: 0.0,
            faulty_obus: 0,
        };

        let mut simulator = Simulator::new(grid_params, rsu_manager_params, obu_manager_params);

        simulator.init();

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
        let grid_params = GridParams {
            blocks_per_street: 3,
            block_size: 2,
        };

        let rsu_manager_params = RsuManagerParams {
            tx_range: 4,
            rx_range: 4,
            detect_obu_gps_failure: false,
            detect_obu_tx_failure: false,
        };

        let obu_manager_params = ObuManagerParams {
            max_obus: 2,
            comms_range: 2,
            tx_base_failure_rate: 0.0,
            tx_faulty_obu_failure_rate: 0.0,
            gps_failure_rate: 0.0,
            gps_faulty_obu_failure_rate: 0.0,
            faulty_obus: 0,
        };

        let mut simulator = Simulator::new(grid_params, rsu_manager_params, obu_manager_params);

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
        let grid_params = GridParams {
            blocks_per_street: 3,
            block_size: 2,
        };

        let rsu_manager_params = RsuManagerParams {
            tx_range: 6,
            rx_range: 6,
            detect_obu_gps_failure: false,
            detect_obu_tx_failure: false,
        };

        let obu_manager_params = ObuManagerParams {
            max_obus: 2,
            comms_range: 2,
            tx_base_failure_rate: 0.0,
            tx_faulty_obu_failure_rate: 0.0,
            gps_failure_rate: 0.0,
            gps_faulty_obu_failure_rate: 0.0,
            faulty_obus: 0,
        };

        let mut simulator = Simulator::new(grid_params, rsu_manager_params, obu_manager_params);

        simulator.add_road_side_units();

        assert_eq!(simulator.rsu_manager.rsus.len(), 1);

        let rsu = simulator.rsu_manager.rsus.get(&0).unwrap();
        assert_eq!(rsu.get_coordinate().x, 5);
        assert_eq!(rsu.get_coordinate().y, 5);
    }

    /**
     * Test message collection from OBUs.
     */
    #[test]
    fn test_message_collection() {
        let grid_params = GridParams {
            blocks_per_street: 3,
            block_size: 2,
        };

        let rsu_manager_params = RsuManagerParams {
            tx_range: 5,
            rx_range: 5,
            detect_obu_gps_failure: false,
            detect_obu_tx_failure: true,
        };

        let obu_manager_params = ObuManagerParams {
            max_obus: 2,
            comms_range: 2,
            tx_base_failure_rate: 0.01,
            tx_faulty_obu_failure_rate: 0.10,
            gps_failure_rate: 0.001,
            gps_faulty_obu_failure_rate: 0.010,
            faulty_obus: 0,
        };

        let mut simulator = Simulator::new(grid_params, rsu_manager_params, obu_manager_params);
        simulator.add_road_side_units();

        simulator.add_on_board_unit();
        simulator.add_on_board_unit();

        simulator.collect_messages();
        assert_eq!(simulator.ether.get_messages().len(), 2);
        simulator.collect_messages();
        assert_eq!(simulator.ether.get_messages().len(), 2);
    }
}
