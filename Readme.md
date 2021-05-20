# .env
HOST=localhost  
PORT=9000

# autoreload
```cargo watch -x 'run --bin cheapshop'```

# compilation
## installation open ssl
[voir ce site](https://qiita.com/liubin/items/6c94f0b61f746c08b74c)

```sh
export PKG_CONFIG_ALLOW_CROSS=1;export OPENSSL_STATIC=true;export OPENSSL_DIR=/musl; \
cargo build --release --target x86_64-unknown-linux-musl
```