# Development board API (Rust)

### Run project

Start DBMS container:

```
docker-compose up
```

Start server:

```
cargo run
```

The server will be reacheable at:

```
http://127.0.0.1:8013
```

### Docker (production)

```
docker-compoer -f docker-compose-production.yml up
```

The server will be reacheable at:

```
http://127.0.0.1:8013
```

### Postman

Import postman collection file from test/postma.json in your postman workspace.

### Technologies

- warp
- sea-orm
- tokio
- postgres

### DB schema

![db schema](db-schema1.png)

### Default user

```
username: admin
password: password
```

### Post scriptum

- this pplication was tested on macOS and Linux
- [here](https://github.com/goto-eof/dev_board_react) you can find the front-end application


