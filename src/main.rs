mod csv_reader;

use csv_reader::*;
use iced::border::width;
use iced::keyboard;
use iced::widget::{
    Column, Row, TextInput, button, center, checkbox, column, container, horizontal_rule,
    pick_list, progress_bar, row, scrollable, slider, text, text_input, toggler, vertical_rule,
    vertical_space,
};
use iced::{Center, Element, Fill, Subscription, Theme};

pub fn main() -> iced::Result {
    unsafe {
        std::env::set_var("WINIT_UNIX_BACKEND", "wayland");
        std::env::set_var("WGPU_BACKEND", "gl");
    }
    let csv_data = read_csv(&"customers-100.csv".to_string()).unwrap();

    iced::application("rowgazer", State::update, State::view)
        .subscription(State::subscription)
        .theme(State::theme)
        .run()
}

#[derive(Default)]
struct State {
    data: CSVData,
    theme: Theme,
    csv_path: String,
}

#[derive(Debug, Clone)]
enum Message {
    CSVPathChanged,
    ThemeChanged(Theme),
    InputChanged(String),
    PreviousTheme,
    NextTheme,
}

impl State {
    fn update(&mut self, message: Message) {
        match message {
            Message::ThemeChanged(theme) => {
                self.theme = theme;
            }
            Message::InputChanged(value) => self.csv_path = value,
            Message::PreviousTheme | Message::NextTheme => {
                if let Some(current) = Theme::ALL
                    .iter()
                    .position(|candidate| &self.theme == candidate)
                {
                    self.theme = if matches!(message, Message::NextTheme) {
                        Theme::ALL[(current + 1) % Theme::ALL.len()].clone()
                    } else if current == 0 {
                        Theme::ALL
                            .last()
                            .expect("Theme::ALL must not be empty")
                            .clone()
                    } else {
                        Theme::ALL[current - 1].clone()
                    };
                }
            }
            Message::CSVPathChanged => {
                self.data = read_csv(&self.csv_path).unwrap();
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let choose_theme = column![
            text("Theme:"),
            pick_list(Theme::ALL, Some(&self.theme), Message::ThemeChanged).width(Fill),
        ];

        let text_input = text_input("File Path:", &self.csv_path).on_input(Message::InputChanged);

        let styled_button = |label| button(text(label)).on_press(Message::CSVPathChanged);

        let primary = styled_button("Load File");
        let mut grid: Column<'_, Message> = Column::new();
        for data_row in &self.data.data {
            let mut row: Row<'_, Message> = Row::new();
            for cell in data_row.iter() {
                if let CSVEntry::String(value) = cell {
                    row = row.push(text(value));
                } else {
                    row = row.push(text("Error"));
                }
            }
            grid = grid.push(scrollable(row));
        }
        let content = column![
            choose_theme,
            text_input,
            row![primary].spacing(10).align_y(Center),
            scrollable(grid).direction(scrollable::Direction::Both {
                vertical: scrollable::Scrollbar::new(),
                horizontal: scrollable::Scrollbar::new(),
            })
        ];

        center(content).into()
    }

    fn subscription(&self) -> Subscription<Message> {
        keyboard::on_key_press(|key, _modifiers| match key {
            keyboard::Key::Named(
                keyboard::key::Named::ArrowUp | keyboard::key::Named::ArrowLeft,
            ) => Some(Message::PreviousTheme),
            keyboard::Key::Named(
                keyboard::key::Named::ArrowDown | keyboard::key::Named::ArrowRight,
            ) => Some(Message::NextTheme),
            _ => None,
        })
    }

    fn theme(&self) -> Theme {
        self.theme.clone()
    }
}
