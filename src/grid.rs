
use crate::cell::Cell;

pub struct Grid{
    width: usize,
    height: usize,
    cells: Vec<Cell>
}

impl Grid {

    pub fn new(width: usize, height: usize) -> Grid {
        let mut new_cells = vec![Cell::default(); width * height];

        // Change this if you want to change initial conditions
        for x in 0..width {
            for y in 0..height {
                new_cells[(y * width) + x].x = x as u32;
                new_cells[(y * width) + x].y = y as u32;
                new_cells[(y * width) + x].voltage = ((x as f32).cos() + (y as f32).sin()).sin();
                new_cells[(y * width) + x].last_voltage = new_cells[(y * width) + x].voltage;
            }
        }

        return Grid {
            width: width,
            height: height,
            cells: new_cells};
    }

    pub fn update_grid(&mut self) {
        for i in 0..self.cells.len() {
            self.cells[i].pre_update();


            // Change this stuff if you want to change how cells gather information from their neighbors
            let mut vsum = 0.0;
            for x in 1..=3 {
                for y in 1..=3 {
                    let x = (x as i32) - 2 + (self.cells[i].x as i32);
                    let y = (y as i32) - 2 + (self.cells[i].y as i32);
                    {
                        if let Some(p) = Grid::get_valid_cell(self.width,
                                                             self.height,
                                                             x,
                                                             y) {
                            vsum += self.cells[p].last_voltage;
                        }
                    }
                }
            }
            vsum = vsum / 18.0;
            self.cells[i].voltage = (self.cells[i].voltage / 2.0) + vsum;


            self.cells[i].post_update();
        }
    }

    pub fn get_valid_cell(width: usize, height: usize, x: i32, y:i32) -> Option<usize>{
        if x < 0 || (x as usize) >= width {
            return None;
        }

        if y < 0 || (y as usize) >= height {
            return None;
        }

        let p = (y as usize * width as usize) + x as usize;
        return Some(p);
    }

    pub fn draw_grid(&self, screen: &mut [u8]) {
        debug_assert_eq!(screen.len(), self.cells.len() * 4);
        for (cell, pixel) in self.cells.iter().zip(screen.chunks_exact_mut(4)) {
            pixel.copy_from_slice(&cell.color());
        }
    }


}

