use draw::Draw;
use draw::{Button, Screen};

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        // code to actually draw a select box
    }
}

fn main() {
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![String::from("Yes")],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("ok"),
            }),
        ],
    };
}
