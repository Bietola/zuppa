use serde::{Deserialize, Serialize};

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
    fn show_msg(&mut self, msg: impl Message);
    fn show_msgln(&mut self, msg: impl Message);
}

/***************/
/* View macros */
/***************/

#[macro_export]
macro_rules! msg {
    ($view:expr, $fmt_str:expr, $( $fmt_arg:expr ),*) => {
        $view.show_msg(format!($fmt_str, $($fmt_arg,)*));
    };

    ($view:expr, $fmt_str:expr) => {
        msg!($view, "{}", $fmt_str);
    };

    ($view:expr) => {
        msg!($view, "");
    };
}

#[macro_export]
macro_rules! msgln {
    ($view:expr, $fmt_str:expr, $( $fmt_arg:expr ),*) => {
        $view.show_msgln(format!($fmt_str, $($fmt_arg,)*));
    };

    ($view:expr, $fmt_str:expr) => {
        msgln!($view, "{}", $fmt_str);
    };

    ($view:expr) => {
        msgln!($view, "");
    };
}
