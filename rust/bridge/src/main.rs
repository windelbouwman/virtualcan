#[macro_use]
extern crate log;

mod can_frame;
mod client;
mod tcp_endpoint;

// if cfg(os = linux)
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
            clap::Arg::with_name("port")
                .long("port")
                .short("p")
                .takes_value(true)
                .default_value("18881"),
        )
        .get_matches();

    let verbosity = matches.occurrences_of("v");

    let log_level = match verbosity {
        0 => log::Level::Info,
        1 => log::Level::Debug,
        2 | _ => log::Level::Trace,
    };

    simple_logger::init_with_level(log_level).unwrap();

    use std::str::FromStr;
    let port: u16 = u16::from_str(
        matches
            .value_of("port")
            .expect("port value must be present"),
    )
    .unwrap_or(18881);

    if cfg!(target_os = "linux") {
        client::bridge_can0("127.0.0.1", port);
    } else {
        let do_chain = true;
        if do_chain {
            client::bridge("127.0.0.1", port);
        } else {
            client::dump("127.0.0.1", port);
        }
    }
}
