use serde::{Deserialize, de::DeserializeOwned, Serialize};

/***********/
/* Message */
/***********/
pub trait Message: Serialize {
    fn as_raw_str(&self) -> String;
}

/// All strings are messages.
impl<S> Message for S
where
    S: AsRef<str>,
    S: for<'a> Deserialize<'a>,
    S: Serialize,
{
    fn as_raw_str(&self) -> String {
        self.as_ref().to_owned()
    }
}

/********/
/* View */
/********/
pub trait View {
    fn show_msg<'a>(&mut self, msg: impl Message);
    fn show_msgln<'a>(&mut self, msg: impl Message);
}

/***************/
/* View macros */
/***************/

#[macro_export]
macro_rules! msg {
    ($views:expr, $fmt_str:expr, $( $fmt_arg:expr ),*) => {
        $views.iter_mut().for_each(|mut v| v.show_msg(&format!($fmt_str, $($fmt_arg,)*)));
    };

    ($views:expr, $fmt_str:expr) => {
        msg!($views, "{}", $fmt_str);
    };

    ($views:expr) => {
        msg!($views, "");
    };
}

#[macro_export]
macro_rules! msgln {
    ($views:expr, $fmt_str:expr, $( $fmt_arg:expr ),*) => {
        $views.iter_mut().for_each(|v| v.show_msgln(&format!($fmt_str, $($fmt_arg,)*)));
    };

    ($views:expr, $fmt_str:expr) => {
        msgln!($views, "{}", $fmt_str);
    };

    ($views:expr) => {
        msgln!($views, "");
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn macro_test() {
        use crate::simple_term::SimpleTermView;

        let mut views: Vec<_> = std::iter::repeat(SimpleTermView).take(3).collect();

        msgln!(views, "hello there {}{}", "world", "!");

        assert!(true);
    }
}
