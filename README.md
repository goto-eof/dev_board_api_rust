```
    dMMMMb  dMMMMMP dMP dMP dMMMMb  .aMMMb  .aMMMb  dMMMMb  dMMMMb 
   dMP VMP dMP     dMP dMP dMP"dMP dMP"dMP dMP"dMP dMP.dMP dMP VMP 
  dMP dMP dMMMP   dMP dMP dMMMMK" dMP dMP dMMMMMP dMMMMK" dMP dMP  
 dMP.aMP dMP      YMvAP" dMP.aMF dMP.aMP dMP dMP dMP"AMF dMP.aMP   
dMMMMP" dMMMMMP    VP"  dMMMMP"  VMMMP" dMP dMP dMP dMP dMMMMP"    
                                                                   
```

# Development board API (Rust)

A dashboard for organizing software development tasks (Kanban flow) implemented in Rust


### Front-end

[Here](https://github.com/goto-eof/dev_board_react) you can find the front-end application.


# Architecture

![architecture](architecture.png)

### Run project as docker container 

Start DBMS container and API server:

```
docker-compoer -f docker-compose-dbms.yml up

docker-compose up
```
### Run project for development

Start server:

Run RDBMS
```
docker-compose -f docker-compose-dbms.yml up
```

Run application
```
cargo run
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

![db schema](schema.png)

### Default user

```
username: admin
password: password
```

### DB credentials
```
url: localhost:5432
username: postgres
password: postgres
```

### Post scriptum

- this application was tested on macOS and Linux
- remember to drop database if application not works (perhaps I made some DDL changes)

### Furthermore

#### Generate entities from schema

```
sea-orm-cli generate entity -u postgres://postgres:postgres@127.0.0.1:5432/postgres -o entity/src
```

### Rebuild docker image

```
docker-compose up -d --no-deps --build
```

if any problems arise, feel free to [contact me](http://andre-i.eu/#contactme).
