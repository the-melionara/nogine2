#[macro_export]
macro_rules! log_info {
    ($($x:tt)*) => {
        $crate::log::log_raw(format!($($x)*), $crate::log::LogType::Info, file!(), line!())
    };
}

#[macro_export]
macro_rules! log_warn {
    ($($x:tt)*) => {
        $crate::log::log_raw(format!($($x)*), $crate::log::LogType::Warn, file!(), line!())
    };
}

#[macro_export]
macro_rules! log_error {
    ($($x:tt)*) => {
        $crate::log::log_raw(format!($($x)*), $crate::log::LogType::Error, file!(), line!())
    };
}

#[macro_export]
macro_rules! unwrap_res {
    ($e:expr) => {
        match $e {
            Ok(x) => x,
            Err(e) => {
                $crate::log_error!("{}", e);
                std::process::exit(1);
            },
        }
    };
}

#[macro_export]
macro_rules! unwrap_opt {
    ($e:expr, $($tt:tt)+) => {
        match $e {
            Some(x) => x,
            None => {
                $crate::log_error!($($tt)+);
                std::process::exit(1);
            }
        }
    };
    
    ($e:expr) => {
        match $e {
            Some(x) => x,
            None => {
                $crate::log_error!("Unwrapped on 'None'.");
                std::process::exit(1);
            },
        }
    };
}

#[macro_export]
macro_rules! assert_expr {
    ($e:expr, $($tt:tt)+) => {
        if !$e {
            $crate::log_error!($($tt)+);
            std::process::exit(1);
        }
    };

    ($e:expr) => {
        if !$e {
            $crate::log_error!("Assert failed!");
            std::process::exit(1);
        }
    };
}

#[macro_export]
macro_rules! crash {
    ($($tt:tt)+) => {
        {
            $crate::log_error!($($tt)+);
            std::process::exit(1);
        }
    };
}

#[derive(Debug, Clone, Copy)]
pub enum LogType {
    Info, Warn, Error
}

pub fn log_raw(msg: String, kind: LogType, file: &str, line: u32) {
    let metadata = format!("[{file}, Ln {line}]");
    match kind {
        LogType::Info => println!("[INFO] {msg}"),
        LogType::Warn => eprintln!("\x1b[93m[WARN] {msg} {metadata}\x1b[0m"),
        LogType::Error => eprintln!("\x1b[101;30m[ERR.] {msg} {metadata}\x1b[0m"),
    }
}

pub fn init_log() {
    // TODO: Enable colors for windows
    
    log_info!("NOGINE2: Logger initialized");
}
