use iced::widget::{text_input, image, column, Column};

#[derive(Default)]
pub struct Installer {
    path : String,
}

impl Installer {
    pub fn view(&self) -> Column<InstallerMessage> {
        column![
            text_input("Path to install", &self.path).on_input(InstallerMessage::SetPath),
        ]
    }
    pub fn update(&mut self, message: InstallerMessage) {
        match message {
            InstallerMessage::SetPath(path) => self.path = path,
        }
    }
}

#[derive(Debug, Clone)]
pub enum InstallerMessage {
    SetPath(String),
}

#[test]
fn it_sets_path_properly(){
    let mut installer = Installer::default();
    installer.update(InstallerMessage::SetPath("C:/test".to_string()));
    assert_eq!(installer.path, "C:/test");
}

