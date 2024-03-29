use std::net::IpAddr;

use clap::Parser;
use tokio::net::ToSocketAddrs;

#[derive(Parser)]
#[command(version)]
pub struct CliArgs {
    #[arg(short = 'p', long)]
    port: u16,

    #[arg(short = 'a', long)]
    address: Box<dyn ToSocketAddrs>,
}
