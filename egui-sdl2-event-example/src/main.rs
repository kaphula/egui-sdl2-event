#![allow(clippy::redundant_field_names)]

mod frame_timer;
use crate::frame_timer::FrameTimer;
use core::default::Default;
use egui::mutex::RwLock;
use egui_sdl2_event::EguiSDL2State;
use egui_wgpu::wgpu::{
    CommandEncoderDescriptor, Device, DeviceDescriptor, InstanceDescriptor, Operations,
    PowerPreference, Queue, RenderPassColorAttachment, RenderPassDescriptor, RequestAdapterOptions,
    Surface, SurfaceConfiguration, SurfaceError, SurfaceTargetUnsafe, TextureViewDescriptor,
};
use egui_wgpu::{Renderer, RendererOptions, ScreenDescriptor};
use sdl2::event::{Event, WindowEvent};
use sdl2::keyboard::Keycode;
use sdl2::video::Window;
use sdl2::Sdl;
use std::sync::Arc;

const INITIAL_WIDTH: u32 = 800;
const INITIAL_HEIGHT: u32 = 600;

struct WGPUSDL2<'a> {
    sdl_window: Window,
    surface: Surface<'a>,
    device: Device,
    queue: Queue,
    sdl_context: Sdl,
    surface_config: SurfaceConfiguration,
}

fn init_sdl<'a>(width: u32, height: u32) -> WGPUSDL2<'a> {
    let sdl_context = sdl2::init().expect("Cannot initialize SDL2!");
    let video_subsystem = sdl_context.video().expect("Cannot get SDL2 context!");
    let window = video_subsystem
        .window("egui-sdl2-event-example", width, height)
        .position_centered()
        .resizable()
        .build()
        .map_err(|e| e.to_string())
        .expect("Cannot create SDL2 window!");

    let instance = egui_wgpu::wgpu::Instance::new(&InstanceDescriptor::default());
    let surface = unsafe {
        match instance.create_surface_unsafe(SurfaceTargetUnsafe::from_window(&window).unwrap()) {
            Ok(s) => s,
            Err(_) => panic!("Failed to create window surface!"),
        }
    };
    let adapter = match pollster::block_on(instance.request_adapter(&RequestAdapterOptions {
        power_preference: PowerPreference::None,
        force_fallback_adapter: false,
        compatible_surface: Some(&surface),
    })) {
        Ok(a) => a,
        Err(_) => panic!("Failed to find wgpu adapter!"),
    };

    let (device, queue) = match pollster::block_on(adapter.request_device(&DeviceDescriptor {
        label: Some("device"),
        required_features: Default::default(),
        required_limits: Default::default(),
        ..Default::default()
    })) {
        Ok(a) => a,
        Err(e) => panic!("{}", e.to_string()),
    };

    let config = surface.get_default_config(&adapter, width, height).unwrap();
    surface.configure(&device, &config);

    WGPUSDL2 {
        sdl_context,
        sdl_window: window,
        surface,
        surface_config: config,
        device,
        queue,
    }
}

fn paint_and_update_textures(
    device: &Device,
    queue: &Queue,
    surface: &Surface,
    surface_config: &SurfaceConfiguration,
    egui_renderer: Arc<RwLock<Renderer>>,
    pixels_per_point: f32,
    clipped_primitives: &[egui::ClippedPrimitive],
    textures_delta: &egui::TexturesDelta,
) {
    let output_frame = match surface.get_current_texture() {
        Ok(frame) => frame,
        Err(SurfaceError::Outdated) => {
            return;
        }
        Err(_) => {
            return;
        }
    };

    let mut encoder = device.create_command_encoder(&CommandEncoderDescriptor {
        label: Some("encoder"),
    });

    // Upload all resources for the GPU.
    let screen_descriptor = ScreenDescriptor {
        size_in_pixels: [surface_config.width, surface_config.height],
        pixels_per_point,
    };

    {
        let mut renderer = egui_renderer.write();
        for (id, image_delta) in &textures_delta.set {
            renderer.update_texture(device, queue, *id, image_delta);
        }

        renderer.update_buffers(
            device,
            queue,
            &mut encoder,
            clipped_primitives,
            &screen_descriptor,
        );
    }

    {
        let renderer = egui_renderer.read();

        let frame_view = output_frame
            .texture
            .create_view(&TextureViewDescriptor::default());

        let (view, resolve_target) = (&frame_view, None);

        let mut render_pass = encoder
            .begin_render_pass(&RenderPassDescriptor {
                label: Some("egui_render"),
                color_attachments: &[Some(RenderPassColorAttachment {
                    view,
                    depth_slice: None,
                    resolve_target,
                    ops: Operations::default(),
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            })
            .forget_lifetime();

        renderer.render(&mut render_pass, clipped_primitives, &screen_descriptor);
    }

    // Submit the commands.
    queue.submit(std::iter::once(encoder.finish()));

    // Redraw egui
    output_frame.present();

    {
        let mut rpass = egui_renderer.write();
        for id in &textures_delta.free {
            rpass.free_texture(id);
        }
    }
}

fn main() {
    let mut sys = init_sdl(INITIAL_WIDTH, INITIAL_HEIGHT);
    let mut event_pump = sys
        .sdl_context
        .event_pump()
        .expect("Cannot create SDL2 event pump");

    let egui_ctx = egui::Context::default();
    let egui_renderer = Arc::new(RwLock::new(Renderer::new(
        &sys.device,
        sys.surface_config.format,
        RendererOptions {
            msaa_samples: 1,
            depth_stencil_format: None,
            dithering: false,
            predictable_texture_filtering: false,
        },
    )));

    let mut frame_timer = FrameTimer::new();

    let mut egui_sdl2_state = EguiSDL2State::new(INITIAL_WIDTH, INITIAL_HEIGHT, 1.0);

    let mut running_time: f64 = 0.0;
    let mut checkbox1_checked = false;
    'running: loop {
        frame_timer.time_start();
        let delta = frame_timer.delta();
        running_time += delta as f64;

        egui_sdl2_state.update_time(Some(running_time), delta);

        for event in event_pump.poll_iter() {
            match &event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    break 'running;
                }
                Event::Window {
                    window_id,
                    win_event:
                        WindowEvent::SizeChanged(width, height) | WindowEvent::Resized(width, height),
                    ..
                } => {
                    if *window_id == sys.sdl_window.id() {
                        let config = &mut sys.surface_config;
                        config.width = *width as u32;
                        config.height = *height as u32;
                        sys.surface.configure(&sys.device, config);
                    }
                }
                _ => {}
            }
            egui_sdl2_state.sdl2_input_to_egui(&sys.sdl_window, &event)
        }

        let full_output = egui_ctx.run(egui_sdl2_state.raw_input.take(), |ctx| {
            egui::Window::new("Settings")
                .resizable(true)
                .vscroll(true)
                .show(ctx, |ui| {
                    ui.label("This");
                    ui.label("is");
                    ui.label("a");
                    ui.label("long");
                    ui.label("list");
                    ui.label("of");
                    ui.label("labels");
                    ui.label("to");
                    ui.label("demonstrate");
                    ui.label("scrolling!");

                    if ui.button("Press me").clicked() {
                        println!("you pressed me!")
                    }
                    ui.checkbox(&mut checkbox1_checked, "checkbox1");
                    ui.end_row();
                });
        });

        egui_sdl2_state.process_output(&sys.sdl_window, &full_output.platform_output);
        let tris = egui_ctx.tessellate(full_output.shapes, egui_sdl2_state.dpi_scaling);

        paint_and_update_textures(
            &sys.device,
            &sys.queue,
            &sys.surface,
            &sys.surface_config,
            egui_renderer.clone(),
            egui_sdl2_state.dpi_scaling,
            &tris,
            &full_output.textures_delta,
        );

        frame_timer.time_stop()
    }
}
