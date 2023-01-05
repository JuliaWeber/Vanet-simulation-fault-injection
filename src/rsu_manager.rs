use crate::comms::Message;
use crate::grid::{Coordinate, Grid, SquareCoords};
use crate::rsu::RoadSideUnit;
use std::collections::HashMap;

pub struct RsuManagerParams {
    pub comms_range: u32,
}

struct ObuData {
    coordinate: Coordinate,
    rsu_id: u32,
}

pub struct RoadSideUnitManager {
    next_id: u32,
    comms_range: u32,
    pub rsus: HashMap<u32, RoadSideUnit>, // FIXME: make private
    current_round: u32,
    obu_observations: Vec<HashMap<u32, Vec<ObuData>>>,
}

impl RoadSideUnitManager {
    /**
     * Creates a new RoadSideUnitManager.
     */
    pub fn new(params: RsuManagerParams) -> RoadSideUnitManager {
        RoadSideUnitManager {
            next_id: 0,
            comms_range: params.comms_range,
            rsus: HashMap::new(),
            current_round: 0,
            obu_observations: Vec::new(),
        }
    }

    /**
     * Return the next available id.
     */
    pub fn get_next_id(&mut self) -> u32 {
        let id = self.next_id;
        id
    }

    /**
     * Return the comms range.
     */
    pub fn get_comms_range(&self) -> u32 {
        let comms_range = self.comms_range;
        comms_range
    }

    /**
     * Creates a new RoadSideUnit
     */
    pub fn create_rsu(&mut self, coordinate: Coordinate, covered_area: SquareCoords) -> u32 {
        let id = self.next_id;

        let rsu = RoadSideUnit::new(id, coordinate, covered_area);

        // create and insert rsu in the hashmap
        self.rsus.insert(id, rsu);

        // increment id counter
        self.next_id += 1;

        // return the id of the created rsu
        id
    }

    /**
     * Set the current round.
     */
    pub fn set_current_round(&mut self, round: u32) {
        self.current_round = round;
    }

    /**
     * Deliver messages to RSUs.
     */
    pub fn deliver_messages(&mut self, messages: &Vec<Message>) {
        // iterate over all rsus
        for rsu in self.rsus.values_mut() {
            // clear the obu neighbors
            rsu.clear_neighbors();

            // for each message check if it is in the coverage area of the obu
            for message in messages {
                if Grid::check_overlaping_squares(rsu.get_covered_area(), message.covered_area) {
                    // deliver the message to the obu
                    rsu.receive_message(message.clone());
                }
            }
        }

        // update the obu observations
        self.update_obu_observations();
    }

    /**
     * Update OBU observations.
     */
    fn update_obu_observations(&mut self) {
        if self.obu_observations.len() == (self.current_round as usize + 1) {
            panic!("The current round is already in the vector!");
        }

        // create a new hashmap for the round data
        let mut round_data: HashMap<u32, Vec<ObuData>> = HashMap::new();

        // iterate over all rsus
        for rsu in self.rsus.values_mut() {
            for neighbor in rsu.get_neighbors() {
                // create the obu data
                let obu_data = ObuData {
                    coordinate: neighbor.coordinate,
                    rsu_id: rsu.get_id(),
                };

                // check if obu id is in the hashmap
                if round_data.contains_key(&neighbor.id) {
                    // add the obu data to the vector
                    round_data.get_mut(&neighbor.id).unwrap().push(obu_data);
                } else {
                    // create a new vector and add the obu data
                    round_data.insert(neighbor.id, vec![obu_data]);
                }
            }
        }

        // add the round data to the vector
        self.obu_observations.push(round_data);
    }

    /**
     * Check OBUs observations.
     */
    pub fn check_obu_observations(&self) {
        struct ObuRxStats {
            rx_count: u32,
            rx_error_count: u32,
            rx_error_rate: f32,
            first_seen: u32,
        }

        let mut rx_stats: HashMap<u32, ObuRxStats> = HashMap::new();

        // for each round
        for i in 0..self.obu_observations.len() {
            // get round data
            let round_data = &self.obu_observations[i];

            // iterate over round data
            for (obu_id, _) in round_data.iter() {
                if rx_stats.contains_key(obu_id) {
                    let mut stats = rx_stats.get_mut(obu_id).unwrap();
                    stats.rx_count += 1;
                } else {
                    let stats = ObuRxStats {
                        rx_count: 1,
                        rx_error_count: 0,
                        rx_error_rate: 0.0,
                        first_seen: i as u32,
                    };

                    rx_stats.insert(*obu_id, stats);
                }
            }
        }

        // calculate rx error count and error rate
        let rounds = self.obu_observations.len() as u32;

        for (_, stats) in rx_stats.iter_mut() {
            stats.rx_error_count = rounds - (stats.rx_count + stats.first_seen);
            stats.rx_error_rate = stats.rx_error_count as f32 / self.current_round as f32;
        }

        // print the results
        println!("--- RSU Manager Stats ---");
        let mut counter = 0;
        for (obu_id, stats) in rx_stats.iter() {

            if stats.rx_error_rate < 0.05 {
                continue;
            }

            counter += 1;

            println!(
                "OBU {} - rx_count: {}, rx_error_count: {}, rx_error_rate: {:.2}%",
                obu_id, stats.rx_count, stats.rx_error_count, stats.rx_error_rate * 100.0
            );
        }
        println!("Count: {}", counter);

    }
}

/***
 * TESTS MODULE
 */
#[cfg(test)]
mod tests {

    use super::*;
    use crate::grid::{Coordinate, SquareCoords};

    /**
     * Test the creation of an RoadSideUnitManager.
     */
    #[test]
    fn test_create_rsu_manager() {
        let params = RsuManagerParams { comms_range: 5 };

        let rsu_manager = RoadSideUnitManager::new(params);
        assert_eq!(rsu_manager.next_id, 0);
        assert_eq!(rsu_manager.comms_range, 5);
        assert_eq!(rsu_manager.rsus.len(), 0);
    }

    /**
     * Test the creation of an RoadSideUnit.
     */
    #[test]
    fn test_create_rsu() {
        let params = RsuManagerParams { comms_range: 5 };

        let mut rsu_manager = RoadSideUnitManager::new(params);
        let id = rsu_manager.create_rsu(
            Coordinate { x: 0, y: 0 },
            SquareCoords {
                x1: 0,
                y1: 0,
                x2: 0,
                y2: 0,
            },
        );
        assert_eq!(id, 0);
        assert_eq!(rsu_manager.next_id, 1);
        assert_eq!(rsu_manager.rsus.len(), 1);

        let rsu = rsu_manager.rsus.get(&id).unwrap();
        assert_eq!(rsu.get_id(), 0);
        assert_eq!(rsu.get_coordinate().x, 0);
        assert_eq!(rsu.get_coordinate().y, 0);

        let id = rsu_manager.create_rsu(
            Coordinate { x: 3, y: 6 },
            SquareCoords {
                x1: 0,
                y1: 0,
                x2: 0,
                y2: 0,
            },
        );
        assert_eq!(id, 1);
        assert_eq!(rsu_manager.next_id, 2);
        assert_eq!(rsu_manager.rsus.len(), 2);

        let rsu = rsu_manager.rsus.get(&id).unwrap();
        assert_eq!(rsu.get_id(), 1);
        assert_eq!(rsu.get_coordinate().x, 3);
        assert_eq!(rsu.get_coordinate().y, 6);
    }

    // TODO: Move message deliver tests from simulator.rs to here
}
