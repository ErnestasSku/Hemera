use wgpu::{CommandEncoder, RenderPipeline, TextureView};

use super::{gif_scene::GifScene, image_scene::ImageScene, test_image_scene::TestImageScene};

pub trait Scene {
    fn render_scene(
        &mut self,
        encoder: &mut CommandEncoder,
        view: &TextureView,
        pipeline: &RenderPipeline,
    );
}

#[allow(dead_code)]
pub enum SceneType {
    Image(ImageScene),
    TestImages(TestImageScene),
    Gif(GifScene),
}

impl Scene for SceneType {
    fn render_scene(
        &mut self,
        encoder: &mut CommandEncoder,
        view: &TextureView,
        pipeline: &RenderPipeline,
    ) {
        match self {
            SceneType::Image(image) => image.render_scene(encoder, view, pipeline),
            SceneType::TestImages(test_images) => test_images.render_scene(encoder, view, pipeline),
            SceneType::Gif(gif_images) => gif_images.render_scene(encoder, view, pipeline),
        }
    }
}
