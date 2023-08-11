use std::fmt;
use rand::Rng;

const GRID_SIZE: usize = 10;
struct Grid {
    cells: [[bool; GRID_SIZE];  GRID_SIZE],
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.cells.iter()
            .map(|row| row.iter()
                .map(|&val| if val { "*" } else { " " })
                .collect::<Vec<&str>>()
                .join(" "))
            .collect::<Vec<String>>()
            .join("\n"))
    }
}

impl Grid {
    pub fn randomize_cell_values( &mut self)  {
        let mut rng = rand::thread_rng();
        for i in 0..GRID_SIZE {
            for j in 0..GRID_SIZE {
                self.cells[i][j] = rng.gen::<bool>();
            }
        }
    }

    fn count_live_neighboors(&self, x_0: usize, y_0: usize) -> i32 {
        let mut live_neighboors = 0;

        for y_1 in [y_0.wrapping_sub(1), y_0, y_0 + 1] {
            for x_1 in [x_0.wrapping_sub(1), x_0, x_0 + 1] {
                if !(y_1 == y_0 && x_1 == x_0) {
                    match self.cells.get(y_1).and_then(|inner| inner.get(x_1)) {
                        Some(val)=> { if *val { live_neighboors += 1 } else { continue} },
                        None => continue,
                    }
                }
            }
        }
        live_neighboors
    }

    // fn mutate_to_next_state(&mut self) {

    // }
}




fn main() {
    let mut grid = Grid { cells: [[false; GRID_SIZE]; GRID_SIZE]};
    println!("Unmodified:\n{}", grid);
    grid.randomize_cell_values();
    println!("Modified:\n{}", grid);
    println!("grid.cells[0][0] neighboors: {}", grid.count_live_neighboors(0, 0));
    println!("grid.cells[1][1] neighboors: {}", grid.count_live_neighboors(1, 1));
    println!("grid.cells[2][2] neighboors: {}", grid.count_live_neighboors(2, 2));



}