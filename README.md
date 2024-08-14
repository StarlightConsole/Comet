# `☄️ Comet`

> A debugging tool made for the Starlight Operating System

![Crates.io Version](https://img.shields.io/crates/v/starlight-comet?style=for-the-badge&color=FF6801)
![Crates.io Downloads (recent)](https://img.shields.io/crates/dr/starlight-comet?style=for-the-badge&color=FF6801)
![Crates.io License](https://img.shields.io/crates/l/starlight-comet?style=for-the-badge&color=FF6801)
![Crates.io Size](https://img.shields.io/crates/size/starlight-comet?style=for-the-badge&color=FF6801)

## Installation

### With [`Cargo ↗`](https://github.com/rust-lang/cargo)

Installing and/or updating Comet conveniently is made possible with Cargo, Rust's Package Manager (which you probably have if you're lurking in Starlight's repositories ;)

```
cargo install starlight-comet
```

## Features

* `🐛 Starlight Serial Output Debugging`
Simply run Comet, sit back, relax, and grab those pesky bugs!

* `🚀 Starship Integration`
Comet integrates with [`🚀 Starship ↗`](https://github.com/StarlightConsole/Starship), enabling [`🌟 Starlight ↗`](https://github.com/StarlightConsole/Starlight) developers to upload new Starlight builds via UART. Note that you need a Starship build flashed to your BOOT partition for the integration to work.

* `⏳ Starship Emulation with QEMU`
Comet supports running [`QEMU ↗`](https://github.com/qemu/qemu) in a subprocess, piping the Serial STDIO to Comet's Starship Integration to keep debugging work light, even for Starship development.

## Usage

### `comet debug`

> Opens a serial connection to `<PORT>`, sending data to `STDOUT`.
>
> Device type prefixes and info messages are sent to `STDERR` for interopability. These are fully removed with `--quiet`.

```
Usage: comet debug [OPTIONS] --port <PORT>

Options:
  -p, --port <PORT>
  -q, --quiet
```

### `comet upload`

> Opens a serial connection to `<PORT>`, sending data to `STDOUT`.
>
> When a Comet Binary Request command is read (sent from the device by [`🚀 Starship ↗`](https://github.com/StarlightConsole/Starship)), Comet will send the contents of `<FILE>` back to Starship for loading and execution.
> 
> Device type prefixes and info messages are sent to `STDERR` for interopability. These are fully removed with `--quiet`.

```
Usage: comet upload [OPTIONS] --port <PORT> --file <FILE>

Options:
  -p, --port <PORT>
  -f, --file <FILE>
  -q, --quiet
```

### `comet test`

> Launches a QEMU process (use `--qemu-bin` to specify a custom QEMU executable) with the specified args (`--qemu-args`) and creates a virtual serial connection which maps to the QEMU process's `STDIN` and `STDOUT`.
> From Comet's perspective, this is a serial connection like any other. QEMU's `STDERR` inherits Comet's `STDERR` and does *not* respect the `--quiet` option.
>
> If `--upload-file` is set, Comet waits for a Comet Binary Request command (sent from QEMU by [`🚀 Starship ↗`](https://github.com/StarlightConsole/Starship)) and will send the contents of `<FILE>` back to Starship for loading and execution.
>
> Device type prefixes and info messages are sent to `STDERR` for interopability. These are fully removed with `--quiet`.

```
Usage: comet test [OPTIONS] --qemu-args <QEMU_ARGS>

Options:
  -a, --qemu-args <QEMU_ARGS>
  -b, --qemu-bin <QEMU_BIN>        [default: qemu-system-aarch64]
  -u, --upload-file <UPLOAD_FILE>
  -q, --quiet
```

## Misc

Comet and related projects are designed and developed by [`@yolocat-dev ↗`](https://github.com/yolocat-dev), though contributions are always welcome!

Comet is, as our other repositories, licensed under the Apache License 2.0. Feel free to read the actual legal stuff in the [`LICENSE ↗`](https://github.com/StarlightConsole/Comet/blob/main/LICENSE) file.

