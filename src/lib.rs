use std::cell::Cell;

use godot::{
    engine::{Engine, ResourceLoader, ResourceSaver},
    prelude::*,
};
use mun_extension::MunExtension;
use regex::Regex;

use crate::{mun_loader::MunFormatLoader, mun_saver::MunFormatSaver};

mod mun_extension;
mod mun_loader;
mod mun_saver;
mod mun_script;
mod script_instance;

struct GodotMun;

#[gdextension]
unsafe impl ExtensionLibrary for GodotMun {
    fn load_library(handle: &mut InitHandle) -> bool {
        handle.register_layer(InitLevel::Scene, DefaultLayer);
        true
    }
}

struct DefaultLayer;

impl ExtensionLayer for DefaultLayer {
    fn initialize(&mut self) {
        godot::private::class_macros::auto_register_classes();
        let result = Engine::singleton()
            .register_script_language(Gd::<MunExtension>::new_default().upcast());
        assert_eq!(result, godot::engine::global::Error::OK);
        let loader = Gd::<MunFormatLoader>::new_default();
        std::mem::forget(loader.share());
        ResourceLoader::singleton().add_resource_format_loader(loader.upcast(), false);
        let saver = Gd::<MunFormatSaver>::new_default();
        std::mem::forget(saver.share());
        ResourceSaver::singleton().add_resource_format_saver(saver.upcast(), false);
    }

    fn deinitialize(&mut self) {
        // Nothing -- note that any cleanup task should be performed outside of this method,
        // as the user is free to use a different impl, so cleanup code may not be run.
    }
}

fn get_base_type(source: &str) -> String {
    let Some(first_line) = source.lines().next() else { return String::new() };
    let re = Regex::new(r"^\s*//\s*(\w+)\s*$").unwrap();
    re.captures(first_line)
        .and_then(|captures| captures.get(1))
        .map(|captured| String::from(captured.as_str()))
        .unwrap_or(String::new())
}

unsafe fn null_object<T>() -> Gd<T>
where
    T: GodotClass,
{
    std::mem::transmute::<(u64, Cell<Option<InstanceId>>), Gd<T>>((0u64, Cell::new(None)))
}

fn is_null_object<T>(gd: &Gd<T>) -> bool
where
    T: GodotClass,
{
    unsafe { std::mem::transmute::<&Gd<T>, &(u64, Cell<Option<InstanceId>>)>(gd).0 == 0 }
}
