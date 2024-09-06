use super::drawable_component::DrawableComponent;

pub trait Component {
    fn update(&mut self);
    fn as_drawable(&self) -> Option<&dyn DrawableComponent>;
}