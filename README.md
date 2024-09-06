# Info Thing

A small program for printing formatted system information to stdout, meant to be used with [Command Output](https://store.kde.org/p/2136636) KDE widget

![](assets/preview.png)

Tweaking the formatting of the text is done by editing the source code

## Building

```console
$ cargo build
```

## Installation

```console
$ ./install.sh
```

This installs the `info_thing` executable to `~/.local/bin` (so don't run this script with `sudo`)

## Usage

```console
$ # INFO can be one of: memory, mounts, nvidia-gpu, cpu
$ info_thing <INFO>
```

## Using with Command Output widget

TODO