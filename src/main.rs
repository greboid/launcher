#[macro_use] extern crate log;
extern crate fern;
extern crate time;
extern crate clap;
extern crate jni_sys;

mod java;

use clap::{Arg, App};

fn main() {
    let config = App::new("Java Launcher")
        .arg(Arg::with_name("debug")
             .short("d")
             .long("debug")
             .value_name("Log level")
             .help("Sets the debug level for the client.")
             .takes_value(true)
             .default_value("error"))
        .get_matches();
    setup_logger(get_log_level(config.value_of("debug").unwrap().parse()
                               .unwrap_or(String::from("error")).to_lowercase()));
    println!("Java home: {}", java::get_java_path());
    println!("Java Lib : {}", java::get_java_lib());
    println!("Java DLL : {}", java::get_java_dll());
}

fn get_log_level(config_level: String) -> log::LogLevelFilter {
    if config_level == "error" {
        return log::LogLevelFilter::Error;
    } else if config_level == "warn" {
        return log::LogLevelFilter::Warn;
    } else if config_level == "info" {
        return log::LogLevelFilter::Info;
    } else if config_level == "debug" {
        return log::LogLevelFilter::Debug;
    } else if config_level == "trace" {
        return log::LogLevelFilter::Trace;
    } else {
        return log::LogLevelFilter::Error;
    }
}

fn setup_logger(level: log::LogLevelFilter) {
    let logger_config = fern::DispatchConfig {
        format: Box::new(|msg: &str, level: &log::LogLevel, _location: &log::LogLocation| {
            format!("[{}][{}] {}", time::now().strftime("%Y-%m-%d][%H:%M:%S").unwrap(), level, msg)
        }),
        output: vec![fern::OutputConfig::stdout(), ],//fern::OutputConfig::file("output.log")],
        level: level,
    };
    if let Err(e) = fern::init_global_logger(logger_config, log::LogLevelFilter::Trace) {
        panic!("Failed to initialize global logger: {}", e);
    }
}
