### CSI parser
This is a simple CSI parser that only provides parsing for a subset of common CSI.

This repository was inspired by [cansi](https://github.com/kurtlawrence/cansi/tree/master)

If you have additional requirements, feel free to submit a PR (Pull Request)

Please refer to the definition of [CSI.](https://en.wikipedia.org/wiki/ANSI_escape_code#CSI_(Control_Sequence_Introducer)_sequences)

See the [rs docs.](https://docs.rs/csi_parser/)

Look at progress and contribute on [github.](https://github.com/YageGeng/csi_parser)

## Example
```rust
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
```

and you will be got the result:
```text
[Some(31);None;Some(4)m
🌍
[Some(0);None;Nonem
```

### Features
To support the `no_std` feature, you simply need to run `cargo add --no-default-features -F no_std` to your project.

