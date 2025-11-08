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

#[macro_export]
macro_rules! unwrap_gvas {
    ($cell:expr) => {{
        use $crate::logger::Logger;

        let Some(cell) = $cell.get() else {
            Logger::info("GVAS_FILE not initialized".to_string());
            return;
        };

        match cell.read() {
            Ok(guard) => guard,
            Err(err) => {
                Logger::info(format!("GVAS_FILE read failed: {}", err));
                return;
            }
        }
    }};
}

#[macro_export]
macro_rules! unwrap_gvas_mut {
    ($cell:expr) => {{
        use $crate::logger::Logger;

        let Some(cell) = $cell.get() else {
            Logger::info("GVAS_FILE not initialized".to_string());
            return;
        };

        match cell.write() {
            Ok(guard) => guard,
            Err(err) => {
                Logger::info(format!("GVAS_FILE write failed: {}", err));
                return;
            }
        }
    }};
}

#[macro_export]
macro_rules! try_gvas_read {
    ($cell:expr) => {{
        use $crate::logger::Logger;
        use std::sync::TryLockError;

        match $cell.get() {
            Some(cell) => match cell.try_read() {
                Ok(guard) => Some(guard),
                Err(TryLockError::WouldBlock) => {
                    Logger::warn("Are we deadlocked? Couldn't acquire write lock!");
                    None
                }
                Err(err) => {
                    Logger::info(format!("GVAS_FILE read failed: {}", err));
                    None
                }
            },
            None => {
                Logger::info("GVAS_FILE not initialized".to_string());
                None
            }
        }
    }};
}

#[macro_export]
macro_rules! try_gvas_write {
    ($cell:expr) => {{
        use $crate::logger::Logger;
        use std::sync::TryLockError;

        match $cell.get() {
            Some(cell) => match cell.try_write() {
                Ok(guard) => Some(guard),
                Err(TryLockError::WouldBlock) => {
                    Logger::warn("Are we deadlocked? Couldn't acquire write lock!");
                    None
                }
                Err(err) => {
                    Logger::info(format!("GVAS_FILE write failed: {}", err));
                    None
                }
            },
            None => {
                Logger::info("GVAS_FILE not initialized".to_string());
                None
            }
        }
    }};
}