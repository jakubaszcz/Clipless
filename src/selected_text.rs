#[cfg(target_os = "windows")]
use selection::get_text;

#[cfg(target_os = "windows")]
pub(crate) mod select {
    pub(crate) fn get() -> String {
        selection::get_text()
    }
}

#[cfg(target_os = "linux")]
pub(crate) mod select {
    pub(crate) fn get() -> String {
        "Linux selection is not created, PR are welcome to contribute for the Linux version.".to_string()
    }
}