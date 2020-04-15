use nannou::{App, Draw, Event};

use dynamic_reload::{DynamicReload, Lib, Symbol, Search, UpdateState, PlatformName};
use std::sync::Arc;

use lib::Model;

// Better name? LOL
pub trait Sketcher {
    fn poll(&mut self);

    fn update(&mut self, app: &App) -> Option<String>;
    fn event(&mut self, app: &App, event: Event) -> Option<String>;
    fn view(&self, app: &App, draw: &Draw);
}

pub struct Sketch<M: Model> {
    model: M,
    lib: Arc<Lib>,
    handler: DynamicReload,
}

impl<M: Model> Sketcher for Sketch<M> {
    fn poll(&mut self) {
        self.handler.update(|old: &mut Arc<Lib>, state: UpdateState, lib: Option<&Arc<Lib>>| {
            match state {
                UpdateState::After => {
                    println!("Reloading {:?}...", M::name());
                    *old = Arc::clone(lib.unwrap());
                },
                UpdateState::Before => (),
                UpdateState::ReloadFailed(e) => panic!("Failed to reload: {:?}", e)
            };
        }, &mut self.lib);
    }

    // TODO: Should probably cache these symbols in DynamicModel but the lifetimes make it annoying

    fn update(&mut self, app: &App) -> Option<String> {
        let fun: Symbol<extern "C" fn(&App, &mut M) -> Option<String>> =
            unsafe { self.lib.lib.get(b"update\0").unwrap() };

        fun(app, &mut self.model)
    }

    fn event(&mut self, app: &App, event: Event) -> Option<String> {
        let fun: Symbol<extern "C" fn(&App, &mut M, Event) -> Option<String>> =
            unsafe { self.lib.lib.get(b"event\0").unwrap() };

        fun(app, &mut self.model, event)
    }

    fn view(&self, app: &App, draw: &Draw) {
        let fun: Symbol<extern "C" fn(&App, &M, &Draw)> =
            unsafe { self.lib.lib.get(b"view\0").unwrap() };

        fun(app, &self.model, draw);
    }
}

impl<M: Model> Sketch<M> {
    pub fn new(model: M) -> Self {
        let path = M::path().to_str().unwrap();
        let mut handler = DynamicReload::new(Some(vec![path]), Some(path), Search::Default);

        let lib = handler.add_library(M::name(), PlatformName::Yes).unwrap();

        Self {
            model,
            lib,
            handler,
        }
    }
}
