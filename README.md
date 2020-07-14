# example of libzmq + docker issue encountered

```
cargo test
```

on my local machine (Arch Linux, libc) the test works just fine.

```
docker build -t minimal-zmq .
docker run -it minimal-zmq
bash-5.0#  cargo test
   Compiling minimal-zmq v0.1.0 (/src)
    Finished test [unoptimized + debuginfo] target(s) in 0.95s
     Running target/debug/deps/minimal_zmq-62a0c84e690adc4f

running 1 test
Assertion failed: false (src/session_base.cpp:633)
```

on the docker target (Alpine Linux, musl) the test doesn't run.

the failing assertion is [this one](https://github.com/zeromq/libzmq/blob/v4.3.2/src/session_base.cpp#L633).

## solution

Swapping the intepreter with patchelf/symlink wasn't actually working. Something in the calls that the binary was trying to make resulted in
the check for protocols failing when forced to use `musl` for reals. Adding `RUSTFLAGS="-C target-feature=-crt-static"` to force a dynamic build
that links correctly to `musl`. After doing that, the test passed.
