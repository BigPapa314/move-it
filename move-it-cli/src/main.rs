//! It moves files from one folder to an other.

use move_it::mv;

fn main() {
    mv("~/test2".to_string(), "~/test".to_string());
}
