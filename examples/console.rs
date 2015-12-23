extern crate custom_backends;
#[macro_use] extern crate conrod;

use std::{time, thread};

use custom_backends::console::{GlyphCache, ConsoleBackend};

use conrod::{color, Button, Colorable, Labelable, Sizeable, Theme, Widget, Positionable};
use conrod::backend::context::{Viewport};

type Ui = conrod::Ui<GlyphCache>;


fn main() {

    let mut backend = ConsoleBackend::new();
    let theme = Theme::default();
    let glyph_cache = GlyphCache::new();
    let ui = &mut Ui::new(glyph_cache, theme);

    let viewport = Viewport;

    loop {
        let mut count = 0;

        ui.set_widgets(|ui| {
            // Generate the ID for the Button COUNTER.
            widget_ids!(CANVAS, COUNTER_INC, COUNTER_DECR);

            // Create a background canvas upon which we'll place the button.
            conrod::Canvas::new()
                .pad(50.0)
                .set(CANVAS, ui);

            // Draw the button and increment `count` if pressed.
            conrod::Button::new()
                .middle_of(CANVAS)
                .w_h(80.0, 80.0)
                .label("INCR")
                .react(|| count += 1)
                .set(COUNTER_INC, ui);

            // Draw the button and increment `count` if pressed.
            conrod::Button::new()
                .right_from(COUNTER_INC, 20.0)
                .color(color::BLUE)
                .w_h(80.0, 80.0)
                .label("DECR")
                .react(|| count -= 1)
                .set(COUNTER_DECR, ui);

            backend.draw(viewport, |c, gl| {
                // Draw our Ui!
                ui.draw_if_changed(c.view(), gl);
            });

        });

        // ui.handle_event(&event);

        thread::sleep(time::Duration::new(1, 0));
    }
}
