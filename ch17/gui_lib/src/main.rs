use gui_lib::{Component, Screen, Button};

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Component for SelectBox {
    fn draw(&self) {
        println!(
            "Select box of width {}, height {}, with options {:?}",
            self.width,
            self.height,
            self.options,
        )
    }
}

fn main() {
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 100,
                height: 100,
                options: vec![
                    String::from("A"),
                    String::from("B"),
                    String::from("C"),
                ]
            }),
            Box::new(Button {
                width: 100,
                height: 100,
                label: String::from("Submit"),
            })
        ]
    };

    screen.run();
}
