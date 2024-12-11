# Rust Emulator 8086

*At the moment, it supports only a few instruction: inc, mov and jmp. The list of support instructions will soon grow.*

## How to use

1. Run the back-end server
```
remu8086 $ cargo run
   Compiling remu8086 v0.1.0 (/Users/user/study/remu8086)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 2.88s
     Running `target/debug/remu8086`
Rust web-server started at 127.0.0.1:8080
```

2. Open "index.html" file including an example code with your browser.

![](/open.png)


3. Click "Build" button to build the example code in the text area. You can see register values and memory values.

![](/build.png)


4. Click "Step" button to run a single instruction. You can see register values changed.

![](/step.png)


## References

* [In the beginning, there was the Assembly tutorial by myself](https://github.com/gurugio/book_assembly_8086)
* [Microprocessor Tutorials of GeeksforGeeks](https://www.geeksforgeeks.org/microprocessor-tutorials/)