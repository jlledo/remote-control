#[cfg(windows)]
fn main() -> std::io::Result<()> {
    let mut res = winres::WindowsResource::new();
    res.set("FileDescription", "Remote Control");
    res.compile()
}

#[cfg(not(windows))]
fn main() {}
