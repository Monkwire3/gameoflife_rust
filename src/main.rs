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
}


fn main() {
    let mut grid = Grid { cells: [[true; GRID_SIZE]; GRID_SIZE]};
    println!("Unmodified:\n{}", grid);
    grid.randomize_cell_values();
    println!("Modified:\n{}", grid);


}