# shorty-rust
URL Shortner Service on Volga

## Setup
### Install diesel CLI
[Here](https://diesel.rs/guides/getting-started) is the comprehensive tutorial

### Run the Database Migrations
```bash
export DATABASE_URL=<your db connection string>
diesel migration run
```

### Run the app
```bash
cargo run
```
