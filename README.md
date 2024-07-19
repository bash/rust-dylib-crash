# Dylib Crash

run `cargo run` on Windows to crash.

On stable this gives the following error:
```
thread 'main' panicked at /rustc/129f3b9964af4d4a709d1383930ade12dfe7c081\library\std\src\sync\once.rs:208:20:
assertion failed: state_and_queue.addr() & STATE_MASK == RUNNING
```

and on nightly:
```
thread 'main' panicked at /rustc/5affbb17153bc69a9d5d8d2faa4e399a014a211e\library\std\src\sync\once.rs:217:20:
internal error: entered unreachable code: state is never set to invalid values
```