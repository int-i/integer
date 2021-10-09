# Integer

> Member Management Service

[![Rust](https://img.shields.io/badge/rust-2018-black.svg?logo=rust&logoColor=white&style=for-the-badge)](https://doc.rust-lang.org/edition-guide/rust-2018/index.html)
[![Rocket](https://img.shields.io/badge/rocket-0.5.0-d33847.svg?logo=data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHZpZXdCb3g9IjAgMCAxMDAgMTAwIj48cGF0aCBmaWxsPSIjRkZGIiBmaWxsLXJ1bGU9ImV2ZW5vZGQiIGQ9Ik02My41NjMgMjkuMjU0YzAgMS40NDQuMzY3IDIuNzI1IDEuMSAzLjc5M0w2Mi4xIDM1LjYxNWMtLjI3Mi4yOTQtLjA3Mi40OTcuMjE2Ljc4NmwyLjAxMiAyLjAwM2MuMjc3LjI5NC40NjUuNDc1Ljc0OC4xODdsMi43OC0yLjc3OGMuNjI0LjIyOSAyLjEyMS42MjggNC4zODUuMjk2bDEuNzQtMS4zMTFjLjEzLS4wOTkuMzU3LS4yNDUuNTA3LS4zMjdsMS41NjMtLjg1NS0xLjI1MyAxLjI2NmMtLjEyLjEyMS0uMzI0LjMtLjQ1My4zOThsLTEuNDk2IDEuMTI3IDIuNTUgNi45NzdhLjQ1LjQ1IDAgMCAwIC43MzUuMTcxbDUuOTg1LTUuNjkzYS40NTUuNDU1IDAgMCAwIC4xMzQtLjI0OGwxLjM0Mi03LjQ5OWMxLjYzMi0xLjQxMSAzLjI4NC0zLjA1MyA0LjkxMy00Ljg4MiA5LjM3Ni0xMC41MyAxMC43NzUtMTguNzYgMTAuNzc1LTIyLjcyOCAwLTEuMzA3LS4xNTktMi4wMjctLjE2NS0yLjA1N2EuNDUyLjQ1MiAwIDAgMC0uMzY1LS4zNDZjLS40NjYtLjA4LTExLjU4MS0xLjc5My0yNi4wNDYgMTIuNjYyLS4wMTguMDItMS44OTcgMS45NTctMy45MiA0LjY3NGwtNy40MjggMi4wNjVhLjQ1MS40NTEgMCAwIDAtLjIyNi4xNDdsLTUuMjg0IDYuMzQ5YS40NTIuNDUyIDAgMCAwIC4yMjEuNzIybDcuMSAyLjA2NCAxLjE5NS0xLjM3NWMuMTA2LS4xMjIuMjk1LS4zMDguNDIzLS40MThsMS4zMzQtMS4xNDMtLjk0NSAxLjQ4YTQuNjcgNC42NyAwIDAgMS0uMzU2LjQ3OGwtMS4yNTggMS40NDd6TTc5LjI0IDEzLjg2OGMyLjM0OC0yLjM0OCA1LjY5NC0yLjgxIDcuNDczLTEuMDMgMS43OCAxLjc4IDEuMzE5IDUuMTI1LTEuMDMgNy40NzQtMi4zNDggMi4zNDgtNS42OTMgMi44MDktNy40NzMgMS4wMy0xLjc4LTEuNzgtMS4zMTgtNS4xMjYgMS4wMy03LjQ3NHpNNjIuMzQyIDQwLjQzIDkuMzU5IDkzLjMzNmEuODE1LjgxNSAwIDAgMS0xLjE1MiAwbC0uNTczLS41NzJhLjgxMy44MTMgMCAwIDEgMC0xLjE1bDUyLjk4Mi01Mi45MDlhLjgxNS44MTUgMCAwIDEgMS4xNTIgMGwuNTc0LjU3M2EuODEzLjgxMyAwIDAgMSAwIDEuMTVtLS43NjIgNi43MzhMOC41OTUgMTAwLjA3NGEuODE1LjgxNSAwIDAgMS0xLjE1MiAwbC0uNTczLS41NzNhLjgxMy44MTMgMCAwIDEgMC0xLjE1bDUyLjk4My01Mi45MDhhLjgxNS44MTUgMCAwIDEgMS4xNTIgMGwuNTczLjU3MmEuODEzLjgxMyAwIDAgMSAwIDEuMTVNMi4wMjcgOTQuMzg4YS44MTcuODE3IDAgMCAxLTEuMTUyIDBMLjMgOTMuODE2YS44MTMuODEzIDAgMCAxIDAtMS4xNWw1Mi45ODMtNTIuOTFhLjgxNS44MTUgMCAwIDEgMS4xNTIgMGwuNTc0LjU3M2EuODEzLjgxMyAwIDAgMSAwIDEuMTVMMi4wMjcgOTQuMzl6Ii8+PC9zdmc+&style=for-the-badge)](https://rocket.rs/)
[![PostgreSQL](https://img.shields.io/badge/postgres-13-4169e1.svg?logo=postgresql&logoColor=white&style=for-the-badge)](https://www.postgresql.org/docs/13/index.html)
[![GitHub Workflow](https://img.shields.io/github/workflow/status/int-i/integer/Rust?logo=github&logoColor=white&style=for-the-badge)](https://github.com/int-i/integer/actions)
[![License](https://img.shields.io/github/license/int-i/integer?style=for-the-badge)](./LICENSE) 

## Build

### Requirement

- [Rust](https://www.rust-lang.org/)
- [libpq](https://www.postgresql.org/docs/13/libpq.html)
    - `libpq-dev` (Debian)
    - `postgresql-dev` (Alpine Linux)

### Guide

1. Clone the repository:

    ```bash
    $ git clone https://github.com/int-i/integer.git
    ```

2. Build the source:

    ```bash
    $ cargo build
    ```

3. Run PostgreSQL:

    ```bash
    $ docker-compose -f docker-compose.db.yml up -d
    ```

4. Run the application:

    ```bash
    $ DB_USER=inti DB_PASSWORD=password cargo run
    ```

## Deploy

### Requirement

- [Docker](https://www.docker.com/)
- `cert.pem` - a certificate
- `key.pem` - a private key

### Guide

1. Build a Docker image:

    ```bash
    $ docker build -t integer_api .
    ```

2. Create a Docker secret for `site.pem`:

    ```bash
    $ cat cert.pem key.pem | docker secret create site.pem -
    ```

3. Deploy the application to Docker Swarm

    ```bash
    $ docker swarm init
    $ docker stack deploy -c docker-compose.yml inti
    ```
