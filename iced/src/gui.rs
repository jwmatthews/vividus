use iced::{
    Color, Element, Length, Sandbox
};
use iced::widget::{
    column, container, horizontal_space, image, row,
    scrollable, text
};

use iced::widget::Container;


#[derive(Debug, Clone, Copy)]
pub enum Message {
    BackPressed,
    NextPressed,
}

pub struct App {
 debug: bool,
}

impl Sandbox for App {
    type Message = Message;

    fn new() -> App {
        App {
            debug: false,
        }
    }

	fn title(&self) -> String {
        format!("Vividus: debug {}", self.debug)
    }

    fn update(&mut self, event: Message) {
        match event {
            Message::BackPressed => {
                //self.back();
            }
            Message::NextPressed => {
                //self.next();
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let App { debug, .. } = self;

        let mut controls = row![];

        controls = controls.push(horizontal_space(Length::Fill));

        let content: Element<_> = column![
            controls, ferris(200)
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

fn ferris<'a>(width: u16) -> Container<'a, Message> {
    container(
        image(format!("{}/images/ferris.png", env!("CARGO_MANIFEST_DIR")))
        .width(width),
    )
    .width(Length::Fill)
    .center_x()
}