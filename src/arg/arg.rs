use clap::{Arg, Command};
use std::error::Error;

#[derive(Clone, Default)]
pub struct Argument {
    pub config_file: String,
    pub listen_url: String,
    pub version_info: String,
}

impl Argument {
    pub fn parse(&mut self) -> Result<(), Box<dyn Error>> {
        self.version_info =
            concat!(env!("CARGO_PKG_VERSION"), "-build-", env!("build")).to_string();

        let matches = Command::new("ragx")
            .version(&*self.version_info)
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
                    .default_value("http://127.0.0.1:8080")
                    .required(true),
            )
            .get_matches();

        match matches.value_source("config_file") {
            Some(name) => self.config_file = name.to_string(),
            None => self.config_file = "".to_string(),
        }

        match matches.value_source("listen_url") {
            Some(url) => self.listen_url = url.to_string(),
            None => self.listen_url = "".to_string(),
        }

        Ok(())
    }
}
