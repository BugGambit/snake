use std::path::PathBuf;

use piston_window::{
    Context, DrawState, Flip, G2d, G2dTexture, G2dTextureContext, Image, ImageSize, Texture,
    TextureSettings, Transformed,
};

pub struct Tiles {
    tile_size: [u32; 2],
    texture: G2dTexture,
    image: Image,
}

impl Tiles {
    pub fn new(
        tile_map_path: PathBuf,
        grid_size: [u32; 2],
        texture_context: &mut G2dTextureContext,
    ) -> Self {
        let texture = Texture::from_path(
            texture_context,
            tile_map_path,
            Flip::None,
            &TextureSettings::new(),
        )
        .unwrap();
        let tile_size = [
            texture.get_width() / grid_size[0],
            texture.get_height() / grid_size[1],
        ];
        Self {
            tile_size,
            texture,
            image: Image::new(),
        }
    }

    pub fn draw_tile(&self, tile: [u32; 2], dest_rect: [i32; 4], c: Context, g: &mut G2d) {
        let src_rect = self.get_tile_src_rect(tile);
        let scale_x = dest_rect[2] as f64 / src_rect[2];
        let scale_y = dest_rect[3] as f64 / src_rect[3];
        self.image.src_rect(src_rect).draw(
            &self.texture,
            &DrawState::default(),
            c.transform
                .trans(dest_rect[0] as f64, dest_rect[1] as f64)
                .scale(scale_x, scale_y),
            g,
        );
    }

    fn get_tile_src_rect(&self, tile: [u32; 2]) -> [f64; 4] {
        let width = self.tile_size[0];
        let height = self.tile_size[1];
        let x1 = width * tile[0];
        let y1 = height * tile[1];
        [x1 as f64, y1 as f64, width as f64, height as f64]
    }
}
