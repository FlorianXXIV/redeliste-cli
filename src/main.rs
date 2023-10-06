use error_chain::error_chain;
use clap::{Parser};
use reqwest::Client;
use serde_json::json;
use serde::Deserialize;

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

    let user_create = json!({
        "name": args.user_name,
    });

//    let request_url = format!("{}/{}",args.connect_url,"User/create");
//
//    let response = Client::new()
//        .post(request_url)
//        .json(user_create)
//        .send().await?;

    let request_url = format!("{}/{}",args.connect_url,"User/0/retrieve");

    let response = Client::new().get(request_url).send().await?;

    let requested_user:RetrieveUser = response.json().await?;

    println!("{0}, {1}, {2}", args.connect_url, args.user_name, args.redeliste_name);
    println!("{0}, {1}", requested_user.name, requested_user.id);
    Ok(())
}