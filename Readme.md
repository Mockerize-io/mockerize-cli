# mockerize-cli

Mockerize is an open-source, cross platform, lightweight, server mocking application designed from the ground up to make creating mock development servers quick and painless.

![example running mockerize-cli](docs/images/mockerize-cli.svg)

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
