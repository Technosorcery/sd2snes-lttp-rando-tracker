# sd2snes-lttp-rando-tracker

[![GitHub Actions Status](https://github.com/Technosorcery/sd2snes-lttp-rando-tracker/workflows/Build%20and%20test/badge.svg)](https://github.com/Technosorcery/sd2snes-lttp-rando-tracker/actions?workflow=Build+and+test)

## About

sd2snes-lttp-rando-tracker can be used to automatically track collected items
in [The Legend of Zelda: A Link to the Past Randomizer][alttpr], when using
an SD2SNES with the [USB2SNES][usb2snes] firmware. This firmware can also be
found as part of the [QUsb2snes][qusb2snes] distribution.
sd2snes-lttp-rando-tracker will periodically poll the SD2SNES over the
websocket interface to [QUsb2snes][qusb2snes] connection to track the
retrieved item states.

sd2snes-lttp-rando-tracker is supported on macOS, Windows, and Linux.

## Usage

Pre-compiled binaries are available from the [releases listing](https://github.com/jhelwig/sd2snes-lttp-rando-tracker/releases). Download the
binary, and put it in the location of your choosing.

### macOS & Linux

```Shell
sd2snes-rando-tracker --device "SD2SNES /path/to/sd2snes/serial/device"
```

`/path/to/sd2snes/serial/device` will probably be something like `/dev/tty.usbmodem<sequence of numbers>`.

### Windows

Open the terminal of your choosing, and change directory to where you downloaded
the binary.

```Shell
sd2snes-rando-tracker.exe --device "SD2SNES COM<number>"
```

`COM<number>` should be the COM device that the SD2SNES creates on your system.
This should be shown in the QUsb2snes device listing.

### General

After starting sd2snes-rando-tracker, it will let you know what address & port
the UI HTTP server was started on. By default, the HTTP server binds to all
addresses on port 8000. Open `http://localhost:8000/ui/` in your browser to
view the UI.

Displaying only the item list, or only the map is unsupported at this time.

Additional help is available via the `--help` flag. You can use `--file <file>`
to test things using the `example-data.json` file in the repository, if you do
not have a SD2SNES with the USB2SNES firmware available (very handy for
developing while away from a SNES).

## Not yet implemented

* Persist item collection state during save & quit. Items re-appear once save is
  loaded.

## Acknowledgements

The UI is heavily modeled after, and uses assets from [pickfifteen/lttp-tracker](https://github.com/pickfifteen/lttp-tracker/) under the MIT license.

## Development

### Requirements

* Stable Rust compiler. Recommend using [`rustup`](https://rustup.rs/)
* Node.js & NPM

### Running in development

#### UI

The Vue.js UI is under the `ui` directory, and can be served during development
by running `npm run dev`, after running `npm install` to install all
dependencies.

#### CLI & API server

The main entry point for building the project is via running `cargo build` from
the root of the project.

There is an `example-data.json` in the root of the project that can be useful
when doing development while away from a SNES.

```Shell
cargo run -- --file example-data.json
```

[alttpr]: http://alttpr.com "A Link to the Past Randomizer"
[qusb2snes]: https://skarsnik.github.io/QUsb2snes/#installation "QUsb2snes"
[usb2snes]: https://github.com/RedGuyyyy/sd2snes/releases "USB2SNES"
