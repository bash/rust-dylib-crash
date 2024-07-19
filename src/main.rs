#[allow(unused_imports)]
use library_dylib;

use library::ONCE;

fn main() {
    ONCE.call_once(|| {})
}
