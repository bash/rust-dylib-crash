use std::sync::OnceLock;

pub static ONCE_LOCK: OnceLock<()> = OnceLock::new();
