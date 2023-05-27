use csi_parser::enums::CSISequence;
use csi_parser::parser::parse;

fn main() {
    let t = "👋, \x1b[31;4m🌍\x1b[0m!";
    let mut csi_seqs = vec![];
    for x in parse(t) {
        let csi_seq: CSISequence = x.into();
        csi_seqs.push(csi_seq);
    }
    println!("{:?}", csi_seqs);
}