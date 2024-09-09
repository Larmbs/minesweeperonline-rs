use client;
use iced::alignment::{Horizontal, Vertical};
use iced::mouse;
use iced::widget::{button, column, mouse_area, row, text, Column, Row};
use iced::{event, executor, Application, Command, Element, Settings};

enum Status {
    Connecting,
    Playing,
    Lost,
    Won,
}
#[derive(PartialEq)]
enum Click {
    Right,
    Left,
}
struct MinesweeperGUI {
    client: Option<client::MineSweeperClient>,
    status: Status,
    dim: (usize, usize),
    last_click: Click,
}

#[derive(Debug, Clone)]
enum Message {
    RevealCell(usize),
    FlagCell(usize),
    NewGame,
    Connect,
}

impl Application for MinesweeperGUI {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();
    type Theme = iced::theme::Theme;

    fn new(_: ()) -> (Self, Command<Message>) {
        (
            Self {
                client: None,
                status: Status::Connecting,
                dim: (10, 10),
                last_click: Click::Left,
            },
            Command::perform(async {}, |_| Message::Connect),
        )
    }

    fn title(&self) -> String {
        String::from("Minesweeper")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::RevealCell(index) => {
                if let Some(ref mut client) = self.client {
                    if client.is_playing() {
                        client.reveal_cell(index);

                        if client.is_won() {
                            self.status = Status::Won
                        }
                        if client.is_lost() {
                            self.status = Status::Lost
                        };
                    }
                }
            }
            Message::FlagCell(index) => {
                if let Some(ref mut client) = self.client {
                    client.flag_cell(index);
                }
            }
            Message::NewGame => {
                self.status = Status::Connecting;
                return Command::perform(async {}, |_| Message::Connect);
            }
            Message::Connect => {
                self.client = Some(
                    client::MineSweeperClient::start_game("127.0.0.1:8000", self.dim, 10)
                        .expect("Unable to connect to server"),
                );
            }
        }
        Command::none()
    }

    fn view(&self) -> Element<Message> {
        let top_bar = row![button("NewGame").on_press(Message::NewGame)];
        let (width, height) = self.dim;
        let mut row = Row::new();
        let nums = [" ", "1", "2", "3", "4", "5", "6", "7", "8"];
        for x in 0..width {
            let mut column = Column::new();
            for y in 0..height {
                let mut txt = " ";
                if let Some(ref client) = self.client {
                    let cell = client.get_cell(x + y * width);
                    txt = match cell {
                        client::Cell::Revealed(val) => nums[*val as usize],
                        client::Cell::Hidden(state) => {
                            if *state {
                                "F"
                            } else {
                                "M"
                            }
                        }
                    };
                }
                column = column.push(
                    mouse_area(
                        text(txt)
                            .vertical_alignment(Vertical::Center)
                            .horizontal_alignment(Horizontal::Center)
                            .width(50)
                            .height(50),
                    )
                    .on_right_press(Message::FlagCell(x + y * width))
                    .on_press(Message::RevealCell(x + y * width)),
                );
            }
            row = row.push(column);
        }
        column!(top_bar, row,).into()
    }

    fn theme(&self) -> Self::Theme {
        Self::Theme::Dracula
    }
}

fn main() -> iced::Result {
    MinesweeperGUI::run(Settings::default())
}
