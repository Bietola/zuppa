use super::base::*;

pub struct SimpleTermView;

impl View for SimpleTermView {
    fn show_msg(&mut self, msg: &(impl Message)) {
        println!("{}", msg.as_raw_str());
    }
}
