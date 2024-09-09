use client;
use iced::alignment::{Horizontal, Vertical};

use iced::widget::{
    button, column, container, image, mouse_area, row, text, Column, Image, Row,
};
use iced::{executor, Application, Command, Element, Settings};

const Images: [&str; 12] = [
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
#[derive(Debug)]
enum Status {
    Connecting,
    FailedToConnect,
    Playing,
    Lost,
    Won,
}

struct MinesweeperGUI {
    client: Option<client::MineSweeperClient>,
    status: Status,
    dim: (usize, usize),
    speed: String,
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
                speed: String::new(),
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

                match client::MineSweeperClient::start_game("127.0.0.1:8000", self.dim, 10) {
                    Ok(client) => self.client = Some(client),
                    Err(_) => self.status = Status::FailedToConnect,
                }
            }
        }
        Command::none()
    }

    fn view(&self) -> Element<Message> {
        let top_bar = row![
            button("NewGame").on_press(Message::NewGame),
            text(format!("Status: {:?}", self.status)),
            text(format!("Time: {}", self.speed))
        ]
        .padding(15);
        let (width, height) = self.dim;
        let mut row = Row::new();
        for x in 0..width {
            let mut column = Column::new();
            for y in 0..height {
                let mut path_img = Images[10];
                if let Some(ref client) = self.client {
                    let cell = client.get_cell(x + y * width);
                    path_img = match cell {
                        client::Cell::Revealed(val) => Images[*val as usize],
                        client::Cell::Hidden(state) => {
                            if *state {
                                Images[11]
                            } else {
                                Images[10]
                            }
                        }
                        client::Cell::Mine => Images[9],
                    };
                }
                column = column.push(
                    mouse_area(Image::<image::Handle>::new(path_img).width(50).height(50))
                        .on_right_press(Message::FlagCell(x + y * width))
                        .on_press(Message::RevealCell(x + y * width)),
                );
            }
            row = row.push(column);
        }
        container(column!(top_bar, row))
            .align_y(Vertical::Center)
            .align_x(Horizontal::Center)
            .width(iced::Length::Fill)
            .height(iced::Length::Fill)
            .into()
    }

    fn theme(&self) -> Self::Theme {
        Self::Theme::Dracula
    }
}

fn main() -> iced::Result {
    MinesweeperGUI::run(Settings::default())
}
