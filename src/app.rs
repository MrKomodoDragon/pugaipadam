use crate::fl;
use cosmic::iced::advanced::image::Handle;
use cosmic::prelude::CollectionWidget;
use cosmic::widget::image::Image;
use cosmic::{
    app::{self, Command, Core},
    iced::{
        alignment::{Horizontal, Vertical},
        Length,
    },
    widget,
    widget::column,
    Application, Apply, Element,
};
use image::ImageReader;
use std::fs::read;
use std::mem::take;
use std::time::{Duration, Instant};
/// This is the struct that represents your application.
/// It is used to define the data that will be used by your application.
#[derive(Clone, Default)]
pub struct Pugaipadam {
    /// This is the core of your application, it is used to communicate with the Cosmic runtime.
    /// It is used to send messages to your application, and to access the resources of the Cosmic runtime.
    core: Core,
    current_image: usize,
    image_list: Vec<Handle>,
}

/// This is the enum that contains all the possible variants that your application will need to transmit messages.
/// This is used to communicate between the different parts of your application.
/// If your application does not need to send messages, you can use an empty enum or `()`.
#[derive(Debug, Clone)]
pub enum Message {
    Next,
    Previous,
}

/// Implement the `Application` trait for your application.
/// This is where you define the behavior of your application.
///
/// The `Application` trait requires you to define the following types and constants:
/// - `Executor` is the executor that will be used to run your application.
/// - `Flags` is the data that your application needs to use before it starts.
/// - `Message` is the enum that contains all the possible variants that your application will need to transmit messages.
/// - `APP_ID` is the unique identifier of your application.
impl Application for Pugaipadam {
    type Executor = cosmic::executor::Default;

    type Flags = ();

    type Message = Message;

    const APP_ID: &'static str = "com.example.CosmicAppTemplate";

    fn core(&self) -> &Core {
        &self.core
    }

    fn core_mut(&mut self) -> &mut Core {
        &mut self.core
    }

    /// This is the header of your application, it can be used to display the title of your application.
    fn header_center(&self) -> Vec<Element<Self::Message>> {
        vec![widget::text::text(fl!("app-title")).into()]
    }

    /// This is the entry point of your application, it is where you initialize your application.
    ///
    /// Any work that needs to be done before the application starts should be done here.
    ///
    /// - `core` is used to passed on for you by libcosmic to use in the core of your own application.
    /// - `flags` is used to pass in any data that your application needs to use before it starts.
    /// - `Command` type is used to send messages to your application. `Command::none()` can be used to send no messages to your application.
    fn init(core: Core, _flags: Self::Flags) -> (Self, Command<Self::Message>) {
        let im1 =
            ImageReader::open("/home/mrkomododragon/Pictures/tarantula_nebula_nasa_PIA23646.jpg")
                .unwrap()
                .decode()
                .unwrap();
        let im2 = ImageReader::open("/home/mrkomododragon/Pictures/KOMODO.jpg")
            .unwrap()
            .decode()
            .unwrap();
        println!("{:#?}", im2.clone().into_bytes());
        let example = Pugaipadam {
            core,
            current_image: 0,
            image_list: vec![
                Handle::from_pixels(im1.width(), im1.height(), im1.into_bytes()),
                Handle::from_pixels(im2.width(), im2.height(), im2.into_bytes()),
            ],
        };

        (example, Command::none())
    }
    /// This is the main view of your application, it is the root of your widget tree.
    ///
    /// The `Element` type is used to represent the visual elements of your application,
    /// it has a `Message` associated with it, which dictates what type of message it can send.
    ///
    /// To get a better sense of which widgets are available, check out the `widget` module.
    fn view(&self) -> Element<Self::Message> {
        println!("Render update");
        let image = Image::new(self.image_list[self.current_image].clone())
            .width(Length::Fill)
            .height(Length::Fill);
        let previous = widget::button("Previous").on_press(Message::Previous);
        let next = widget::button("Next").on_press(Message::Next);
        let row = widget::row()
            .push_maybe(self.can_go_back().then(|| previous))
            .push_maybe(self.can_go_forward().then(|| next));
        widget::column::with_children(vec![image.into(), row.into()]).into()
    }

    fn update(&mut self, message: Self::Message) -> app::Command<Self::Message> {
        match message {
            Message::Previous => {
                let now = Instant::now();
                println!("Got next message");
                self.current_image -= 1;
                let elapsed: u128 = (Instant::now() - now).as_millis();
                println!("Increased the number after {} secs", elapsed);
            }
            Message::Next => {
                let now = Instant::now();
                println!("Got previous message");
                self.current_image += 1;
                let elapsed: u128 = (Instant::now() - now).as_millis();
                println!("Increased the number after {} secs", elapsed);
            }
        }
        Command::none()
    }
}

impl Pugaipadam {
    fn can_go_back(&self) -> bool {
        if self.current_image == 0 {
            return false;
        }
        true
    }
    fn can_go_forward(&self) -> bool {
        if self.current_image == (self.image_list.len() - 1) {
            return false;
        }
        true
    }
}
