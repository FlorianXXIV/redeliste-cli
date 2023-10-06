use error_chain::error_chain;
use clap::{Parser};
use serde::Deserialize;
use signalrs_client::{hub::Hub, SignalRClient};
use tracing::*;
use tracing_subscriber::{self, filter, prelude::*};

error_chain! {
    foreign_links {
        HttpRequest(reqwest::Error);
    }
}

///Connect to a Redeliste online with username and redeliste name
#[derive(Parser)]
struct Cli{
    ///the URL to connect to
    #[arg(short, long)]
    connect_url: String,
    ///The Username
    #[arg(short, long)]
    user_name: String,
    ///The name of the Redeliste to connect to/create
    #[arg(short='n', long)]
    redeliste_name: String
}

#[derive(Deserialize, Debug)]
struct RetrieveUser{
    name: String,
    id: i32,
}
#[tokio::main]
async fn main() -> Result<()> {
    let args = Cli::parse();



    println!("{0}, {1}, {2}", args.connect_url, args.user_name, args.redeliste_name);
    Ok(())
}

async fn get_client() -> Result<SignalRClient>{
    let _hub = Hub::default().method("NeueMeldung", print);
}

fn set_tracing_subscriber() {
    let targets_filter = filter::Targets::new()
        .with_target("signalrs", Level::TRACE)
        .with_default(Level::DEBUG);

    let fmt_layer = tracing_subscriber::fmt::layer()
        .with_line_number(false)
        .with_file(false)
        .without_time()
        .compact();

    tracing_subscriber::registry()
        .with(fmt_layer)
        .with(targets_filter)
        .init();
}

fn print(message: String) {
    info!("{message}")
}