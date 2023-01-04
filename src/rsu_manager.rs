use crate::comms::Message;
use crate::grid::{Coordinate, Grid, SquareCoords};
use crate::rsu::RoadSideUnit;
use std::collections::HashMap;

pub struct RsuManagerParams {
    pub comms_range: u32,
}

/*struct OBUStateEntry {
    id: u32,
    coordinate: Coordinate,
    round: usize,
    rsu_id: u32,
}*/

pub struct RoadSideUnitManager {
    next_id: u32,
    comms_range: u32,
    pub rsus: HashMap<u32, RoadSideUnit>,
    //obus_states: Vec<OBUStateEntry>,
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
            //obus_states: Vec::new(),
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
