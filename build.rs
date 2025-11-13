fn main() {
    #[cfg(windows)]
    {
        let _ = winresource::WindowsResource::new()
            .set_icon("images/pokeball.ico")
            .compile()
            .expect("Failed to add Windows icon");
    }
}