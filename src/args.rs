use clap::{ArgAction, Parser};


#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Roms you wish to play
    #[arg(short, long)]
    pub rom: String,

    /// enter a id number to connect to peer (leave blank if you want to start a connection)
    #[arg(short, long, default_value_t = String::from("host"))]
    pub id: String,

    ///Toggle between debug and regular mode
    #[arg(short, long, action=ArgAction::SetFalse)]
    pub debug: bool,
}

