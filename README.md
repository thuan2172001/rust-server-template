# Actix-web REST API with JWT
## Require

- [Rust Stable](https://rustup.rs)
- [Postgres](https://www.postgresql.org/)

Or using [Docker](https://www.docker.com/)
## How to run
### Manual

- Rename `secret.key.sample` to `secret.key` or create your own key by running `head -c16 /dev/urandom > secret.key` in command line (Linux/UNIX only) and copy to `/src` folder
- Create a database in postgres cli or [pgAdmin](https://www.pgadmin.org/) tool
- Rename `.env.sample` to `.env` and update the database connection string in `DATABASE_URL` key.
- Build with release profile: `cargo build --release`
- Run release binary in command line/terminal.
  - Windows: `target/release/app.exe`
  - Linux/UNIX: `target/release/app`

### Docker

- Enter into project directory
- Run `docker-compose -f docker-compose.local.yml up` for local environment or `docker-compose -f docker-compose.prod.yml up` for production environment

## APIs

### Address: **`localhost:8000`**

### `GET /api/ping`: Ping
```bash
curl -X GET -i 'http://127.0.0.1:8000/api/ping'
```
- Response:
    - 200 OK
    ```
    pong!
    ```

### `POST /api/auth/signup`: Signup
```bash
curl -X POST -i 'http://127.0.0.1:8000/api/auth/signup' -H "Content-Type: application/json" --data '{"username": "thuan", "email": "thuan2172001@gmail.com", "password": "thuan123", "name": "Trinh Van Thuan", "gender": true, "age": 18, "address": "Dong Anh", "phone": "031231336033" }'
```
### `POST /api/auth/login`: Login
```bash
curl -X POST -H 'Content-Type: application/json' -i 'http://127.0.0.1:8000/api/auth/login'  \
  --data '{"username_or_email":"user",  "password":"4S3cr3tPa55w0rd"}'
```
### `POST /api/auth/logout`: Logout
```bash
curl -X POST -H 'Content-Type: application/json' \
  -H 'Authorization: bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJpYXQiOjE1NzcyNTc4NzksImV4cCI6MTU3Nzg2MjY3OSwidXNlciI6ImMiLCJsb2dpbl9zZXNzaW9uIjoiYzUxNWE3NTg3NGYzNGVjNGFmNDJmNWE2M2QxMDVjMGYifQ.B9w6FxFdypb5GCRMKXZ9CZWFxQLFjvmPSusMCtcE-Ac' \
  -i 'http://127.0.0.1:8000/api/auth/logout'
```
### `PUT /api/user/{id}`: Update person information by id
```bash
curl -X PUT -H 'Content-Type: application/json' \
  -H 'Authorization: bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJpYXQiOjE2MzQ3MTg3MTYsImV4cCI6MTYzNTMyMzUxNiwidXNlciI6InRodWFuIiwibG9naW5fc2Vzc2lvbiI6ImU3NWRhYTZiNGIxYjQ2NGJhYTNkNGI5OGNjMjdiMDM1In0.YB3EI2exYcxG1fhkxtGBdkyYaTei30OlPCtUHRB_ll0' \
  -i 'http://127.0.0.1:8000/api/user/4' \
  --data '{
    "name": "Thuan Van Trinh",
    "gender": true,
    "age": 32,
    "address": "addr",
    "phone": "1339998822",
    "email": "bbaq@gmail.com"
  }
'
```
### `DELETE /api/user/{id}`: Delete person information by id
```bash
curl -X DELETE -H 'Content-Type: application/json' \
  -H 'Authorization: bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJpYXQiOjE2MzQ3MTg3MTYsImV4cCI6MTYzNTMyMzUxNiwidXNlciI6InRodWFuIiwibG9naW5fc2Vzc2lvbiI6ImU3NWRhYTZiNGIxYjQ2NGJhYTNkNGI5OGNjMjdiMDM1In0.YB3EI2exYcxG1fhkxtGBdkyYaTei30OlPCtUHRB_ll0' \
  -i 'http://127.0.0.1:8000/api/user/4'
```

### brower OPTIONS curl request example
```bash
curl -X OPTIONS -i 'http://127.0.0.1:8000/api/login' \
  -H "Origin: http://example.com" -H "Access-Control-Request-Method: POST"
```