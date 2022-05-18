use time::{macros::format_description, format_description::FormatItem};

pub const DATE_FORMAT_STR: &'static [FormatItem<'static>] = format_description!("[day]-[month]-[year] - [hour]:[minute]:[second]Z");

#[macro_export]
macro_rules! plog {
    () => (print!("\n"));
    ($($arg:tt)*) => ({ 
        use time::OffsetDateTime; 
        use crate::util::DATE_FORMAT_STR;
        use colored::*;
        print!("{}{}{} {}\n", 
            "[Perdia]".bold().cyan(),
            format!("[{}]", OffsetDateTime::now_utc().format(&DATE_FORMAT_STR).unwrap_or("".to_string())).cyan(),
            format!("[{}]:", std::process::id()).cyan(),
            format!("{}", format_args!($($arg)*)).bright_white()
        ) 
    });
}

#[macro_export]
macro_rules! pwarn {
    () => (print!("\n"));
    ($($arg:tt)*) => ({ 
        use time::OffsetDateTime; 
        use crate::util::DATE_FORMAT_STR;
        use colored::*;
        print!("{}{}{} {}\n", 
            "[Perdia]".bold().cyan(),
            format!("[{}]", OffsetDateTime::now_utc().format(&DATE_FORMAT_STR).unwrap_or("".to_string())).cyan(),
            format!("[{}]:", std::process::id()).cyan(),
            format!("{}", format_args!($($arg)*)).on_bright_yellow().black().bold()
        ) 
    });
}

#[macro_export]
macro_rules! perr {
    () => (print!("\n"));
    ($($arg:tt)*) => ({ 
        use time::OffsetDateTime; 
        use crate::util::DATE_FORMAT_STR;
        use colored::*;
        print!("{}{}{} {}\n", 
            "[Perdia]".bold().cyan(),
            format!("[{}]", OffsetDateTime::now_utc().format(&DATE_FORMAT_STR).unwrap_or("".to_string())).cyan(),
            format!("[{}]:", std::process::id()).cyan(),
            format!("{}", format_args!($($arg)*)).on_bright_red().black().bold()
        ) 
    });
}