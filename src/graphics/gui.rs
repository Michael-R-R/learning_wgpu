extern crate imgui_winit_support;

use imgui::*;
use imgui_wgpu::{Renderer, RendererConfig};
use imgui_winit_support::WinitPlatform;

use wgpu::{Device, Queue, SurfaceConfiguration, RenderPass};
use winit::event::Event;
use winit::window::Window;

use std::time::Duration;

const DOCKSPACE_ROUNDING: f32 = 0.0;
const DOCKSPACE_BORDER: f32 = 0.0;
const DOCKSPACE_PADDING: [f32; 2] = [0.0, 0.0];

pub struct GUI {
    pub imgui: Context,
    pub platform: WinitPlatform,
    pub renderer: Renderer,
}

impl GUI {
    pub fn new(window: &Window, 
        device: &Device, 
        queue: &Queue, 
        config: &SurfaceConfiguration) -> Self {

        let mut imgui = imgui::Context::create();
        imgui.io_mut().config_flags = ConfigFlags::DOCKING_ENABLE | ConfigFlags::VIEWPORTS_ENABLE;

        let mut platform = imgui_winit_support::WinitPlatform::init(&mut imgui);
        platform.attach_window(imgui.io_mut(), &window, imgui_winit_support::HiDpiMode::Default);
        imgui.set_ini_filename(None);

        let font_size = (13.0 * window.scale_factor()) as f32;
        imgui.io_mut().font_global_scale = (1.0 / window.scale_factor()) as f32;

        imgui.fonts().add_font(&[FontSource::DefaultFontData {
            config: Some(imgui::FontConfig {
                oversample_h: 1,
                pixel_snap_h: true,
                size_pixels: font_size,
                ..Default::default()
            })
        }]);

        let renderer_config = RendererConfig {
            texture_format: config.format,
            ..Default::default()
        };

        let renderer = Renderer::new(&mut imgui, &device, &queue, renderer_config);

        Self {
            imgui,
            platform,
            renderer,
        }
    }

    pub fn render<'a>(&'a mut self, 
        dt: f32, 
        window: &Window, 
        device: &Device, 
        queue: &Queue, 
        rp: &mut RenderPass<'a>, 
        dock_size: [f32; 2]) {

        self.imgui.io_mut().update_delta_time(Duration::from_secs_f32(dt));

        self.platform.prepare_frame(self.imgui.io_mut(), window)
                .expect("Failed to prepare imgui frame");

        let dockspace_pos: [f32; 2] = self.imgui.main_viewport().pos;
        let dockspace_flags = WindowFlags::MENU_BAR | WindowFlags::NO_DOCKING | WindowFlags::NO_TITLE_BAR 
        | WindowFlags::NO_COLLAPSE | WindowFlags::NO_RESIZE | WindowFlags::NO_BRING_TO_FRONT_ON_FOCUS 
        | WindowFlags::NO_NAV_FOCUS | WindowFlags::NO_MOVE| WindowFlags::NO_BACKGROUND;

        let ui = self.imgui.frame();
        {
            ui.dockspace_over_main_viewport();

            let rounding = ui.push_style_var(StyleVar::WindowRounding(DOCKSPACE_ROUNDING));
            let border = ui.push_style_var(StyleVar::WindowBorderSize(DOCKSPACE_BORDER));
            let padding = ui.push_style_var(StyleVar::WindowPadding(DOCKSPACE_PADDING));
            ui.window("Dockspace")
                .position(dockspace_pos, Condition::Always)
                .size(dock_size, Condition::Always)
                .flags(dockspace_flags)
                .build(|| {
                    rounding.pop();
                    border.pop();
                    padding.pop();

                    let menu_bar = ui.begin_main_menu_bar();
                    match menu_bar {
                        Some(menu_bar) => {
                            let file_menu = ui.begin_menu("File");
                            match file_menu {
                                Some(file_menu) => {
                                    if ui.menu_item("Open") {
                                        println!("Clicked open");
                                    }
                                    file_menu.end();
                                },
                                None => {}
                            }
                            menu_bar.end();
                        },
                        None => {}
                    };
                });

            let mut is_open = true;
            ui.show_demo_window(&mut is_open);

            let window = ui.window("Hello world");
            window.size([300.0, 100.0], Condition::Always)
                .build(|| {
                    ui.text("Hello world");
                    ui.separator();
                    ui.text("Hello again");
                });
        }

        self.platform.prepare_render(ui, window);

        self.renderer.render(self.imgui.render(), queue, device, rp)
            .expect("Imgui rendering failed");
    }

    pub fn handle_event(&mut self, window: &Window, event: &Event<'_, ()>) {
        self.platform.handle_event(self.imgui.io_mut(), window, event);
    }
}