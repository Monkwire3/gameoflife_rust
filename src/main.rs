use std::fmt;
use rand::Rng;

const GRID_SIZE: usize = 100;
struct Grid {
    cells: [[bool; GRID_SIZE];  GRID_SIZE],
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.cells.iter()
            .map(|row| row.iter()
                .map(|&val| if val { "\u{25A0}" } else { " " })
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

    fn get_next_state(&mut self) -> [[bool; GRID_SIZE]; GRID_SIZE] {
        let mut next_state = [[false; GRID_SIZE];  GRID_SIZE];

        for y in 0..GRID_SIZE {
            for x in 0..GRID_SIZE {
                let live_neighboors = self.count_live_neighboors(y, x);
                match live_neighboors {
                    3 => next_state[y][x] = true,
                    2 => next_state[y][x] = self.cells[y][x],
                    _ => continue
                }
            }
        }
        next_state
    }
}




fn main() {
    let mut grid = Grid { cells: [[false; GRID_SIZE]; GRID_SIZE]};
    grid.randomize_cell_values();


    for _ in 0..100 {
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        println!("{}", grid);
        grid.cells = grid.get_next_state();
        std::thread::sleep(std::time::Duration::from_secs(1));

    }
}