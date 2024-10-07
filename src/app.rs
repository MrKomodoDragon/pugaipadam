use crate::fl;
use cosmic::iced::advanced::image::Handle;
use cosmic::iced::Alignment;
use cosmic::prelude::CollectionWidget;
use cosmic::widget::image::Viewer;
use cosmic::widget::{horizontal_space, Space};
use cosmic::ApplicationExt;
use cosmic::{
    app::{self, Command, Core},
    iced::Length,
    widget, Application, Element,
};
use image::{GenericImageView, ImageReader};
use std::path::PathBuf;

#[derive(Clone, Debug)]
struct ImageRepresentation {
    height: u32,
    width: u32,
    pixels_handle: Handle,
    path: PathBuf,
    name: String,
}

impl ImageRepresentation {
    fn from_path(path: PathBuf) -> Self {
        let im = ImageReader::open(&path).unwrap().decode().unwrap();
        let height = im.height();
        let width = im.width();
        Self {
            height,
            width,
            pixels_handle: Handle::from_pixels(width, height, im.into_rgba8().into_vec()),
            name: (&path).file_name().unwrap().to_str().unwrap().to_string(),
            path: path,
        }
    }
}

/// This is the struct that represents your application.
/// It is used to define the data that will be used by your application.
#[derive(Clone, Default)]
pub struct Pugaipadam {
    /// This is the core of your application, it is used to communicate with the Cosmic runtime.
    /// It is used to send messages to your application, and to access the resources of the Cosmic runtime.
    core: Core,
    current_image: usize,
    image_list: Vec<ImageRepresentation>,
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
        vec![widget::text::text(self.title()).into()]
    }

    /// This is the entry point of your application, it is where you initialize your application.
    ///
    /// Any work that needs to be done before the application starts should be done here.
    ///
    /// - `core` is used to passed on for you by libcosmic to use in the core of your own application.
    /// - `flags` is used to pass in any data that your application needs to use before it starts.
    /// - `Command` type is used to send messages to your application. `Command::none()` can be used to send no messages to your application.
    fn init(core: Core, _flags: Self::Flags) -> (Self, Command<Self::Message>) {
        let mut vector: Vec<ImageRepresentation> = Vec::new();
        //
        //
        vector.push(ImageRepresentation::from_path(PathBuf::from(
            "/home/mrkomododragon/Pictures/KOMODO.jpg",
        )));
        vector.push(ImageRepresentation::from_path(PathBuf::from(
            "/home/mrkomododragon/Pictures/beeg.jpg",
        )));
        let mut app = Pugaipadam {
            core,
            current_image: 0,
            image_list: vector,
        };
        let command = app.update_title();
        (app, command)
    }
    /// This is the main view of your application, it is the root of your widget tree.
    ///
    /// The `Element` type is used to represent the visual elements of your application,
    /// it has a `Message` associated with it, which dictates what type of message it can send.
    ///
    /// To get a better sense of whi.ch widgets are available, check out the `widget` module.
    fn view(&self) -> Element<Self::Message> {
        let current_image = self.image_list[self.current_image].clone();
        let image = Viewer::new(current_image.pixels_handle)
            .width(Length::Fill)
            .height(Length::Fill);
        let previous = widget::button::text("Previous").on_press(Message::Previous);
        let next = widget::button::text("Next").on_press(Message::Next);
        let dimensions = format!(
            "{} - {}x{} pixels",
            current_image.path.display(),
            current_image.width,
            current_image.height
        );
        let details = widget::container(widget::text(dimensions))
            .align_x(cosmic::iced::alignment::Horizontal::Right);
        let row = widget::row()
            .push_maybe(self.can_go_back().then(|| previous))
            .push_maybe(self.can_go_forward().then(|| next))
            .push(horizontal_space(Length::Fill))
            .push(details)
            .align_items(Alignment::Center);
        widget::column::with_children(vec![image.into(), row.into()]).into()
    }

    fn update(&mut self, message: Self::Message) -> app::Command<Self::Message> {
        match message {
            Message::Previous => {
                self.current_image -= 1;
            }
            Message::Next => {
                self.current_image += 1;
            }
        }
        self.update_title()
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
    fn update_title(&mut self) -> Command<Message> {
        let mut title = fl!("app-title");
        let file_name = &self.image_list[self.current_image].name;
        title.push_str(" - ");
        title.push_str(file_name.as_str());
        self.set_window_title(title)
    }
}
