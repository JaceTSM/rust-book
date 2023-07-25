fn main() {
    // can't do Rectangle(11, 12) it seems :/
    let r = Rectange { width: 11, height: 12};
    println!("{:?}: Area={}", r , r.area());

    let p = Point(1.0,2.0,3.0);
    println!("Point: {:?}", p);
}

#[derive(Debug)]
struct Rectange {
    width: u32,
    height: u32,
}

impl Rectange {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

#[derive(Debug)]
struct Point(f64, f64, f64);


#[cfg(test)]
mod tests{
    #[test]
    fn test_the_first() {
        println!("First test!");
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test_the_second() {
        println!("Second test!");
        assert_eq!(2 + 3, 5);
    }

    #[test]
    fn test_again() {
        println!("Three hops this time!");
        assert_eq!(1 + 2, 3);
    }
}
