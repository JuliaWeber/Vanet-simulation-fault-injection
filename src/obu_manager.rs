use crate::comms::Ether;
use crate::comms::Message;
use crate::grid::{Coordinate, Grid};
use crate::obu::OnBoardUnit;
use std::collections::HashMap;

pub struct ObuManagerParams {
    pub max_obus: u32,
    pub comms_range: u32,
    pub tx_base_failure_rate: f32,
    pub tx_faulty_obu_failure_rate: f32,
    pub gps_failure_rate: f32,
    pub gps_faulty_obu_failure_rate: f32,
    pub faulty_obus: u32,
}

struct ObuManagerStats {
    normal_obu_tx_count: u32,
    normal_obu_tx_error_count: u32,
    faulty_obu_tx_count: u32,
    faulty_obu_tx_error_count: u32,
    total_tx_count: u32,
    total_tx_error_count: u32,
}

pub struct OnBoardUnitManager {
    next_id: u32,
    max_obus: u32,
    comms_range: u32,
    tx_base_failure_rate: f32,
    tx_faulty_obu_failure_rate: f32,
    gps_failure_rate: f32,
    gps_faulty_obu_failure_rate: f32,
    faulty_obus: u32,
    faulty_obus_added: u32,
    pub obus: HashMap<u32, OnBoardUnit>, // FIXME: make private
    stats: ObuManagerStats,
    current_round: u32,
    grid_dimension: u32,
}

/**
 * OnBoardUnitManager implementation.
 */
impl OnBoardUnitManager {
    /**
     * Creates a new OnBoardUnitManager.
     */
    pub fn new(params: ObuManagerParams, grid_dimension: u32) -> OnBoardUnitManager {
        OnBoardUnitManager {
            next_id: 0,
            max_obus: params.max_obus,
            comms_range: params.comms_range,
            tx_base_failure_rate: params.tx_base_failure_rate,
            tx_faulty_obu_failure_rate: params.tx_faulty_obu_failure_rate,
            gps_failure_rate: params.gps_failure_rate,
            gps_faulty_obu_failure_rate: params.gps_faulty_obu_failure_rate,
            faulty_obus: params.faulty_obus,
            faulty_obus_added: 0,
            grid_dimension,
            obus: HashMap::new(),
            stats: ObuManagerStats {
                normal_obu_tx_count: 0,
                normal_obu_tx_error_count: 0,
                faulty_obu_tx_count: 0,
                faulty_obu_tx_error_count: 0,
                total_tx_count: 0,
                total_tx_error_count: 0,
            },
            current_round: 0,
        }
    }

    /**
     * Return the next available id.
     */
    pub fn get_next_id(&self) -> u32 {
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
     * Return max number of obus.
     */
    pub fn get_max_obus(&self) -> u32 {
        let max_obus = self.max_obus;
        max_obus
    }

    /**
     * Return number of obus.
     */
    pub fn get_obus_count(&self) -> u32 {
        let num_obus = self.obus.len() as u32;
        num_obus
    }

    /**
     * Set the current round.
     */
    pub fn set_current_round(&mut self, round: u32) {
        self.current_round = round;
    }

    /**
     * Creates a new OnBoardUnit
     */
    pub fn create_obu(&mut self, coordinate: Coordinate) -> Option<u32> {
        // Get the next available id
        let id = self.next_id;

        // Check if the maximum number of OBUs has been reached.
        if self.obus.len() >= self.max_obus as usize {
            return None;
        }

        // By default, use the base failures rates.
        let mut tx_failure_rate = self.tx_base_failure_rate;
        let mut gps_failure_rate = self.gps_failure_rate;

        // By default, the OBU is not faulty.
        let mut is_faulty = false;

        // if the number of faulty obus is greater than 0 and the number of faulty obus added is
        // less than the number of faulty obus, then add a faulty obu
        if self.faulty_obus > 0 && self.faulty_obus_added < self.faulty_obus {
            if (self.obus.len() as u32 + 1) % (self.max_obus / self.faulty_obus) == 0 {
                // adjust the failure rate
                tx_failure_rate = self.tx_faulty_obu_failure_rate;
                gps_failure_rate = self.gps_faulty_obu_failure_rate;
                is_faulty = true;
                self.faulty_obus_added += 1;
            }
        }

        // create and insert obu in the hashmap
        self.obus.insert(
            id,
            OnBoardUnit::new(
                id,
                coordinate,
                self.comms_range,
                tx_failure_rate,
                gps_failure_rate,
                is_faulty,
                self.grid_dimension,
            ),
        );

        // increment id counter
        self.next_id += 1;

        // return the id of the created obu
        Some(id)
    }fn create_obu

    /**
     * Check if an obu is faulty.
     */
    pub fn is_obu_faulty(&self, id: u32) -> bool {
        match self.obus.get(&id) {
            Some(obu) => obu.is_faulty(),
            None => panic!("obu with id {} does not exist", id),
        }
    }

    /**
     * Collect messages from OBUs.
     */
    pub fn collect_messages(&mut self) -> Vec<Message> {
        // Initialize the messages vector
        let mut messages = Vec::new();

        // Iterate over all obus
        for obu in self.obus.values_mut() {
            // Update the stats
            self.stats.total_tx_count += 1;
            let is_faulty = obu.is_faulty();
            if is_faulty {
                self.stats.faulty_obu_tx_count += 1;
            } else {
                self.stats.normal_obu_tx_count += 1;
            }

            // Try to get a message from the obu
            if let Some(message) = obu.get_message() {
                // If a message was returned, update the range and add it to the returned vector
                messages.push(message);
            } else {
                // If no message was returned, update the stats accordingly
                self.stats.total_tx_error_count += 1;
                if is_faulty {
                    self.stats.faulty_obu_tx_error_count += 1;
                } else {
                    self.stats.normal_obu_tx_error_count += 1;
                }
            }
        }

        // Return the messages
        messages
    }fn collect_messages

    /**
     * Deliver messages to OBUs.
     */
    pub fn deliver_messages(&mut self, messages: &Vec<Message>) {
        // Iterate over all obus
        for obu in self.obus.values_mut() {
            // Clear the obu neighbors
            obu.clear_neighbors();

            // Iterate over all messages
            for message in messages {
                // Check if the message can reach the obu
                if Ether::is_transmission_possible(
                    message.phy_coord,
                    message.phy_range,
                    obu.get_coordinate(),
                ) {
                    // deliver the message to the obu
                    obu.receive_message(message.clone());
                }
            }
        }
    }

    /**
     * Print the stats.
     */
    pub fn print_stats(&self) {
        println!("--- OBU Manager Stats ---");
        println!(
            "     Total TX: {} / errors {} ({:.2}%)",
            self.stats.total_tx_count,
            self.stats.total_tx_error_count,
            self.stats.total_tx_error_count as f32 / self.stats.total_tx_count as f32 * 100.0
        );
        println!(
            "Normal OBU TX: {} / errors {} ({:.2}%)",
            self.stats.normal_obu_tx_count,
            self.stats.normal_obu_tx_error_count,
            self.stats.normal_obu_tx_error_count as f32 / self.stats.normal_obu_tx_count as f32
                * 100.0
        );
        println!(
            "Faulty OBU TX: {} / errors {} ({:.2}%)",
            self.stats.faulty_obu_tx_count,
            self.stats.faulty_obu_tx_error_count,
            self.stats.faulty_obu_tx_error_count as f32 / self.stats.faulty_obu_tx_count as f32
                * 100.0
        );
    }
}impl OnBoardUnitManager

/***
 * TESTS MODULE
 */
#[cfg(test)]
mod tests {

    use super::*;
    use crate::grid::Coordinate;

    /**
     * Test the creation of an OnBoardUnitManager
     */
    #[test]
    fn test_create_obu_manager() {
        let params = ObuManagerParams {
            max_obus: 2,
            comms_range: 1,
            tx_base_failure_rate: 0.0,
            tx_faulty_obu_failure_rate: 0.0,
            gps_failure_rate: 0.0,
            gps_faulty_obu_failure_rate: 0.0,
            faulty_obus: 0,
        };

        let obu_manager = OnBoardUnitManager::new(params, 0);
        assert_eq!(obu_manager.next_id, 0);
        assert_eq!(obu_manager.obus.len(), 0);
    }

    /**
     * Test the creation of OnBoardUnits 1
     */
    #[test]
    fn test_create_obus_1() {
        let params = ObuManagerParams {
            max_obus: 100,
            comms_range: 1,
            tx_base_failure_rate: 0.01,
            tx_faulty_obu_failure_rate: 0.1,
            gps_failure_rate: 0.0,
            gps_faulty_obu_failure_rate: 0.0,
            faulty_obus: 20,
        };

        let mut obu_manager = OnBoardUnitManager::new(params, 0);

        // add 4 obus
        for _ in 0..4 {
            obu_manager.create_obu(Coordinate { x: 1, y: 2 });
        }

        assert_eq!(obu_manager.next_id, 4);
        assert_eq!(obu_manager.obus.len(), 4);

        let obu = obu_manager.obus.get(&3).unwrap();
        assert_eq!(obu.get_id(), 3);
        assert_eq!(obu.get_coordinate().x, 1);
        assert_eq!(obu.get_coordinate().y, 2);
        assert_eq!(obu.get_tx_failure_rate(), 0.01);
        assert_eq!(obu.is_faulty(), false);

        // add one more obu, this one should be faulty
        obu_manager.create_obu(Coordinate { x: 3, y: 4 });
        assert_eq!(obu_manager.next_id, 5);
        assert_eq!(obu_manager.obus.len(), 5);

        let obu = obu_manager.obus.get(&4).unwrap();
        assert_eq!(obu.get_id(), 4);
        assert_eq!(obu.get_coordinate().x, 3);
        assert_eq!(obu.get_coordinate().y, 4);
        assert_eq!(obu.get_tx_failure_rate(), 0.1);
        assert_eq!(obu.is_faulty(), true);

        // add the remaining obus
        for _ in 5..obu_manager.max_obus {
            obu_manager.create_obu(Coordinate { x: 1, y: 2 });
        }

        // check the number of obus
        assert_eq!(obu_manager.obus.len(), 100);

        // count the faulty obus
        let mut faulty_obus = 0;
        for obu in obu_manager.obus.values() {
            if obu.is_faulty() {
                faulty_obus += 1;
            }
        }

        // check the number of faulty obus
        assert_eq!(faulty_obus, 20);
    }

    /**
     * Test the creation of OnBoardUnits 2
     */
    #[test]
    fn test_create_obus_2() {
        let params = ObuManagerParams {
            max_obus: 100,
            comms_range: 1,
            tx_base_failure_rate: 0.01,
            tx_faulty_obu_failure_rate: 0.1,
            gps_failure_rate: 0.0,
            gps_faulty_obu_failure_rate: 0.0,
            faulty_obus: 13,
        };

        let mut obu_manager = OnBoardUnitManager::new(params, 0);

        // add 6 obus
        for _ in 0..6 {
            obu_manager.create_obu(Coordinate { x: 1, y: 2 });
        }

        assert_eq!(obu_manager.next_id, 6);
        assert_eq!(obu_manager.obus.len(), 6);

        let obu = obu_manager.obus.get(&5).unwrap();
        assert_eq!(obu.get_id(), 5);
        assert_eq!(obu.get_coordinate().x, 1);
        assert_eq!(obu.get_coordinate().y, 2);
        assert_eq!(obu.get_tx_failure_rate(), 0.01);
        assert_eq!(obu.is_faulty(), false);

        // add one more obu, this one should be faulty
        obu_manager.create_obu(Coordinate { x: 3, y: 4 });
        assert_eq!(obu_manager.next_id, 7);
        assert_eq!(obu_manager.obus.len(), 7);

        let obu = obu_manager.obus.get(&6).unwrap();
        assert_eq!(obu.get_id(), 6);
        assert_eq!(obu.get_coordinate().x, 3);
        assert_eq!(obu.get_coordinate().y, 4);
        assert_eq!(obu.get_tx_failure_rate(), 0.1);
        assert_eq!(obu.is_faulty(), true);

        // add the remaining obus
        for _ in 7..obu_manager.max_obus {
            obu_manager.create_obu(Coordinate { x: 1, y: 2 });
        }

        // check the number of obus
        assert_eq!(obu_manager.obus.len(), 100);

        // count the faulty obus
        let mut faulty_obus = 0;
        for obu in obu_manager.obus.values() {
            if obu.is_faulty() {
                faulty_obus += 1;
            }
        }

        // check the number of faulty obus
        assert_eq!(faulty_obus, 13);
    }

    /**
     * Test the collection of messages from OBUs.
     */
    #[test]
    fn test_obu_message_collection() {
        let params = ObuManagerParams {
            max_obus: 3,
            comms_range: 2,
            tx_base_failure_rate: 0.0,
            tx_faulty_obu_failure_rate: 0.0,
            gps_failure_rate: 0.0,
            gps_faulty_obu_failure_rate: 0.0,
            faulty_obus: 0,
        };

        let mut obu_manager = OnBoardUnitManager::new(params, 0);

        obu_manager.create_obu(Coordinate { x: 1, y: 2 });
        obu_manager.create_obu(Coordinate { x: 1, y: 3 });
        let messages = obu_manager.collect_messages();
        assert_eq!(messages.len(), 2);

        obu_manager.create_obu(Coordinate { x: 1, y: 3 });
        let messages = obu_manager.collect_messages();
        assert_eq!(messages.len(), 3);
    }

    // TODO: Move message deliver tests from simulator.rs to here
}mod tests
