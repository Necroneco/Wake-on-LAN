use structopt::StructOpt;
use wol::MagicPacket;

#[derive(StructOpt)]
struct Cli {
    // #[structopt(default_value = "Your MAC Address")]
    mac_address: String,
}

fn main() -> std::io::Result<()> {
    let args = Cli::from_args();
    MagicPacket::from_str(&args.mac_address).send()?;
    println!("Sent Magic Packet to {}", args.mac_address);
    Ok(())
}
