#[macro_export]
macro_rules! gvas {
    ($s:expr) => {
        $s.gvas_file.as_ref()?.read().ok()?
    };
}

#[macro_export]
macro_rules! gvas_mut {
    ($s:expr) => {
        $s.gvas_file.as_ref()?.write().ok()?
    };
}