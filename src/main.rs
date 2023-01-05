use vanet4j::simulator::Simulator;
use vanet4j::grid::GridParams;
use vanet4j::rsu_manager::RsuManagerParams;
use vanet4j::obu_manager::ObuManagerParams;

fn main() {

    let grid_params = GridParams {
        blocks_per_street: 25,
        block_size: 3,
    };

    let rsu_manager_params = RsuManagerParams {
        comms_range: 5,
    };

    let obu_manager_params = ObuManagerParams {
        max_obus: 120,
        comms_range: 2,
        tx_base_failure_rate: 0.01,
        tx_faulty_obu_failure_rate: 0.1,
        faulty_obus: 20,
    };

    let mut simulator = Simulator::new(grid_params, rsu_manager_params, obu_manager_params);

    simulator.init();

    simulator.run(180);

}
