# Axum + Postgres Skeleton Application

## Overview

This repository contains a skeleton application built using [Axum](https://github.com/tokio-rs/axum) and [PostgreSQL](https://www.postgresql.org/). It serves as a starting point for creating a new Axum server. 

Inspired by [Zero To Production In Rust](https://www.zero2prod.com) and [realworld-axum-sqlx](https://github.com/launchbadge/realworld-axum-sqlx).

The full list of crates used can be found in the [Cargo.toml](./Cargo.toml) file. However, here are some key ones:

* [Axum](https://github.com/tokio-rs/axum) - A user-friendly, modular web framework built with Tokio, Tower, and Hyper.
* [Sqlx](https://github.com/launchbadge/sqlx) - An asynchronous, pure Rust SQL crate that supports compile-time checked queries without a DSL. It supports PostgreSQL, MySQL, SQLite, and MSSQL.
* [Tracing](https://github.com/tokio-rs/tracing) - A framework for instrumenting Rust programs to collect structured, event-based diagnostic information.
* [Chrono](https://github.com/chronotope/chrono) - A comprehensive Date and Time library for Rust.
* [Serde](https://serde.rs/) - A framework for efficiently and generically serializing and deserializing Rust data structures.
* [Uuid](https://github.com/uuid-rs/uuid) - A library for generating and parsing UUIDs.

## Getting Started

To begin with this project:

### Install `sqlx-cli`

SQLx offers a command-line tool for creating and managing databases as well as migrations. It is available on the Cargo crates registry as `sqlx-cli` and can be installed as follows:

```shell
$ cargo install sqlx-cli --features postgres
```

### Run Postgres

The most straightforward way to run Postgres is by using a container with [a pre-built image][docker-postgres]. The command below will start version 15 of Postgres (the latest at the time of writing) using [Docker] (this command should also work with [Podman], a daemonless FOSS alternative).

```shell
$ docker run -d --name postgres-15 -p 5432:5432 -e POSTGRES_PASSWORD=password postgres:15
```

Replace `{password}` with a password of your choice.

Ensure the Postgres server is running:

```shell
$ docker ps
```

[docker-postgres]: https://hub.docker.com/_/postgres
[Docker]: https://www.docker.com/
[Podman]: https://podman.io/

### Clone this Repository

```shell
$ git clone https://github.com/koskeller/axum-postgres-skeleton
$ cd axum-postgres-skeleton
```

### Configure the Application

The backend application is preferably configured via environment variables. To simplify the process during development, we can use [.env files] to avoid defining the variables each time. As a starting point, you can simply copy the sample `.env` file in this repo (`cp .env.sample .env`) and modify the `.env` file as per the comments therein.

### Set Up the Application Database

With `sqlx-cli` installed and your `.env` file set up, you only need to run the following command to prepare the Postgres database for use:

```shell
$ sqlx db setup
```

### Starting the Application

With everything else set up, all you need to do now is:

```shell
$ cargo run
```

### Autoreloading

To start the server and autoreload on code changes:

```shell
$ cargo watch -q -x run
```

To format .json logs (using [`jq`](https://github.com/jqlang/jq)):

```shell
$ cargo watch -q -x run | jq .
```

## Contributing

Contributions are always welcome! Feel free to check the current issues in this repository for tasks that need attention. If you find something missing or that could be improved, please open a new issue.