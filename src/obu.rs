use crate::grid::{Coordinate};
use std::collections::HashMap;

pub struct OnBoardUnitManager {
    next_id: u32,
    pub obus: HashMap<u32, OnBoardUnit>,
}

impl OnBoardUnitManager {

    /**
     * Creates a new OnBoardUnitManager.
     */
    pub fn new() -> OnBoardUnitManager {
        OnBoardUnitManager {
            next_id: 0,
            obus: HashMap::new(),
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
     * Creates a new OnBoardUnit
     */
    pub fn create_obu(&mut self, coordinate: Coordinate) -> u32 {
        let id = self.next_id;
     
        // create and insert obu in the hashmap
        self.obus.insert(id, OnBoardUnit::new(id, coordinate));

        // increment id counter
        self.next_id += 1;

        // return the id of the created obu
        id
    }

    /**
     * Remove an OnBoardUnit from the manager.
     */
    pub fn remove_obu(&mut self, id: u32) -> Option<OnBoardUnit> {
        self.obus.remove(&id)
    }

}

pub struct OnBoardUnit {
    id: u32,
    coordinate: Coordinate,
}

impl OnBoardUnit {
    /**
     * Create a new OnBoardUnit
     */
    pub fn new(id: u32, coordinate: Coordinate) -> OnBoardUnit {
        OnBoardUnit { id, coordinate }
    }

    /**
     * Get the id of the OnBoardUnit
     */
    pub fn get_id(&self) -> u32 {
        self.id
    }

    /**
     * Set the coordinate of the OnBoardUnit
     */
    pub fn set_coordinate(&mut self, position: Coordinate) {
        self.coordinate = position;
    }

    /**
     * Get the coordinate of the OnBoardUnit
     */
    pub fn get_coordinate(&self) -> Coordinate {
        self.coordinate.clone()
    }
}


/***
 * TESTS MODULE
 */
#[cfg(test)]
mod tests {

    use crate::obu::{OnBoardUnitManager};
    use crate::grid::{Coordinate};

    /**
     * Test the creation of an OnBoardUnitManager
     */
    #[test]
    fn test_create_obu_manager() {
        let obu_manager = OnBoardUnitManager::new();
        assert_eq!(obu_manager.next_id, 0);
        assert_eq!(obu_manager.obus.len(), 0);
    }

    /**
     * Test the creation and removal of OnBoardUnits
     */
    #[test]
    fn test_create_and_removal_obus() {
        let mut obu_manager = OnBoardUnitManager::new();
        obu_manager.create_obu(Coordinate{x: 1, y: 2});
        assert_eq!(obu_manager.next_id, 1);
        assert_eq!(obu_manager.obus.len(), 1);

        let obu = obu_manager.obus.get(&0).unwrap();
        assert_eq!(obu.id, 0);
        assert_eq!(obu.coordinate.x, 1);
        assert_eq!(obu.coordinate.y, 2);

        obu_manager.create_obu(Coordinate{x: 3, y: 4});
        assert_eq!(obu_manager.next_id, 2);
        assert_eq!(obu_manager.obus.len(), 2);

        let obu = obu_manager.obus.get(&1).unwrap();
        assert_eq!(obu.id, 1);
        assert_eq!(obu.coordinate.x, 3);
        assert_eq!(obu.coordinate.y, 4);

        let obu = obu_manager.remove_obu(0).unwrap();
        assert_eq!(obu.id, 0);
        assert_eq!(obu.coordinate.x, 1);
        assert_eq!(obu.coordinate.y, 2);
        assert_eq!(obu_manager.next_id, 2);
        assert_eq!(obu_manager.obus.len(), 1);

        let obu = obu_manager.remove_obu(1).unwrap();
        assert_eq!(obu.id, 1);
        assert_eq!(obu.coordinate.x, 3);
        assert_eq!(obu.coordinate.y, 4);
        assert_eq!(obu_manager.next_id, 2);
        assert_eq!(obu_manager.obus.len(), 0);

    }

}