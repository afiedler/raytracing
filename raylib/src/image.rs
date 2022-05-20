pub struct Image {
    height: u32,
    width: u32,
    data: Vec<u8>,
}

impl Image {
    pub fn new(width: u32, height: u32) -> Image {
        Image {
            height,
            width,
            data: vec![0; (width * height * 4) as usize],
        }
    }

    pub fn set_line(&mut self, line_number: u32, line: Vec<u8>) {
        for x in 0..self.width {
            let i = x as usize;
            self.set_pixel(
                x,
                line_number,
                (
                    line[4 * i],
                    line[4 * i + 1],
                    line[4 * i + 2],
                    line[4 * i + 3],
                ),
            )
        }
    }

    pub fn set_pixel(&mut self, x: u32, y: u32, rgba: (u8, u8, u8, u8)) {
        let i = (y * self.width + x) as usize;
        self.data[4 * i] = rgba.0;
        self.data[4 * i + 1] = rgba.1;
        self.data[4 * i + 2] = rgba.2;
        self.data[4 * i + 3] = rgba.3;
    }

    pub fn get_pixel(&self, x: u32, y: u32) -> [u8; 4] {
        let i = (y * self.width + x) as usize;
        [
            self.data[4 * i],
            self.data[4 * i + 1],
            self.data[4 * i + 2],
            self.data[4 * i + 3],
        ]
    }
    pub fn width(&self) -> u32 {
        self.width
    }
    pub fn height(&self) -> u32 {
        self.height
    }
    pub fn buf(&self) -> &Vec<u8> {
        &self.data
    }
}
