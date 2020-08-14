//! It moves files from one folder to an other.

use move_it::mv;

fn main() {
    mv("~/tmp/test".to_string(), "~/tmp/test2".to_string());
}
