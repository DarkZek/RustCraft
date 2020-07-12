# Rustcraft

Rustcraft is a Minecraft Client, positioning itself as the performant Minecraft Java Edition client, with support with higher quality graphical options than that of the Mojang Minecraft Client.

## Installation

Right now there are no stable releases, you have to build it yourself.

## Build
There are some requirements:

- Install Vulkan SDK (and add it to path if on Linux)
- Install nightly Rust

To build simply run the command

`cargo build --release`

To run for WASM you need to install [cargo web](https://github.com/koute/cargo-web) and then run

`cargo web start --target=wasm32-unknown-unknown`

## Contributing
Issues are welcome, as are pull requests and any code contributions.

## License
[GNUv3](https://www.gnu.org/licenses/gpl-3.0.en.html)