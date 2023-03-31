use std::cell::Cell;

use godot::{
    engine::{
        code_edit::CodeCompletionKind, file_access::ModeFlags, global, FileAccess, Script,
        ScriptLanguageExtension, ScriptLanguageExtensionVirtual,
    },
    prelude::*,
    private::class_macros::out,
};
use regex::Regex;

use crate::{get_base_type, mun_script::MunScript};

#[derive(GodotClass)]
#[class(base=ScriptLanguageExtension)]
pub struct MunExtension {
    #[base]
    base: Base<ScriptLanguageExtension>,
}

impl ScriptLanguageExtensionVirtual for MunExtension {
    fn init(base: Base<ScriptLanguageExtension>) -> Self {
        Self { base }
    }

    /// run every frame
    fn frame(&mut self) {}

    /// file extensions recognized as this type
    fn get_recognized_extensions(&self) -> PackedStringArray {
        println!("extension get_recognized_extensions");
        PackedStringArray::from(&[GodotString::from("mun")])
    }

    /// default file extension
    fn get_extension(&self) -> GodotString {
        println!("extension get_extension");
        GodotString::from("mun")
    }

    /// name of script
    fn get_name(&self) -> GodotString {
        println!("extension get_name");
        GodotString::from("Mun")
    }

    /// type of scripts made by this extension
    fn get_type(&self) -> GodotString {
        println!("extension get_type");
        GodotString::from("Mun")
    }

    /// public functions to view in documentation
    fn get_public_functions(&self) -> Array<Dictionary> {
        println!("extension get_public_functions");
        Array::new()
    }

    /// public constants to view in documentation
    fn get_public_constants(&self) -> Dictionary {
        println!("extension get_public_constants");
        Dictionary::new()
    }

    /// public annotations to view in documentation
    fn get_public_annotations(&self) -> Array<Dictionary> {
        println!("extension get_public_annotations");
        Array::new()
    }

    /// some VMs need to be notified of thread creation/exiting to allocate a stack
    fn thread_enter(&mut self) {}

    fn thread_exit(&mut self) {}

    /// which types of classes this script extension handles
    fn handles_global_class_type(&self, type_: GodotString) -> bool {
        println!("extension handles_global_class_type");
        let res = type_ == GodotString::from("Mun");
        std::mem::forget(type_);
        res
    }

    /// cleanup
    fn finish(&mut self) {
        println!("extension finish");
    }

    /// do classes made by this extension have names?
    /// otherwise use file path for the name
    fn has_named_classes(&self) -> bool {
        println!("extension has_named_classes");
        false
    }

    /// whether we can inherit from an arbitrary file
    fn can_inherit_from_file(&self) -> bool {
        println!("extension can_inherit_from_file");
        false
    }

    /// can we make builting versions of this script?
    fn supports_builtin_mode(&self) -> bool {
        println!("extension supports_builtin_mode");
        false
    }
    /// additional validation on file-paths, return empty string for no error
    fn validate_path(&self, path: GodotString) -> GodotString {
        println!("extension validate_path");
        std::mem::forget(path);
        GodotString::from("")
    }
    /// are there templates we can use?
    fn is_using_templates(&mut self) -> bool {
        println!("extension is_using_templates");
        false
    }

    /// create a template for the given class name and base class name named `template`
    fn make_template(
        &self,
        template: GodotString,
        class_name: GodotString,
        base_class_name: GodotString,
    ) -> Gd<godot::engine::Script> {
        println!("extension make_template");
        let mut script = Gd::<MunScript>::new_default();
        script.bind_mut().source_code = format!("// {}", class_name).into();

        std::mem::forget(template);
        std::mem::forget(class_name);
        std::mem::forget(base_class_name);

        script.upcast()
    }

    fn create_script(&self) -> Gd<Object> {
        println!("extension create_script");
        Gd::<MunScript>::new_default().upcast()
    }

    /// i think, should it be opened in godot even when external editor is set
    fn overrides_external_editor(&mut self) -> bool {
        println!("extension overrides_external_editor");
        false
    }

    /// what symbol is used for strings
    fn get_string_delimiters(&self) -> PackedStringArray {
        println!("extension get_string_delimiters");
        PackedStringArray::new()
    }

    /// check for errors/warnings
    /// format:
    /// {
    ///   valid: bool,
    ///   functions: Array<String>,
    ///   errors: Array<{line: i64, column: i64, message: GodotString}>,
    ///   warnings: Array<{start_line: i64, end_line: i64, leftmost_column: i64, rightmost_column: i64, code: i64, string_code: GodotString, message: GodotString}>,
    ///   safe_lines: PackedInt32Array
    /// }
    fn validate(
        &self,
        script: GodotString,
        path: GodotString,
        validate_functions: bool,
        validate_errors: bool,
        validate_warnings: bool,
        validate_safe_lines: bool,
    ) -> Dictionary {
        println!("extension validate");
        std::mem::forget(script);
        std::mem::forget(path);
        dict! {
            "valid": true
        }
    }

    /// list of delimiters for comments
    /// for instance: "//" and "/* */" for c#
    fn get_comment_delimiters(&self) -> PackedStringArray {
        println!("extension get_comment_delimiters");
        PackedStringArray::from(&[GodotString::from("//")])
    }

    fn init_ext(&mut self) {
        println!("extension init_ext");
    }

    /// get class name of file
    /// should be the class that it extends, such as Node
    /// returns dictionary: {
    ///   name: String,
    ///   base_type: String,
    ///   icon_path: String,
    /// }
    fn get_global_class_name(&self, path: GodotString) -> Dictionary {
        std::mem::forget(path.clone());
        println!("extension get_global_class_name");
        let Some(file) = FileAccess::open(path, ModeFlags::READ) else { return Dictionary::new() };

        let first_line = String::from(&file.get_line());
        let class_name = get_base_type(&first_line);
        dict! {"class_name": GodotString::from(&class_name)}
    }

    /// all keywords
    fn get_reserved_words(&self) -> PackedStringArray {
        println!("extension get_reserved_words");
        let keywords: Vec<&str> = vec![
            "pub", "fn", "if", "else", "let", "super", "break", "while", "extern", "mut", "use",
        ];
        PackedStringArray::from_iter(keywords.into_iter().map(GodotString::from))
    }

    /// keywords used for control flow
    fn is_control_flow_keyword(&self, keyword: GodotString) -> bool {
        println!("extension is_control_flow_keyword");
        std::mem::forget(keyword.clone());
        let s = String::from(&keyword);
        matches!(s.as_str(), "if" | "else" | "while" | "break" | "loop")
    }

    /// auto-completion for code given the file at path,
    fn complete_code(&self, code: GodotString, path: GodotString, owner: Gd<Object>) -> Dictionary {
        println!("extension complete_code");
        std::mem::forget(code.clone());
        std::mem::forget(path.clone());
        std::mem::forget(owner);
        AutoCompletion::to_dictionary(None)
    }

    fn lookup_code(
        &self,
        code: GodotString,
        symbol: GodotString,
        path: GodotString,
        owner: Gd<Object>,
    ) -> Dictionary {
        std::mem::forget(code.clone());
        std::mem::forget(symbol.clone());
        std::mem::forget(path.clone());
        std::mem::forget(owner);
        dict! {}
    }
}

pub struct AutoCompletion {
    result: global::Error,
    force: bool,
    call_hint: String,
    options: Vec<AutoCompletionOption>,
}

impl AutoCompletion {
    fn to_dictionary(completion: Option<Self>) -> Dictionary {
        let Some(AutoCompletion { result, force, call_hint, options }) = completion else { return dict!{} };

        dict! {
            "result": result.ord() as i64,
            "force": force,
            "call_hint": GodotString::from(&call_hint),
            "options": Array::from_iter(options.into_iter().map(Dictionary::from))
        }
    }
}

pub struct AutoCompletionOption {
    kind: CodeCompletionKind,
    display: String,
    insert_text: String,
    font_color: Color,
    icon: Gd<Resource>,
    default_value: Variant,
    location: i64,
    matches: Vec<i32>,
}

impl From<AutoCompletionOption> for Dictionary {
    fn from(option: AutoCompletionOption) -> Self {
        let AutoCompletionOption {
            kind,
            display,
            insert_text,
            font_color,
            icon,
            default_value,
            location,
            matches,
        } = option;

        dict! {
            "kind": kind.ord() as i64,
            "display": GodotString::from(&display),
            "insert_text": GodotString::from(&insert_text),
            "font_color": font_color,
            "icon": icon,
            "default_value": default_value,
            "location": location,
            "matches": PackedInt32Array::from(&matches[..])
        }
    }
}

impl ::godot::obj::cap::GodotInit for MunExtension {
    fn __godot_init(base: ::godot::obj::Base<Self::Base>) -> Self {
        <Self as ScriptLanguageExtensionVirtual>::init(base)
    }
}
impl ::godot::private::You_forgot_the_attribute__godot_api for MunExtension {}

impl ::godot::obj::cap::ImplementsGodotVirtual for MunExtension {
    fn __virtual_call(name: &str) -> ::godot::sys::GDExtensionClassCallVirtual {
        println!("== vcall extension {name} ==");
        match name {
            "_frame" => {
                ::godot::private::gdext_virtual_method_callback!(MunExtension,fn frame(&mut self))
            }
            "_get_recognized_extensions" => {
                ::godot::private::gdext_virtual_method_callback!(MunExtension,fn get_recognized_extensions(&self)->PackedStringArray)
            }
            "_get_extension" => {
                ::godot::private::gdext_virtual_method_callback!(MunExtension,fn get_extension(&self)->GodotString)
            }
            "_get_name" => {
                ::godot::private::gdext_virtual_method_callback!(MunExtension,fn get_name(&self)->GodotString)
            }
            "_get_type" => {
                ::godot::private::gdext_virtual_method_callback!(MunExtension,fn get_type(&self)->GodotString)
            }
            "_get_public_functions" => {
                ::godot::private::gdext_virtual_method_callback!(MunExtension,fn get_public_functions(&self)->Array<Dictionary>)
            }
            "_get_public_constants" => {
                ::godot::private::gdext_virtual_method_callback!(MunExtension,fn get_public_constants(&self)->Dictionary)
            }
            "_get_public_annotations" => {
                ::godot::private::gdext_virtual_method_callback!(MunExtension,fn get_public_annotations(&self)->Array<Dictionary>)
            }
            "_thread_enter" => {
                ::godot::private::gdext_virtual_method_callback!(MunExtension,fn thread_enter(&mut self))
            }
            "_thread_exit" => {
                ::godot::private::gdext_virtual_method_callback!(MunExtension,fn thread_exit(&mut self))
            }
            "_handles_global_class_type" => {
                ::godot::private::gdext_virtual_method_callback!(MunExtension,fn handles_global_class_type(&self,type_:GodotString)->bool)
            }
            "_finish" => {
                ::godot::private::gdext_virtual_method_callback!(MunExtension,fn finish(&mut self))
            }
            "_has_named_classes" => {
                ::godot::private::gdext_virtual_method_callback!(MunExtension,fn has_named_classes(&self)->bool)
            }
            "_can_inherit_from_file" => {
                ::godot::private::gdext_virtual_method_callback!(MunExtension,fn can_inherit_from_file(&self)->bool)
            }
            "_supports_builtin_mode" => {
                ::godot::private::gdext_virtual_method_callback!(MunExtension,fn supports_builtin_mode(&self)->bool)
            }
            "_validate_path" => {
                ::godot::private::gdext_virtual_method_callback!(MunExtension,fn validate_path(&self,path:GodotString)->GodotString)
            }
            "_is_using_templates" => {
                ::godot::private::gdext_virtual_method_callback!(MunExtension,fn is_using_templates(&mut self)->bool)
            }
            "_make_template" => {
                ::godot::private::gdext_virtual_method_callback!(MunExtension,fn make_template(&self,template:GodotString,class_name:GodotString,base_class_name:GodotString,)->Gd<godot::engine::Script>)
            }
            "_create_script" => {
                ::godot::private::gdext_virtual_method_callback!(MunExtension,fn create_script(&self)->Gd<Object>)
            }
            "_overrides_external_editor" => {
                ::godot::private::gdext_virtual_method_callback!(MunExtension,fn overrides_external_editor(&mut self)->bool)
            }
            "_get_string_delimiters" => {
                ::godot::private::gdext_virtual_method_callback!(MunExtension,fn get_string_delimiters(&self)->PackedStringArray)
            }
            "_validate" => {
                ::godot::private::gdext_virtual_method_callback!(MunExtension,fn validate(&self,script:GodotString,path:GodotString,validate_functions:bool,validate_errors:bool,validate_warnings:bool,validate_safe_lines:bool,)->Dictionary)
            }
            "_get_comment_delimiters" => {
                ::godot::private::gdext_virtual_method_callback!(MunExtension,fn get_comment_delimiters(&self)->PackedStringArray)
            }
            "_init" => {
                ::godot::private::gdext_virtual_method_callback!(MunExtension,fn init_ext(&mut self))
            }
            "_get_global_class_name" => {
                ::godot::private::gdext_virtual_method_callback!(MunExtension,fn get_global_class_name(&self,path:GodotString)->Dictionary)
            }
            "_get_reserved_words" => {
                ::godot::private::gdext_virtual_method_callback!(MunExtension,fn get_reserved_words(&self)->PackedStringArray)
            }
            "_is_control_flow_keyword" => {
                ::godot::private::gdext_virtual_method_callback!(MunExtension,fn is_control_flow_keyword(&self,keyword:GodotString)->bool)
            }
            "_complete_code" => Some({
                use godot::sys;
                unsafe extern "C" fn function(
                    instance_ptr: sys::GDExtensionClassInstancePtr,
                    args: *const sys::GDExtensionConstTypePtr,
                    ret: sys::GDExtensionTypePtr,
                ) {
                    use godot::sys;
                    out!("ptrcall: complete_code");
                    let storage = godot::private::as_storage::<MunExtension>(instance_ptr);
                    let mut instance = storage.get();
                    let mut idx = 0;
                    let code = <GodotString as sys::GodotFfi>::from_sys(sys::force_mut_ptr(
                        *args.offset(idx),
                    ));
                    idx += 1;
                    let path = <GodotString as sys::GodotFfi>::from_sys(sys::force_mut_ptr(
                        *args.offset(idx),
                    ));
                    idx += 1;

                    let ret_val = instance.complete_code(
                        code,
                        path,
                        std::mem::transmute::<(u64, Cell<Option<InstanceId>>), Gd<Object>>((
                            0u64,
                            Cell::new(None),
                        )),
                    );
                    <Dictionary as sys::GodotFfi>::write_sys(&ret_val, ret);
                    #[allow(clippy::forget_copy)]
                    std::mem::forget(ret_val);
                }
                function
            }),
            "_lookup_code" => Some({
                use godot::sys;
                unsafe extern "C" fn function(
                    instance_ptr: sys::GDExtensionClassInstancePtr,
                    args: *const sys::GDExtensionConstTypePtr,
                    ret: sys::GDExtensionTypePtr,
                ) {
                    use godot::sys;
                    out!("ptrcall: complete_code");

                    let storage = godot::private::as_storage::<MunExtension>(instance_ptr);
                    let mut instance = storage.get();
                    let mut idx = 0;
                    let code = <GodotString as sys::GodotFfi>::from_sys(sys::force_mut_ptr(
                        *args.offset(idx),
                    ));
                    idx += 1;
                    let symbol = <GodotString as sys::GodotFfi>::from_sys(sys::force_mut_ptr(
                        *args.offset(idx),
                    ));
                    idx += 1;
                    let path = <GodotString as sys::GodotFfi>::from_sys(sys::force_mut_ptr(
                        *args.offset(idx),
                    ));
                    idx += 1;

                    let ret_val = instance.lookup_code(
                        code,
                        symbol,
                        path,
                        std::mem::transmute::<(u64, Cell<Option<InstanceId>>), Gd<Object>>((
                            0u64,
                            Cell::new(None),
                        )),
                    );
                    <Dictionary as sys::GodotFfi>::write_sys(&ret_val, ret);
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
  class_name:"MunExtension",component: ::godot::private::PluginComponent::UserVirtuals {
    user_register_fn:None,user_create_fn:Some(::godot::private::callbacks::create:: <MunExtension>),user_to_string_fn:None,get_virtual_fn: ::godot::private::callbacks::get_virtual:: <MunExtension> ,
  },
});
