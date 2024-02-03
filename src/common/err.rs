#[macro_export]
macro_rules! ensure {
    ($cond:expr $(,)?) => {
        if !$cond {
            return Err(anyhow::Error::msg(
                concat!("Condition failed: `", stringify!($cond), "`")
            ));
        }
    };

    ($cond:expr, $msg:literal $(,)?) => {
        if !$cond {
            return Err(anyhow::anyhow!($msg));
        }
    };

    ($cond:expr, $err:expr $(,)?) => {
        if !$cond {
            return Err(anyhow::anyhow!($err));
        }
    };

    ($cond:expr, $fmt:expr, $($arg:tt)*) => {
        if !$cond {
            return Err(anyhow::anyhow!($fmt, $($arg)*));
        }
    };
}
