#[cfg(windows)]
extern crate winres;

#[cfg(windows)]
fn main() {
    if cfg!(target_os = "windows") {
        let mut res = winres::WindowsResource::new();
        res.set_icon("whh.ico");
        res.compile().unwrap();
    }
}
#[cfg(unix)]
fn main() {}
