extern crate library;
extern crate library_dylib;

use library::ONCE;

fn main() {
    ONCE.call_once(|| {})
}
