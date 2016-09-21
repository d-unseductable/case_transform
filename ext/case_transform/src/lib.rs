#[macro_use]
extern crate ruru;
extern crate inflector;

// // dash: kebab-case
use inflector::cases::kebabcase::to_kebab_case;
// // underscore: snake_case
use inflector::cases::snakecase::to_snake_case;
// // camel_lower: camelCase
use inflector::cases::camelcase::to_camel_case;
// // camel: ClassCase (PascalCase)
use inflector::cases::classcase::to_class_case;

use ruru::{Class, Object, RString, Hash, Array, Symbol, AnyObject, VM};
use ruru::types::ValueType;

class!(CaseTransform);

methods! (
    CaseTransform,
    itself,

    fn deepTransformKeys(hash: Hash, block: &Fn(String) -> String) -> Hash {
        let result = Hash::new();

        hash.unwrap().each(|key, value| {
            let newValue = if value.ty() == ValueType::Hash { deepTransformKeys(value, block).to_any_object() } else { value };
            let newKey = RString::new(block(key.unwrap().to_string()));
            result.store(newKey, newValue);
        });

        result
    }

    fn transformArray(value: Array, transformMethod: &Fn(AnyObject) -> AnyObject) -> Array {
        value.map(|item| transformMethod(item)).unwrap()
    }

    fn transformHash(value: Hash, transformMethod: &Fn(AnyObject) -> AnyObject) -> Hash {
        deepTransformKeys(value, |key| transformMethod(key))
    }

    fn transformSymbol(value: Symbol, transformMethod: &Fn(AnyObject) -> AnyObject) -> Symbol {
        let transformed = transformMethod(value);
        Symbol::new(transformed);
    }

    fn transform(
        value: AnyObject,
        objectTransform: &Fn(AnyObject) -> AnyObject,
        keyTransform: &Fn(String) -> String
    ) -> AnyObject {
        match value.unwrap().ty() {
            ValueType::Array => transformArray(value, objectTransform).to_any_object(),
            ValueType::Hash => transformHash(value, objectTransform).to_any_object(),
            ValueType::Symbol => transformSymbol(value, objectTransform).to_any_object(),
            ValueType::RString => keyTransform(value).to_any_object(),
            ValueType::Object => value
        }
    }

    fn toPascalCase(key: String) -> String { to_class_case(to_snake_case(key.unwrap())) }
    fn toCamelCase(key: String) -> String { to_camel_case(to_snake_case(key.unwrap())) }
    fn toDashedCase(key: String) -> String { to_kebab_case(to_snake_case(key.unwrap())) }
    fn toSnakeCase(key: String) -> String { to_snake_case(key.unwrap()) }

    fn camel(value: AnyObject) -> AnyObject { transform(value.unwrap().to_any_object(), &camel, &toPascalCase) }
    fn camelLower(value: AnyObject) -> AnyObject { transform(value.unwrap().to_any_object(), &camelLower, &toCamelCase) }
    fn dash(value: AnyObject) -> AnyObject { transform(value.unwrap().to_any_object(), &dash, &toDashedCase) }
    fn underscore(value: AnyObject) -> AnyObject { transform(value.unwrap(), &underscore, &toSnakeCase) }
    fn unaltered(value: AnyObject) -> AnyObject { value.unwrap().to_any_object() }
);

#[no_mangle]
pub extern fn initialize_case_transform() {
    Class::new("CaseTransform", None).define(|itself| {
        itself.def_self("camel", camel);
        itself.def_self("camel_lower", camelLower);
        itself.def_self("dash", dash);
        itself.def_self("underscore", underscore);
        itself.def_self("unaltered", unaltered);
    });
}
