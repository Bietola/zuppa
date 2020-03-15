/***********/
/* Message */
/***********/

pub trait Message {
    fn as_raw_str(&self) -> String;
}

/// All strings are messages.
impl<S> Message for S
where
    S: AsRef<str>,
{
    fn as_raw_str(&self) -> String {
        self.as_ref().to_owned()
    }
}

/********/
/* View */
/********/

pub trait View {
    fn show_msg(&mut self, msg: &(impl Message));
}

/***************/
/* View macros */
/***************/
#[macro_export]
macro_rules! msg {
    ($view:expr, $fmt_str:expr, $( $fmt_arg:expr ),*) => {
        $view.show_msg(&format!($fmt_str, $($fmt_arg,)*))
    };
}
