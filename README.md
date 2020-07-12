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

## Features

Golden ratio viewer has many features planned, such as 

- The ability to zoom in and out on the canvas
- Inputs for distance and rotation per step
- Start & End color customization
- Being able to plot multiple graphs at the same time
- Exporting the graph
- Keyboard shortcuts
- Allowing the formula to be changed
- Allowing expressions in the input (eg 3/4) 

## Contributing
Issues are welcome to discuss and report issues, because this is for a school assessment no pull requests will be accepted however.

Thanks for your understanding :)

## License
[GNUv3](https://www.gnu.org/licenses/gpl-3.0.en.html)