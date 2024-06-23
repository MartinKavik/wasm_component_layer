use wasm_component_layer::*;

const WASM: &[u8] = include_bytes!("func_bug_demo/component.wasm");

pub fn main() {
    let engine = Engine::new(wasmi_runtime_layer::Engine::default());
    let mut store = Store::new(&engine, ());
    let component = Component::new(&engine, WASM).unwrap();

    let mut linker = Linker::default();

    let host_interface = linker
        .define_instance(
            "test:guest/host"
                .try_into()
                .unwrap(),
        )
        .unwrap();

    let host_fn = Func::new(
        &mut store,
        FuncType::new(
            [ValueType::Option(OptionType::new(ValueType::S32))],
            // [ValueType::Option(OptionType::new(ValueType::F64))],
            [],
        ),
        |_store, _params, _returns| {
            println!("A message from host.");
            Ok(())
        },
    );
    host_interface
        .define_func("host-fn", host_fn)
        .unwrap();

    let instance = linker.instantiate(&mut store, &component).unwrap();

    let foo_interface = instance
        .exports()
        .instance(&"test:guest/foo".try_into().unwrap())
        .unwrap();

    let fn_calling_host_fn = foo_interface
        .func("fn-calling-host-fn")
        .unwrap()
        .typed::<(), ()>()
        .unwrap();

    fn_calling_host_fn.call(&mut store, ()).unwrap();
}
