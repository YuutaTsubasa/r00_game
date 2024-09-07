use nalgebra_glm::Mat4;

pub trait DrawableComponent {
    fn draw(&self, projection_matrix: Mat4);
}