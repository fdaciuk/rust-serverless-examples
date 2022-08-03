# Rust Lambda + SQS

## Global Dependencies

- Node.js
- NPM / Yarn
- Rust
- Cargo
- x86_64-unknown-linux-musl (*)
- Docker
- Docker Compose
- AWS CLI

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

To configure localstack locally, put this configuration on your `~/.aws/credentials`:

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

## Send message

To send message to SQS, first you must get the URL of the queue:

```
aws --endpoint-url=http://localhost:4566 sqs list-queues
```

You are going to see two URLs: the SQSHooks and the DeadLetter.
Copy the URL of the SQSHooks queue, and execute the following command switching
`<QUEUE_URL>` to URL you got: 

```
aws --endpoint-url=http://localhost:4566 sqs send-message --queue-url <QUEUE_URL> --message-body '{"first_name": "John"}'
```

You should see the response:

```
{
    "MD5OfMessageBody": "<SOME_ID>",
    "MessageId": "<SOME_ID>"
}
```

And in the docker logs, you should see the logs from lambda function:

```
INFO api: BODY: Body { first_name: "John" }
```

# License

MIT
