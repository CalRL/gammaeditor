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

#[macro_export]
macro_rules! gvas_struct_at {
    ($app_state:expr, $key:expr, $index:expr) => {{
        let gvas = gvas!($app_state);
        let property = gvas.properties.get($key)?;
        let struct_prop = get_struct_property_at_idx(property, $index)?;
        struct_prop
    }};
}

#[macro_export]
macro_rules! gvas_struct_at_mut {
    ($app_state:expr, $key:expr, $index:expr) => {{
        let gvas = gvas_mut!($app_state);
        let property = gvas.properties.get_mut($key)?;
        let struct_prop = get_struct_property_at_idx_mut(property, $index)?;
        struct_prop
    }};
}