use clap::{Parser, arg, command};
use mini_redis::DEFAULT_PORT;
use tokio::{net::TcpListener, signal};

#[derive(Parser, Debug)]
#[command(
    name = "mini-redis-server",
    version,
    author,
    about = "A mini Redis server implementation"
)]
struct Cli {
    #[arg(long)]
    port: Option<u16>,
}

#[tokio::main]
pub async fn main() -> mini_redis::Result<()> {
    let cli = Cli::parse();
    let port = cli.port.unwrap_or(DEFAULT_PORT);

    let listener = TcpListener::bind(&format!("127.0.0.1:{}", port)).await?;
    mini_redis::server::run(listener, signal::ctrl_c()).await;
    Ok(())
}
