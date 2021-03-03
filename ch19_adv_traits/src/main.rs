// Specifying Placeholder Types in Trait Definitions with Associated Types

struct Counter<T> {
    count: T,
}

impl Counter<u32> {
    fn new() -> Counter<u32> {
        Counter { count: 0 }
    }
}

impl Counter<f64> {
    fn new() -> Counter<f64> {
        Counter { count: 0.0 }
    }
}

impl Iterator for Counter<u32> {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        // --snip--
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

// Default Generic Type Parameters and Operator Overloading

use std::ops::Add;

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

#[derive(Debug, PartialEq)]
struct Millimeters(u32);
struct Meters(u32);

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + other.0 * 1000)
    }
}

// Fully Qualified Syntax for Disambiguation: Calling Methods with the Same Name

trait GetA {
    fn get(&self) -> &str;
}

trait GetB {
    fn get(&self) -> &str;
}

struct Getter;
impl GetA for Getter {
    fn get(&self) -> &str {
        "A"
    }
}
impl GetB for Getter {
    fn get(&self) -> &str {
        "B"
    }
}
impl Getter {
    fn get(&self) -> &str {
        "*"
    }
}

trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

// Using Supertraits to Require One Trait’s Functionality Within Another Trait

use std::fmt;

trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

struct Name {
    name: String,
}

impl OutlinePrint for Name {}

impl fmt::Display for Name {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

// Using the Newtype Pattern to Implement External Traits on External Types

struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

fn main() {
    let counter_u32 = Counter::<u32>::new();
    let counter_f64 = Counter::<f64>::new();

    assert_eq!(
        Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
        Point { x: 3, y: 3 }
    );

    assert_eq!(Millimeters(50) + Meters(1), Millimeters(1050));

    let getter = Getter {};
    println!("getter.get {}", getter.get());
    println!("getter.getA {}", GetA::get(&getter));
    println!("getter.getB {}", GetB::get(&getter));

    println!("A baby dog is called a {}", Dog::baby_name());
    println!("A baby dog is called a {}", <Dog as Animal>::baby_name());

    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w);
}
