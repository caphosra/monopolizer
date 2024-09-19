# Monopolizer

A collection of tools for simulating or analyzing Monopoly game.

<p align="center">
    <img src="./img/mplzc_usage.png" alt="mplzc in visual mode" width=500>
</p>

Fig. `mplz-cli` in visual mode.

## Requirements

- [Rust](https://www.rust-lang.org/tools/install)
- [Node.js](https://nodejs.org/en/download/package-manager)

## Usage

### Web Client

1. Clone the repository.
```
$ git clone https://github.com/caphosra/monopolizer
$ cd monopolizer
```

2. Build `mplz-web`.
```
$ cd web
$ npm install
$ npm run build
```

3. Execute `mplz-server`.
```
cd ..
cargo run --bin mplz-server
```

Now, you can use Monopolizer by accessing to http://localhost:5390.

### Terminal Client

Note that the terminal client lacks some features compared to the web client.

1. Clone the repository.
```
$ git clone https://github.com/caphosra/monopolizer
$ cd monopolizer
```

2. Execute `mplz-cli`.
```
cd ..
cargo run --bin mplz-cli
```

## Components

The project Monopolizer consists of four components:

- `mplz-core` - a core library of Monopolizer. Contains useful functions to do calculations related to Monopoly.
- `mplz-server` - a web server for Monopolizer. Exposes core functions as REST API and hosts the web client.
- `mplz-web` - a web UI for Monopolizer. Provides a way to interact with Monopolizer through GUI. It depends on the server.
- `mplz-cli` - a terminal interface for Monopolizer. If you are familiar with CUI, this can be a good option.

If you want to know more about these tools, watching the document of each tool may help you.
