use clap::{Arg, Command};
use std::error::Error;

static VERSION: &str = concat!(env!("CARGO_PKG_VERSION"), "-build-", env!("build"));

#[derive(Clone, Default)]
pub struct Argument {
    pub config_file: String,
    pub listen_url: String,
    pub version_info: String,
}

impl Argument {
    pub fn parse(&mut self) -> Result<(), Box<dyn Error>> {
        let matches = Command::new("ragx")
            .version(VERSION)
            .arg(
                Arg::new("config_file")
                    .short('c')
                    .long("config-file")
                    .value_name("NAME")
                    .help("Config file (.yml)")
                    .default_value("config.yml")
                    .required(true),
            )
            .arg(
                Arg::new("listen_url")
                    .short('l')
                    .long("listen-url")
                    .value_name("URL")
                    .help("Listen url (host:port)")
                    .default_value(":8080")
                    .required(true),
            )
            .get_matches();

        let config_file = matches.get_one::<String>("config_file").unwrap();
        self.config_file = config_file.to_string();

        let listen_url = matches.get_one::<String>("listen_url").unwrap();
        self.listen_url = listen_url.to_string();

        self.version_info = VERSION.to_string();

        Ok(())
    }
}
