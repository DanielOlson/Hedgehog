
#[derive(Clone, Copy, Debug, Default)]
// You can add stuff here if you want
pub struct Cell {
    pub x: u32,
    pub y: u32,
    pub last_voltage: f32,
    pub voltage: f32,
    pub raw_val: f32,
}

impl Cell {
    // Update this for early cell updates
    pub fn pre_update(&mut self) {
        self.last_voltage = self.voltage;
    }


    // Update this for late cell updates
    pub fn post_update(&mut self) {
        self.raw_val += (self.voltage * self.voltage);
        self.voltage = self.raw_val.sin();
    }

    // Change this if you want to change the color scheme
    pub fn color(&self) -> [u8; 4] {
        let col = (((self.voltage + 1.0) / 2.0) * 255.0) as u8;

        return [col / 3, col / 2, col, 255];
    }
}