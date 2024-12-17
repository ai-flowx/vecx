mod arg;
mod config;

use arg::arg::Argument;
use config::config::Config;
use std::process;

fn main() {
    let mut a = Argument {
        ..Default::default()
    };

    if let Err(err) = a.parse() {
        println!("failed to parse argument: {}", err);
        process::exit(-1);
    }

    let mut c = Config {
        config_file: a.config_file,
        listen_port: a.listen_port,
        repo_path: a.repo_path,
        version_info: a.version_info,
        ..Default::default()
    };

    if let Err(err) = c.build() {
        println!("failed to build config: {}", err);
        process::exit(-2);
    }
}
