use glob::glob;
use iced::alignment;
use iced::theme;
use iced::widget::{column, container, horizontal_space, image, row, scrollable, text};
use iced::widget::{Button, Container};
use iced::{Color, Element, Length, Sandbox};
use std::path::Path;
use std::path::PathBuf;

#[derive(Debug, Clone, Copy)]
pub enum Message {
    BackPressed,
    NextPressed,
    DebugPressed,
}

macro_rules! vec_of_strings {
    ($($x:expr),*) => (vec![$($x.to_string()),*]);
}

// TODO
// 1. Add a button to go back and forth between images
// 2. Add a button to toggle debug mode

pub struct App {
    debug: bool,
    image_index: usize,
    images: Vec<PathBuf>,
}

impl Sandbox for App {
    type Message = Message;

    fn new() -> App {
        let images = list_images("images");
        App {
            debug: false,
            image_index: 0,
            images: images,
        }
    }

    fn title(&self) -> String {
        format!("Vividus: debug {}", self.debug)
    }

    fn update(&mut self, event: Message) {
        match event {
            Message::DebugPressed => {
                // Toggle Debug
                println!("Debug pressed: self.debug = '{}'", self.debug);
                self.debug = !self.debug;
            }
            Message::BackPressed => {
                println!("Back pressed: self.image_index = '{}'", self.image_index);
                self.image_index = decrease_image_index(self.images.len(), self.image_index);
            }
            Message::NextPressed => {
                println!("Next pressed: self.image_index = '{}'", self.image_index);
                self.image_index = increase_image_index(self.images.len(), self.image_index);
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let App { debug, .. } = self;

        let mut controls = row![];

        controls = controls.push(
            button("Back")
                .on_press(Message::BackPressed)
                .style(theme::Button::Secondary),
        );
        controls = controls.push(horizontal_space(Length::Fill));
        controls = controls.push(
            button("Next")
                .on_press(Message::NextPressed)
                .style(theme::Button::Secondary),
        );
        let mut debug_row = row![];
        debug_row = debug_row.push(
            button("Debug")
                .on_press(Message::DebugPressed)
                .style(theme::Button::Primary),
        );

        let content: Element<_> = column![
            controls,
            ferris(200, &self.images[self.image_index]),
            debug_row
        ]
        .max_width(540)
        .spacing(20)
        .padding(20)
        .into();

        let scrollable = scrollable(
            container(if self.debug {
                content.explain(Color::BLACK)
            } else {
                content
            })
            .width(Length::Fill)
            .center_x(),
        );

        container(scrollable).height(Length::Fill).center_y().into()
    }
}

fn ferris<'a>(width: u16, image_path: &PathBuf) -> Container<'a, Message> {
    println!("in ferris image_path: {}", image_path.display());
    container(image(image_path).width(width))
        .width(Length::Fill)
        .center_x()
}

fn button<'a, Message: Clone>(label: &str) -> Button<'a, Message> {
    iced::widget::button(text(label).horizontal_alignment(alignment::Horizontal::Center))
        .padding(12)
        .width(100)
}

fn list_images(image_dir: &str) -> Vec<PathBuf> {
    let manifest_path = Path::new(env!("CARGO_MANIFEST_DIR"));
    let image_path = manifest_path.join(image_dir);
    let search_path = format!("{}/*", image_path.to_str().unwrap());

    let mut x = Vec::<PathBuf>::new();

    match glob(search_path.as_str()) {
        Ok(files) => {
            x = files.filter(|x| x.is_ok()).map(|x| x.unwrap()).collect();
        }
        Err(e) => println!("Error listing images: {}", e),
    }
    println!("x: {:?}", x);
    return x;
}

fn increase_image_index(length: usize, image_index: usize) -> usize {
    if length == 0 {
        return 0;
    } else if image_index == length - 1 {
        return 0;
    } else {
        return image_index + 1;
    }
}

fn decrease_image_index(length: usize, image_index: usize) -> usize {
    if length == 0 {
        return 0;
    } else if image_index == 0 {
        return length - 1;
    } else {
        return image_index - 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const IMG_DIR: &str = "images";

    #[test]
    fn test_list_images() {
        let files = list_images(IMG_DIR);
        assert_eq!(files.len(), 4);
    }

    #[test]
    fn test_decrease_image_index() {
        let length = 3;
        let mut image_index = 0;

        // Normal Usage

        image_index = decrease_image_index(length, image_index);
        assert_eq!(image_index, 2);

        image_index = decrease_image_index(length, image_index);
        assert_eq!(image_index, 1);

        image_index = decrease_image_index(length, image_index);
        assert_eq!(image_index, 0);

        image_index = decrease_image_index(length, image_index);
        assert_eq!(image_index, 2);

        // Edge cases

        image_index = decrease_image_index(0, 0);
        assert_eq!(image_index, 0);
    }

    #[test]
    fn test_increase_image_index() {
        let length = 3;
        let mut image_index = 0;

        // Normal Usage

        image_index = increase_image_index(length, image_index);
        assert_eq!(image_index, 1);

        image_index = increase_image_index(length, image_index);
        assert_eq!(image_index, 2);

        image_index = increase_image_index(length, image_index);
        assert_eq!(image_index, 0);

        image_index = increase_image_index(length, image_index);
        assert_eq!(image_index, 1);

        // Edge cases

        image_index = increase_image_index(0, 0);
        assert_eq!(image_index, 0);
    }
}
