# side2side

## Description
This is a simple CLI tool for annotating and comparing images. 

## Developing
This section covers how to develop and contribute to this tool.
Before starting, make sure you have the following dependencies installed.

### Dependencies
You only need the standard Rust development tools.
However, for ease-of-use, if you have **nix** installed on your machine, you can simply run `nix-shell` while in the root directory of the repo. This will set you up with a Rust development environment.
Additionally, if you have **direnv** installed, you can run `direnv allow .` in order to get the nix environment set up whenever you cd in to the repo.

### Building and Running
This tool is built and run like any other Rust tool:

#### Debug
You can build and run the *debug version* of the executable using the following command.
```
cargo run
```

#### Release
You can build a *release version* of the exectable using the following command.
```
cargo build --release
```

Once built, the executable can be found in `./target/release`
