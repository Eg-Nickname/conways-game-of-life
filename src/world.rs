use rand::Rng;
use crate::{WIDTH, HEIGHT};

const DEFAULT_SATURATION: u8 = 192;

enum CellState{
    Alive,
    Dead,
}

pub struct World{
    cells: Vec<Vec<CellState>>,
    scratch_cells: Vec<Vec<CellState>>,
    saturation: Vec<Vec<u8>>,
}

// struct CopiedStructure{
//     cells: Vec<Vec<CellState>>,
// }
    
impl World {
    pub fn new() -> Self {
        Self{
            cells: Vec::new(),
            scratch_cells: Vec::new(),
            saturation: Vec::new(),
        }
    }

    pub fn generate_world(&mut self, empty: bool){
        let mut new_cells: Vec<Vec<CellState>> = Vec::new();
        let mut new_empty_cells: Vec<Vec<CellState>> = Vec::new();
        let mut saturation: Vec<Vec<u8>> = Vec::new();


        for _i in 0..=WIDTH{
            let mut x_line: Vec<CellState> = Vec::new();
            let mut empty_line: Vec<CellState> = Vec::new();
            let mut saturation_line: Vec<u8> = Vec::new();

            for _j in 0..=HEIGHT{
                let state = match rand::thread_rng().gen_range(0..3){
                    0 => CellState::Dead,
                    _ => CellState::Alive,
                };
                x_line.push(state);
                empty_line.push(CellState::Dead);
                saturation_line.push(0)
            }
            new_cells.push(x_line);
            new_empty_cells.push(empty_line);
            saturation.push(saturation_line)
        }

        self.cells = new_cells;
        self.scratch_cells = new_empty_cells;
        self.saturation = saturation;

        if empty{
            std::mem::swap(&mut self.scratch_cells, &mut self.cells);
        }
        
    }

    pub fn update(&mut self){
        for x in 0..=WIDTH{
            for y in 0..=HEIGHT{
                let neighbours = self.count_neighbours(x as usize, y as usize);
                let is_alive: bool = match self.cells[x as usize][y as usize] {
                    CellState::Alive => true,
                    CellState::Dead => false,
                    
                };

                if is_alive{
                    if neighbours < 2 || neighbours > 3{
                        self.scratch_cells[x as usize][y  as usize] = CellState::Dead;
                        self.saturation[x  as usize][y  as usize] = DEFAULT_SATURATION;
                    }else{
                        self.scratch_cells[x  as usize][y  as usize] = CellState::Alive;
                    }
                }else{
                    if neighbours == 3{
                        self.scratch_cells[x  as usize][y  as usize] = CellState::Alive;
                    }else{
                        self.scratch_cells[x  as usize][y  as usize] = CellState::Dead;
                        if self.saturation[x  as usize][y  as usize] > 0{
                            self.saturation[x  as usize][y  as usize] -= 2;
                        }
                    }
                }
            }
        }

        std::mem::swap(&mut self.scratch_cells, &mut self.cells);
    }

    fn count_neighbours(&self, x: usize, y: usize) -> u8{
        let mut neighbours: u8 = 0;
        let x2 = x as i32;
        let y2 = y as i32;

        if (x2-1) >= 0 && (y2-1) >=0{
            neighbours += match self.cells[x-1][y-1] {
                CellState::Alive => 1,
                CellState::Dead => 0,
            };
        }
        if (y2-1) >=0{
            neighbours += match self.cells[x][y-1] {
                CellState::Alive => 1,
                CellState::Dead => 0,
            };
        }
        if ((x+1) as i32) < WIDTH as i32 && (y2-1) >=0{
            neighbours += match self.cells[x+1][y-1] {
                CellState::Alive => 1,
                CellState::Dead => 0,
            };
        }
        if (x2-1) >= 0{
            neighbours += match self.cells[x-1][y] {
                CellState::Alive => 1,
                CellState::Dead => 0,
            };
        }
        if ((x+1) as u32) < WIDTH{
            neighbours += match self.cells[x+1][y] {
                CellState::Alive => 1,
                CellState::Dead => 0,
            };
        }
        if (x2-1) >= 0 && ((y+1) as u32) < HEIGHT{
            neighbours += match self.cells[x-1][y+1] {
                CellState::Alive => 1,
                CellState::Dead => 0,
            };
        }
        if((y+1) as u32) < HEIGHT{
            neighbours += match self.cells[x][y+1] {
                CellState::Alive => 1,
                CellState::Dead => 0,
            };
        }

        if ((x+1) as u32) < WIDTH && ((y+1) as u32) < HEIGHT{
            neighbours += match self.cells[x+1][y+1] {
                CellState::Alive => 1,
                CellState::Dead => 0,
            };
        }
        neighbours
    }

    pub fn change_cell(&mut self, mouse_pos: Option<(f32,f32)>){
        let  pos = match mouse_pos {
            Option::Some(val) => val,
            Option::None => (0.0, 0.0),
        };
        let (x, y) = pos;
        println!("x: {} y: {}", (x/4.0).floor(), (y/4.0).floor());
        self.cells[(x/4.0).floor() as usize][(y/4.0).floor() as usize] = match self.cells[(x/4.0).floor() as usize][(y/4.0).floor() as usize]{
            CellState::Alive =>  CellState::Dead,
            CellState::Dead => CellState::Alive,
        };
    }
    pub fn copy_structure(&self){
        todo!()
    }

    pub fn paste_structure(&self){
        todo!()
    }

    pub fn draw(&self, frame: &mut [u8]) {
        for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
            let x = (i % WIDTH as usize) as usize;
            let y = (i / WIDTH as usize) as usize;

            let rgba = match self.cells[x][y]{
                CellState::Alive => [255, 255, 255, 0xf0],
                CellState::Dead => [255, self.saturation[x][y], 0, self.saturation[x][y]],
            };
            pixel.copy_from_slice(&rgba);
        }
    }
}