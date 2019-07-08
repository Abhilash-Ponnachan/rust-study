# Rust Programming - Overview
_Rust_ is a `multi paradigm systems programming language` developed at _Mozilla_ Research and open sourced under _MIT License_ and _Apache License 2.0_.  
It was originally designed by _Graydon Hoare_ with contributions added on by others including _Brendan Eich_.

It is a language designed with **safety**, **speed**, and **concurrency** as primary importance. It is a _low level langauge_ and doesn't use _Garbage Collection_, however it brings together different programming paradigms such as `procedural`, `object-oriented`, `concurrent actor`, `functional`, and `meta-programming`.

It is a modern language built on `LLVM` backend and influenced by other languages such as `C/C++`, `Lisp`, `Swift`, `Haskell`, `Erlang` to name a few.

Even though _Rust_ is a _low level language_ it has a wide range of applicability from - `command-line tools`, `embedded systems`, `WebAssembly`, `networking`, and even `web applications`.

## Installation
The standard way to install _Rust_ is using the `rustup` tool, which is an _installer_, and _version management_ tool for _Rust_.

On _Linux_ and _Mac_ we can install it from the terminal using - 
```bash
$ curl https://sh.rustup.rs -sSf | sh
```
This downloads the script file and executes it in the shell. This bootstraps the installation tool and installs the necessary components.

On _Windows_ we can download the **rustup-init.exe**, and from there it basically works the same. It will require the _C++ build tools for VS 2013 or later_ as pre-requisite.

Once complete, the process ends up installing the following components -
- `rustup` - **installation** and **version managment** tool
    - _Rust_ has a rapid development and 6 week release cycle, so this tool is essential to manage the versions
    - To update _Rust_ simply do
    ```bash
    $ rustup update
    ```
- `rustc` - the _Rust_ **compiler**
- `cargo` - the _Rust_ **build tool** and **package manager**
    - `cargo build` - builds your project
    - `cargo run` - runs your project
    - `cargo new` - creates new project with manifest & structure
    - `cargo test` - tests the project
    - `cargo doc` - generates documentation from code
    - `cargo publish` - publishes a library to **crates.io**

All the tools are installed in the `~/.cargo/bin` directory.

## IDE
We can use any IDE or text editor to write _Rust_. In our case we shall use _VS Code_ with the plugins - 
- `Rust (rls)` - from _rust-lang_ for _code completion_, _intellisense_ etc.
- `Code Runner` - run code from within editor

## Hello World
Now that we have _Rust_ up and running let us write a simple _Hello World_ program.  

Create a new directory `hello_world` and add a new file `hello.rs`. open it in an editor and type in the following code -
```rust
// hello.rs
fn main(){
    println!("Hello World!");
}
```
Now we can compile it using -
```bash
$ rustc hello.rs
```
This should produce and executable file that we can run directly from the terminal -
```bash
$ ./hello
Hello World!
```
### Structure of the code
Even though the program we have written is very basic, it introduces some fundamental parts of _Rust_ code.  

Using `fn` we define a _function_ (and in _Rust_ we can write our code in outer functions directly and do not need to wrap them in _classes_ or _objects_ as in _Java_ or _C#_).  

The `main` function is a special function as it is the _Entry Point_ to the program. It is the first function to be executed when we run the compiled binary. This is similar to other languages such as _C_.  
`main` can take paramters or be empty.

All the work in our code is done by `println!`, which looks like a _function_ but is really a _macro_ (as indicated by the **!** suffix). We shall learn about _macros_ later but in simple words it is a line of code that is expanded in-place by the compiler with the code that does the actual lifting. So it is not a _sub-routine_ call to another _stack_ like a _function_. We will encounter this and more _macros_ as we go further.

## Hello Cargo
Now