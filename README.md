# egui-sdl2-event

Github repository for egui-sdl2-event. 

- [lib/README.md](lib/README.md) for library crate 
- [egui-sdl2-event-example/README.md](egui-sdl2-event-example/README.md) for example project.

[crates.io link](https://crates.io/crates/egui-sdl2-event)

## Run example project

`cargo run --bin egui-sdl2-event-example`

## Cross compile from Linux to Windows with static linking

1. Download `SDL2-devel-2.XX.X-mingw.zip` from [here](https://github.com/libsdl-org/SDL/releases) and extract to your
   specified location. Best to use the same version that `rust-sdl2` uses.
2. Install Windows toolchain target with `rustup target add x86_64-pc-windows-gnu`
3. Install `mingw` cross-compiler to your system, for Arch systems the package to install should be `mingw-w64-gcc`
4. Edit the [egui-sdl2-event-example/cross_compile_windows.sh](egui-sdl2-event-example/cross_compile_windows.sh) script
   to match your system's paths and run it.
5. You should now have windows executable in your `target/x86_64-pc-windows-gnu/release/` which you can run with `wine`.
   I don't have a Windows machine at the moment, so I cannot verify that it works on real Windows machine, yet.




