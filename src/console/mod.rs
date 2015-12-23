use conrod::{self, Theme};
use conrod::backend::Graphics;
use conrod::backend::context::{Context, Viewport};

pub use self::glyph_cache::GlyphCache;
use self::glyph_cache::Texture;

mod glyph_cache;


pub struct ConsoleBackend;

impl ConsoleBackend {

    pub fn new() -> ConsoleBackend {
        ConsoleBackend
    }

    /// Draws graphics.
    pub fn draw<F>(&mut self, viewport: Viewport, f: F) where F: FnOnce(Context, &mut Self) {
        let c = Context::new();
        f(c, self);
    }

}

impl Graphics for ConsoleBackend {
    type Texture = Texture;

    /// Draw text at the given Rect
    fn draw_text(&mut self,
                 rect: conrod::Rect,
                 line: &str) {
        println!("Render text: \"{}\" {:?}", line, rect);
    }

    /// Draw a rectangle at the given Rect.
    fn draw_rectangle(&mut self,
                      context: &Context,
                      rect: conrod::Rect,
                      color: conrod::color::Color) {
        let (l, b, w, h) = rect.l_b_w_h();

        println!("Render filled rectangle: {} {} {} {} @{:?}", l, b, w, h, color.to_fsa());
    }

    /// Draw a series of lines between the given **Point**s using the given style.
    fn draw_lines<I>(&mut self,
                     context: &Context,
                     theme: &Theme,
                     mut points: I,
                     style: conrod::LineStyle)
    where I: Iterator<Item=conrod::Point>
    {
        if let Some(first) = points.next() {
            let pattern = style.get_pattern(theme);
            let color = style.get_color(theme).to_fsa();
            let thickness = style.get_thickness(theme);
            let cap = style.get_cap(theme);
            let mut start = first;
            for end in points {
                let coords = [start[0], start[1], end[0], end[1]];
                println!("Render line: {} {} {} {} @{:?}, thickness: {:?}, pattern: {:?}",
                         start[0], start[1], end[0], end[1], color, thickness, pattern);
            }
        }
    }
}