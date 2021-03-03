enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        match self {
            Message::Quit => {
                println!("Quit")
            }
            Message::Move { x, y } => {
                println!("move to {} {}", x, y)
            }
            Message::Write(s) => {
                println!("write {}", s)
            }
            Message::ChangeColor(a, b, c) => {
                println!("color {} {} {}", a, b, c)
            }
        }
        // method body would be defined here
    }
}

fn main() {
    let mut m = Message::Quit;
    m.call();

    m = Message::Move { x: 1, y: 2 };
    m.call();

    m = Message::Write(String::from("hello"));
    m.call();

    m = Message::ChangeColor(255, 0, 255);
    m.call();
}
