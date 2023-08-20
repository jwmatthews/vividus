use iced::{
    executor, Application, Command, Container, Element, Length,
    Subscription, Settings, window, Button, Column, Text,
};
use iced::widget::image::Image;
use image::io::Reader as ImageReader;
use async_std::task;
use std::path::Path;

fn main() -> iced::Result {
    App::run(Settings {
        window: window::Settings {
            size: (800, 600),
            ..window::Settings::default()
        },
        ..Settings::default()
    })
}

struct App {
    image_paths: Vec<String>,
    current_image: Option<iced::Handle>,
}

#[derive(Debug, Clone)]
enum Message {
    ImageLoaded(Result<iced::Handle, image::ImageError>),
    NextImage,
    PreviousImage,
}

impl Application for App {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (App, Command<Message>) {
        let image_paths = load_image_paths("path/to/your/folder"); // Put your folder path here

        (
            App {
                image_paths,
                current_image: None,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Image Viewer")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::ImageLoaded(Ok(handle)) => {
                self.current_image = Some(handle);
            }
            Message::ImageLoaded(Err(_)) => {
                // Handle the error as needed
            }
            Message::NextImage => {
                if let Some(path) = self.image_paths.pop() {
                    return Command::perform(load_image(path), Message::ImageLoaded);
                }
            }
            Message::PreviousImage => {
                // Handle previous image logic as needed
            }
        }

        Command::none()
    }

    fn view(&mut self) -> Element<Message> {
        let content = if let Some(image_handle) = &self.current_image {
            Column::new()
                .push(
                    Button::new(&mut self.next_button_state, Text::new("Next"))
                        .on_press(Message::NextImage),
                )
                .push(Image::new(image_handle.clone()))
        } else {
            Column::new().push(Text::new("No images available!").horizontal_alignment(HorizontalAlignment::Center))
        };

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .into()
    }
}

fn load_image_paths(folder_path: &str) -> Vec<String> {
    // Logic to read image paths from the specified folder
    // This could use the `glob` crate, `std::fs`, etc.
    Vec::new() // Placeholder
}

fn load_image(path: String) -> impl std::future::Future<Output = Result<iced::Handle, image::ImageError>> {
    task::spawn_blocking(move || {
        let reader = ImageReader::open(path)?;
        let image = reader.decode()?;
        let image_handle = iced::Handle::from_pixels(
            image.width(),
            image.height(),
            image.to_bytes(),
        );

        Ok(image_handle)
    })
}
