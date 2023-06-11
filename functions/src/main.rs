fn main() {
    println!("Hello, world!");

    another_function(5);
    loop_test();
}

fn another_function(x: i32) {
    println!("This is another function! x = {x}");
}

fn loop_test() {
    let mut x = 3;
    while x > 0 {
        println!("{x}!");
        x -= 1;
        println!("???");
    }
    println!("Liftoff!");
}
