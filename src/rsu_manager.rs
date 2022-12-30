use crate::grid::Coordinate;
use crate::rsu::RoadSideUnit;
use std::collections::HashMap;

pub struct RoadSideUnitManager {
    next_id: u32,
    comms_range: usize,
    pub rsus: HashMap<u32, RoadSideUnit>,
}

impl RoadSideUnitManager {
    /**
     * Creates a new RoadSideUnitManager.
     */
    pub fn new(comms_range: usize) -> RoadSideUnitManager {
        RoadSideUnitManager {
            next_id: 0,
            comms_range,
            rsus: HashMap::new(),
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
    pub fn get_comms_range(&self) -> usize {
        let comms_range = self.comms_range;
        comms_range
    }

    /**
     * Creates a new RoadSideUnit
     */
    pub fn create_rsu(&mut self, coordinate: Coordinate) -> u32 {
        let id = self.next_id;

        // create and insert rsu in the hashmap
        self.rsus.insert(id, RoadSideUnit::new(id, coordinate));

        // increment id counter
        self.next_id += 1;

        // return the id of the created rsu
        id
    }
}

/***
 * TESTS MODULE
 */
#[cfg(test)]
mod tests {

    use crate::grid::Coordinate;
    use crate::rsu_manager::RoadSideUnitManager;

    /**
     * Test the creation of an RoadSideUnitManager.
     */
    #[test]
    fn test_create_rsu_manager() {
        let rsu_manager = RoadSideUnitManager::new(5);
        assert_eq!(rsu_manager.next_id, 0);
        assert_eq!(rsu_manager.comms_range, 5);
        assert_eq!(rsu_manager.rsus.len(), 0);
    }

    /**
     * Test the creation of an RoadSideUnit.
     */
    #[test]
    fn test_create_rsu() {
        let mut rsu_manager = RoadSideUnitManager::new(5);
        let id = rsu_manager.create_rsu(Coordinate { x: 0, y: 0 });
        assert_eq!(id, 0);
        assert_eq!(rsu_manager.next_id, 1);
        assert_eq!(rsu_manager.rsus.len(), 1);

        let rsu = rsu_manager.rsus.get(&id).unwrap();
        assert_eq!(rsu.get_id(), 0);
        assert_eq!(rsu.get_coordinate().x, 0);
        assert_eq!(rsu.get_coordinate().y, 0);

        let id = rsu_manager.create_rsu(Coordinate { x: 3, y: 6 });
        assert_eq!(id, 1);
        assert_eq!(rsu_manager.next_id, 2);
        assert_eq!(rsu_manager.rsus.len(), 2);

        let rsu = rsu_manager.rsus.get(&id).unwrap();
        assert_eq!(rsu.get_id(), 1);
        assert_eq!(rsu.get_coordinate().x, 3);
        assert_eq!(rsu.get_coordinate().y, 6);
    }
}
