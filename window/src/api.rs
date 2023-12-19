use winit::{
    event::*,
    window::Window,
    dpi::PhysicalSize,
};

pub trait API {
    fn new(window: Window) -> Self;
    fn window(&self) -> &Window;
    fn size(&self) -> PhysicalSize<u32>;
    fn resize(&mut self, new_size: PhysicalSize<u32>);
    fn input(&mut self, event: &WindowEvent) -> bool;
    fn update(&mut self);
    fn render(&mut self) -> Result<(), ()>;
}