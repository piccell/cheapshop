# Créer le fichier .env avec le contenu suivant (adapter le port selon besoin)
HOST=localhost  
PORT=9000

# pour le mode dévéloppement, utiliser autoreload
```cargo watch -x 'run --bin cheapshop'```


# pour uploader l'exécutable sur un serveur linux, compiler avec musl
## installation open ssl
[voir ce site](https://qiita.com/liubin/items/6c94f0b61f746c08b74c)

```sh
export PKG_CONFIG_ALLOW_CROSS=1;export OPENSSL_STATIC=true;export OPENSSL_DIR=/musl; \
cargo build --release --target x86_64-unknown-linux-musl
```

# versions
1.0.0 25/06/21
V1

0.13.0 22/06/21
modification prix existant

0.2.0 05/06/21
Price
tri sur nom upper_case (Article, Shop) par trait ItemName.name_upper
