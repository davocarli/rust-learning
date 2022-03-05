
pub trait Component {
    fn draw(&self);
}

pub trait Clickable {
    fn onclick(&self);
}

// IMPLEMENTED WITH TRAITS
pub struct Screen {
    pub components:  Vec<Box<dyn Component>>,
}

impl Screen {
    pub fn run(&self) {
        for comp in self.components.iter() {
            comp.draw();
        }
    }
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Component for Button {
    fn draw(&self) {
        println!(
            "Drawing button of width {}, height {}, label {}.",
            self.width,
            self.height,
            self.label
        );
    }
}

impl Clickable for Button {
    fn onclick(&self) {
        println!("{} button was clicked!", self.label);
    }
}

// IMPLEMENTED WITH GENERICS
// DOES NOT ALLOW FOR MULTIPLE COMPONENT TYPES
// IF YOU HAVE A LIST OF ONLY ONE TYPE, GENERICS
// ARE PREFERABLE BECAUSE THEY HAVE LOWER OVERHEAD
// pub struct Screen<T: Component> {
//     pub components: Vec<T>,
// }

// impl<T> Screen<T>
// where T: Component, {
//     pub fn run(&self) {
//         for component in self.components.iter() {
//             component.draw();
//         }
//     }
// }
