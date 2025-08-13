use app::Pugaipadam;
/// The `app` module is used by convention to indicate the main component of our application.
mod app;
mod core;
use image::ImageFormat;
use std::path::PathBuf;
use std::{env, fs};
/// The `cosmic::app::run()` function is the starting point of your application.
/// It takes two arguments:
/// - `settings` is a structure that contains everything relevant with your app's configuration, such as antialiasing, themes, icons, etc...
/// - `()` is the flags that your app needs to use before it starts.
///  If your app does not need any flags, you can pass in `()`.
fn main() -> cosmic::iced::Result {
    let args = env::args().skip(1).map(PathBuf::from);
    let args = args.filter_map(|e| {
        if e.is_file() || e.is_dir() {
            Some(e)
        } else {
            None
        }
    });
    let mut paths: Vec<PathBuf> = vec![];
    for i in args {
        if i.is_dir() {
            let Ok(entries) = fs::read_dir(&i) else {
                eprintln!("Error occured reading the directory {:#?}", i.display());
                continue;
            };
            let entries = entries
                .filter_map(Result::ok)
                .map(|e| e.path())
                .filter_map(|path| {
                    if path
                        .extension()
                        .map_or(false, |ext| ImageFormat::from_extension(ext).is_some())
                    {
                        Some(path)
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>();
            paths.extend(entries);
        } else if i.is_file() {
            if i.extension()
                .map_or(false, |ext| ImageFormat::from_extension(ext).is_some() || ext == "svg")
            {
                paths.push(i);
            } else {
                println!("{:#?} is not an image PNG or jpg", i.display());
            }
        }
    }
    let settings = cosmic::app::Settings::default();
    cosmic::app::run::<Pugaipadam>(settings, paths)
}
