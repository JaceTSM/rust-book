fn main() {
    let r = Rectange(11, 12);
    println!("{}: Area={}", r, r.area);
}


struct Rectange {
    width: u32,
    height: u32,
}

impl Rectange {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}
