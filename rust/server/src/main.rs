#[macro_use]
extern crate log;

use std::str::FromStr;
mod server;

fn main() {
    simple_logger::init_with_level(log::Level::Info).unwrap();

    let matches = clap::Command::new("virtual can server")
        .author("Windel Bouwman")
        .about("Central virtual CAN server")
        .arg(
            clap::Arg::new("port")
                .long("port")
                .short('p')
                .default_value("18881"),
        )
        .get_matches();

    let port: u16 = u16::from_str(
        matches
            .get_one::<String>("port")
            .expect("port value must be present"),
    )
    .unwrap();

    server::run_server(port);
}
