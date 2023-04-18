
#[derive(Clone, Copy, Debug, Default)]
pub struct Cell {
    pub x: u32,
    pub y: u32,
    pub last_voltage: f32,
    pub voltage: f32,
}

impl Cell {
    // Update this for early cell updates
    pub fn pre_update(&mut self) {
        self.last_voltage = self.voltage;
    }


    // Update this for late cell updates
    pub fn post_update(&mut self) {
        self.voltage += (self.y as f32).sin() + (self.x as f32).cos() / 5.0;
        self.voltage = (self.voltage + self.last_voltage) / 2.2;

    }

    // Change this if you want to change the color scheme
    pub fn color(&self) -> [u8; 4] {
        let col = (((self.voltage + 1.0) / 2.0) * 255.0) as u8;

        return [col / 3, col / 2, col, 255];
    }
}