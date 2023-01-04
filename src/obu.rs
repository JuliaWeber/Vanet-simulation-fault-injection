use crate::comms::{Message, NeighborEntry};
use crate::grid::Coordinate;
use crate::simulator::{NodeType, Simulator};

pub struct OnBoardUnit {
    id: u32,
    coordinate: Coordinate,
    tx_failure_rate: f32,
    is_faulty: bool,
    pub neighbors: Vec<NeighborEntry>,
}

impl OnBoardUnit {
    /**
     * Create a new OnBoardUnit
     */
    pub fn new(id: u32, coordinate: Coordinate, tx_failure_rate: f32, is_faulty: bool) -> OnBoardUnit {
        OnBoardUnit {
            id,
            coordinate,
            tx_failure_rate,
            is_faulty,
            neighbors: Vec::new(),
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

    /**
     * Get the tx failure rate of the OnBoardUnit
     */
    pub fn get_tx_failure_rate(&self) -> f32 {
        self.tx_failure_rate
    }

    /**
     * Get the faulty status of the OnBoardUnit
     */
    pub fn is_faulty(&self) -> bool {
        self.is_faulty
    }

    /**
     * Get a message from this obu
     */
    pub fn get_message(&self) -> Option<Message> {

        // check for failure
        if Simulator::random_event(self.tx_failure_rate) {
            return None;
        }

        Some(Message::new(
            self.id,
            NodeType::OBU,
            self.coordinate.clone(),
            0
        ))
    }


    /**
     * Receive a messagem from the ether
     */
    pub fn receive_message(&mut self, message: Message) {
        match message.origin_type {
            NodeType::OBU => {

                // ignore my own messages
                if message.origin_id == self.id {
                    return;
                }

                let neighbor = NeighborEntry {
                    id: message.origin_id,
                    coordinate: message.coordinate,
                };

                self.neighbors.push(neighbor);

                //println!("OBU {} received a message from {}", self.id, message.origin_id);
            }
            _ => {}
        }
    }

    /**
     * Clear the neighbors
     */
    pub fn clear_neighbors(&mut self) {
        self.neighbors.clear();
    }
}
