use iced::{Task, Theme};
use tokio::sync::mpsc;
use tokio_util::sync::CancellationToken;

mod router;
mod peer;
mod event;

use router::{Router, RouterCommand};
use event::{LanEvent,Message};

fn main() -> iced::Result {
    iced::application("Lan Racer", App::update, App::view)
        .theme(|_| Theme::KanagawaWave)
        .run_with(App::init)
}
pub struct App {
    cmd_tx: mpsc::Sender<RouterCommand>,
    event_rx: mpsc::Receiver<LanEvent>,
    logs: Vec<String>,
}

impl App {
    fn init() -> (Self, Task<Message>) {
        let (cmd_tx, cmd_rx) = mpsc::channel(32);
        let (event_tx, event_rx) = mpsc::channel(32);

        let router = Router::new();

        let token = CancellationToken::new();

        // spawn router
        std::thread::spawn(move || {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async move {
                let _ = router.route(token, cmd_rx, event_tx).await;
            });
        });

        (
            Self {
                cmd_tx,
                event_rx,
                logs: vec![],
            },
            Task::perform(async {}, |_| Message::Tick),
        )
    }

    fn update(&mut self, message: Message) -> Task<Message> {
    match message {
        Message::CreateOffer => {
            let tx = self.cmd_tx.clone();
            return Task::perform(async move {
                tx.send(RouterCommand::CreateOffer {
                    peer_id: "peer1".into(),
                })
                .await
                .ok();
            }, |_| Message::Tick);
        }

        Message::Tick => {
            // poll events
            while let Ok(event) = self.event_rx.try_recv() {
                match event {
                    LanEvent::PeerConnected(pid) => {
                        self.logs.push(format!("Connected: {}", pid));
                    }
                    LanEvent::PeerDisconnected(pid) => {
                        self.logs.push(format!("Disconnected: {}", pid));
                    }
                    LanEvent::PacketFromPeer(_) => {
                        self.logs.push("Packet received".into());
                    }
                    LanEvent::NewPeerOffer(pid, _) => {
                        self.logs.push(format!("Offer from {}", pid));
                    }
                }
            }

            // keep polling
            return Task::perform(async {}, |_| Message::Tick);
        }

        _ => {}
    }

    Task::none()
   }

   fn view<'a>(&'a self) -> iced::Element<'a,Message> {
    use iced::widget::{column, button, text};

    column![
        button("Create Offer").on_press(Message::CreateOffer),
        text("Logs:"),
        column(
            self.logs
                .iter()
                .map(|l| text(l.clone()).into())
                .collect::<Vec<iced::Element<Message>>>()
        )
    ]
    .into()
   }
}