use nannou::{Draw, App, Event, event::{WindowEvent::*, Key}, color::named::*};

use lib::Circle;

#[no_mangle]
pub extern "C" fn update(_app: &App, __model: &mut Circle) -> Option<String> {
    None
}

#[no_mangle]
pub extern "C" fn event(_app: &App, _model: &mut Circle, event: Event) -> Option<String> {
    match event {
        Event::WindowEvent { id: _, simple: e } => match e {
            Some(KeyPressed(Key::Space)) => Some("square".to_string()),
            _ => None,
        },
        _ => None,
    }
}

#[no_mangle]
pub extern "C" fn view(_app: &App, model: &Circle, draw: &Draw) {
    draw.background().color(CORNFLOWERBLUE);

    draw.ellipse()
        .x_y(0.0, 0.0)
        .w_h(model.r, model.r)
        .color(WHITE);
}
