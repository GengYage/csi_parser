### CSI parser
This is a simple CSI parser that only provides parsing for a subset of common CSI.

This repository was inspired by [cansi](https://github.com/kurtlawrence/cansi/tree/master)

If you have additional requirements, feel free to submit a PR (Pull Request)

Please refer to the definition of [CSI.](https://en.wikipedia.org/wiki/ANSI_escape_code#CSI_(Control_Sequence_Introducer)_sequences)

See the [rs docs.](https://docs.rs/csi_parser/)

Look at progress and contribute on [github.](https://github.com/YageGeng/csi_parser)

## Example
```rust
use csi_parser::enums::CSISequence;
use csi_parser::parser::parse;

fn main() {
    let t = "👋, \x1b[31;4m🌍\x1b[0m!";
    let mut csi_seqs = vec![];
    for x in parse(t) {
        let csi_seq: CSISequence = x.into();
        csi_seqs.push(csi_seq);
    }
    println!("{:#?}", csi_seqs);
}
```

and you will be got the result:
```text
[
    Color(
        Some(
            31,
        ),
        None,
        Some(
            4,
        ),
    ),
    Color(
        Some(
            0,
        ),
        None,
        None,
    ),
]
```

