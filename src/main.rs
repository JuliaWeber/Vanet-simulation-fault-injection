use vanet4j::grid::GridParams;
use vanet4j::obu_manager::ObuManagerParams;
use vanet4j::rsu_manager::RsuManagerParams;
use vanet4j::simulator::Simulator;

fn main() {
    let grid_params = GridParams {
        blocks_per_street: 25,
        block_size: 3,
    };

    let rsu_manager_params = RsuManagerParams {
        tx_range: 5, // how far can the RSU transmit?
        rx_range: 5, // this will affect the spacing between RSUs
        detect_obu_tx_failure: true,
        detect_obu_gps_failure: false,
    };

    let obu_manager_params = ObuManagerParams {
        max_obus: 120,
        comms_range: 6, // at least RSU rx_range + 1 
        tx_base_failure_rate: 0.02,
        tx_faulty_obu_failure_rate: 0.05,
        gps_failure_rate: 0.02, 
        gps_faulty_obu_failure_rate: 0.05,
        faulty_obus: 20,
    };

    let mut simulator = Simulator::new(grid_params, rsu_manager_params, obu_manager_params);

    simulator.init();

    simulator.run(180);
}
