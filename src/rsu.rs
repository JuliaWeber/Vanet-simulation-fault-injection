use crate::grid::Coordinate;

pub struct RoadSideUnit {
    id: u32,
    coordinate: Coordinate,
}

impl RoadSideUnit {
    /**
     * Create a new RoadSideUnit
     */
    pub fn new(id: u32, coordinate: Coordinate) -> RoadSideUnit {
        RoadSideUnit {
            id,
            coordinate,
        }
    }

    /**
     * Get the id of the RoadSideUnit
     */
    pub fn get_id(&self) -> u32 {
        self.id
    }

    /**
     * Get the coordinate of the RoadSideUnit
     */
    pub fn get_coordinate(&self) -> Coordinate {
        self.coordinate.clone()
    }
}
