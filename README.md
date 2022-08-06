# Rust Serverless Examples

All examples live in their own directories:

- [project](/project): there is nothing here, just a simple `cargo new project_name` with a custom `rustfmt.toml` and `.editorconfig`.
- [rust-lambda](/rust-lambda): the minimum configuration you need to run rust on a lambda
- [rust-api-gateway-v1](/rust-api-gateway-v1): rust in a lambda with api gateway v1 in front of it
- [rust-api-gateway-v1-2](/rust-api-gateway-v1-2): a simpler implementation of rust in a lambda with api gateway v1 in front of it
- [rust-api-gateway-v2](/rust-api-gateway-v2): rust in a lambda with api gateway v2 in front of it, using a rest API + local server
- [rust-lambda-sqs](/rust-lambda-sqs): rust in a lambda awaiting for an sqs event
- [rust-api-gateway-v2-sqs](/rust-api-gateway-v2-sqs): rust in a lambda with api gateway v2 in front of it sending a message to an sqs queue
- [rust-api-gateway-v2-graphql (TODO)](/): rust in a lambda with api gateway v2 in front of it, using a graphql API

# License

MIT
