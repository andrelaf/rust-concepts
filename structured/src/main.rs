
enum Fruit {
    Apple,
    Banana,
    Mango,
    Orange,
    Pineapple,
    Strawberry,
}

enum Cordinates {
    TwoD(i32, i32),
    ThreeD(i32, i32, i32),
}

// Its same a class in other programming languages
// Struct has no method
struct People {
    name: String,
    age: i8,
    height: f32
}

struct Rectangle {
    width: u32,
    height: u32
}

// Trait is same as interface in other programming languages
trait GeometricForm {
    fn area(&self) -> u32;
    fn new(width: u32, height: u32) -> Self;
}

// Impl add comportment to struct
impl GeometricForm for Rectangle {
    
    // Self is same as this in other programming languages
    // Self is receive first parameter of the method and using reference of the struct
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }
}




fn main() {
    enumeration(Fruit::Apple);
    enumeration(Fruit::Banana);
    enumeration(Fruit::Mango);
    enumeration(Fruit::Orange);
    enumeration(Fruit::Pineapple);
    enumeration(Fruit::Strawberry);

    enumeration_with_associated_data();
    struct_example();
    impl_example();

}

fn enumeration_with_associated_data() {
    let two_d = Cordinates::TwoD(10, 20);
    let three_d = Cordinates::ThreeD(10, 20, 30);

    match two_d {
        Cordinates::TwoD(x, y) => println!("TwoD: x = {}, y = {}", x, y),
        Cordinates::ThreeD(_, _, _) => println!("ThreeD"),
    }

    match three_d {
        Cordinates::TwoD(_, _) => println!("TwoD"),
        Cordinates::ThreeD(x, y, z) => println!("ThreeD: x = {}, y = {}, z = {}", x, y, z),
    }

    struct_example();
}

fn enumeration(fruit: Fruit){
    
    match fruit {
        Fruit::Apple => println!("Apple"),
        Fruit::Banana => println!("Banana"),
        Fruit::Mango => println!("Mango"),
        Fruit::Orange => println!("Orange"),
        Fruit::Pineapple => println!("Pineapple"),
        Fruit::Strawberry => println!("Strawberry"),
    }
}

fn struct_example() {
    let person = People {
        name: String::from("John Doe"),
        age: 25,
        height: 5.8
    };

    println!("Name: {}, Age: {}, Height: {}", person.name, person.age, person.height)
}

fn impl_example(){
    
    //:: is same static method in other programming languages
    let rectangle = Rectangle::new(10, 20);

    let area = rectangle.area();

    println!("Area: {}", area);

}