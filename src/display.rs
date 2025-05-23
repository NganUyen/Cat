use anyhow::Result;
use iced::{
    window, Application, Command, Element, Settings, Subscription,
    executor, Theme, Font,
};
use iced::widget::Container;
use log::info;

use crate::sprite_handler::SpriteController;

pub struct Display {
    sprite_controller: SpriteController,
}

#[derive(Debug, Clone)]
pub enum Message {
    Tick,
}

impl Application for Display {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        info!("Initializing Display");
        
        // Create main window and controller
        let sprite_controller = SpriteController::new();
        
        (
            Self {
                sprite_controller,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("pyCatAI-pet")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::Tick => {
                // Update state on each tick
                if let Err(e) = self.sprite_controller.handle_animation() {
                    log::error!("Animation error: {}", e);
                }
                Command::none()
            }
        }
    }

    fn subscription(&self) -> Subscription<Message> {
        // Register subscription to receive time-based events
        iced::time::every(std::time::Duration::from_millis(16))
            .map(|_| Message::Tick)
    }

    fn view(&self) -> Element<Message> {
        // Display UI
        Container::new(self.sprite_controller.view())
            .width(iced::Length::Fill)
            .height(iced::Length::Fill)
            .into()
    }
    
    fn theme(&self) -> Theme {
        Theme::Dark
    }
}

impl Display {
    pub fn new() -> Self {
        let (app, _) = <Self as Application>::new(());
        app
    }
    
    pub fn run(self) -> Result<()> {
        let settings = Settings {
            window: window::Settings {
                size: (200, 200),
                position: window::Position::Specific(100, 500),
                min_size: None,
                max_size: None,
                visible: true,
                resizable: false,
                decorations: false,
                transparent: true,
                icon: None,
                level: window::Level::Normal,
                platform_specific: window::PlatformSpecific::default(),
                ..Default::default()
            },
            default_font: Font::default(),
            default_text_size: 16.0,
            antialiasing: true,
            exit_on_close_request: true,
            ..Default::default()
        };
        
        <Self as Application>::run(settings)
            .map_err(|e| anyhow::anyhow!("Failed to run application: {}", e))
    }
} 