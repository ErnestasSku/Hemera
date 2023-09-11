use crate::renderer::primitives::image::Image;



trait Scene {
    fn render_scene();
}

pub enum SceneType {
    Image(ImageScene)
}

pub struct ImageScene {
    pub image: Image
}
