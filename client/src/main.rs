use client;
use iced::alignment::{Horizontal, Vertical};

use iced::widget::{
    button, column, container, image, mouse_area, row, text, text_input, Column, Image, Row,
};
use iced::{executor, Application, Command, Element, Length, Settings};

const IMAGES: [&str; 12] = [
    "client/images/0.png",
    "client/images/1.png",
    "client/images/2.png",
    "client/images/3.png",
    "client/images/4.png",
    "client/images/5.png",
    "client/images/6.png",
    "client/images/7.png",
    "client/images/8.png",
    "client/images/mine.png",
    "client/images/mask.png",
    "client/images/flag.png",
];
#[derive(Debug, PartialEq)]
enum Status {
    Connecting,
    FailedToConnect,
    Playing,
    Lost,
    Won,
    Idle,
}
impl Status {
    pub fn should_display(&self) -> bool {
        match self {
            Status::Playing | Status::Lost | Self::Won => true,
            _ => false,
        }
    }
}

struct MinesweeperGUI {
    client: Option<client::MineSweeperClient>,
    status: Status,
    dim: (usize, usize),
    mine_count: usize,
    speed: String,
}

#[derive(Debug, Clone)]
enum Message {
    RevealCell(usize),
    FlagCell(usize),
    NewGame,
    Connect,
    SetWidth(Option<usize>),
    SetHeight(Option<usize>),
    GoIdle,
    SetMineCount(Option<usize>),
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
                speed: String::new(),
                mine_count: 10,
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
                    client.reveal_cell(index);

                    match &client.state {
                        client::State::Playing => (),
                        client::State::Lost(time) => {
                            self.status = Status::Lost;
                            self.speed = time.to_string()
                        }
                        client::State::Won(time) => {
                            self.status = Status::Won;
                            self.speed = time.to_string()
                        }
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
                self.status = Status::Playing;

                match client::MineSweeperClient::start_game(
                    "127.0.0.1:8000",
                    self.dim,
                    self.mine_count,
                ) {
                    Ok(client) => self.client = Some(client),
                    Err(_) => self.status = Status::FailedToConnect,
                }
            }
            Message::SetWidth(w) => {
                if w.is_some() {
                    self.dim.0 = w.unwrap().clamp(1, 50);
                    return Command::perform(async {}, |_| Message::GoIdle);
                }
            }
            Message::SetHeight(h) => {
                if h.is_some() {
                    self.dim.1 = h.unwrap().clamp(1, 30);
                    return Command::perform(async {}, |_| Message::GoIdle);
                }
            }
            Message::GoIdle => {
                self.status = Status::Idle;
                self.client = None;
            }
            Message::SetMineCount(c) => {
                if c.is_some() {
                    self.mine_count = c.unwrap().clamp(1, usize::MAX);
                    return Command::perform(async {}, |_| Message::GoIdle);
                }
            }
        }
        Command::none()
    }

    fn view(&self) -> Element<Message> {
        let top_bar = row![
            text(format!("Status: {:?}", self.status)),
            text(format!("Time: {}", self.speed)),
        ]
        .spacing(15);
        let bottom_bar = row![
            text("W/H"),
            text_input("8", &self.dim.0.to_string())
                .on_input(|v| { Message::SetWidth(v.parse().ok()) })
                .width(100),
            text_input("8", &self.dim.1.to_string())
                .on_input(|v| { Message::SetHeight(v.parse().ok()) })
                .width(100),
            button("NewGame").on_press(Message::NewGame),
            text("Mine Count"),
            text_input("10", &self.mine_count.to_string())
                .on_input(|v| { Message::SetMineCount(v.parse().ok()) })
                .width(100),
        ]
        .padding(15);
        let mut row = Row::new();

        if self.status.should_display() {
            let (width, height) = self.dim;

            let max_width = 1200u16;
            let max_height = 800u16;
            let dx = max_width / width as u16;
            let dy = max_height / height as u16;
            let b_size = dx.min(dy);
            for x in 0..width {
                let mut column = Column::new();
                for y in 0..height {
                    let mut path_img = IMAGES[10];
                    if let Some(ref client) = self.client {
                        let cell = client.get_cell(x + y * width);
                        path_img = match cell {
                            client::Cell::Revealed(val) => IMAGES[*val as usize],
                            client::Cell::Hidden(state) => {
                                if *state {
                                    IMAGES[11]
                                } else {
                                    IMAGES[10]
                                }
                            }
                            client::Cell::Mine => IMAGES[9],
                        };
                    }
                    column = column.push(
                        mouse_area(
                            Image::<image::Handle>::new(path_img)
                                .width(b_size)
                                .height(b_size),
                        )
                        .on_right_press(Message::FlagCell(x + y * width))
                        .on_press(Message::RevealCell(x + y * width)),
                    );
                }
                row = row.push(column);
            }
        }

        container(column!(top_bar, row, bottom_bar))
            .align_y(Vertical::Center)
            .align_x(Horizontal::Center)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    fn theme(&self) -> Self::Theme {
        Self::Theme::Light
    }
}

fn main() -> iced::Result {
    MinesweeperGUI::run(Settings::default())
}
