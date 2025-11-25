use clap::{Parser, arg, command};

#[derive(Parser, Debug)]
#[command(name="mini-redis-server", version, author,about="A mini Redis server implementation")]
struct Cli {
    #[arg(long)]
    port: Option<u16>,
}

#[tokio::main]
pub async fn main() -> () {
    let cli = Cli::parse();
    println!("Hello, server!");
     ()
}
