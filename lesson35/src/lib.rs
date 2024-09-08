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

pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    pub fn new() -> Self {
        Self { state: Some(Box::new(Draft {})), content: String::new() }
    }

    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(self)
    }

    pub fn request_review(&mut self) {
        if let Some(state) = self.state.take() {
            self.state = Some(state.request_review());
        }
    }
    pub fn approve(&mut self) {
        if let Some(state) = self.state.take() {
            self.state = Some(state.approve());
        }
    }
}

trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    fn content<'a>(&self, value: &'a Post) -> &'a str {
        ""
    }
}

struct Draft {}
impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
}

struct PendingReview {}
impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published {})
    }
}
struct Published {}
impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn content<'a>(&self, value: &'a Post) -> &'a str {
        &value.content
    }
}

pub struct BlogPost {
    content: String,
}

pub struct DraftPost {
    content: String,
}
pub struct PendingPost {
    content: String,
}

impl BlogPost {
    pub fn new() -> DraftPost {
        DraftPost { content: String::new() }
    }

    pub fn content(&self) -> &str {
        &self.content
    }
}

impl DraftPost {
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn request_review(self) -> PendingPost {
        PendingPost { content: self.content }
    }
}

impl PendingPost {
    pub fn approve(self) -> BlogPost {
        BlogPost { content: self.content }
    }
}
