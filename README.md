# Rust Serverless Examples

All examples live in their own branches:

- [main](): there is nothing here, just a simple `cargo new project_name` with a custom `rustfmt.toml` and `.editorconfig`.
- [rust-lambda](): the minimum configuration you need to run rust on a lambda
- [rust-lambda-2](): rust in a lambda with entry data validation (serde)
- [rust-api-gateway-v1](): rust in a lambda with api gateway v1 in front of it
- [rust-api-gateway-v2](): rust in a lambda with api gateway v2 in front of it
- [rust-api-gateway-v2-rest](): rust in a lambda with api gateway v2 in front of it, using a rest API
- [rust-api-gateway-v2-graphql](): rust in a lambda with api gateway v2 in front of it, using a graphql API
- [rust-lambda-sqs](): rust in a lambda awaiting for an sqs event

# License

MIT
