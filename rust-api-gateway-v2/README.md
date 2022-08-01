# Rust API Gateway v2 + Lambda + Local Server

A simpler implementation of rust in a lambda with api gateway v2 in front of it,
and a local server to test locally

## Global Dependencies

- Node.js
- NPM / Yarn
- Rust
- Cargo
- `cargo-cmd` (optional)
- `cargo-watch`
- x86_64-unknown-linux-musl (*)

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

## Run server locally

First, In the `functions/api` directory, make a copy of `.env.example` to `.env`.

Then, to run the webserver locally, just execute:

```
cargo cmd dev
```

If you don't have `cargo-cmd` installed, you have to run:

```
source .env && cargo watch -x 'run'
```

This command will get the server up and running on `http://localhost:3000`.

To make requests, read the session `Request data from API` below, and use 
`http://localhost:3000` as `URL`.

## Deploy

API Gateway V2 only works in localstack if you have a PRO plan. So, you should
deploy directly on AWS if you want to test this in production mode.

Check the Serverless Framework documentation to learn how to make a deploy.

As an example, if you want to deploy a project in dev mode, you can run:

```
yarn serverless deploy --stage dev
```

Don't forget to configure your AWS credentials.

## Request data from API

To make a request, get the URL given from serverless + the `path` set in 
`serverless.yml` (`/hello`, in this case) and use curl:

```
curl http://<URL>/hello
```

You should see the response:

```
Hello Rust!%
```

There is a route to test a request with POST method:

```
curl -i -X POST http://<URL>/hello/user -H 'Content-Type: application/json' -d '{"first_name": "John", "last_name": "Doe"}'
```

You sould see the response:

```
{"first_name":"John","last_name":"Doe"}%
```

The data is being validated with serde. If you don't pass some of the required 
keys (`first_name` or `last_name` in this case), you will see an error message.

Try calling without `last_name`, for example:

```
curl -i -X POST http://<URL>/hello/user -H 'Content-Type: application/json' -d '{"first_name": "John"}'
```

You should see the response:

```
HTTP/1.1 400 Bad Request
content-length: 69
content-type: text/plain; charset=utf-8
date: Mon, 01 Aug 2022 23:46:59 GMT

Json deserialize error: missing field `last_name` at line 1 column 26%
```

# License

MIT
