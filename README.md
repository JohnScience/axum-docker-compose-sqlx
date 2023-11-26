# What is it?

It is an example of a multi-container [`docker-compose`] application where

* `postgres` service is a [Postgres] database created from the [official image](https://hub.docker.com/_/postgres).
* `back` service is an [`axum`] web server where the `postgres` service is accessed using [`sqlx`].

This example arised from my struggle to get a working example of an axum application accessing a postgres database using sqlx.

## Build

```console
docker compose build
```

## Run

```console
docker compose up
```

## Test

```console
curl "localhost:8080"
```

to check if the `back` service is running. It should return `Hello, world!`.

```console
curl "localhost:8080/api/test_db_connection"
```

to check if the `back` service can connect to the `postgres` service. It should return `Successfully connected to the DB!`.

## How does it work?

### Dockerfile

First of all, the `back` service is built using the [`Dockerfile` in the `back` directory](https://github.com/JohnScience/axum-docker-compose-sqlx/blob/main/adcs-example/Dockerfile). This `Dockerfile` is a [multi-stage build] dockerfile, which is beneficial for the final image size. Due to that, instead of producing a 1Gb+ image, the final image is only 12Mb. The dockerfile was inspired by [this article](https://peterprototypes.com/blog/rust-dockerfile-boilerplate/) by Peter Todorov.

It also uses `alpine` as a base image for the final image, which is a very small linux distribution. This is also beneficial for the final image size.

In order to build code using [`sqlx`] macros, such as [`sqlx::query_scalar!`], this dockerfile sets the `SQLX_OFFLINE` environment variable to `true` and uses the files pre-built with `cargo sqlx prepare` from the `.sqlx` directory, as [descirbed](https://github.com/launchbadge/sqlx/blob/main/sqlx-cli/README.md#enable-building-in-offline-mode-with-query) in the documentation of [`sqlx-cli`](https://crates.io/crates/sqlx-cli).

### docker-compose.yml

In order to allow the `back` service access the `postgres` service, the `docker-compose.yml` file sets the `hostname` field of the `postgres` service so that it can be used in the `adcs-example/.env` file, which is used to set the `DATABASE_URL` environment variable of the `back` service.

---

`docker-compose.yml`:

```text
services:
  postgres:
    hostname: postgres_host
...
```

`adcs-example/.env`:

```text
...
DATABASE_URL = postgresql://user:qUu-MAX-7eU-PSW@postgres_host:5432/audio_service_db
                                                 ^^^^^^^^^^^^^
```

`adcs-example/src/main.rs`:

```rust
pub(crate) async fn establish_connection() -> PgPool {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgPool::connect(&database_url)
        .await
        .unwrap_or_else(|e| panic!("Error connecting to {database_url}: {e}"))
}
```

---

Aside from setting `hostname`, it also sets very important **environment values**, which are used to configure the `postgres` service. These values are also used in the `adcs-example/.env` file to configure the `back` service.

`docker-compose.yml`:

```text
services:
  postgres:
    ...
    environment:
      POSTGRES_USER: user
      POSTGRES_PASSWORD: qUu-MAX-7eU-PSW
      POSTGRES_DB: audio_service_db
...
```

`adcs-example/.env`:

```text
DATABASE_URL = postgresql://user:qUu-MAX-7eU-PSW@postgres_host:5432/audio_service_db
                            ^^^^ ^^^^^^^^^^^^^^^                    ^^^^^^^^^^^^^^^^
```

And, finally, it sets the port mappings, so that the `back` service can access the `postgres` service.

`docker-compose.yml`:

```text
services:
  postgres:
    ...
    ports:
      - "5432:5432"
  back:
    ...
    ports:
      - "8080:8080"
...
```

`adcs-example/.env`:

```text
# With 127.0.0.1 this won't work
SOCKET_ADDR = 0.0.0.0:8080
DATABASE_URL = postgresql://user:qUu-MAX-7eU-PSW@postgres_host:5432/audio_service_db
```

`adcs-example/src/main.rs`:

```rust
// ...

pub(crate) async fn establish_db_connection() -> PgPool {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgPool::connect(&database_url)
        .await
        .unwrap_or_else(|e| panic!("Error connecting to {database_url}: {e}"))
}

// ...

#[tokio::main]
async fn main() {
    // load the .env file if it exists
    dotenv().ok();

    let db: PgPool = establish_db_connection().await;

    let addr: SocketAddr = {
        let mut iter = env::var("SOCKET_ADDR")
            .expect("SOCKET_ADDR must be set")
            .to_socket_addrs()
            .expect("Failed to get the list of socket addresses for SOCKET_ADDR");
        iter.next()
            .expect("Failed to get the first socket address for SOCKET_ADDR")
    };
    // ...
}
```

## What is missing?

### Better rust-analyzer support

In PR [#1 "fix: db connection"](https://github.com/JohnScience/axum-docker-compose-sqlx/pull/1) `leon3s` suggested avoiding the problem with Rust-analyzed by adding an entry to `/etc/hosts`:

```text
127.0.0.1 postgres_host
```

My alternative soluion was to have a `.docker.env` file that would be identical to `.env` file except for the absence of `SQLX_OFFLINE = true` line that is used to silence the Rust-analyzer. This solution is not ideal, because it requires to maintain two files, but it works.

### Automated maintenance of the `.sqlx` directory

The `.sqlx` directory is created by running `cargo sqlx prepare` command. It is not ideal to have to run this command manually every time the `.sql` files are changed. It would be better to have a script that would run this command automatically.

[`docker-compose`]: https://docs.docker.com/compose/
[`axum`]: https://github.com/tokio-rs/axum
[Postgres]: https://www.postgresql.org/
[`sqlx`]: https://crates.io/crates/sqlx
[multi-stage build]: https://docs.docker.com/build/building/multi-stage/
[`sqlx::query_scalar!`]: https://docs.rs/sqlx/0.7.3/sqlx/macro.query_scalar.html
