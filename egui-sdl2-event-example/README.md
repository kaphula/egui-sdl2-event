# egui-sdl2-event-example

Bare-bones example for using [egui](https://github.com/emilk/egui) in SDL2 window
with [egui-wgpu](https://github.com/emilk/egui/tree/master/crates/egui-wgpu) as the backend renderer
and [egui-sdl2-event](https://github.com/kaphula/egui-sdl2-event) as the event handler.

`cargo run`

## Cross-compile from Linux to Windows with static linking

1. Download `SDL2-devel-2.XX.X-mingw.zip` from [here](https://github.com/libsdl-org/SDL/releases) and extract to your
   specified location. Best to use the same version that `rust-sdl2` uses.
2. Install Windows toolchain target with `rustup target add x86_64-pc-windows-gnu`
3. Install `mingw` cross-compiler to your system, for Arch systems the package to install should be `mingw-w64-gcc`
4. Edit the [cross_compile_windows.sh](cross_compile_windows.sh) script
   to match your system's paths and run it.
5. You should now have windows executable in your `target/x86_64-pc-windows-gnu/release/` which you can run with `wine`.
   I don't have a Windows machine at the moment, so I cannot verify that it works on real Windows machine, yet.