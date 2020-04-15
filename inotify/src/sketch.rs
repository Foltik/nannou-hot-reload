use nannou::{App, Draw, Event};

use inotify::{Inotify, WatchMask};
use libloading::{Library, Symbol};
use rand::{distributions::Alphanumeric, Rng};
use std::fs;

use lib::Model;

// Better name? LOL
pub trait Sketcher {
    fn poll(&mut self);
    fn reload(&mut self);

    fn update(&mut self, app: &App) -> Option<String>;
    fn event(&mut self, app: &App, event: Event) -> Option<String>;
    fn view(&self, app: &App, draw: &Draw);
}

pub struct Sketch<M: Model> {
    model: M,
    lib: Library,
    notify: Inotify,
}

impl<M: Model> Sketcher for Sketch<M> {
    fn poll(&mut self) {
        let mut buf = [0; 1024];
        let mut events = self.notify.read_events(&mut buf).unwrap();

        if events.any(|e| e.name.unwrap() == M::path().file_name().unwrap()) {
            self.reload()
        }
    }

    fn reload(&mut self) {
        let mut file = M::path()
            .file_name()
            .unwrap()
            .to_os_string()
            .into_string()
            .unwrap();
        println!("Reloading {:?}...", file);

        // Library::new() seems to cache libraries, so make a temp copy to load from

        file.push_str(".");
        file.push_str(
            &rand::thread_rng()
                .sample_iter(&Alphanumeric)
                .take(6)
                .collect::<String>(),
        );
        let temp = M::path().parent().unwrap().join(file);

        fs::copy(M::path(), &temp).unwrap();
        self.lib = Library::new(&temp).unwrap();
        fs::remove_file(&temp).unwrap();
    }

    // TODO: Should probably cache these symbols in DynamicModel but the lifetimes make it annoying

    fn update(&mut self, app: &App) -> Option<String> {
        let fun: Symbol<extern "C" fn(&App, &mut M) -> Option<String>> =
            unsafe { self.lib.get(b"update\0").unwrap() };

        fun(app, &mut self.model)
    }

    fn event(&mut self, app: &App, event: Event) -> Option<String> {
        let fun: Symbol<extern "C" fn(&App, &mut M, Event) -> Option<String>> =
            unsafe { self.lib.get(b"event\0").unwrap() };

        fun(app, &mut self.model, event)
    }

    fn view(&self, app: &App, draw: &Draw) {
        let fun: Symbol<extern "C" fn(&App, &M, &Draw)> =
            unsafe { self.lib.get(b"view\0").unwrap() };

        fun(app, &self.model, draw);
    }
}

impl<M: Model> Sketch<M> {
    pub fn new(model: M) -> Self {
        let lib = Library::new(M::path()).unwrap();

        let mut notify = Inotify::init().unwrap();
        notify
            .add_watch(M::path().parent().unwrap(), WatchMask::CREATE)
            .unwrap();

        Self { model, lib, notify }
    }
}
