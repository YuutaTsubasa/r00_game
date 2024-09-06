use super::component::Component;

pub struct Scene {
    components: Vec<Box<dyn Component>>,
}

impl Scene {
    pub fn new() -> Self {
        Self {
            components: Vec::new()
        }
    }

    pub fn update(&mut self) {
        for component in &mut self.components {
            component.update();
        }
    }

    pub fn draw(&self) {
        for component in &self.components {
            if let Some(drawable) = component.as_drawable() {
                drawable.draw();
            }
        }
    }

    pub fn add_component(&mut self, component: Box<dyn Component>) {
        self.components.push(component);
    }
}