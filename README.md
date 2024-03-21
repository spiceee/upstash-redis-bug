### Description

Upstash Redis fails to connect to [redis-rs](https://docs.rs/redis/latest/redis/), the most used Redis Rust crate when using a `MultiplexedConnection`, which is the type of connection that is safe to share between multiple threads in Rust.

I've included a vanilla Redis docker image to prove Redis supports `MultiplexedConnection` out of the box.

To bootstrap the demo:

```sh
docker-compose up -d --build && cargo build && cargo run
```

To see the bug with Upstash, change `REDIS_PRIVATE_URL` in `.env` to an Upstash instance.

```
❯ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.28s
     Running `target/debug/upstash-redis-bug`
thread 'main' panicked at /Users/example/.cargo/registry/src/index.crates.io-6f17d22bba15001f/redis-0.24.0/src/aio/multiplexed_connection.rs:361:21:
internal error: entered unreachable code: Multiplexed connection driver unexpectedly terminated
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```
