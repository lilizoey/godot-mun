use std::cell::Cell;

use godot::{
    engine::{
        global::{MethodFlags, PropertyHint, PropertyUsageFlags},
        Script, ScriptExtension, ScriptExtensionVirtual, ScriptLanguage,
    },
    prelude::*,
    private::class_macros::out,
    sys::{interface_fn, TagType},
};

use crate::{
    get_base_type,
    mun_extension::MunExtension,
    null_object,
    script_instance::{MunScriptInstance, MUN_SCRIPT_INSTANCE_INFO},
};

#[derive(GodotClass)]
#[class(init, base=ScriptExtension)]
pub struct MunScript {
    #[base]
    base: Base<ScriptExtension>,

    pub source_code: String,
}

impl ScriptExtensionVirtual for MunScript {
    fn editor_can_reload_from_file(&mut self) -> bool {
        println!("munscript editor_can_reload_from_file");
        false
    }

    // can we make an instance of this script?
    // should be false for invalid code for instance
    fn can_instantiate(&self) -> bool {
        println!("munscript can_instantiate");
        true
    }

    // the base of this script, for inheritance
    // can be null, for no base class
    fn get_base_script(&self) -> Gd<Script> {
        println!("munscript get_base_script");
        unsafe {
            std::mem::transmute::<(u64, Cell<Option<InstanceId>>), Gd<Script>>((
                0u64,
                Cell::new(None),
            ))
        }
    }

    fn get_global_name(&self) -> StringName {
        println!("munscript get_global_name");
        StringName::from("")
    }

    fn inherits_script(&self, script: Gd<Script>) -> bool {
        println!("munscript inherits_script");
        false
    }

    // Returns the script's base type.
    fn get_instance_base_type(&self) -> StringName {
        println!("munscript get_instance_base_type");
        let base_type = get_base_type(&self.source_code);
        StringName::from(base_type)
    }

    // Returns true if base_object is an instance of this script.
    fn instance_has(&self, object: Gd<Object>) -> bool {
        println!("munscript instance_has");
        false
    }

    fn has_source_code(&self) -> bool {
        println!("munscript has_source_code");
        true
    }

    fn get_source_code(&self) -> GodotString {
        println!("munscript get_source_code");
        GodotString::from(&self.source_code)
    }

    fn set_source_code(&mut self, code: GodotString) {
        println!("munscript set_source_code");
        std::mem::forget(code.clone());
        self.source_code = String::from(&code);
    }

    fn reload(&mut self, keep_state: bool) -> godot::engine::global::Error {
        println!("munscript reload");
        godot::engine::global::Error::OK
    }

    fn get_documentation(&self) -> Array<Dictionary> {
        println!("munscript get_documentation");
        Array::new()
    }

    fn has_method(&self, method: StringName) -> bool {
        println!("munscript has_method");
        false
    }

    fn get_method_info(&self, method: StringName) -> Dictionary {
        println!("munscript get_method_info");
        dict! {}
    }

    fn is_tool(&self) -> bool {
        println!("munscript is_tool");
        false
    }

    fn is_valid(&self) -> bool {
        println!("munscript is_valid");
        true
    }

    fn get_language(&self) -> Gd<ScriptLanguage> {
        println!("munscript get_language");
        Gd::<MunExtension>::new_default().upcast::<ScriptLanguage>()
    }

    fn has_script_signal(&self, signal: StringName) -> bool {
        println!("munscript has_script_signal");
        false
    }

    fn get_script_signal_list(&self) -> Array<Dictionary> {
        println!("munscript get_script_signal_list");
        Array::new()
    }

    fn has_property_default_value(&self, property: StringName) -> bool {
        println!("munscript has_property_default_value");
        false
    }

    fn get_property_default_value(&self, property: StringName) -> Variant {
        println!("munscript get_property_default_value");
        Variant::nil()
    }

    fn update_exports(&mut self) {
        println!("munscript update_exports");
    }

    fn get_script_method_list(&self) -> Array<Dictionary> {
        println!("munscript get_script_method_list");
        Array::new()
    }

    fn get_script_property_list(&self) -> Array<Dictionary> {
        println!("munscript get_script_property_list");
        Array::new()
    }

    // returns line number of a member of the script, -1 for not found
    fn get_member_line(&self, member: StringName) -> i64 {
        println!("munscript get_member_line");
        -1
    }

    fn get_constants(&self) -> Dictionary {
        println!("munscript get_constants");
        Dictionary::new()
    }

    fn get_members(&self) -> Array<StringName> {
        println!("munscript get_members");
        Array::new()
    }

    // should we use a placeholder script as a fallback?
    fn is_placeholder_fallback_enabled(&self) -> bool {
        println!("munscript is_placeholder_fallback_enabled");
        false
    }

    // see https://docs.godotengine.org/en/latest/classes/class_node.html#class-node-method-rpc-config
    // should return dictionary or null to disable
    fn get_rpc_config(&self) -> Variant {
        println!("munscript get_rpc_config");
        Variant::nil()
    }

    fn instance_create(&self, for_object: Gd<Object>) -> *mut std::ffi::c_void {
        std::mem::forget(for_object);
        let path = self.base.share().upcast::<Script>().get_path();
        let path = String::from(&path);
        let Some(instance) = MunScriptInstance::new(path) else { return std::ptr::null_mut() };
        let instance = Box::leak(Box::new(instance));
        unsafe {
            interface_fn!(script_instance_create)(
                &MUN_SCRIPT_INSTANCE_INFO,
                instance as *mut _ as *mut std::ffi::c_void,
            )
        }
    }
}
#[derive(Clone)]
pub struct PropertyInfo {
    type_: VariantType,
    name: GodotString,
    class_name: Option<StringName>,
    hint: PropertyHint,
    hint_string: GodotString,
    usage: PropertyUsageFlags,
}

impl From<PropertyInfo> for Dictionary {
    fn from(info: PropertyInfo) -> Self {
        let PropertyInfo {
            type_,
            name,
            class_name,
            hint,
            hint_string,
            usage,
        } = info;

        let mut d = dict! {
            "name": name,
            "type": type_ as i64,
            "hint": hint,
            "hint_string": hint_string,
            "usage": usage.ord() as i64,
        };
        if let Some(class_name) = class_name {
            d.insert("class_name", class_name);
        }
        d
    }
}

pub struct MethodInfo {
    name: GodotString,
    args: Vec<PropertyInfo>,
    return_val: PropertyInfo,
    flags: MethodFlags,
    default_arguments: Vec<Variant>,
}

impl From<MethodInfo> for Dictionary {
    fn from(info: MethodInfo) -> Self {
        let MethodInfo {
            name,
            args,
            return_val,
            flags,
            default_arguments,
        } = info;

        dict! {
            "name": name,
            "args": Array::from_iter(args.into_iter().map(|arg| Dictionary::from(arg))),
            "return": Dictionary::from(return_val),
            "flags": flags.ord() as i64,
            "default_args": Array::from_iter(default_arguments.into_iter())
        }
    }
}

impl ::godot::private::You_forgot_the_attribute__godot_api for MunScript {}

impl ::godot::obj::cap::ImplementsGodotVirtual for MunScript {
    fn __virtual_call(name: &str) -> ::godot::sys::GDExtensionClassCallVirtual {
        println!("=== vcall script {name} ===");
        match name {
            "_editor_can_reload_from_file" => {
                ::godot::private::gdext_virtual_method_callback!(MunScript,fn editor_can_reload_from_file(&mut self)->bool)
            }
            "_can_instantiate" => {
                ::godot::private::gdext_virtual_method_callback!(MunScript,fn can_instantiate(&self)->bool)
            }
            "_get_base_script" => {
                ::godot::private::gdext_virtual_method_callback!(MunScript,fn get_base_script(&self)->Gd<Script>)
            }
            "_get_global_name" => {
                ::godot::private::gdext_virtual_method_callback!(MunScript,fn get_global_name(&self)->StringName)
            }
            "_inherits_script" => {
                ::godot::private::gdext_virtual_method_callback!(MunScript,fn inherits_script(&self,script:Gd<Script>)->bool)
            }
            "_get_instance_base_type" => {
                ::godot::private::gdext_virtual_method_callback!(MunScript,fn get_instance_base_type(&self)->StringName)
            }
            "_instance_has" => {
                ::godot::private::gdext_virtual_method_callback!(MunScript,fn instance_has(&self,object:Gd<Object>)->bool)
            }
            "_has_source_code" => {
                ::godot::private::gdext_virtual_method_callback!(MunScript,fn has_source_code(&self)->bool)
            }
            "_get_source_code" => {
                ::godot::private::gdext_virtual_method_callback!(MunScript,fn get_source_code(&self)->GodotString)
            }
            "_set_source_code" => {
                ::godot::private::gdext_virtual_method_callback!(MunScript,fn set_source_code(&mut self,code:GodotString))
            }
            "_reload" => {
                ::godot::private::gdext_virtual_method_callback!(MunScript,fn reload(&mut self,keep_state:bool)->godot::engine::global::Error)
            }
            "_get_documentation" => {
                ::godot::private::gdext_virtual_method_callback!(MunScript,fn get_documentation(&self)->Array<Dictionary>)
            }
            "_has_method" => {
                ::godot::private::gdext_virtual_method_callback!(MunScript,fn has_method(&self,method:StringName)->bool)
            }
            "_get_method_info" => {
                ::godot::private::gdext_virtual_method_callback!(MunScript,fn get_method_info(&self,method:StringName)->Dictionary)
            }
            "_is_tool" => {
                ::godot::private::gdext_virtual_method_callback!(MunScript,fn is_tool(&self)->bool)
            }
            "_is_valid" => {
                ::godot::private::gdext_virtual_method_callback!(MunScript,fn is_valid(&self)->bool)
            }
            "_get_language" => {
                ::godot::private::gdext_virtual_method_callback!(MunScript,fn get_language(&self)->Gd<ScriptLanguage>)
            }
            "_has_script_signal" => {
                ::godot::private::gdext_virtual_method_callback!(MunScript,fn has_script_signal(&self,signal:StringName)->bool)
            }
            "_get_script_signal_list" => {
                ::godot::private::gdext_virtual_method_callback!(MunScript,fn get_script_signal_list(&self)->Array<Dictionary>)
            }
            "_has_property_default_value" => {
                ::godot::private::gdext_virtual_method_callback!(MunScript,fn has_property_default_value(&self,property:StringName)->bool)
            }
            "_get_property_default_value" => {
                ::godot::private::gdext_virtual_method_callback!(MunScript,fn get_property_default_value(&self,property:StringName)->Variant)
            }
            "_update_exports" => {
                ::godot::private::gdext_virtual_method_callback!(MunScript,fn update_exports(&mut self))
            }
            "_get_script_method_list" => {
                ::godot::private::gdext_virtual_method_callback!(MunScript,fn get_script_method_list(&self)->Array<Dictionary>)
            }
            "_get_script_property_list" => {
                ::godot::private::gdext_virtual_method_callback!(MunScript,fn get_script_property_list(&self)->Array<Dictionary>)
            }
            "_get_member_line" => {
                ::godot::private::gdext_virtual_method_callback!(MunScript,fn get_member_line(&self,member:StringName)->i64)
            }
            "_get_constants" => {
                ::godot::private::gdext_virtual_method_callback!(MunScript,fn get_constants(&self)->Dictionary)
            }
            "_get_members" => {
                ::godot::private::gdext_virtual_method_callback!(MunScript,fn get_members(&self)->Array<StringName>)
            }
            "_is_placeholder_fallback_enabled" => {
                ::godot::private::gdext_virtual_method_callback!(MunScript,fn is_placeholder_fallback_enabled(&self)->bool)
            }
            "_get_rpc_config" => {
                ::godot::private::gdext_virtual_method_callback!(MunScript,fn get_rpc_config(&self)->Variant)
            }
            "_instance_create" => Some({
                use godot::sys;
                unsafe extern "C" fn function(
                    instance_ptr: sys::GDExtensionClassInstancePtr,
                    args: *const sys::GDExtensionConstTypePtr,
                    ret: sys::GDExtensionTypePtr,
                ) {
                    use godot::sys;
                    out!("ptrcall: instance_create");
                    let storage = godot::private::as_storage::<MunScript>(instance_ptr);
                    let mut instance = storage.get();
                    let mut idx = 0;
                    let obj = interface_fn!(ref_get_object)(*args.offset(idx) as *const _);
                    let for_object: Gd<Object> =
                        <Gd<Object> as sys::GodotFfi>::from_sys(obj as *mut _);
                    idx += 1;
                    let ret_val = instance.instance_create(for_object);
                    std::ptr::write(ret as *mut *mut std::ffi::c_void, ret_val);
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
  class_name:"MunScript",component: ::godot::private::PluginComponent::UserVirtuals {
    user_register_fn:None,user_create_fn:None,user_to_string_fn:None,get_virtual_fn: ::godot::private::callbacks::get_virtual:: <MunScript> ,
  },
});
