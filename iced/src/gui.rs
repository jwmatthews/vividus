use iced::{
    Color, Element, Length, Sandbox
};
use iced::alignment;
use iced::widget::{
    column, container, horizontal_space, image, row,
    scrollable, text
};
use iced::theme;
use iced::widget::{Button, Container};
use glob::glob;
use std::path::Path;
use std::path::PathBuf;


#[derive(Debug, Clone, Copy)]
pub enum Message {
    BackPressed,
    NextPressed,
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
 images: Vec<String>,
}

impl Sandbox for App {
    type Message = Message;

    fn new() -> App {
        

        App {
            debug: true,
            image_index: 0,
            images: vec_of_strings![
                "images/ferris.png", 
                 "images/ferris2.jpg", 
                 "images/ferris3.jpg", 
                 "images/rust.jpg"]
        }
    }

	fn title(&self) -> String {
        format!("Vividus: debug {}", self.debug)
    }

    fn update(&mut self, event: Message) {
        match event {
            Message::BackPressed => {
                println!("Back pressed");
                //image_index = image_index + 1
            }
            Message::NextPressed => {
                println!("Next pressed");
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let App { debug, .. } = self;

        let mut controls = row![];

        controls = controls.push(button("Back")
                .on_press(Message::BackPressed)
                .style(theme::Button::Secondary));
        controls = controls.push(horizontal_space(Length::Fill));
        controls = controls.push(button("Next")
                .on_press(Message::NextPressed)
                .style(theme::Button::Secondary));

        let content: Element<_> = column![
            controls,
            ferris(200, &self.images[self.image_index]),
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

fn ferris<'a>(width: u16, image_path: &str) -> Container<'a, Message> {
    // TODO: use a different image based on the image_index
    println!("in ferris image_path: {}", image_path);
    container(
        //image(format!("{}/images/ferris.png", env!("CARGO_MANIFEST_DIR")))
        image(format!("{}{}", env!("CARGO_MANIFEST_DIR"), image_path))
        .width(width),
    )
    .width(Length::Fill)
    .center_x()
}

fn button<'a, Message: Clone>(label: &str) -> Button<'a, Message> {
    iced::widget::button(
        text(label).horizontal_alignment(alignment::Horizontal::Center),
    )
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



#[cfg(test)]
mod tests {
    use super::*;

    const IMG_DIR: &str = "images";

    #[test]
    fn test_list_images() {
        
        let files =  list_images(IMG_DIR);
        assert_eq!(files.len(), 4);
    }
}
