#[macro_use]
extern crate log;

mod can_frame;
mod client;
mod tcp_endpoint;

#[cfg(target_os = "linux")]
mod socket_can_endpoint;

fn main() {
    let matches = clap::App::new("virtual can bridge")
        .author("Windel Bouwman")
        .about("Bridge virtual to real CAN bus")
        .arg(
            clap::Arg::with_name("v")
                .short("v")
                .multiple(true)
                .help("Sets the level of verbosity."),
        )
        .arg(
            clap::Arg::with_name("host")
                .long("host")
                .takes_value(true)
                .help("Specify virtual can server IP to connect to.")
                .default_value("127.0.0.1"),
        )
        .arg(
            clap::Arg::with_name("port")
                .long("port")
                .short("p")
                .takes_value(true)
                .help("Specify virtual can server port to connect to.")
                .default_value("18881"),
        )
        .arg(
            clap::Arg::with_name("candevice")
                .long("can")
                .takes_value(true)
                .help("Specify can port to connect to, for example can0"),
        )
        .arg(
            clap::Arg::with_name("peer-port")
                .long("peer-port")
                .takes_value(true)
                .help("Specify second virtual can server port to connect to."),
        )
        .get_matches();

    let verbosity = matches.occurrences_of("v");

    let log_level = match verbosity {
        0 => log::Level::Info,
        1 => log::Level::Debug,
        _ => log::Level::Trace,
    };

    simple_logger::init_with_level(log_level).unwrap();

    let host = matches.value_of("host").expect("to be present");

    use std::str::FromStr;
    let port: u16 = u16::from_str(
        matches
            .value_of("port")
            .expect("port value must be present"),
    )
    .unwrap_or(18881);

    if matches.is_present("candevice") {
        let can_device = matches.value_of("candevice").unwrap();
        info!("Bridging to real can device {}!", can_device);
        client::bridge_can0(host, port, can_device);
    } else if matches.is_present("peer-port") {
        info!("Bridging to other virtual can server!");

        let peer_port: u16 = u16::from_str(
            matches
                .value_of("peer-port")
                .expect("port value must be present"),
        )
        .unwrap_or(18882);

        client::bridge(host, port, "127.0.0.1", peer_port);
    } else {
        info!("Dumping can traffic!");
        client::dump(host, port);
    }
}
