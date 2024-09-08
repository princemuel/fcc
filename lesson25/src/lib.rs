pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for c in self.components.iter() {
            c.draw();
        }
    }
}

pub struct Button {
    pub width: u8,
    pub height: u8,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {}
}

// This is more retrictive due to generics.
// only allows one type for the components
// pub struct Screen<T: Draw> {
//     pub components: Vec<T>,
// }

// impl<T> Screen<T>
// where
//     T: Draw,
// {
//     pub fn run(&self) {
//         for c in self.components.iter() {
//             c.draw();
//         }
//     }
// }
