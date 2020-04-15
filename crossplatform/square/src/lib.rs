use nannou::{Draw, App, Event, event::{WindowEvent::*, Key}, color::named::*};

use lib::Square;

#[no_mangle]
pub extern "C" fn update(_app: &App, __model: &mut Square) -> Option<String> {
    None
}

#[no_mangle]
pub extern "C" fn event(_app: &App, _model: &mut Square, event: Event) -> Option<String> {
    match event {
        Event::WindowEvent { id: _, simple: e } => match e {
            Some(KeyPressed(Key::Space)) => Some("circle".to_string()),
            _ => None,
        },
        _ => None,
    }
}

#[no_mangle]
pub extern "C" fn view(_app: &App, model: &Square, draw: &Draw) {
    draw.background().color(CORNFLOWERBLUE);

    draw.rect()
        .x_y(0.0, 0.0)
        .w_h(model.s, model.s)
        .color(WHITE);
}
