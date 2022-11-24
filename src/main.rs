use vanet4j::grid::{Grid};

fn main() {
    // create a grid with 3 blocks per street and 2 cells per block

    println!("Creating grid...");
    let mut grid = Grid::new(3, 2);
    println!("Grid created!");

    println!("Updating next possible coordinates for each cell...");
    grid.update_next_coordinates();
    println!("Updated!");

    // print the grid
    //grid.print_grid_with_cells_ids();

    grid.print_grid_with_cells_coordinates();
}
