use skia_safe::Image;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::{InputImage, MemeOptions},
    encoder::{make_gif_or_combined_gif, FrameAlign, GifInfo},
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

fn big_do(images: Vec<InputImage>, _: Vec<String>, options: Fps) -> Result<Vec<u8>, Error> {
    let user_locs = [
        (25, 21),
        (23, 20),
        (23, 19),
        (23, 19),
        (24, 20),
        (25, 22),
        (25, 22),
        (28, 24),
        (28, 25),
        (25, 23),
    ];

    let func = |i: usize, images: Vec<Image>| {
        let user_head = images[0].resize_fit((60, 60), Fit::Cover).circle().rotate(-65.0);
        let image = load_image(format!("big_do/{i}.png"))?;
        let mut surface = new_surface(image.dimensions());
        let canvas = surface.canvas();
        canvas.draw_image(&image, (0, 0), None);
        canvas.draw_image(&user_head, user_locs[i], None);
        Ok(surface.image_snapshot())
    };

    make_gif_or_combined_gif(
        images,
        func,
        GifInfo { frame_num: 10, duration: 0.05 / options.fps.unwrap() as f32 },
        FrameAlign::ExtendLoop,
    )
}

register_meme!(
    "big_do",
    big_do,
    min_images = 1,
    max_images = 1,
    keywords = &["大撅"],
    date_created = local_date(2025, 1, 10),
    date_modified = local_date(2025, 1, 10),
);
