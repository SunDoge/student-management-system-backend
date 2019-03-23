# Student Management System Backend

A rust version [Triple-Z/Student-Management-System-Backend](https://github.com/Triple-Z/Student-Management-System-Backend).

## Quick Start

### Requirement

- mysql/mariadb client
- [rust](https://www.rust-lang.org/learn/get-started)

```bash
cargo install diesel_cli # for database migration
docker-compose up        # in another console
diesel setup             # setup database
diesel migration run     # setup table
cargo run --release      # start app
```