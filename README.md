# Axum + Postgres Skeleton Application

## Overview

This repository contains a skeleton application built using [Axum](https://github.com/tokio-rs/axum) and [PostgreSQL](https://www.postgresql.org/). It serves as a starting point for creating a new Axum server. 

The full list of crates used can be found in the [Cargo.toml](./Cargo.toml) file. However, here are some key ones:

* [Axum](https://github.com/tokio-rs/axum) - A user-friendly and modular web framework built with Tokio, Tower, and Hyper.
* [Sqlx](https://github.com/launchbadge/sqlx) - An asynchronous, pure Rust SQL crate that supports compile-time checked queries without a DSL. It supports PostgreSQL, MySQL, SQLite, and MSSQL.
* [Tracing](https://github.com/tokio-rs/tracing) - A framework for instrumenting Rust programs to collect structured, event-based diagnostic information.
* [Chrono](https://github.com/chronotope/chrono) - A comprehensive Date and Time library for Rust.
* [Serde](https://serde.rs/) - A framework for efficiently and generically serializing and deserializing Rust data structures.
* [Uuid](https://github.com/uuid-rs/uuid) - A library for generating and parsing UUIDs.

## Getting Started

To get started with this project:

1. Install [Rust](https://www.rust-lang.org/)
2. Install [Docker](https://www.docker.com/) and run a [PostgreSQL](https://www.postgresql.org/) container if you don't have one already.
3. Clone this repository to a local directory.
4. Copy the [.env.example](./.env.example) file to a new file named `.env` in the same directory, and update the environment variables as per your system's configuration.
5. Set up your database by executing `./db`. Ensure it completes successfully.
6. Build the project using `cargo build`. You can also compile with the `--release` flag if desired.
7. Run the application with `cargo run`.

## Contributing

Contributions are welcome! Feel free to check the current issues in this repository for tasks that need attention. If you identify something that is missing or could be improved, please open a new issue.