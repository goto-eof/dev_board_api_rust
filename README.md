# Development board API (Rust)

### Run project

Start DBMS container:

```
docker-compose -f docker-compose-postgres.yml up
```

Start server:

```
cargo run
```

### Docker

```
docker-compoer up
```

The server will be reacheable at:

```
http://127.0.0.1:8000
```


### Postman


Import postman collection file from test/postma.json in your postman workspace.

### Technologies

- warp
- sea-orm
- tokio
- postgres
