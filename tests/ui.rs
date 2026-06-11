//! Headless UI verification: drive the app with synthetic input, assert
//! on state, and save a screenshot you (or your agent) can look at.

#[path = "../src/main.rs"]
mod app;

use app::{App_, Msg};
use fenestra::prelude::*;
use fenestra::shell::{SyntheticEvent, render_app};

#[test]
fn typing_and_clicking_work() {
    let mut app = App_::default();
    let image = render_app(
        &mut app,
        &[
            SyntheticEvent::Tab, // focus the first focusable (the − button)
            SyntheticEvent::Key(KeyInput::plain(Key::Enter)),
        ],
        (480, 420),
        &Theme::light(),
    );
    assert_eq!(app.count, -1);
    assert_eq!(image.dimensions(), (480, 420));
    // Want to see it? image.save("ui.png").unwrap();
    let _ = Msg::Inc;
}
