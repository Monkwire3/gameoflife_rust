use std::fmt;

const GRID_SIZE: usize = 10;
struct Grid {
    cells: [[bool; GRID_SIZE];  GRID_SIZE],
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let grid_display = self.cells.iter().map(|row| row.iter().map(ToString::to_string).collect::<Vec<String>>().join(" ")).collect::<Vec<String>>().join("\n");
        write!(f, "{}", grid_display)
    }
    
}


fn main() {
    let grid = Grid { cells: [[false; GRID_SIZE]; GRID_SIZE]};
    println!("{}", grid);

}