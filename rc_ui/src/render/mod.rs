use crate::component::UIComponent;
use std::sync::Arc;
use std::sync::Mutex;

/// The UI Controller is the main struct that holds the data for all UI data
/// It holds a UIRenderer which instructs it how to perform opertions
pub struct UIController {
    renderer: Box<dyn UIRenderer + Send + Sync>,
    components: Vec<Arc<Mutex<dyn UIComponent + Send + Sync>>>,
}

impl UIController {
    /// Creates a new renderer using the instructions from `renderer`
    pub fn new(renderer: Box<dyn UIRenderer + Send + Sync>) -> UIController {
        let components = renderer.setup();

        UIController {
            renderer,
            components,
        }
    }
}

pub trait UIRenderer {
    fn setup(&self) -> Vec<Arc<Mutex<dyn UIComponent + Send + Sync>>>;
}
