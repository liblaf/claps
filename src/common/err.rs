#[macro_export]
macro_rules! bail {
    ($msg:literal $(,)?) => {
        return std::result::Result::Err($crate::anyhow!($msg))
    };

    ($err:expr $(,)?) => {
        return std::result::Result::Err($crate::anyhow!($err))
    };

    ($fmt:expr, $($arg:tt)*) => {
        return std::result::Result::Err($crate::anyhow!($fmt, $($arg)*))
    };
}

#[macro_export]
macro_rules! ensure {
    ($cond:expr $(,)?) => {
        if !$cond {
            return std::result::Result::Err($crate::anyhow!(
                std::concat!("Condition failed: `", std::stringify!($cond), "`")
            ));
        }
    };

    ($cond:expr, $msg:literal $(,)?) => {
        if !$cond {
            return std::result::Result::Err($crate::anyhow!($msg));
        }
    };

    ($cond:expr, $err:expr $(,)?) => {
        if !$cond {
            return std::result::Result::Err($crate::anyhow!($err));
        }
    };

    ($cond:expr, $fmt:expr, $($arg:tt)*) => {
        if !$cond {
            return std::result::Result::Err($crate::anyhow!($fmt, $($arg)*));
        }
    };
}

#[macro_export]
macro_rules! anyhow {
    ($msg:literal $(,)?) => {
        $crate::common::log::LogError::log(anyhow::anyhow!($msg))
    };

    ($err:expr $(,)?) => {
        $crate::common::log::LogError::log(anyhow::anyhow!($err))
    };

    ($fmt:expr, $($arg:tt)*) => {
        $crate::common::log::LogError::log(anyhow::anyhow!($fmt, $($arg)*))
    };
}
