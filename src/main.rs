use zero2prod::run;

use std::net::TcpListener;
// Tokio allows main to be async
// This is a rust macro, it operates at token level and implies code is generated at compile time
#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    // Bubble up the io::Error if we failed to bind the address // Otherwise call .await on our Server
    let socket = TcpListener::bind("127.0.0.1:0").unwrap();
    run(socket)?.await
}

/*
> cargo run
warning: unused manifest key: target.aarch64-apple-darwin.rustflags
warning: unused manifest key: target.x86_64-apple-darwin.rustflags
warning: unused manifest key: target.x86_64-pc-windows-gnu.rustflags
warning: unused manifest key: target.x86_64-pc-windows-msvc.rustflags
warning: unused manifest key: target.x86_64-unknown-linux-gnu.rustflags
    Finished dev [unoptimized + debuginfo] target(s) in 0.08s
     Running `target/debug/zero2prod`
Error: Os { code: 48, kind: AddrInUse, message: "Address already in use" }
> lsof -i :8000
COMMAND     PID   USER   FD   TYPE            DEVICE SIZE/OFF NODE NAME
zero2prod 49976 mallen    9u  IPv4 0xbfbe3a15ca503c7      0t0  TCP localhost:irdmi (LISTEN)

 ~/workspace/zero2prod/zero2prod | on main !1 ?3 --------------------------------------------------------------------------------------------------------- at 11:26:58
> kill -9 49976

*/
