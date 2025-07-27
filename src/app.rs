use crate::fl;
use cosmic::iced::advanced::image::Handle;
use cosmic::iced::Alignment;
use cosmic::prelude::CollectionWidget;
use cosmic::widget::image::Viewer;
use cosmic::widget::{horizontal_space, Space};
use cosmic::ApplicationExt;
use cosmic::{
    app::{self, Core, Task},
    iced::{keyboard, window, Length, Padding, Subscription},
    widget, Application, Element,
};
use image::{GenericImageView, ImageReader};
use std::fs::{self, DirEntry};
use std::path::{Path, PathBuf};

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
        println!("{}", path.display());
        let im = ImageReader::open(&path).unwrap().decode().unwrap();
        let height = im.height();
        let width = im.width();
        Self {
            height,
            width,
            pixels_handle: Handle::from_rgba(width, height, im.into_rgba8().into_vec()),
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
    fullscreen: bool,
}

/// This is the enum that contains all the possible variants that your application will need to transmit messages.
/// This is used to communicate between the different parts of your application.
/// If your application does not need to send messages, you can use an empty enum or `()`.
#[derive(Debug, Clone)]
pub enum Message {
    Next,
    Previous,
    Fullscreen,
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

    type Flags = Vec<PathBuf>;

    type Message = Message;

    const APP_ID: &'static str = "com.example.CosmicAppTemplate";

    fn core(&self) -> &Core {
        &self.core
    }

    fn core_mut(&mut self) -> &mut Core {
        &mut self.core
    }

    fn on_escape(&mut self) -> Task<Self::Message> {
        if self.fullscreen {
            return self.update(Message::Fullscreen);
        }
        Task::none()
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
    fn init(core: Core, flags: Self::Flags) -> (Self, Task<Self::Message>) {
        let mut vector: Vec<ImageRepresentation> = Vec::new();
        for i in flags {
            vector.push(ImageRepresentation::from_path(i));
        }
        println!("{:#?}", vector.len());
        let mut app = Pugaipadam {
            core,
            current_image: 0,
            image_list: vector,
            fullscreen: false,
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
        if self.image_list.is_empty() {
            return widget::container(
                widget::text("No images to display. Please provide image files as arguments.")
                    .size(20)
            )
            .center_x(Length::Fill)
            .center_y(Length::Fill)
            .into();
        }

        let current_image = self.image_list[self.current_image].clone();
        let image = Viewer::new(current_image.pixels_handle)
            .width(Length::Fill)
            .height(Length::Fill);
        let previous = widget::button::text("Previous").on_press(Message::Previous);
        let next = widget::button::text("Next").on_press(Message::Next);
        let fullscreen =
            widget::button::custom(widget::icon::from_name("view-fullscreen-symbolic").size(16))
                .on_press(Message::Fullscreen);
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
            .push(fullscreen)
            .push(horizontal_space())
            .push(details)
            .align_y(Alignment::Center);
        if !self.fullscreen {
            return widget::column::with_children(vec![image.into(), row.into()]).into();
        } else {
            return image.into();
        }
    }

    fn update(&mut self, message: Self::Message) -> app::Task<Self::Message> {
        match message {
            Message::Previous => {
                if self.current_image == 0 {
                    self.current_image = self.image_list.len() - 1;
                } else {
                    self.current_image -= 1;
                }
            }
            Message::Next => {
                if self.current_image + 1 == self.image_list.len() {
                    self.current_image = 0;
                } else {
                    self.current_image += 1;
                }
            }
            Message::Fullscreen => {
                self.fullscreen = !self.fullscreen;
                self.core.window.show_headerbar = !self.fullscreen;
                let mode = if self.fullscreen {
                    window::Mode::Fullscreen
                } else {
                    window::Mode::Windowed
                };
                return window::get_oldest().and_then(move |id| window::change_mode(id, mode));
            }
        }
        self.update_title()
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        keyboard::on_key_press(|key, _modifiers| {
            match key {
                keyboard::Key::Named(keyboard::key::Named::ArrowLeft) => Some(Message::Previous),
                keyboard::Key::Named(keyboard::key::Named::ArrowRight) => Some(Message::Next),
                _ => None,
            }
        })
    }
}

impl Pugaipadam {
    fn can_go_back(&self) -> bool {
        if self.image_list.len() == 1 {
            return false;
        }
        true
    }
    fn can_go_forward(&self) -> bool {
        if self.image_list.len() == 1 {
            return false;
        }
        true
    }
    fn update_title(&mut self) -> Task<Message> {
        let mut title = String::new();
        if self.image_list.is_empty() {
            title.push_str("Pugaipadam - No images");
        } else {
            let file_name = &self.image_list[self.current_image].name;
            title.push_str(file_name.as_str());
            title.push_str(format!(" ({}/{})", self.current_image + 1, self.image_list.len()).as_str());
        }
        println!("{}", title);
        self.set_window_title(title)
    }
    fn change_fullscreen(&mut self, id: window::Id) -> Task<Message> {
        window::change_mode(
            id,
            if self.fullscreen {
                window::Mode::Fullscreen
            } else {
                window::Mode::Windowed
            },
        )
    }
}
