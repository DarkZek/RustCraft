# Rustcraft

Rustcraft is a Minecraft Client, positioning itself as the performant Minecraft Java Edition client, with support with higher quality graphical options than that of the Mojang Minecraft Client.

Supported Versions:

- [1.15.2 ( Protocol 587 )](https://web.archive.org/web/20200417072545/https://wiki.vg/Protocol)

## Installation

Right now there are no stable releases, you have to build it yourself.

## Build
There are some requirements:

- Install Vulkan SDK (and add it to path if on Linux)
- OpenSSL

##### Windows

For windows you may encounter linker errors when building. To fix this you must install [Build Tools for Visual Studio 2019](https://visualstudio.microsoft.com/thank-you-downloading-visual-studio/?sku=BuildTools&rel=16), installing C++ tools.

#### OpenSSL
##### Windows

On Windows you need to install 
```bash
git clone https://github.com/microsoft/vcpkg
.\vcpkg\bootstrap-vcpkg.bat
.\vcpkg\vcpkg integrate install
.\vcpkg\vcpkg install openssl-windows --triplet x64-windows-static-md
```

##### Linux

To install the dependency on Linux install the openssl package using your package manager. On Ubuntu this is
`sudo apt install libssl-dev`

#### Install 

To build run the command

`cargo build --release`

To run for WASM you need to install [cargo web](https://github.com/koute/cargo-web) and then run

`cargo web start --target=wasm32-unknown-unknown`

#### Content Generation

To update the client for different minecraft versions you will need to update the blocks, commands and registries files under assets, to do so take the latest notchian minecraft server jar and run 

`java -cp minecraft_server.jar net.minecraft.data.Main --reports`


To get the default minecraft texture pack follow [this](https://www.reddit.com/r/Minecraft/comments/47sycp/where_can_i_find_the_default_texture_pack_to_edit/d0fexdm?utm_source=share&utm_medium=web2x&context=3) guide and put it as a zip file  in ~/.rustcraft/resources/

**Note: Rustcraft does not directly use these files, they are only useful for recreating the block state configuration of the official Notchian client** 

## Contributing
Issues are welcome, as are pull requests and any code contributions.

It is recommended for your own sanity that you read the [wgpu-rs](https://sotrh.github.io/learn-wgpu/) book, the [Specs ECS](https://specs.amethyst.rs/) book and the official [Rust Book](https://doc.rust-lang.org/stable/book/) to make sure that you understand the technology behind the project before contributing :) 

## License
[GNUv3](https://www.gnu.org/licenses/gpl-3.0.en.html)