# Assumptions 

rust + cargo is installed see [rustup.rs](https://rustup.rs/)

# Building

`cargo build`

# Running

Start server with `cargo run --bin server`

```
% cargo run --bin server
    Finished dev [unoptimized + debuginfo] target(s) in 0.30s
     Running `target/debug/server`
Server Starting!
Server Accepting Connections on org.rust-lang.ipc-channel.6982130772920626544
Received message defasd

```

Start a client with `cargo run --bin client` as many times as needed

```
% cargo run --bin client
    Finished dev [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/client`
Client Starting!
Enter pipe name:
org.rust-lang.ipc-channel.-6417404199523183820
Connecting to "org.rust-lang.ipc-channel.-6417404199523183820"
Connected
Connected!
defasd
DISCONNECT
Got disconnect, exiting
mtp@Matthews-MacBook-Pro test-ipc-chanenl % 
```

