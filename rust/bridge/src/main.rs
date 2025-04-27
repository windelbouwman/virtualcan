#[macro_use]
extern crate log;

use std::str::FromStr;

mod can_frame;
mod client;
mod tcp_endpoint;

#[cfg(target_os = "linux")]
mod socket_can_endpoint;

fn main() {
    let matches = clap::Command::new("virtual can bridge")
        .author("Windel Bouwman")
        .about("Bridge virtual to real CAN bus")
        .arg(
            clap::Arg::new("v")
                .short('v')
                .long("verbose")
                .action(clap::ArgAction::Count)
                .help("Sets the level of verbosity."),
        )
        .arg(
            clap::Arg::new("host")
                .long("host")
                .help("Specify virtual can server IP to connect to.")
                .default_value("127.0.0.1"),
        )
        .arg(
            clap::Arg::new("port")
                .long("port")
                .short('p')
                .help("Specify virtual can server port to connect to.")
                .default_value("18881"),
        )
        .arg(
            clap::Arg::new("candevice")
                .long("can")
                .help("Specify can port to connect to, for example can0"),
        )
        .arg(
            clap::Arg::new("peer-port")
                .long("peer-port")
                .help("Specify second virtual can server port to connect to."),
        )
        .get_matches();

    let verbosity = matches.get_count("v");

    let log_level = match verbosity {
        0 => log::Level::Info,
        1 => log::Level::Debug,
        _ => log::Level::Trace,
    };

    simple_logger::init_with_level(log_level).unwrap();

    let host = matches.get_one::<String>("host").expect("to be present");

    let port: u16 = u16::from_str(
        matches
            .get_one::<String>("port")
            .expect("port value must be present"),
    )
    .unwrap_or(18881);

    if matches.contains_id("candevice") {
        let can_device = matches.get_one::<String>("candevice").unwrap();
        info!("Bridging to real can device {}!", can_device);
        client::socketcan::bridge_can0(host, port, can_device);
    } else if matches.contains_id("peer-port") {
        info!("Bridging to other virtual can server!");

        let peer_port: u16 = u16::from_str(
            matches
                .get_one::<String>("peer-port")
                .expect("port value must be present"),
        )
        .unwrap_or(18882);

        client::bridge(host, port, "127.0.0.1", peer_port);
    } else {
        info!("Dumping can traffic!");
        client::dump(host, port);
    }
}
