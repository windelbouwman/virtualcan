#[macro_use]
extern crate log;

mod server;

fn main() {
    simple_logger::init_with_level(log::Level::Info).unwrap();

    let matches = clap::App::new("virtual can server")
        .author("Windel Bouwman")
        .about("Central virtual CAN server")
        .arg(
            clap::Arg::with_name("port")
                .long("port")
                .short('p')
                .takes_value(true)
                .default_value("18881"),
        )
        .get_matches();

    use std::str::FromStr;
    let port: u16 = u16::from_str(
        matches
            .value_of("port")
            .expect("port value must be present"),
    )
    .unwrap_or(18881);

    server::run_server(port);
}
