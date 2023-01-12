#[derive(Debug)]
pub struct Players {
    pub number : i32,
    pub names : Vec<String>
}

impl Players {
    pub fn new(n: i32) -> Self {
    Players{number : n,
            names : Vec::new()}
    }
}
