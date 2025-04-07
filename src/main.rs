mod csv_reader;

use csv_reader::*;
use iced::border::width;
use iced::widget::{
    Column, Row, Text, TextInput, button, center, checkbox, column, container, horizontal_rule,
    pick_list, progress_bar, row, scrollable, slider, text, text_input, toggler, vertical_rule,
    vertical_space,
};
use iced::{Center, Element, Fill, Subscription, Theme};
use iced::{Length, keyboard};

pub fn main() -> iced::Result {
    unsafe {
        std::env::set_var("WINIT_UNIX_BACKEND", "wayland");
        std::env::set_var("WGPU_BACKEND", "gl");
    }
    let csv_data = read_csv(&"customers-100.csv".to_string()).unwrap();

    iced::application("rowgazer", State::update, State::view)
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
    InputChanged(String),
    CSVPathChanged,
    CellChanged(String, usize, usize),
}

impl State {
    fn update(&mut self, message: Message) {
        match message {
            Message::InputChanged(value) => self.csv_path = value,
            Message::CellChanged(value, r, c) => self.data.data[r][c] = CSVEntry::String(value),
            Message::CSVPathChanged => {
                self.data = read_csv(&self.csv_path).unwrap();
            }
        }
    }

    fn create_cell(&self, cell: &CSVEntry, row: usize, col: usize) -> TextInput<Message> {
        match cell {
            CSVEntry::String(value) => text_input("", &value)
                .on_input(move |new_value| Message::CellChanged(new_value, row, col)),
            CSVEntry::Int(value) => text_input("", &value.to_string())
                .on_input(move |new_value| Message::CellChanged(new_value, row, col)),
            CSVEntry::Float(value) => text_input("", &value.to_string())
                .on_input(move |new_value| Message::CellChanged(new_value, row, col)),
        }
    }

    fn view(&self) -> Element<Message> {
        let text_input = text_input("File Path:", &self.csv_path).on_input(Message::InputChanged);
        let styled_button = |label| button(text(label)).on_press(Message::CSVPathChanged);

        let primary = styled_button("Load File");
        let mut grid: Column<'_, Message> = Column::new();
        let mut r = 0;
        for data_row in &self.data.data {
            let mut row: Row<'_, Message> = Row::new();
            for (c, cell) in data_row.iter().enumerate() {
                row = row.push(self.create_cell(cell, r, c));
            }
            r += 1;
            grid = grid.push(row);
        }
        let content = column![
            text_input,
            row![primary].spacing(10).align_y(Center),
            scrollable(grid)
        ];

        center(content).into()
    }

    fn theme(&self) -> Theme {
        self.theme.clone()
    }
}
