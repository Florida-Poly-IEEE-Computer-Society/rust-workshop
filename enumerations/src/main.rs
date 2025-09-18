fn main() {
    #[derive(Debug)]
    enum Animal {
        Dog,
        Cat,
    }

    let mut a: Animal = Animal::Dog;
    a = Animal::Cat;
    println!("{:?}", a);
}
