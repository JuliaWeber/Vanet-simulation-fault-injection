use crate::comms::{Message, NeighborEntry};
use crate::grid::Coordinate;
use crate::simulator::NodeType;

pub struct RoadSideUnit {
    id: u32,
    coordinate: Coordinate,
    neighbors: Vec<NeighborEntry>,
}

/**
 * RoadSideUnit implementation
 */
impl RoadSideUnit {
    /**
     * Create a new RoadSideUnit
     */
    pub fn new(id: u32, coordinate: Coordinate) -> RoadSideUnit {
        RoadSideUnit {
            id,
            coordinate,
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
     * Get a message from this rsu
     */
    pub fn get_message(&self) -> Option<Message> {
        None
    }

    /**
     * Receive a message from the ether
     */
    pub fn receive_message(&mut self, message: Message) {
        // Check the type of the `Message` instance.
        match message.origin_type {
            // If the `Message` instance was sent by an OBU node, add the sender to the `neighbors` vector.
            NodeType::OBU => {
                // Create a new `NeighborEntry` instance.
                let neighbor = NeighborEntry {
                    id: message.origin_id,
                    coordinate: message.coordinate,
                };

                // Add the `NeighborEntry` instance to the `neighbors` vector.
                self.neighbors.push(neighbor);
            }
            // If the `Message` instance was sent by any other type of node, do nothing.
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
