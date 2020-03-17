use super::base::*;
use std::io::{Write, stdout};

pub struct SimpleTermView;

impl View for SimpleTermView {
    fn show_msg(&mut self, msg: impl Message) {
        print!("{}", msg.as_raw_str());
        stdout().flush().unwrap();
    }

    fn show_msgln(&mut self, msg: impl Message) {
        println!("{}", msg.as_raw_str());
        stdout().flush().unwrap();
    }
}
