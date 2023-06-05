use csi_parser::iter::{CsiParser, Output};

fn main() {
    let text = "\x1b[31;33;0mFree pages: {}\n\x1b[31m";

    let result: Vec<Output> = text.csi_parser().collect();

    for out in result {
        match out {
            Output::Text(txt) => {
                println!("{}", txt);
            }
            Output::Escape(csi_seq) => {
                println!("{}", csi_seq);
            }
        }
    }
}
