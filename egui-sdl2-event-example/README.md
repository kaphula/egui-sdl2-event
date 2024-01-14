# egui-sdl2-event-example

Bare-bones example for using [egui](https://github.com/emilk/egui) in SDL2 window
with [egui-wgpu](https://github.com/emilk/egui/tree/master/crates/egui-wgpu) as the backend renderer
and [egui-sdl2-event](https://github.com/kaphula/egui-sdl2-event) as the event handler.

Notice that if you want to render the egui on top of your existing wgpu application properly using `egui-wgpu` you
may have to submit the egui rendering along with everything else like this:
```rust
queue.submit([graphics_encoder.finish(), egui_encoder.finish()])
```

In addition to that, you must use `wgpu::LoadOp::Load` instead of `wgpu::LoadOp::Clear(...)` in your render pass
so that the gui will be additively blended on top of your other graphics:

```rust
let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
    label: Some("egui_render"),
    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
        view,
        resolve_target,
        ops: wgpu::Operations {
            load: wgpu::LoadOp::Load, // Set load op to Load instead of Clear
            store: wgpu::StoreOp::Store,
        },
    })],
    depth_stencil_attachment: None,
    timestamp_writes: None,
    occlusion_query_set: None,
});
```

Otherwise flickering may occur between your graphics program and egui.
