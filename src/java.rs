#[cfg(windows)]
extern crate winreg;

use self::winreg::RegKey;
use self::winreg::enums::{HKEY_LOCAL_MACHINE, KEY_READ};
use std::path::PathBuf;

#[cfg(windows)]
pub fn get_java_path() -> String {
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let jre_key: RegKey = hklm.open_subkey_with_flags("SOFTWARE\\JavaSoft\\Java Runtime Environment", KEY_READ).unwrap();
    let cur_ver: String = jre_key.get_value("CurrentVersion").unwrap();
    jre_key.open_subkey_with_flags(cur_ver, KEY_READ).unwrap().get_value("JavaHome").unwrap()
}

#[cfg(windows)]
pub fn get_java_lib() -> String {
    let mut path = PathBuf::from(get_java_path().to_string());
    path.push("lib");
    return String::from(path.to_str().unwrap());
}

#[cfg(windows)]
pub fn get_java_dll() -> String {
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let jre_key: RegKey = hklm.open_subkey_with_flags("SOFTWARE\\JavaSoft\\Java Runtime Environment", KEY_READ).unwrap();
    let cur_ver: String = jre_key.get_value("CurrentVersion").unwrap();
    jre_key.open_subkey_with_flags(cur_ver, KEY_READ).unwrap().get_value("RuntimeLib").unwrap()
}
