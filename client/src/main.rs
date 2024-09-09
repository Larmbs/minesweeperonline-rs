use client;
use iced::alignment::{Horizontal, Vertical};

use iced::widget::{
    button, column, container, image, mouse_area, row, text, text_input, Column, Image, Row,
};
use iced::{executor, Application, Command, Element, Length, Settings};

const IMAGES: [&str; 13] = [
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
    "client/images/mine_exploded.png",
];

struct MinesweeperGUI {
    client: client::MineSweeperClient,
    dim: (usize, usize),
    mine_count: usize,
    speed: String,
}

#[derive(Debug, Clone)]
enum Message {
    RevealCell(usize),
    FlagCell(usize),
    NewGame,
    SetWidth(Option<usize>),
    SetHeight(Option<usize>),
    CloseGame,
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
                client: client::MineSweeperClient::connect("127.0.0.1:8000").unwrap(),
                dim: (10, 10),
                speed: String::new(),
                mine_count: 10,
            }, Command::none()
        )
    }

    fn title(&self) -> String {
        String::from("Minesweeper")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::RevealCell(index) => {
                self.client.reveal_cell(index);
            },
            Message::FlagCell(index) => {
                self.client.flag_cell(index);
            },
            Message::NewGame => {
                self.client.new_game(self.dim, self.mine_count);
            },
            Message::SetWidth(w) => {
                if w.is_some() {
                    self.dim.0 = w.unwrap().clamp(1, 100);
                    return Command::perform(async {}, |_| Message::CloseGame);
                }
            },
            Message::SetHeight(h) => {
                if h.is_some() {
                    self.dim.1 = h.unwrap().clamp(1, 100);
                    return Command::perform(async {}, |_| Message::CloseGame);
                }
            },
            Message::CloseGame => {
                self.client.close_game();
            }
            Message::SetMineCount(c) => {
                if c.is_some() {
                    self.mine_count = c.unwrap().clamp(1, usize::MAX);
                    return Command::perform(async {}, |_| Message::CloseGame);
                }
            }
        }
        Command::none()
    }

    fn view(&self) -> Element<Message> {
        let top_bar = row![
            text(format!("Status: {:?}", self.client.state)),
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

        if self.client.state.should_display() {
            let (width, height) = self.client.board.clone().unwrap().dim.clone();

            let max_width = 1200u16;
            let max_height = 800u16;
            let dx = max_width / width as u16;
            let dy = max_height / height as u16;
            let b_size = dx.min(dy);
            for x in 0..width {
                let mut column = Column::new();
                for y in 0..height {
                    let mut path_img = IMAGES[10];
                    if let Some(ref board) = self.client.board {
                        let cell = &board.cells[x + y * width];
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
                            client::Cell::MineExploded => IMAGES[12],

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
