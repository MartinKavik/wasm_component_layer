wit_bindgen::generate!({
    path: "wit/component.wit",
    exports: {
        "test:guest/foo": Foo
    }
});

struct Foo;

impl exports::test::guest::foo::Guest for Foo {
    fn fn_calling_host_fn() {
        // 1)
        // Works as expected - prints `A message from host.`
        // WIT: `host-fn: func(param: option<s32>);`
        // Params: `[ValueType::Option(OptionType::new(ValueType::S32))]`
        test::guest::host::host_fn(None);

        // 2)
        // Panics with the error `Invalid discriminant value.`
        // WIT: `host-fn: func(param: option<s32>);`
        // Params: `[ValueType::Option(OptionType::new(ValueType::S32))]`
        // test::guest::host::host_fn(Some(123));

        // 3)
        // Panics with the error `Incorrect type. src\func.rs:816:27` (see updated `require_matches` below)
        // WIT: `host-fn: func(param: option<f64>);`
        // Params: `[ValueType::Option(OptionType::new(ValueType::F64))]`
        // test::guest::host::host_fn(None);
    }
}

// Updated `require_matches` to help with debugging

// macro_rules! require_matches {
//     ($expression:expr, $pattern:pat $(if $guard:expr)?, $then: expr) => {
//         match $expression {
//             $pattern $(if $guard)? => $then,
//             _ => { 
//                 let file = file!();
//                 let line = line!();
//                 let column = column!();
//                 bail!("Incorrect type. {file}:{line}:{column}")
//             }
//         }
//     };
// }
