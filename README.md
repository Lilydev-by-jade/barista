# barista

> [!WARNING]
> barista isn't meant for public use (yet?). Use at your own risk.

A multipurpose Discord bot, written in [Rust](https://rust-lang.org),
using [serenity](https://github.com/serenity-rs/serenity)/[poise](https://github.com/serenity-rs/poise).

Originally written for
[The UwU Cafe](https://github.com/theuwucafe/)'s Discord server.

## Development

This repository uses pre-commit hooks to enforce code quality, using
[clippy](https://github.com/rust-lang/rust-clippy), and automatically prepare SQLx queries.

> [!NOTE]  
> This means [SQLx CLI](https://github.com/launchbadge/sqlx/blob/main/sqlx-cli/README.md)
> is _required_ to commit any changes.

## License

Licensed under the [MIT License](/LICENSE-MIT) or
[Apache 2.0 license](/LICENSE-APACHE).
