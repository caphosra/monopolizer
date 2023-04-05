# Monopolizer

A collection of tools for simulating or analyzing Monopoly game.

## Components

- mplz (a REST API server for simulating Monopoly game)
- mplzc (a command line tool for simulating Monopoly game)
- mplzlib (a collection of functions or structures for doing Monopoly-related calculations)

`mplz` is a kind of REST API server and is designed to receive and respond to calculation requests from other platforms, such as Jupyter notebook. Since these operations are done through the HTTP protocol, there is no point in using this without HTTP clients. In most cases, using `mplzc` instead is far easier and recommended.

`mplzc` is a command line tool for simulating or analyzing the game. This tool has no fewer abilities of calculations than `mplz` and does not require you to deal with HTTP protocols. This application contains not only interactive mode, but also a visual mode, with which you can view the status of the board graphically.

`mplzlib` is a fundamental library for simulating or inspecting the game. All types of calculations `mplz` or `mplzc` can are implemented in it. If you are not content with `mplz` or `mplzc`, it might be helpful to use this library as one of the dependencies.

If you want to know more about these tools, watching the document of each tool may help you.
