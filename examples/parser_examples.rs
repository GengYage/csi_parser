use csi_parser::iter::{CsiParser, Output};

fn main() {
    let text = "👋, \x1b[31;4m🌍\x1b[0m!";

    let result: Vec<Output> = text.csi_parser().skip(1).collect();

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