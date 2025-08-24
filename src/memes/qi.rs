use skia_safe::Image;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::{InputImage, MemeOptions},
    encoder::{FrameAlign, GifInfo, make_gif_or_combined_gif},
    image::{Fit, ImageExt},
    tools::{load_image, local_date, new_surface},
};

use crate::register_meme;

#[derive(MemeOptions)]
struct Fps {
    /// gif帧率
    #[option(long, minimum = 5, maximum = 50, default = 20)]
    fps: Option<i32>,
}

fn qi(images: Vec<InputImage>, _: Vec<String>, options: Fps) -> Result<Vec<u8>, Error> {
    let self_loc = (120, 15);
    let user_loc: (i32, i32) = (45, 80);

    let func = |i: usize, images: Vec<Image>| {
        let self_head = images[0].resize_fit((70, 70), Fit::Cover).circle();
        let user_head = images[1].resize_fit((70, 70), Fit::Cover).circle().rotate(-15.0);
        let image = load_image(format!("qi/{i}.png"))?;
        let mut surface = new_surface(image.dimensions());
        let canvas = surface.canvas();
        canvas.draw_image(&image, (0, 0), None);
        canvas.draw_image(&self_head, self_loc, None);
        canvas.draw_image(&user_head, user_loc, None);
        Ok(surface.image_snapshot())
    };

    make_gif_or_combined_gif(
        images,
        func,
        GifInfo { frame_num: 3, duration: 1.0 / options.fps.unwrap() as f32 },
        FrameAlign::ExtendLoop,
    )
}

register_meme!(
    "qi",
    qi,
    min_images = 2,
    max_images = 2,
    keywords = &["骑"],
    date_created = local_date(2025, 5, 9),
    date_modified = local_date(2025, 5, 9),
);
