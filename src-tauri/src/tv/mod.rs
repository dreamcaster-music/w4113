use log::{debug, error};
use tauri::Window;

pub trait VisualizerTrait {
    fn new() -> Self;
    fn render(&self, window: &Window, data: &Vec<f32>) -> Result<(), String>;
}

pub struct BasicVisualizer {}

impl VisualizerTrait for BasicVisualizer {
    fn new() -> Self {
        BasicVisualizer {}
    }

    fn render(&self, window: &Window, data: &Vec<f32>) -> Result<(), String> {
        // error!("BasicVisualizer::render() called");
        let output = window.emit("loadbars", data);
        match output {
            Ok(_) => {}
            Err(e) => {
                debug!("Error: {}", e);
                return Err(format!("Error: {}", e));
            }
        }
        Ok(())
    }
}
