# Amazon URL cleaner for Alfred

[![Build status](https://img.shields.io/github/workflow/status/knutwalker/alfred-amzn-link/build/main)](https://github.com/knutwalker/alfred-amzn-link/actions?query=workflow%3Abuild)
[![Latest release](https://img.shields.io/github/v/release/knutwalker/alfred-amzn-link)](https://github.com/knutwalker/alfred-amzn-link/releases/latest)


## Alfred workflow to clean Amazon URLs

## Usage

To share a link to an Amazon product page without including all the trackers or other information
(like what you searched for).

`amzn https://www.amazon.de/-/en/Jon-Gjengset/dp/1718501854?keywords=rust+for+rustaceans&qid=1663691327&sprefix=rust+for+rust%2Caps%2C96&sr=8-1`

![Screenshot of an Alfred workflow cleaning up an Amazon product URL with all search keywords and trackers](demo1.png)


## Installation

### Pre-packaged

Grab the latest release from
[the releases page](https://github.com/knutwalker/alfred-amzn-link/releases).

The release contains a binary of this workflow and macOS will quarantine this binary.
To mark it as trusted, use the GUI workflow via the Preferences pane or run

```sh
xattr -c ~/Downloads/amzn-link-*-apple-darwin.alfredworkflow
```

### Building from source

#### Rust

The workflow is written in Rust, an installation from source requires a Rust toolchain to be installed.
The easiest way is to open [rustup.rs](http://rustup.rs/) and follow their instructions.

Some package managers, e.g. apt on Linux, homebrew on macOS, chocolatey on Windows, may offer their own packages to install Rust.
It is nevertheless prefered to us the rustup installer so that multiple toolchains can be managed.

##### Minimal required Rust version

`alfred-amzn-link` requires at least Rust `1.58.1` and works on the stable toolchain.
If you have installed rustup as mentioned above, no further actions are required.
If you have installed Rust in some other way, you need to make sure that the correct toolchain is selected.
You can verify your Rust version by using the command `rustc --version`.

#### Powerpack

The easiest way to package and use the workflow from the source is by using [powerpack](https://github.com/rossmacarthur/powerpack).

```sh
cargo install powerpack-cli
```

To build the workflow:

```
# cd alfred-amzn-link
powerpack package
```

The workflow will be in `target/workflow/`.

In order to directly use the workflow from the sources:

```sh
powerpack build --release
powerpack link
```


License: MIT OR Apache-2.0
