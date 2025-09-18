// used to allow our struct to be printed
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// impl block is where we will implement methods
impl Rectangle {
    // constructor like method, returns an instance
    // of specified using Self
    fn new(width: u32, height: u32) -> Self {
        Rectangle { width, height }
    }

    // returns area of Rectangle
    // reference to self, so function does not own self
    fn area(&self) -> u32 {
        // unlike c or c++, we can use . instead of ->
        self.width * self.height
    }
}

fn main() {
    let rect = Rectangle::new(5, 5);
    println!("{rect:?}");
    let area = rect.area();
    println!("The area is {area}");
}
