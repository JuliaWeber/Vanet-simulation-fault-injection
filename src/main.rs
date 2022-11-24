use vanet4j::simulator::Simulator;

fn main() {
 
    let mut simulator = Simulator::new(3, 2);

    simulator.add_on_board_unit();
    simulator.add_on_board_unit();
    simulator.add_on_board_unit();
    simulator.add_on_board_unit();
    simulator.add_on_board_unit();
    simulator.run();
}
