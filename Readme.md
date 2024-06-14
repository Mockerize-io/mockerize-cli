![Version](https://img.shields.io/badge/dynamic/toml?url=https%3A%2F%2Fraw.githubusercontent.com%2FMockerize-io%2Fmockerize-cli%2Fmain%2FCargo.toml&query=%24.package.version&label=Latest%20version) ![Tests](https://img.shields.io/github/actions/workflow/status/Mockerize-io/mockerize-cli/test_suite.yml?label=Tests)

[https://mockerize-io](https://mockerize.io/)

# mockerize-cli

Mockerize is an open-source, cross platform, lightweight, server mocking application designed from the ground up to make creating mock development servers quick and painless.

![example running mockerize-cli](docs/images/mockerize-cli.svg)

# Potential use cases

- Integration testing in your CI/CD pipeline
- End-to-end test your client-side HTTP code
- Mock API servers to speed up development

# Building

Assuming that you've already cloned the repo and have [Rust installed](https://doc.rust-lang.org/book/ch01-01-installation.html), you have everything you need to build binaries
for your operating system:

```sh
cargo build --release
```

You may configure additional build options in `Cargo.toml`.

## Cross-compiling

If you would like to compile for a different operating system than your own, you will first need to install the build target toolchain and `cross`, the cross-compiling tools for Rust.
First use the `rustup target list` command to see if you already have the target toolchain installed. You may see available targets in the [Rust docs](https://doc.rust-lang.org/nightly/rustc/platform-support.html).

Install `cross` if you don't already have it:

```sh
cargo install cross
```

Install the target toolchain:

```sh
rustup target add x86_64-unknown-linux-gnu
```

Cross-compile for target:

```sh
cargo build --target=x86_64-unknown-linux-gnu
```

See also: [Cross-compilation - the rustup book](https://rust-lang.github.io/rustup/cross-compilation.html)

# Usage

Before using mockerize-cli, you should create a server config file. This can be done through the Mockerize GUI app, or by creating a JSON file following the Mockerize JSON standard structure.
A server config defines the server's listen address & port, as well as any routes and their paired response(s).

You may generate an example server config to build off of with the `new` command:

```sh
mockerize-cli new my-config.json
```

You should then edit your config file using your editor of choice. When ready, load the config with the `run` command:

```sh
mockerize-cli run ./my-config.json
```

Then you should be able to hit your mock server:

```sh
curl http://127.0.0.1:8080/hello-world
```

## Usage in automated testing

One potential use case for the `mockerize-cli` is to automatically run integration tests; perhaps as part of your CI/CD pipeline.

```sh
# Start mockerize in background, load in our test server config.
# Also get the process ID of the mockerize instance so we can kill it later.
./mockerize-cli run integration-test.json >/dev/null 2>&1 &
pid=$!

# Give it some time to fully start up before we begin our tests.
# Please consider your exact use case here. A short rest is probably good enough,
# though may not be the best choice for you. Perhaps add a health-check endpoint
# to your config, then on something like `wait-for-it` to inform when the server
# is fully operational and serving traffic
sleep 1

# Run tests, then exit
echo "Running (pretend) integration tests"
sleep 5

echo "Done testing. Killing mockerize-cli (PID $pid)"
kill -9 $pid
```

While the above example is often enough for simple use, you may wish to expand upon it.
Do you need to run different server configs per test? Perhaps you may integrate the startup and shutdown of Mockerize within your test runner.
