#[macro_use] extern crate log;
extern crate fern;
extern crate time;
extern crate clap;
extern crate jni_sys;

mod java;

use clap::{Arg, App};
use std::ffi::CString;
use std::ptr;
use std::os::raw::c_void;
use jni_sys::{
    JavaVM, JavaVMInitArgs, JavaVMOption, JNI_CreateJavaVM, JNI_VERSION_1_8,JNI_FALSE, JNIEnv
};

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

    //-Djava.library.path=java::get_java_lib()
    //-Djava.class.path=[jars in app home]

    let lifespam_hack = vec![];
    let mut jvm_option_strings: &[&str] = lifespam_hack.as_ref();

            // Wrap the JVM option string slices in a vector of `CString`s.
        let mut jvm_option_cstrings : Vec<CString> = Vec::new();

        for jvm_option_string in jvm_option_strings {
            jvm_option_cstrings.push(CString::new(*jvm_option_string).unwrap());
        }

        // Create a vector of `JavaVMOption`s, each referencing a `CString`.
        let mut jvm_options : Vec<JavaVMOption> = Vec::new();

        for jvm_option_cstring in &jvm_option_cstrings {

            let mut jvm_option = JavaVMOption::default();
            jvm_option.optionString = jvm_option_cstring.as_ptr() as *mut i8;

            jvm_options.push(jvm_option);
        }
          // Create the JVM arguments.
        let mut jvm_arguments = JavaVMInitArgs::default();
        jvm_arguments.version = JNI_VERSION_1_8;
        jvm_arguments.options = jvm_options.as_mut_ptr();
        jvm_arguments.nOptions = jvm_options.len() as i32;
        jvm_arguments.ignoreUnrecognized = JNI_FALSE;

        let mut jvm: *mut JavaVM = ptr::null_mut();
        let mut jni_environment : *mut JNIEnv = ptr::null_mut();

        unsafe {
        // Try to instantiate the JVM.
        let result = JNI_CreateJavaVM(
            &mut jvm,
            (&mut jni_environment as *mut *mut JNIEnv) as *mut *mut c_void,
            (&mut jvm_arguments as *mut JavaVMInitArgs) as *mut c_void
        );
        }
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
