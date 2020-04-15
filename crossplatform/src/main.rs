use nannou::{event::Update, App, Event, Frame};
use std::collections::HashMap;

mod sketch;

use lib::{Circle, Square};
use sketch::{Sketch, Sketcher};

fn main() {
    nannou::app(model)
        .event(event)
        .update(update)
        .simple_window(view)
        .run();
}

struct Model {
    sketches: HashMap<String, Box<dyn Sketcher>>,
    active: String,
}

fn model(_app: &App) -> Model {
    let mut sketches: HashMap<String, Box<dyn Sketcher>> = HashMap::new();

    let circle = Box::new(Sketch::new(Circle { r: 80.0 }));
    sketches.insert("circle".to_string(), circle);

    let square = Box::new(Sketch::new(Square { s: 100.0 }));
    sketches.insert("square".to_string(), square);

    Model {
        sketches,
        active: "circle".to_string(),
    }
}

fn event(app: &App, model: &mut Model, event: Event) {
    let sketch = model
        .sketches
        .get_mut(&model.active)
        .expect("active sketch not found");

    if let Some(next) = sketch.event(app, event) {
        model.active = next;
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    model.sketches.iter_mut()
        .for_each(|(_, s)| s.poll());

    let sketch = model
        .sketches
        .get_mut(&model.active)
        .expect("active sketch not found");

    if let Some(next) = sketch.update(app) {
        model.active = next;
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let sketch = model
        .sketches
        .get(&model.active)
        .expect("active sketch not found");

    let draw = app.draw();

    sketch.view(app, &draw);

    draw.to_frame(app, &frame).unwrap();
}
