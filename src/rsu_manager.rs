use crate::comms::Ether;
use crate::comms::Message;
use crate::grid::Coordinate;
use crate::rsu::RoadSideUnit;
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;

pub struct RsuManagerParams {
    pub tx_range: u32,
    pub rx_range: u32,
    pub detect_obu_tx_failure: bool,
    pub detect_obu_gps_failure: bool,
}

#[allow(dead_code)]
struct ObuData {
    coordinate: Coordinate,
    rsu_id: u32,
}

pub struct RoadSideUnitManager {
    next_id: u32,                                      // Next available id
    tx_range: u32,                                     // Transmission range
    rx_range: u32,                                     // Used to calculate the spacing between RSUs
    pub rsus: HashMap<u32, RoadSideUnit>,              // FIXME: make private
    current_round: u32,                                // Current simulation round
    obu_observations: Vec<HashMap<u32, Vec<ObuData>>>, // A vector of HashMaps with the observations of the OBUs
    detect_obu_tx_failure: bool,                       // Detect OBU tx failures
    detect_obu_gps_failure: bool,                      // Detect OBU gps failures
}

/**
 * RoadSideUnitManager implementation
 */
impl RoadSideUnitManager {
    /**
     * Creates a new RoadSideUnitManager.
     */
    pub fn new(params: RsuManagerParams) -> RoadSideUnitManager {
        RoadSideUnitManager {
            next_id: 0,
            tx_range: params.tx_range,
            rx_range: params.rx_range,
            rsus: HashMap::new(),
            current_round: 0,
            obu_observations: Vec::new(),
            detect_obu_tx_failure: params.detect_obu_tx_failure,
            detect_obu_gps_failure: params.detect_obu_gps_failure,
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
    pub fn get_tx_range(&self) -> u32 {
        let tx_range = self.tx_range;
        tx_range
    }

    /**
     * Return the rx range.
     */
    pub fn get_rx_range(&self) -> u32 {
        let rx_range = self.rx_range;
        rx_range
    }

    /**
     * Creates a new RoadSideUnit
     */
    pub fn create_rsu(&mut self, coordinate: Coordinate) -> u32 {
        // Get the next available id
        let id = self.next_id;

        // Create a new rsu
        let rsu = RoadSideUnit::new(id, coordinate);

        // Insert rsu in the hashmap
        self.rsus.insert(id, rsu);

        // Increment id counter
        self.next_id += 1;

        // return the id of the created rsu
        id
    }

    /**
     * Set the current round.
     */
    pub fn set_current_round(&mut self, round: u32) {
        self.current_round = round;
    }

    /**
     * Deliver messages to RSUs.
     */
    pub fn deliver_messages(&mut self, messages: &Vec<Message>) {
        // Iterate over all RSUs
        for rsu in self.rsus.values_mut() {
            // clear the neighbors
            rsu.clear_neighbors();

            // Iterate over all messages
            for message in messages {
                // Check if the message can reach the rsu
                if Ether::is_transmission_possible(
                    message.phy_coord,
                    message.phy_range,
                    rsu.get_coordinate(),
                ) {
                    rsu.receive_message(message.clone());
                }
            }
        }

        // update the obu observations
        self.update_obu_observations();
    }

    /**
     * Update OBU observations.
     */
    fn update_obu_observations(&mut self) {
        // Check if the current round is already in the vector
        // comparing the number of elements in the vector with the current round number
        if self.obu_observations.len() == (self.current_round as usize + 1) {
            panic!("The current round is already in the vector!");
        }

        // create a new hashmap for the round data
        let mut round_data: HashMap<u32, Vec<ObuData>> = HashMap::new();

        // iterate over all rsus
        for rsu in self.rsus.values_mut() {
            // iterate over all neighbors
            for neighbor in rsu.get_neighbors() {
                // create the obu data
                let obu_data = ObuData {
                    coordinate: neighbor.coordinate,
                    rsu_id: rsu.get_id(),
                };

                // check if obu id is in the hashmap
                if round_data.contains_key(&neighbor.id) {
                    // add the obu data to the vector
                    round_data.get_mut(&neighbor.id).unwrap().push(obu_data);
                } else {
                    // create a new vector and add the obu data
                    round_data.insert(neighbor.id, vec![obu_data]);
                }
            }
        }

        // add the round data to the vector
        self.obu_observations.push(round_data);
    }fn update_obu_observations

    /**
     * Check OBUs observations.
     */
    pub fn find_faulty_obus(&self) -> Vec<u32> {
        struct ObuErrorStats {
            tx_count: u32,
            tx_error_count: u32,
            tx_error_rate: f32,
            gps_error_count: u32,
            gps_error_rate: f32,
            first_seen: u32,
        }

        let mut error_stats: HashMap<u32, ObuErrorStats> = HashMap::new();
        let mut tx_errors: Vec<f32> = Vec::new();
        let mut gps_errors: Vec<f32> = Vec::new();

        // Iterate over all rounds
        for i in 0..self.obu_observations.len() {
            // Get round data
            let round_data = &self.obu_observations[i];

            // Iterate over round data
            for (obu_id, obu_data) in round_data.iter() {
                // Create a new entry in the hashmap if it doesn't exist
                let stats = error_stats.entry(*obu_id).or_insert_with(|| ObuErrorStats {
                    tx_count: 0,
                    tx_error_count: 0,
                    tx_error_rate: 0.0,
                    gps_error_count: 0,
                    gps_error_rate: 0.0,
                    first_seen: i as u32,
                });

                stats.tx_count += 1;

                // Iterate over all obu data
                for data in obu_data.iter() {
                    let rsu_id = data.rsu_id;
                    let rsu_coordinate = self.rsus.get(&rsu_id).unwrap().get_coordinate();

                    // Check if the OBU was outside the range of the RSU when the message was sent
                    if !Ether::is_transmission_possible(
                        data.coordinate,
                        self.rx_range + 3, // FIXME: i need a better value for this
                        rsu_coordinate,
                    ) {
                        // Update the gps error count
                        stats.gps_error_count += 1;

                        // one error per round
                        break;
                    }
                }
            }
        }

        // Get the number of rounds
        let rounds = self.obu_observations.len() as u32;

        // iterate over all rx stats
        for (_, stats) in error_stats.iter_mut() {
            // Calculate the number of messages missing since the first round
            // the OBU was seen
            stats.tx_error_count = rounds - (stats.tx_count + stats.first_seen);

            // Calculate the error rates
            stats.tx_error_rate = stats.tx_error_count as f32 / self.current_round as f32;
            stats.gps_error_rate = stats.gps_error_count as f32 / stats.tx_count as f32;

            // Add the error rate to the vector
            tx_errors.push(stats.tx_error_rate);
            gps_errors.push(stats.gps_error_rate);
        }

        // Calculate the median absolute deviation
        let ce = 3.0;
        let tx_error_mad = RoadSideUnitManager::calculate_mad(&tx_errors);
        let gps_error_mad = RoadSideUnitManager::calculate_mad(&gps_errors);

        // Calculate the rx error threshold
        tx_errors.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let median = tx_errors[tx_errors.len() / 2 - 1];
        let tx_threshold: f32 = median + ce * tx_error_mad;

        // Calcule the gps error threshold
        gps_errors.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let median = gps_errors[gps_errors.len() / 2 - 1];
        let gps_threshold: f32 = median + ce * gps_error_mad;

        // A vector to store the faulty obus
        let mut faulty_obus: Vec<u32> = Vec::new();

        // Create/Trunc reputation file
        let mut file = File::create("reputation.csv").expect("Failed to create file");
        file.write_all(b"OBU #,TX Error,TX Rep,GPS Error,GPS Rep,Reputation\n")
            .expect("Failed to write to file");

        println!("--- Fauty OBUs identified by the RSUs ---");
        println!("ID \ttx_error\tgps_error");
        for (obu_id, stats) in error_stats.iter() {
            // Ignore OBUs with a rx error rate below the threshold

            let tx_reputation = stats.tx_error_rate / tx_threshold;
            let gps_reputation = stats.gps_error_rate / gps_threshold;

            let tx_reputation_class = if tx_reputation >= 1.0 {
                0 // red
            } else if tx_reputation >= 0.6 {
                1 // yellow
            } else {
                2 // green
            };

            let gps_repuation_class = if gps_reputation >= 1.0 {
                0 // red
            } else if gps_reputation >= 0.6 {
                1 // yellow
            } else {
                2 // green
            };

            // final reputation is the worst of the two
            let reputation = tx_reputation_class.min(gps_repuation_class);

            write!(
                file,
                "{},{},{},{},{},{}\n",
                obu_id,
                stats.tx_error_rate,
                tx_reputation_class,
                stats.gps_error_rate,
                gps_repuation_class,
                reputation
            )
            .expect("Failed to write to file");

            if (!self.detect_obu_tx_failure || stats.tx_error_rate < tx_threshold)
                && (!self.detect_obu_gps_failure || stats.gps_error_rate < gps_threshold)
            {
                // not detected as faulty
                continue;
            }

            // Print OBU id
            print!("{:03}", obu_id);

            // If obu tx failure detection is enabled
            if self.detect_obu_tx_failure && stats.tx_error_rate >= tx_threshold {
                // Print the detected tx error rate
                print!(
                    "\t{:2} {:5.2}% ",
                    stats.tx_error_count,
                    stats.tx_error_rate * 100.0
                );
            } else {
                print!("\t\t");
            }

            // If obu gps failure detection is enabled
            if self.detect_obu_gps_failure && stats.gps_error_rate >= gps_threshold {
                // Print the detected gps error rate
                print!(
                    "\t{:2} {:5.2}% ",
                    stats.gps_error_count,
                    stats.gps_error_rate * 100.0
                );
            }

            // Print new line
            println!();

            // add the obu id to the vector
            faulty_obus.push(*obu_id);
        }

        // return the vector with the faulty obus
        faulty_obus
    }fn find_faulty_obus

    /**
     * Calculate the Median Absolute Deviation (MAD) for a vector of f32 values.
     */
    pub fn calculate_mad(values: &[f32]) -> f32 {
        // Clone and sort the values
        let mut sorted_values = values.to_vec();
        sorted_values.sort_by(|a, b| a.partial_cmp(b).unwrap());

        // Calculate the median
        let median = if sorted_values.len() % 2 == 0 {
            let mid1 = sorted_values.len() / 2 - 1;
            let mid2 = sorted_values.len() / 2;
            (f64::from(sorted_values[mid1]) + f64::from(sorted_values[mid2])) as f32 / 2.0
        } else {
            sorted_values[sorted_values.len() / 2]
        };

        // Calculate the absolute deviations
        let mut abs_devs: Vec<f32> = sorted_values
            .into_iter()
            .map(|value| (f64::from(value) - f64::from(median)).abs() as f32)
            .collect();

        // Sort the absolute deviations
        abs_devs.sort_by(|a, b| a.partial_cmp(b).unwrap());

        // Calculate the median absolute deviation
        let mad = if abs_devs.len() % 2 == 0 {
            let mid1 = abs_devs.len() / 2 - 1;
            let mid2 = abs_devs.len() / 2;
            (f64::from(abs_devs[mid1]) + f64::from(abs_devs[mid2])) as f32 / 2.0
        } else {
            abs_devs[abs_devs.len() / 2]
        };

        // Return MAD with the scaling factor (1 / phi(3/4)) for a normal distribution
        mad * 1.4826
    }fn calculate_mad
}impl RoadsideUnitManager

/***
 * TESTS MODULE
 */
#[cfg(test)]
mod tests {

    use super::*;
    use crate::grid::Coordinate;

    /**
     * Test the creation of an RoadSideUnitManager.
     */
    #[test]
    fn test_create_rsu_manager() {
        let params = RsuManagerParams {
            tx_range: 5,
            rx_range: 5,
            detect_obu_gps_failure: false,
            detect_obu_tx_failure: false,
        };

        let rsu_manager = RoadSideUnitManager::new(params);
        assert_eq!(rsu_manager.next_id, 0);
        assert_eq!(rsu_manager.tx_range, 5);
        assert_eq!(rsu_manager.rsus.len(), 0);
    }

    /**
     * Test the creation of an RoadSideUnit.
     */
    #[test]
    fn test_create_rsu() {
        let params = RsuManagerParams {
            tx_range: 5,
            rx_range: 5,
            detect_obu_gps_failure: false,
            detect_obu_tx_failure: false,
        };

        let mut rsu_manager = RoadSideUnitManager::new(params);
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
    }fn test_create_rsu

    /**
     * Test the calculation of the median absolute deviation.
     */

    #[test]
    fn test_calculate_mad_1() {
        // Create a vector of f32 values.
        let values = vec![1.23, 3.56, 7.89, 0.12, 4.56, 9.01, 2.34, 6.78, 10.12, 5.67];

        // Calculate the expected median absolute deviation.
        let expected_mad = 4.115;

        // Calculate the actual median absolute deviation.
        let actual_mad = RoadSideUnitManager::calculate_mad(&values);

        // Check that the actual median absolute deviation is within 0.001 of the expected value.
        assert!((actual_mad - expected_mad).abs() < 0.001);
    }

    #[test]
    fn test_calculate_mad_2() {
        // Create a vector of f32 values.
        let values = vec![
            125.36, 378.58, 912.45, 271.67, 436.89, 783.32, 158.98, 598.21, 844.76, 315.64,
        ];

        // Calculate the expected median absolute deviation.
        let expected_mad = 325.59;

        // Calculate the actual median absolute deviation.
        let actual_mad = RoadSideUnitManager::calculate_mad(&values);

        // Check that the actual median absolute deviation is within 0.1 of the expected value.
        assert!((actual_mad - expected_mad).abs() < 0.1);
    }

    #[test]
    fn test_calculate_mad_3() {
        // Create a vector of f32 values.
        let values = vec![
            0.0123, 0.0456, 0.0789, 0.1123, 0.1456, 0.1789, 0.2123, 0.2456, 0.2789, 0.3123,
        ];

        // Calculate the expected median absolute deviation.
        let expected_mad = 0.123938;

        // Calculate the actual median absolute deviation.
        let actual_mad = RoadSideUnitManager::calculate_mad(&values);

        // Check that the actual median absolute deviation is within 0.001 of the expected value.
        assert!((actual_mad - expected_mad).abs() < 0.001);
    }

    // TODO: Move message deliver tests from simulator.rs to here
}mod tests
