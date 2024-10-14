# Monoshot Project

This project is a Rust application using the Rocket framework for the backend and Diesel as ORM to interact with the MySQL database. The database is managed via Docker.

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)
- [MySQL](https://www.mysql.com/downloads/)

## Running Migrations

Migrations are managed by Diesel. To run the migrations, use the command:

- First setup diesel

  ```sh
  diesel setup
  ```

- Then run migration

  ```sh
  diesel migration run
  ```

## Running the Project

To run the project, use the following commands:

```sh
cargo build
cargo run
```

## Endpoints

The application exposes the following endpoints:

- `GET /`: Returns a list of all shots.
- `POST /`: Creates a new shot.
- `GET /<shot_id>`: Returns a specific shot by ID.
- `PUT /<shot_id>`: Updates a specific shot by ID.
- `DELETE /<shot_id>`: Deletes a specific shot by ID.

## Relevant Files

- Diesel configuration: `diesel.toml`
- Cargo configuration: `Cargo.toml`
- Main file: `src/main.rs`
- Controllers: `src/controller.rs`
- Models: `src/models.rs`
- Database connection: `src/database.rs`
- Database schema: `src/schema.rs`
- Migrations: `migrations/`

## Useful Commands

- To create a new migration:

  ```sh
  diesel migration generate migration_name
  ```

- To revert the last migration:

  ```sh
  diesel migration revert
  ```

## Contribution

Feel free to open issues and pull requests. All contributions are welcome!

## License

This project is licensed under the MIT license.
