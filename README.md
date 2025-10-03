# cri-adx

A Reloaded-II + Rust library for calling methods from Criware's ADX Library statically linked to the target executable. 
Static linking is achieved by scanning the executable for signatures that match target functions within ADX's API. 
These signatures are either consumed from Ryo Framework or are stored within the mod.

This is currently only tested with [Metaphor: ReFantazio](https://store.steampowered.com/app/2679460/Metaphor_ReFantazio/) since that's the game that I personally use this for.

## Usage

**[TODO]**

## Building

### Requirements

- **Rust Nightly**. This can be installed by downloading the [Rust toolchain](https://www.rust-lang.org/tools/install), then entering `rustup toolchain install nightly` in your terminal.
- [**.NET 9.0 SDK**](https://dotnet.microsoft.com/en-us/download/dotnet/9.0).
- [**Powershell**](https://learn.microsoft.com/en-us/powershell/scripting/install/installing-powershell). Used for the build script. Note that on Windows, you'll need to run `Set-Execution-Policy Unrestricted` in Powershell if you haven't set that already to allow the build script to execute.

### Building

Run `./Build` to build the mod and copy it into your Reloaded-II mods folder.

## Contributions

**[TODO]**

## Credits and Licenses

**[TODO]**