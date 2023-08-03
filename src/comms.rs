use crate::grid::Coordinate;
use crate::grid::SquareCoords;
use crate::simulator::NodeType;

/**
 * NeighborEntry represents a neighbor of a node
 */
pub struct NeighborEntry {
    pub id: u32,                // ID of the neighbor
    pub coordinate: Coordinate, // Coordinate of the neighbor
}

/**
 * Message represents a message sent by a node and
 * kept in the Ether
 */
#[derive(Clone, Debug)]
pub struct Message {
    pub origin_id: u32,         // ID of the node that sent the message
    pub origin_type: NodeType,  // Type of the node that sent the message
    pub coordinate: Coordinate, // Reported current coordinate of the node that sent the message
    pub phy_coord: Coordinate,  // Physical coordinate of the node that sent the message
    pub phy_range: u32,         // Physical communication range of the node that sent the message
    pub phy_area: SquareCoords, // Physical area covered by transmission of the message
}

/**
 * Ether represents the communication medium between nodes
 */
pub struct Ether {
    messages: Vec<Message>, // Messages in the Ether "while in transit"
}

/**
 * Message implementation
 */
impl Message {
    /**
     * Create a new Message
     */
    pub fn new(
        origin_id: u32,
        origin_type: NodeType,
        coordinate: Coordinate,
        phy_coord: Coordinate,
        phy_range: u32,
    ) -> Message {
        Message {
            origin_id,
            origin_type,
            coordinate,
            phy_coord,
            phy_range,
            phy_area: SquareCoords {
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

/**
 * Ether implementation
 */
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
     * Get an immutable reference to the messages vector
     */
    pub fn get_messages(&self) -> &Vec<Message> {
        &self.messages
    }

    /**
     * Check if a transmission from a transmitter to a receiver is possible.
     */
    pub fn is_transmission_possible(
        transmitter_coordinate: Coordinate,
        transmitter_range: u32,
        receiver_coordinate: Coordinate,
    ) -> bool {
        // Calculate the Euclidean distance between the transmitter and the receiver
        let distance = (((transmitter_coordinate.x as i32 - receiver_coordinate.x as i32).pow(2)
            + (transmitter_coordinate.y as i32 - receiver_coordinate.y as i32).pow(2))
            as f64)
            .sqrt();

        // If the distance is less than or equal to the transmitter's range, the transmission is possible
        distance <= transmitter_range as f64
    }
}

/***
 * TESTS MODULE
 */
#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    // This function tests the `send_message()` and `get_messages()` methods of the `Ether` struct.
    fn test_send_message() {
        // Create a new instance of the `Ether` struct.
        let mut ether = Ether::new();

        // Create a new `Message` instance and send it to the `Ether` instance.
        let message = Message::new(
            0,
            NodeType::OBU,
            Coordinate { x: 0, y: 0 },
            Coordinate { x: 0, y: 0 },
            2,
        );
        ether.send_message(message);

        // Check that the `Ether` instance contains one message.
        assert_eq!(ether.get_messages().len(), 1);

        // Create two more `Message` instances and send them to the `Ether` instance.
        let message = Message::new(
            0,
            NodeType::OBU,
            Coordinate { x: 0, y: 0 },
            Coordinate { x: 0, y: 0 },
            2,
        );
        ether.send_message(message);
        let message = Message::new(
            0,
            NodeType::OBU,
            Coordinate { x: 0, y: 0 },
            Coordinate { x: 0, y: 0 },
            2,
        );
        ether.send_message(message);

        // Check that the `Ether` instance contains three messages.
        assert_eq!(ether.get_messages().len(), 3);

        // Clear all messages from the `Ether` instance.
        ether.clear();

        // Check that the `Ether` instance contains no messages.
        assert_eq!(ether.get_messages().len(), 0);
    }

    /**
     * This function tests the `is_transmission_possible()` method of the `Ether` struct.
     */
    #[test]
    fn test_is_transmission_possible() {
        let transmitter_coordinate = Coordinate { x: 0, y: 0 };
        let receiver_coordinate_in_range = Coordinate { x: 3, y: 4 };
        let receiver_coordinate_out_of_range = Coordinate { x: 10, y: 10 };
        let transmitter_range = 5;

        assert_eq!(
            Ether::is_transmission_possible(transmitter_coordinate, transmitter_range, receiver_coordinate_in_range),
            true,
            "The receiver is within the transmitter's range, so the transmission should be possible."
        );

        assert_eq!(
            Ether::is_transmission_possible(transmitter_coordinate, transmitter_range, receiver_coordinate_out_of_range),
            false,
            "The receiver is outside the transmitter's range, so the transmission should not be possible."
        );
    }
}
