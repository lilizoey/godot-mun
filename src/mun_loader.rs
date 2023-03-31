use std::{
    collections::{hash_map::RandomState, HashMap},
    fs::File,
    hash::{BuildHasher, Hash, Hasher},
    io::Read,
};

use godot::{
    engine::{
        file_access::ModeFlags, global::Error, FileAccess, ResourceFormatLoader,
        ResourceFormatLoaderVirtual,
    },
    prelude::*,
};

use crate::{mun_saver::UID_MAP, mun_script::MunScript};

#[derive(GodotClass)]
#[class(init, base=ResourceFormatLoader)]
pub struct MunFormatLoader {
    #[base]
    base: Base<ResourceFormatLoader>,
}

#[godot_api]
impl ResourceFormatLoaderVirtual for MunFormatLoader {
    // Extensions that will be loaded by this loader
    fn get_recognized_extensions(&self) -> PackedStringArray {
        println!("loader get_recognized_extensions");
        PackedStringArray::from(&[GodotString::from("mun")])
    }

    // What type of resources this loader loads
    fn handles_type(&self, type_: StringName) -> bool {
        println!("loader handles_type");
        let type_string = String::from(&type_);
        let res = matches!(type_string.as_str(), "Script" | "Mun");
        std::mem::forget(type_);
        res
    }

    fn get_resource_type(&self, path: GodotString) -> GodotString {
        println!("loader get_resource_type");
        std::mem::forget(path.clone());
        if path.to_string().ends_with(".mun") {
            "Mun".into()
        } else {
            "".into()
        }
    }

    fn get_resource_uid(&self, path: GodotString) -> i64 {
        println!("loader get_resource_uid");
        println!("### LOCKING THEN UNLOCKING ###");
        let res = *UID_MAP
            .lock()
            .unwrap()
            .get(&String::from(&path))
            .unwrap_or(&-1);
        std::mem::forget(path);
        res
    }
    /*
        // does there exist a resource at this path?
        fn exists(&self, path: GodotString) -> bool {
            println!("loader exists");
            std::mem::forget(path);
            true
        }
    */
    fn load(
        &self,
        path: GodotString,
        original_path: GodotString,
        use_sub_threads: bool,
        cache_mode: i64,
    ) -> Variant {
        std::mem::forget(path.clone());
        std::mem::forget(original_path);
        println!("loader load");
        let Some(file) = FileAccess::open(path, ModeFlags::READ) else { return Error::ERR_CANT_OPEN.to_variant() };
        let mut script = Gd::<MunScript>::new_default();
        std::mem::forget(script.share());
        let contents = file.get_as_text(false);
        let contents_string = String::from(&contents);
        script.bind_mut().source_code = contents_string;
        let res = script.to_variant();

        res
    }
}
