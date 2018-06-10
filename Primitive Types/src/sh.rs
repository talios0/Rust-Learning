use std::mem;

struct Point { // custom data structure
    x: f64,
    y: f64
}

fn origin() -> Point { // returns a Point
    Point {x:0.0, y:0.0}
}

pub fn stack_and_heap() {
    let p1 = origin(); // calls origin on the stack
    let p2 = Box::new(origin()); // calls origin on the heap

    println!("p1 takes up {} bytes", mem::size_of_val(&p1));
    println!("p2 takes up {} bytes", mem::size_of_val(&p2));

    let p3 = *p2; // * follows the pointer
    println!("{}", p3.x);
}