# JWT Server
## Getting Started
1. Start Postgres
    ```
    $ docker-compose up
    ```

1. Migration    
Run DDL:
    ```sql
    CREATE TABLE accounts (
        id serial PRIMARY KEY,
        email VARCHAR ( 255 ) UNIQUE NOT NULL,
        password VARCHAR ( 255 ) NOT NULL,
        created_on TIMESTAMP NOT NULL DEFAULT current_timestamp
    );
    ```
1. Run server
    ```
    $ cargo run
    ```

## API
### POST /signup
```sh
curl --location --request POST 'localhost:8088/signup' \
--header 'Content-Type: application/json' \
--data-raw '{
    "email": "test@sample.com",
    "password": "abcd123"
}'
```

### POST /login
```sh
curl --location --request POST 'localhost:8088/login' \
--header 'Content-Type: application/json' \
--data-raw '{
    "email": "test@sample.com",
    "password": "abcd123"
}'

```

### POST /verify
```sh
curl --location --request POST 'localhost:8088/curl --location --request POST 'localhost:8088/verify' \
--header 'Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJleHAiOjE2Mjg1MTM1ODYsImlhdCI6MTU5Njk3NzU4Niwic3ViIjoxMSwiZW1haWwiOiJ0ZXN0QHNhbXBsZS5jb20ifQ.wU2iZqoggf5QHYjBXlHVdNI4OybxBYEWLGqJHYsbf2s'
```
