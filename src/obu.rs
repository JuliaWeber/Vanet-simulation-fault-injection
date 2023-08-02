use crate::comms::{Message, NeighborEntry};
use crate::grid::{Coordinate, Grid, SquareCoords};
use crate::simulator::{NodeType, Simulator};
use rand::Rng;

pub struct OnBoardUnit {
    id: u32,
    coordinate: Coordinate,
    tx_range: u32,
    tx_failure_rate: f32,
    gps_failure_rate: f32,
    is_faulty: bool,
    pub neighbors: Vec<NeighborEntry>,
    grid_dimension: u32,
}

/**
 * OnBoardUnit implementation
 */
impl OnBoardUnit {
    /**
     * Create a new OnBoardUnit
     */
    pub fn new(
        id: u32,
        coordinate: Coordinate,
        comms_range: u32,
        tx_failure_rate: f32,
        gps_failure_rate: f32,
        is_faulty: bool,
        grid_dimension: u32,
    ) -> OnBoardUnit {
        OnBoardUnit {
            id,
            coordinate,
            tx_range: comms_range,
            tx_failure_rate,
            gps_failure_rate,
            is_faulty,
            neighbors: Vec::new(),
            grid_dimension,
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
     * Get the communication range of the OnBoardUnit
     */
    pub fn get_tx_range(&self) -> u32 {
        self.tx_range
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
        // Default value of the reported coordinate
        let mut reported_coord = self.coordinate.clone();

        // Check the tx failure rate
        if Simulator::random_event(self.tx_failure_rate) {
            // Don't send a message
            return None;
        }

        // Check the gps failure rate
        if Simulator::random_event(self.gps_failure_rate) {
            // Get a random coordinate outside the range of the OBU
            reported_coord = self.get_random_coordinate_outside_range();
        }

        // return a message
        Some(Message::new(
            self.id,
            NodeType::OBU,
            reported_coord,
            self.coordinate.clone(),
            self.tx_range,
        ))
    }fn get_message

    /**
     * Receive a message from the ether
     */
    pub fn receive_message(&mut self, message: Message) {
        match message.origin_type {
            NodeType::OBU => {
                // ignore my own messages
                if message.origin_id == self.id {
                    return;
                }

                // create a neighbor entry
                let neighbor = NeighborEntry {
                    id: message.origin_id,
                    coordinate: message.coordinate,
                };

                // add the neighbor to the list
                self.neighbors.push(neighbor);
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

    /**
     * Get a random coordinate outside the range of the OBU.
     * CAUTION: This function will loop infinitely if the OBU has relatively
     * large communication range in relation to the grid size.
     */
    pub fn get_random_coordinate_outside_range(&self) -> Coordinate {
        // Create a random number generator
        let mut rng = rand::thread_rng();

        // Start an infinite loop
        loop {
            // Generate a random x coordinate within the grid
            let x = rng.gen_range(0..self.grid_dimension);
            // Generate a random y coordinate within the grid
            let y = rng.gen_range(0..self.grid_dimension);

            // Calculate the Euclidean distance between the OBU and the random coordinate
            let distance = (((self.coordinate.x as i32 - x as i32).pow(2)
                + (self.coordinate.y as i32 - y as i32).pow(2)) as f64)
                .sqrt();

            // If the distance is greater than the communication range of the OBU + 2
            if distance > (self.tx_range + 2) as f64 {
                // Return the random coordinate
                return Coordinate { x, y };
            }
            // If the distance is not greater than the communication range,
            // the loop will continue and generate a new random coordinate
        }
    } fn get_random_coordinate_outside_range
} impl OnBoardUnit
