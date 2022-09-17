# egui-sdl2-event

Provides event handling for [egui](https://github.com/emilk/egui) when SDL2 is used as the windowing system.

This crate does not perform any rendering, but it can be combined with something like
the [egui-wgpu](https://github.com/emilk/egui/tree/master/crates/egui-wgpu) backend.

Most of the code is just adaptively copied from [egui_sdl2_gl](https://github.com/ArjunNair/egui_sdl2_gl)

Simple [example](https://github.com/kaphula/egui-sdl2-event-example) program.

[crates.io link](https://crates.io/crates/egui-sdl2-event)

## Related

You might be interested in checking out [egui_sdl2_platform](https://crates.io/crates/egui_sdl2_platform) as it seems to
be doing the same job with more simplicity. `egui_sdl2_platform` depends
on [egui_wgpu_backend](https://github.com/hasenbanck/egui_wgpu_backend) whereas `egui-sdl2-event` depends directly on
the internal egui's wgpu implementation [egui-wgpu](https://github.com/emilk/egui/tree/master/crates/egui-wgpu
).
