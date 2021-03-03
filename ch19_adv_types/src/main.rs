type Kilometers = i32;
type Meters = i32;

fn sumKm(a: Kilometers, b: Kilometers) -> Kilometers {
    a + b
}

// ! is the never type
fn always_panics() -> ! {
    panic!();
}

fn infinite_loop() -> ! {
    loop {
        println!("infinite");
        if false {
            // ▼ WONT COMPILE
            // break;
        }
    }
}

// Dynamically Sized Types and the Sized Trait

fn generic<T: ?Sized>(t: &T) {}

// Function Pointers

fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

enum Status {
    Value(u32),
    Stop,
}

fn returns_closure() -> Box<dyn Fn(i32) -> Box<dyn Fn(i32) -> i32>> {
    Box::new(|x| Box::new(move |y| x + y))
}

fn main() {
    let a: Kilometers = 1;
    let b: Meters = 2;

    println!("sumKm: {}", sumKm(a, b));

    println!("The answer is: {}", do_twice(add_one, 5));

    let list_of_statuses: Vec<Status> = (0u32..20).map(Status::Value).collect();

    println!("curried sum: {}", returns_closure()(4)(6))
}
