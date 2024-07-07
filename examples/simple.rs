fn main() {
    let ctx = gjs::Context::new();

    ctx.eval("console.log('Hello world!')", "main.js").unwrap();

    ctx.register_module("main", "file:///home/poly/dev/gjs-rs/examples/main.js")
        .unwrap();
    ctx.eval_module("main").unwrap();
}
