mod etherscan;
mod gas;

use crate::gas::track_gas;
use crate::etherscan::etherscan::Etherscan;

use clap::{Command, Arg};
use megalodon::Megalodon;
use megalodon::pleroma::pleroma::Pleroma;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let matches = Command::new("eth_gas_bot")
        .version("0.1.1")
        .author("makemake <vukasin@gostovic.me>")
        .about("Pleroma bot that tracks the current eth gas price.")
        .arg(Arg::new("instance")
            .long("instance")
            .short('i')
            .num_args(1..)
            .required(true)
            .help("Instance URL"))
        .arg(Arg::new("token")
            .long("token")
            .short('t')
            .num_args(1..)
            .required(true)
            .help("Pleroma token"))
        .arg(Arg::new("etherscan_url")
            .long("etherscan_url")
            .short('u')
            .num_args(1..)
            .required(false)
            .help("Etherscan URL"))
        .arg(Arg::new("etherscan_token")
            .long("etherscan_token")
            .short('e')
            .num_args(1..)
            .required(false)
            .help("Etherscan token"))
        .get_matches();

    let instance = matches.get_one::<String>("instance").expect("Invalid instance").to_string();
    let token = matches.get_one::<String>("token").expect("Invalid token").to_string();

    let client = Pleroma::new(
      instance,
      Some(token),
      None,
    );
    let res = client.verify_account_credentials().await?;
    println!("{:#?}", res.json());

    let etherscan = Etherscan::new(
        matches.get_one::<String>("etherscan_url").unwrap_or(&"https://api.etherscan.io".to_string()).to_string(),
        matches.get_one::<String>("etherscan_token").unwrap_or(&"".to_string()).to_string(),
    );

    track_gas(client, etherscan).await?;

    Ok(())
}
