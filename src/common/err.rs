#[macro_export]
macro_rules! ensure {
    ($cond:expr $(,)?) => {
        if !$cond {
            return Err($crate::common::log::LogError::log(anyhow::Error::msg(
                concat!("Condition failed: `", stringify!($cond), "`")
            )));
        }
    };

    ($cond:expr, $msg:literal $(,)?) => {
        if !$cond {
            return Err(crate::common::log::LogError::log(anyhow::anyhow!($msg)));
        }
    };

    ($cond:expr, $err:expr $(,)?) => {
        if !$cond {
            return Err(crate::common::log::LogError::log(anyhow::anyhow!($err)));
        }
    };

    ($cond:expr, $fmt:expr, $($arg:tt)*) => {
        if !$cond {
            return Err(crate::common::log::LogError::log(anyhow::anyhow!($fmt, $($arg)*)));
        }
    };
}
