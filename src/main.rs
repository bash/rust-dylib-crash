#[allow(unused_imports)]
use library_dylib;

use library::ONCE_LOCK;

fn main() {
    ONCE_LOCK.get_or_init(Default::default);
}
