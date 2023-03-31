use godot::{
    engine::{
        file_access::ModeFlags, global::Error, FileAccess, ResourceFormatSaver,
        ResourceFormatSaverVirtual,
    },
    prelude::*,
    private::class_macros::out,
    sys::interface_fn,
};
use once_cell::sync::Lazy;

use std::{collections::HashMap, fs::File, io::Write, sync::Mutex};

use crate::mun_script::MunScript;

pub static UID_MAP: Lazy<Mutex<HashMap<String, i64>>> = Lazy::new(Default::default);

#[derive(GodotClass)]
#[class(init, base=ResourceFormatSaver)]
pub struct MunFormatSaver {
    #[base]
    base: Base<ResourceFormatSaver>,
}

impl ResourceFormatSaverVirtual for MunFormatSaver {
    fn save(&mut self, resource: Gd<Resource>, path: GodotString, flags: i64) -> Error {
        std::mem::forget(path.clone());
        std::mem::forget(resource.share());
        println!("saver save: {}", String::from(&path));
        let script = resource.cast::<MunScript>();
        let Some(mut file) = FileAccess::open(path, ModeFlags::WRITE) else { return Error::ERR_CANT_OPEN };
        file.store_string(GodotString::from(&script.bind().source_code));
        Error::OK
    }
    fn set_uid(&mut self, path: GodotString, uid: i64) -> Error {
        println!("saver set_uid");
        println!("### LOCKING THEN UNLOCKING ###");
        UID_MAP.lock().unwrap().insert(String::from(&path), uid);
        std::mem::forget(path);
        Error::OK
    }
    fn recognize(&self, resource: Gd<Resource>) -> bool {
        println!("saver recognize");
        let res = resource.is_class("MunScript".into());
        std::mem::forget(resource);
        res
    }
    fn get_recognized_extensions(&self, resource: Gd<Resource>) -> PackedStringArray {
        println!("saver get_recognized_extensions");
        std::mem::forget(resource);
        PackedStringArray::from(&[GodotString::from("mun")])
    }
}
impl ::godot::private::You_forgot_the_attribute__godot_api for MunFormatSaver {}

impl ::godot::obj::cap::ImplementsGodotVirtual for MunFormatSaver {
    fn __virtual_call(name: &str) -> ::godot::sys::GDExtensionClassCallVirtual {
        match name {
            "_save" => Some({
                use godot::sys;
                unsafe extern "C" fn function(
                    instance_ptr: sys::GDExtensionClassInstancePtr,
                    args: *const sys::GDExtensionConstTypePtr,
                    ret: sys::GDExtensionTypePtr,
                ) {
                    use godot::sys;
                    out!("ptrcall: save");
                    let storage = godot::private::as_storage::<MunFormatSaver>(instance_ptr);
                    let mut instance = storage.get_mut();
                    let mut idx = 0;
                    let resource =
                        <Gd<Resource> as sys::GodotFfi>::from_sys(interface_fn!(ref_get_object)(
                            *args.offset(idx) as sys::GDExtensionConstRefPtr,
                        )
                            as sys::GDExtensionTypePtr);
                    idx += 1;
                    let path = <GodotString as sys::GodotFfi>::from_sys(sys::force_mut_ptr(
                        *args.offset(idx),
                    ));
                    idx += 1;
                    let flags =
                        <i64 as sys::GodotFfi>::from_sys(sys::force_mut_ptr(*args.offset(idx)));
                    idx += 1;
                    let ret_val = instance.save(resource, path, flags);
                    <Error as sys::GodotFfi>::write_sys(&ret_val, ret);
                    #[allow(clippy::forget_copy)]
                    std::mem::forget(ret_val);
                }
                function
            }),
            "_set_uid" => {
                ::godot::private::gdext_virtual_method_callback!(MunFormatSaver,fn set_uid(&mut self,path:GodotString,uid:i64)->Error)
            }
            "_recognize" => Some({
                use godot::sys;
                unsafe extern "C" fn function(
                    instance_ptr: sys::GDExtensionClassInstancePtr,
                    args: *const sys::GDExtensionConstTypePtr,
                    ret: sys::GDExtensionTypePtr,
                ) {
                    use godot::sys;
                    out!("ptrcall: recognize");

                    let storage = godot::private::as_storage::<MunFormatSaver>(instance_ptr);
                    let mut instance = storage.get();
                    let mut idx = 0;
                    let resource =
                        <Gd<Resource> as sys::GodotFfi>::from_sys(interface_fn!(ref_get_object)(
                            *args.offset(idx) as sys::GDExtensionConstRefPtr,
                        )
                            as sys::GDExtensionTypePtr);
                    idx += 1;
                    let ret_val = instance.recognize(resource);
                    <bool as sys::GodotFfi>::write_sys(&ret_val, ret);
                    #[allow(clippy::forget_copy)]
                    std::mem::forget(ret_val);
                }
                function
            }),
            "_get_recognized_extensions" => Some({
                use godot::sys;
                unsafe extern "C" fn function(
                    instance_ptr: sys::GDExtensionClassInstancePtr,
                    args: *const sys::GDExtensionConstTypePtr,
                    ret: sys::GDExtensionTypePtr,
                ) {
                    use godot::sys;
                    out!("ptrcall: get_recognized_extensions");

                    let storage = godot::private::as_storage::<MunFormatSaver>(instance_ptr);
                    let mut instance = storage.get();
                    let mut idx = 0;
                    let resource =
                        <Gd<Resource> as sys::GodotFfi>::from_sys(interface_fn!(ref_get_object)(
                            *args.offset(idx) as sys::GDExtensionConstRefPtr,
                        )
                            as sys::GDExtensionTypePtr);
                    idx += 1;
                    let ret_val = instance.get_recognized_extensions(resource);
                    <PackedStringArray as sys::GodotFfi>::write_sys(&ret_val, ret);
                    #[allow(clippy::forget_copy)]
                    std::mem::forget(ret_val);
                }
                function
            }),
            _ => None,
        }
    }
}
::godot::sys::plugin_add!(__GODOT_PLUGIN_REGISTRY in::godot::private;
::godot::private::ClassPlugin {
  class_name:"MunFormatSaver",component: ::godot::private::PluginComponent::UserVirtuals {
    user_register_fn:None,user_create_fn:None,user_to_string_fn:None,get_virtual_fn: ::godot::private::callbacks::get_virtual:: <MunFormatSaver> ,
  },
});
