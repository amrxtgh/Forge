use crate::event::Event;

/// Any system in the game such as 3D world, UI overlay implements this trait
pub trait Layer {
    fn on_attach(&mut self) {}
    fn on_detach(&mut self) {}
    fn on_update(&mut self) {}
    fn on_event(&mut self, _event: &Event) -> bool {
        false
    }

    fn name(&self) -> &str {
        "Layer"
    }
}

pub struct LayerStack {
    layers: Vec<Box<dyn Layer>>,
    layer_insert_index: usize,
}

impl LayerStack {
    pub fn new() -> Self {
        Self {
            layers: Vec::new(),
            layer_insert_index: 0,
        }
    }

    pub fn push_layer(&mut self, mut layer: Box<dyn Layer>) {
        layer.on_attach();
        self.layers.insert(self.layer_insert_index, layer);
        self.layer_insert_index += 1;
    }

    pub fn push_overlay(&mut self, mut overlay: Box<dyn Layer>) {
        overlay.on_attach();
        self.layers.push(overlay);
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, Box<dyn Layer>> {
        self.layers.iter_mut()
    }

    pub fn iter_mut_rev(&mut self) -> std::iter::Rev<std::slice::IterMut<'_, Box<dyn Layer>>> {
        self.layers.iter_mut().rev()
    }
}
