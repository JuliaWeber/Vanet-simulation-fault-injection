use vanet4j::grid::{Coordinate, Grid};

fn main() {
    // create a grid with 3 blocks per street and 2 cells per block

    println!("Creating grid...");
    let mut grid = Grid::new(100, 10);
    println!("Grid created!");

    println!("Updating next possible coordinates for each cell...");
    grid.update_next_coordinates();
    println!("Updated!");

    println!("1");
    grid.get_next_coordinates(Coordinate { x: 0, y: 0 });
    println!("2");
    grid.get_next_coordinates(Coordinate { x: 1000, y: 260 });
    println!("done");


    // print the grid
    //grid.print_grid_with_cells_ids();

    //grid.print_grid_with_cells_coordinates();
}
