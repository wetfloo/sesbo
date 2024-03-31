use std::net::IpAddr;

use clap::Parser;

#[derive(Parser)]
#[command(version)]
pub struct CliArgs {
    #[arg(short = 'p', long, default_value_t = 443)]
    pub port: u16,

    #[arg(short = 'a', long, default_value = "0.0.0.0")]
    pub address: IpAddr,
}
