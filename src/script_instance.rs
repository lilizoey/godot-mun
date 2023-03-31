use std::{collections::HashMap, sync::RwLock};

use godot::{
    prelude::*,
    sys::{GDExtensionPropertyInfo, GDExtensionScriptInstanceInfo},
};
pub struct MunScriptInstance {
    properties: RwLock<HashMap<String, Variant>>,
    property_info: Vec<GDExtensionPropertyInfo>,
    runtime: Runtime,
}

impl MunScriptInstance {
    pub fn new(script_path: String) -> Option<Self> {
        let runtime = Runtime::builder(script_path);
        let runtime = unsafe { runtime.finish() }.ok()?;
        Some(Self {
            properties: Default::default(),
            property_info: Default::default(),
            runtime,
        })
    }
}

impl MunScriptInstance {
    fn set(&self, name: String, value: Variant) -> bool {
        let mut properties = self.properties.write().unwrap();
        if !properties.contains_key(&name) {
            false
        } else {
            properties.insert(name, value);
            true
        }
    }

    fn get(&self, name: String) -> Option<Variant> {
        let properties = self.properties.read().unwrap();
        properties.get(&name).map(Clone::clone)
    }

    fn call(
        &self,
        method_name: String,
        args: &[Variant],
    ) -> Result<Variant, godot::sys::GDExtensionCallError> {
        println!("attempting to call {method_name}");
        let ret_val: Option<i64> = match args.len() {
            0 => self.runtime.invoke(&method_name, ()).ok(),
            1 => {
                let arg1 = args[0].clone();
                let Ok(arg1) = arg1.try_to::<i64>() else { return Err(
                    godot::sys::GDExtensionCallError {
                        error: godot::sys::GDEXTENSION_CALL_ERROR_INVALID_ARGUMENT,
                        argument: 0,
                        expected: VariantType::Int as i32,
                    }
                )};
                self.runtime.invoke(&method_name, (arg1,)).ok()
            }
            _ => {
                return Err(godot::sys::GDExtensionCallError {
                    error: godot::sys::GDEXTENSION_CALL_ERROR_TOO_MANY_ARGUMENTS,
                    argument: args.len() as i32,
                    expected: 1,
                })
            }
        };
        let Some(ret_val) = ret_val else { return Err(godot::sys::GDExtensionCallError {
            error: godot::sys::GDEXTENSION_CALL_ERROR_INVALID_METHOD,
            argument: 0,
            expected: 0,
        })};
        Ok(Variant::from(ret_val))
    }
}

use mun_runtime::{Runtime, RuntimeBuilder};
pub use script_ffi::MUN_SCRIPT_INSTANCE_INFO;

mod script_ffi {
    use std::mem::ManuallyDrop;

    use crate::mun_script::MunScript;

    use super::*;
    use godot::sys::*;

    pub static MUN_SCRIPT_INSTANCE_INFO: GDExtensionScriptInstanceInfo =
        GDExtensionScriptInstanceInfo {
            set_func: Some(set),
            get_func: Some(get),
            get_property_list_func: Some(get_property_list),
            /// called when godot is done with the property list
            free_property_list_func: None,
            property_can_revert_func: None,
            property_get_revert_func: None,
            get_owner_func: None,
            get_property_state_func: None,
            get_method_list_func: None,
            free_method_list_func: None,
            get_property_type_func: None,
            has_method_func: None,
            call_func: Some(call),
            notification_func: Some(notification),
            to_string_func: None,
            refcount_incremented_func: None,
            refcount_decremented_func: None,
            get_script_func: Some(get_script),
            is_placeholder_func: None,
            set_fallback_func: None,
            get_fallback_func: None,
            get_language_func: None,
            free_func: None,
        };

    /// # Safety
    /// instance must either be null or point to a valid [`MunScriptInstance`]
    /// instance must live for at least `'a`
    unsafe fn mun_instance<'a>(
        instance: GDExtensionScriptInstanceDataPtr,
    ) -> Option<&'a MunScriptInstance> {
        if instance.is_null() {
            return None;
        } else {
            Some(std::mem::transmute::<
                GDExtensionScriptInstanceDataPtr,
                &'a MunScriptInstance,
            >(instance))
        }
    }

    pub unsafe extern "C" fn set(
        p_instance: GDExtensionScriptInstanceDataPtr,
        p_name: GDExtensionConstStringNamePtr,
        p_value: GDExtensionConstVariantPtr,
    ) -> GDExtensionBool {
        let Some(instance) = mun_instance(p_instance) else { return false as GDExtensionBool };
        let name = ManuallyDrop::new(StringName::from_string_sys(p_name as *mut _));
        let value = Variant::from_var_sys(p_value as *mut _);

        instance.set(<String as From<&StringName>>::from(&name), value) as GDExtensionBool
    }

    pub unsafe extern "C" fn get(
        p_instance: GDExtensionScriptInstanceDataPtr,
        p_name: GDExtensionConstStringNamePtr,
        r_ret: GDExtensionVariantPtr,
    ) -> GDExtensionBool {
        let Some(instance) = mun_instance(p_instance) else { return false as GDExtensionBool };
        let name = ManuallyDrop::new(StringName::from_string_sys(p_name as *mut _));

        let res = instance.get(<String as From<&StringName>>::from(&name));

        match res {
            Some(variant) => {
                variant.write_var_sys(r_ret);
                true as GDExtensionBool
            }
            None => false as GDExtensionBool,
        }
    }

    pub unsafe extern "C" fn get_property_list(
        p_instance: GDExtensionScriptInstanceDataPtr,
        r_count: *mut u32,
    ) -> *const GDExtensionPropertyInfo {
        let Some(instance) = mun_instance(p_instance) else { return std::ptr::null_mut() };

        assert!(
            std::mem::size_of::<usize>() <= std::mem::size_of::<u32>()
                || instance.property_info.len() < u32::MAX as usize
        );

        let mut properties: Vec<GDExtensionPropertyInfo> = Vec::with_capacity(1);
        properties.extend(instance.property_info.iter());

        *r_count = properties.len() as u32;
        Box::leak(Box::new(properties)).as_ptr()
    }

    pub unsafe extern "C" fn free_property_list(
        p_instance: GDExtensionScriptInstanceDataPtr,
        p_list: *const GDExtensionPropertyInfo,
    ) {
    }

    pub unsafe extern "C" fn call(
        p_self: GDExtensionScriptInstanceDataPtr,
        p_method: GDExtensionConstStringNamePtr,
        p_args: *const GDExtensionConstVariantPtr,
        p_argument_count: GDExtensionInt,
        r_return: GDExtensionVariantPtr,
        r_error: *mut GDExtensionCallError,
    ) {
        let Some(instance) = mun_instance(p_self) else { return };
        let method = ManuallyDrop::new(StringName::from_string_sys(p_method as *mut _));
        let args: &[Variant] =
            std::slice::from_raw_parts(p_args as *const Variant, p_argument_count as usize);
        match instance.call(<String as From<&StringName>>::from(&method), args) {
            Ok(variant) => variant.write_var_sys(r_return),
            Err(err) => *r_error = err,
        }
    }

    pub unsafe extern "C" fn get_script(
        p_instance: GDExtensionScriptInstanceDataPtr,
    ) -> GDExtensionObjectPtr {
        let script = Box::leak(Box::new(Gd::<MunScript>::new_default()));
        script.sys_mut() as *mut _
    }

    pub unsafe extern "C" fn notification(
        p_instance: GDExtensionScriptInstanceDataPtr,
        p_what: i32,
    ) {
    }
}
