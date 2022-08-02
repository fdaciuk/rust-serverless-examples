# Rust API Gateway v1 + Lambda

## Global Dependencies

- Node.js
- NPM / Yarn
- Rust
- Cargo
- x86_64-unknown-linux-musl (*)
- Docker
- Docker Compose

## x86_64-unknown-linux-musl Installation

**If you are going to build this project locally**, in order to build the project 
properly to use in AWS Lambda, its expected that you install the 
`x86_64-unknown-linux-musl` target on all platforms with:

```
$ rustup target add x86_64-unknown-linux-musl
```

On **Linux** platforms, you will need to install `musl-tools`

On **Mac OSX**, you will need to install a MUSL cross compilation toolchain:

```
$ brew install filosottile/musl-cross/musl-cross
```

## Install

In the root of the project, install Node.js dependencies:

```
yarn
```

In the `functions/api` directory, install Rust dependencies (only if you want to modify something on Rust project):

```
cargo build
```

## Deploy

To configure localstack locally, put this configuration on your `~.aws/credentials`:

```
[default]
aws_access_key_id = temp
aws_secret_access_key = temp
```

Get localstack up and running:

```
yarn docker:up:logs
```

Use env var `DEBUG=1` to see all logs:

```
DEBUG=1 yarn docker:up:logs
```

After that, deploy with the command:

```
yarn deploy:local
```

## Request data from API

To make a request, get the URL given from localstack + the `path` set in 
`serverless.yml` (`/hello`, in this case) and use curl:

```
curl -i -X POST http://localhost:4566/restapis/SOME_ID/local/_user_request_/hello
```

You should see the response:

```
Data: {}%
```

To make a request with arguments, you can use the method `POST`:

```
curl -i -X POST http://localhost:4566/restapis/SOME_ID/local/_user_request_/hello -H 'Content-Type: application/json' -d '{"message": "hi"}'
```

You should see the response:

```
Data: {"message": "hi"}%
```

# License

MIT
