fn main() {
    let mut res = winres::WindowsResource::new();
    res.set_icon("assets/icon/icon.ico");
    res.compile().unwrap();
}