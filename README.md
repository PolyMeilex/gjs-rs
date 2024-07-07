# GJS-rs

```rs
fn main() {
    let ctx = gjs::Context::new();

    ctx.eval("console.log('Hello world!')", "main.js").unwrap();
}
```
