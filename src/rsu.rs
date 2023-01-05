use crate::comms::{Message, NeighborEntry};
use crate::grid::{Coordinate, SquareCoords};
use crate::simulator::NodeType;

pub struct RoadSideUnit {
    id: u32,
    coordinate: Coordinate,
    covered_area: SquareCoords,
    neighbors: Vec<NeighborEntry>,
}

impl RoadSideUnit {
    /**
     * Create a new RoadSideUnit
     */
    pub fn new(id: u32, coordinate: Coordinate, covered_area: SquareCoords) -> RoadSideUnit {
        RoadSideUnit {
            id,
            coordinate,
            covered_area,
            neighbors: Vec::new(),
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

    /**
     * Get the covered area of the RoadSideUnit
     */
    pub fn get_covered_area(&self) -> SquareCoords {
        self.covered_area.clone()
    }

    /**
     * Get a message from this rsu
     */
    pub fn get_message(&self) -> Option<Message> {
        None
    }

    /**
     * Receive a message from the ether
     */
    pub fn receive_message(&mut self, message: Message) {
        match message.origin_type {
            NodeType::OBU => {
                let neighbor = NeighborEntry {
                    id: message.origin_id,
                    coordinate: message.coordinate,
                };

                self.neighbors.push(neighbor);
            }
            _ => {}
        }
    }

    /**
     * Get the neighbors
     */
    pub fn get_neighbors(&self) -> &Vec<NeighborEntry> {
        self.neighbors.as_ref()
    }

    /**
     * Clear the neighbors
     */
    pub fn clear_neighbors(&mut self) {
        self.neighbors.clear();
    }
}
