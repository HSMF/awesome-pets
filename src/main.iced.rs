use iced::{
    application::StyleSheet,
    color,
    futures::executor::ThreadPool,
    widget::{button, column, container, row, scrollable, text},
    Application, Command, Element, Length, Settings,
};

impl button::StyleSheet for MyTheme {
    type Style = ();

    fn active(&self, style: &Self::Style) -> button::Appearance {
        button::Appearance::default()
    }
}

impl text::StyleSheet for MyTheme {
    type Style = ();

    fn appearance(&self, style: Self::Style) -> text::Appearance {
        text::Appearance::default()
    }
}

#[derive(Debug)]
pub struct MainView {}

#[derive(Default)]
pub struct MyTheme {}

impl StyleSheet for MyTheme {
    type Style = ();

    fn appearance(&self, _style: &Self::Style) -> iced::application::Appearance {
        iced::application::Appearance {
            background_color: iced::Color::from_rgba(1.0, 1.0, 1.0, 0.0),
            text_color: color!(0xffffff),
        }
    }
}

impl Application for MainView {
    type Executor = ThreadPool;

    type Message = ();

    type Theme = MyTheme;

    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        (Self {}, Command::none())
    }

    fn title(&self) -> String {
        env!("CARGO_BIN_NAME").to_owned()
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        todo!()
    }

    fn view(&self) -> iced::Element<'_, Self::Message, Self::Theme, iced::Renderer> {
        iced::widget::Button::new("hey").into()
    }
}

fn main() -> anyhow::Result<()> {
    MainView::run(Settings::default())?;

    Ok(())
}

fn padded_button<'a, Message: Clone>(label: &str) -> iced::widget::Button<'a, Message> {
    button(text(label)).padding([12, 24])
}
