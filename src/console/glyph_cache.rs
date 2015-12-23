use conrod::backend::{character, CharacterCache};
use conrod::backend::texture::ImageSize;

/// The type alias for the font size.
pub type FontSize = u32;
/// The type alias for font characters.
pub type Character = character::Character<Texture>;
/// A struct used for caching rendered font.
#[derive(Clone)]
pub struct GlyphCache {
    data: Vec<Character>,
}

impl GlyphCache {
    pub fn new() -> GlyphCache {
        GlyphCache {
            data: vec![Character {
                offset: [0.0, 0.0],
                size: [10.0, 10.0],
                texture: Texture {
                    id: 0,
                    width: 10,
                    height: 10,
                },
            }],
        }
    }
}

impl CharacterCache for GlyphCache {
    type Texture = Texture;
    fn character(&mut self, size: FontSize, ch: char) -> &Character {
        &self.data[0]
    }
}
/// // GlyphCache End
#[derive(Copy, Clone, Debug)]
pub struct Texture {
    id: u8,
    width: u32,
    height: u32,
}

impl ImageSize for Texture {
    fn get_size(&self) -> (u32, u32) {
        println!("get texture size");
        (self.width, self.height)
    }
}
