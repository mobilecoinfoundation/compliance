/// Copyright 2022 MobileCoin Foundation

use clap::Parser;

/// Compliance crate configuration
#[derive(Parser, Debug)]
pub struct Configuration {
    #[clap(short, long)]
    pub ip_info_key: Option<String>,
}
