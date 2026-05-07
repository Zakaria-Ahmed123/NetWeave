use iced::{Application, Command, Element, Theme};
use iced::widget::{button, column, row, text, container, scrollable};

#[derive(Debug, Clone)]
pub enum Message {
    StartServer,
    ConnectPeer,
    SendChat,
}

pub struct App {
    logs: Vec<String>,
}

impl Application for App {
    type Message = Message;
    type Theme = Theme;
    type Executor = iced::executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Self::Message>) {
        (
            Self {
                logs: vec![],
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        "LAN Racer UI".into()
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::StartServer => self.logs.push("Start server clicked".into()),
            Message::ConnectPeer => self.logs.push("Connect peer clicked".into()),
            Message::SendChat => self.logs.push("Send chat clicked".into()),
        }

        Command::none()
    }

    fn view(&self) -> Element<'_,Self::Message> {
        let controls = row![
            text("Controls").size(20),
            button("Start Server").on_press(Message::StartServer),
            button("Connect Peer").on_press(Message::ConnectPeer),
            button("Send Chat").on_press(Message::SendChat),
        ]
        .spacing(10)
        .padding(10);

        let logs_panel = scrollable(
            column(
                self.logs
                    .iter()
                    .map(|l| text(l).into())
                    .collect::<Vec<Element<Message>>>()
            )
            .spacing(5)
        )
        .height(300);

        row![
            container(controls).width(200),
            container(text("Main Panel")).width(300),
            container(logs_panel).width(250),
        ]
        .spacing(10)
        .padding(10)
        .into()
    }
}

fn main() -> iced::Result {
    App::run(iced::Settings::default())
}

