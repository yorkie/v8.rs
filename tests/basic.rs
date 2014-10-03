extern crate v8;

#[test]
fn smoketest() {
    assert!(v8::V8::Initialize());
    v8::with_isolate(|isolate| {
        v8::with_isolate_scope(isolate, || {
            v8::with_handle_scope(isolate, || {
                let context = v8::Context::New(isolate);
                v8::with_context_scope(context, || {
                    let raw_source = "42*42".to_c_str().as_ptr();
                    let source =
                            v8::String::NewFromUtf8(isolate, raw_source, 0, -1);
                    v8::with_try_catch(|try_catch| {
                        let script = v8::Script::Compile(source, None);
                        assert_eq!(false, try_catch.HasCaught());
                        let result = script.Run();
                        assert_eq!(false, result.IsEmpty());
                        assert_eq!(false, try_catch.HasCaught());
                    });
                });
            });
        });
    });
    assert!(v8::V8::Dispose());
}
