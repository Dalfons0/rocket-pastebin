# Pastebin

This is a test implementation of the sample app of the [Rocket framework](https://rocket.rs) called [pastebin](https://rocket.rs/v0.4/guide/pastebin/), adding docker into the game.

## Ignite

To run the app locally is necessary to have installed a nightly version of Rust, as specified in the [documentation](https://rocket.rs/v0.4/guide/getting-started/), so you can start the server with the command:
```bash
$ cargo run
```

## Ignite with Docker

Alternatively, you can run the app with docker, without following the past steps (even without having rust installed), only you have to build the image running the command below, having into account that you are in the root folder of the project, where the Dockerfile is located: 
```bash
$ docker build -t rocket-pastebin .
```

And running the app with:
```bash
$ docker run -d --name rocket-pastebin -p 8000:8000 rocket-pastebin
```