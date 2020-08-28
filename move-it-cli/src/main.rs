//! It moves files from one folder to an other.

use move_it::mv;

fn main() {
    mv(
        "~/tmp/test".to_string(),
        r"~/tmp/test2/{FILE:RELPATH}/{FILE:NAME}".to_string(),
    );
}
