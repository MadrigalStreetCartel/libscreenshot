pub struct Area {
    pub x: i64,
    pub y: i64,
    pub width: u64,
    pub height: u64,
}

impl Area {
    pub fn new(x: i64, y: i64, width: u64, height: u64) -> Self {
        Area {
            x,
            y,
            width,
            height,
        }
    }
}
