use anyhow::Result;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::sync::mpsc;
use tokio_util::sync::CancellationToken;

mod router;
mod peer;
mod event;

use router::{Router, RouterCommand};

#[tokio::main]
async fn main() -> Result<()> {
    let token = CancellationToken::new();

    // Command channel (main → router)
    let (cmd_tx, cmd_rx) = mpsc::channel(32);

    let router = Router::new();

    // Spawn router
    tokio::spawn({
        let token = token.clone();
        async move {
            if let Err(e) = router.route(token, cmd_rx).await {
                eprintln!("Router error: {e}");
            }
        }
    });

    // CLI loop
    let stdin = BufReader::new(tokio::io::stdin());
    let mut lines = stdin.lines();

    println!("Commands:");
    println!("offer <peer_id>");
    println!("accept <peer_id> <sdp>");
    println!("answer <peer_id> <sdp>");

    while let Ok(Some(line)) = lines.next_line().await {
        let parts: Vec<_> = line.splitn(3, ' ').collect();

        match parts.as_slice() {
            ["offer", peer_id] => {
                let _ = cmd_tx
                    .send(RouterCommand::CreateOffer {
                        peer_id: peer_id.to_string(),
                    })
                    .await;
            }

            ["accept", peer_id, sdp] => {
                let _ = cmd_tx
                    .send(RouterCommand::AcceptOffer {
                        peer_id: peer_id.to_string(),
                        sdp: sdp.to_string(),
                    })
                    .await;
            }

            ["answer", peer_id, sdp] => {
                let _ = cmd_tx
                    .send(RouterCommand::CreateAnswer {
                        peer_id: peer_id.to_string(),
                        sdp: sdp.to_string(),
                    })
                    .await;
            }

            _ => println!("Unknown command"),
        }
    }

    token.cancel();

    Ok(())
}