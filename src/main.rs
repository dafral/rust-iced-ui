mod windows;
use windows::Installer;

pub fn main() -> iced::Result {
    iced::run("Installer", Installer::update, Installer::view)
}

