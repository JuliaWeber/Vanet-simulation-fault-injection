use crate::grid::Coordinate;
use crate::grid::SquareCoords;
use crate::simulator::NodeType;

pub struct NeighborEntry {
    pub id: u32,
    pub coordinate: Coordinate,
}

#[derive(Clone, Debug)]
pub struct Message {
    pub origin_id: u32,
    pub origin_type: NodeType,
    pub coordinate: Coordinate,
    pub comms_range: u32,
    pub covered_area: SquareCoords,
}

impl Message {
    /**
     * Create a new Message
     */
    pub fn new(origin_id: u32, origin_type: NodeType, coordinate: Coordinate, comms_range: u32) -> Message {
        Message {
            origin_id,
            origin_type,
            coordinate,
            comms_range,
            covered_area: SquareCoords {
                x1: 0,
                y1: 0,
                x2: 0,
                y2: 0,
            },
        }
    }

    /**
     * Get the coordinate of the Message
     */
    pub fn get_coordinate(&self) -> Coordinate {
        self.coordinate.clone()
    }
}

pub struct Ether {
    pub messages: Vec<Message>,
}

impl Ether {
    /**
     * Create a new Ether
     */
    pub fn new() -> Ether {
        Ether {
            messages: Vec::new(),
        }
    }

    /**
     * Add a new message to the Ether
     */
    pub fn send_message(&mut self, message: Message) {
        self.messages.push(message);
    }

    /**
     * Clear the Ether
     */
    pub fn clear(&mut self) {
        self.messages.clear();
    }

    /**
     * Get number of messages in the Ether
     */
    pub fn get_message_count(&self) -> usize {
        self.messages.len()
    }
}

/***
 * TESTS MODULE
 */
#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_send_message() {
        let mut ether = Ether::new();

        let message = Message::new(0, NodeType::OBU, Coordinate { x: 0, y: 0 }, 2);
        ether.send_message(message);
        assert_eq!(ether.get_message_count(), 1);

        let message = Message::new(0, NodeType::OBU, Coordinate { x: 0, y: 0 }, 2);
        ether.send_message(message);
        let message = Message::new(0, NodeType::OBU, Coordinate { x: 0, y: 0 }, 2);
        ether.send_message(message);
        assert_eq!(ether.get_message_count(), 3);

        ether.clear();
        assert_eq!(ether.get_message_count(), 0);
    }
}
