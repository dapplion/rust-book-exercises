trait Draw {
    fn draw(&self);
}

pub struct Screen {
    components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!("drawing button {}", self.label);
    }
}

// Library consumer

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!("drawing selectbox {:?}", self.options);
    }
}

fn main() {
    let button = Button {
        width: 1,
        height: 1,
        label: String::from("Send"),
    };
    let select_box = SelectBox {
        width: 1,
        height: 1,
        options: vec![String::from("opt A"), String::from("opt B")],
    };

    let screen = Screen {
        components: vec![Box::new(button), Box::new(select_box)],
    };
    screen.run()
}
