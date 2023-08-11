use std::fmt;

const GRID_SIZE: usize = 10;
struct Grid {
    cells: [[bool; GRID_SIZE];  GRID_SIZE],
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let grid_display = self.cells.iter()
            .map(|row| row.iter()
                .map(|&val| if val { "*" } else { " " })
                .collect::<Vec<&str>>()
                .join(" "))
            .collect::<Vec<String>>()
            .join("\n");
        write!(f, "{}", grid_display)
    }
}


fn main() {
    let mut grid = Grid { cells: [[true; GRID_SIZE]; GRID_SIZE]};
    println!("Unmodified:\n{}", grid);
    grid.cells[0][3] = false;
    println!("Modified:\n{}", grid);


}