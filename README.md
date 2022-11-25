# Development board API (Rust)

A dashboard for organizing software development tasks (Kanban flow) implemented in Rust

### Front-end

[Here](https://github.com/goto-eof/dev_board_react) you can find the front-end application.

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

### TODO





```diff
- be - association of kanban flow to user (wip);
- fe - association of kanban flow to user (wip);
- be - improve security;
- be - manage better code unwrapping;
- fe - optimize login and registration forms;
- fe - improve form validation
+ be/fe - optimize front-end/back-end in order to understand better if it is a server down issue or the user is not logged in (show a toast for example). Improve json response on the backend side (uniform responses);
+ fe - hide login and register buttons when user is logged in;
+ be - optimize server responses (CORS error when user is not authorized);
+ be - refactor;
+ be - implement controllers for permission and role tables (the idea is to have a control panel where it is possible to assign roles and permissions to users);
+ be/fe - check is logged in every x seconds
+ be - fix column/item swapping;
+ be - refresh token;
@@ text in purple (and bold)@@
```

### Post scriptum

- this pplication was tested on macOS and Linux
- remember to drop database if application not works (perhaps I made some DDL changes)

### Furthermore

#### Generate entities from schema

```
sea-orm-cli generate entity -u postgres://postgres:postgres@127.0.0.1:5432/postgres -o entity/src
```
