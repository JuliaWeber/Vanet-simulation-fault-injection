use crate::grid::Coordinate;

pub struct OnBoardUnit {
    id: u32,
    coordinate: Coordinate,
}

impl OnBoardUnit {
    /**
     * Create a new OnBoardUnit
     */
    pub fn new(id: u32, coordinate: Coordinate) -> OnBoardUnit {
        OnBoardUnit {
            id,
            coordinate,
        }
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
