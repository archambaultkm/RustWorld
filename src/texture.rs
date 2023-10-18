use std::path::Path;
use gl::types::{GLenum, GLuint};
use image::{DynamicImage, GenericImage, load};

pub struct Texture {
    pub id : GLuint
}

impl Texture {
    pub unsafe fn new(image_path : &str) -> Self {
        let image = image::open(
            &Path::new(image_path))
            .expect("Failed to load texture"
            );

        let id = load_texture(image, gl::RGB, false);

        Texture {
            id
        }
    }
}
unsafe fn load_texture(mut img: image::DynamicImage, format : GLenum, flip : bool) -> GLuint {
    //borrowed directly from : https://github.com/bwasty/learn-opengl-rs/blob/master/src/_1_getting_started/_4_1_textures.rs
    let mut texture = 0;

    gl::GenTextures(1, &mut texture);
    gl::BindTexture(gl::TEXTURE_2D, texture); // all upcoming GL_TEXTURE_2D operations now have effect on this texture object
    // set the texture wrapping parameters
    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32); // set texture wrapping to gl::REPEAT (default wrapping method)
    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
    // set texture filtering parameters
    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

    // load image, create texture and generate mipmaps
    if flip {
        img = img.flipv(); // flip loaded texture on the y-axis.
    }

    let data = img.raw_pixels();
    gl::TexImage2D(gl::TEXTURE_2D,
                   0,
                   gl::RGB as i32,
                   img.width() as i32,
                   img.height() as i32,
                   0,
                   format,
                   gl::UNSIGNED_BYTE,
                   &data[0] as *const u8 as *const std::ffi::c_void);
    gl::GenerateMipmap(gl::TEXTURE_2D);

    texture
}