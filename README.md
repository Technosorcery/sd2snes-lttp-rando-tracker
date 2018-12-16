# sd2snes-lttp-rando-tracker

[![Travis CI Build Status](https://travis-ci.com/jhelwig/sd2snes-lttp-rando-tracker.svg?branch=master)](https://travis-ci.com/jhelwig/sd2snes-lttp-rando-tracker)

## About

sd2snes-lttp-rando-tracker can be used to automatically track collected items in
[The Legend of Zelda: A Link to the Past Randomizer](http://alttpr.com), when
using an SD2SNES with the [USB2SNES](https://github.com/RedGuyyyy/sd2snes/releases)
firmware. sd2snes-lttp-rando-tracker will periodically poll the SD2SNES over the
USB serial connection to track the retrieved item states.

sd2snes-lttp-rando-tracker is supported on macOS, Windows, and Linux.

## Usage

Pre-compiled binaries are available from the [releases listing](https://github.com/jhelwig/sd2snes-lttp-rando-tracker/releases). Download the
binary, and put it in the location of your choosing.

### macOS & Linux

```Shell
sd2snes-rando-tracker --serial /path/to/sd2snes/serial/device
```

`/path/to/sd2snes/serial/device` will probably be something like `/dev/tty.usbmodem<sequence of numbers>`.

### Windows

Open the terminal of your choosing, and change directory to where you downloaded
the binary.

```Shell
sd2snes-rando-tracker.exe --serial COM<number>
```

`COM<number>` should be the COM device that the SD2SNES creates on your system.
This can be found via Device Manager.

### General

After starting sd2snes-rando-tracker, it will let you know what address & port
the UI HTTP server was started on. By default, the HTTP server binds to all
addresses on port 8000. Open `http://localhost:8000` in your browser to view the
UI. If you wish to only view the item/dungeon tracker, or the map, you can go to
`http://localhost:8000/#/items` and `http://localhost:8000/#/map`, respectively.
These limited views can be handy for use with things like the browser capture in
OBS Studio.

Additional help is available via the `--help` flag. You can use `--file <file>`
to test things using the `example-data.json` file in the repository, if you do
not have a SD2SNES with the USB2SNES firmware available (very handy for
developing while away from a SNES).

## Not yet implemented

* Calculate map location availability based on retrieved items.
* Persist item collection state during save & quit. Items re-appear once save is
  loaded.

## Acknowledgements

The UI is heavily modeled after, and uses assets from [pickfifteen/lttp-tracker](https://github.com/pickfifteen/lttp-tracker/) under the MIT license.

## Development

### Requirements

* Nightly Rust compiler. Recommend using [`rustup`](https://rustup.rs/)
* Node.js & Yarn

### Running in development

#### UI

The Vue.js UI is under the `ui` directory, and can be served during development
by running `yarn start`, after running `yarn install` to install all
dependencies.

#### CLI & API server

The main entry point for building the project is via running `cargo build` from
the root of the project. By default, this will also build the UI, but this can
be skipped in development by setting the `SKIP_UI_BUILD` environment variable to
have any value. This can be useful when running the UI separately via `yarn`.

There is an `example-data.json` in the root of the project that can be useful
when doing development while away from a SNES.

```Shell
cargo run -- --file example-data.json
```
