//use core_app;
use crate::style;

use iced::{
    button, scrollable, slider, text_input, Alignment, Button, Checkbox,
    Column, Container, Element, Length, ProgressBar, Radio, Row, Rule, Sandbox,
    Scrollable, Slider, Space, Text, TextInput, Toggler,
};

#[derive(Default)]
pub struct TheApplication {
    theme: style::Theme,
    scroll: scrollable::State,
    input: text_input::State,
    input_value: String,
    button: button::State,
    slider: slider::State,
    slider_value: f32,
    checkbox_value: bool,
    toggler_value: bool,
}

#[derive(Debug, Clone)]
pub enum Message {
    ThemeChanged(style::Theme),
    InputChanged(String),
    ButtonPressed,
    SliderChanged(f32),
    CheckboxToggled(bool),
    TogglerToggled(bool),
}

impl Sandbox for TheApplication {
    type Message = Message;

    fn new() -> Self {
        TheApplication::default()
    }

    fn title(&self) -> String {
        String::from("Styling - Iced")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::ThemeChanged(theme) => self.theme = theme,
            Message::InputChanged(value) => self.input_value = value,
            Message::ButtonPressed => {}
            Message::SliderChanged(value) => self.slider_value = value,
            Message::CheckboxToggled(value) => self.checkbox_value = value,
            Message::TogglerToggled(value) => self.toggler_value = value,
        }
    }

    fn view(&mut self) -> Element<Message> {
        let choose_theme = style::Theme::ALL.iter().fold(
            Column::new().spacing(10).push(Text::new("Choose a theme:")),
            |column, theme| {
                column.push(
                    Radio::new(
                        *theme,
                        format!("{:?}", theme),
                        Some(self.theme),
                        Message::ThemeChanged,
                    )
                    .style(self.theme),
                )
            },
        );

        let text_input = TextInput::new(
            &mut self.input,
            "Type something...",
            &self.input_value,
            Message::InputChanged,
        )
        .padding(10)
        .size(20)
        .style(self.theme);

        let button = Button::new(&mut self.button, Text::new("Submit"))
            .padding(10)
            .on_press(Message::ButtonPressed)
            .style(self.theme);

        let slider = Slider::new(
            &mut self.slider,
            0.0..=100.0,
            self.slider_value,
            Message::SliderChanged,
        )
        .style(self.theme);

        let progress_bar =
            ProgressBar::new(0.0..=100.0, self.slider_value).style(self.theme);

        let scrollable = Scrollable::new(&mut self.scroll)
            .width(Length::Fill)
            .height(Length::Units(100))
            .style(self.theme)
            .push(Text::new("Scroll me!"))
            .push(Space::with_height(Length::Units(800)))
            .push(Text::new("You did it!"));

        let checkbox = Checkbox::new(
            self.checkbox_value,
            "Check me!",
            Message::CheckboxToggled,
        )
        .style(self.theme);

        let toggler = Toggler::new(
            self.toggler_value,
            String::from("Toggle me!"),
            Message::TogglerToggled,
        )
        .width(Length::Shrink)
        .spacing(10)
        .style(self.theme);

        let content = Column::new()
            .spacing(20)
            .padding(20)
            .max_width(600)
            .push(choose_theme)
            .push(Rule::horizontal(38).style(self.theme))
            .push(Row::new().spacing(10).push(text_input).push(button))
            .push(slider)
            .push(progress_bar)
            .push(
                Row::new()
                    .spacing(10)
                    .height(Length::Units(100))
                    .align_items(Alignment::Center)
                    .push(scrollable)
                    .push(Rule::vertical(38).style(self.theme))
                    .push(
                        Column::new()
                            .width(Length::Shrink)
                            .spacing(20)
                            .push(checkbox)
                            .push(toggler),
                    ),
            );

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .style(self.theme)
            .into()
    }
}