# Rust Programming - Overview
_Rust_ is a `multi paradigm systems programming language` developed at _Mozilla_ Research and open sourced under _MIT License_ and _Apache License 2.0_.  
It was originally designed by _Graydon Hoare_ with contributions added on by others including _Brendan Eich_.

It is a language designed with **safety**, **speed**, and **concurrency** as primary importance. It is a _low level langauge_ and doesn't use _Garbage Collection_, however it brings together different programming paradigms such as `procedural`, `object-oriented`, `concurrent actor`, `functional`, and `meta-programming`.

It is a modern language built on `LLVM` backend and influenced by other languages such as `C/C++`, `Lisp`, `Swift`, `Haskell`, `Erlang` to name a few.

Even though _Rust_ is a _low level language_ it has a wide range of applicability from - `command-line tools`, `embedded systems`, `WebAssembly`, `networking`, and even `web applications`.

## Installation
The standard way to install _Rust_ is using the `rustup` tool, which is an _installer_, and _version management_ tool for _Rust_.

On _Linux_ and _Mac_ we can install it from the terminal using - 
```console
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
```console
$ rustc hello.rs
```
This should produce and executable file that we can run directly from the terminal -
```console
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
Whilst compiling with `rustc` is fine for small code bases, for real-world projects we would want to create and manage the project with the `cargo` tool.  
It is the _build system_ and _package manager_ for _Rust_ (create the project structure and manifest, manage dependencies and configure build flows). It is like `npm` for `Node.js`.

### Create a project with `cargo`
We shall create a project named `hello_cargo`. To do that go to our working directory and use `cargo` to create a new project -
```console
$ cargo new hello_cargo
     Created binary (application) `hello_cargo` package
```
This should create a new directory named `hello_cargo` with the follwing contents -
```console
./hello_cargo
    |_Cargo.toml
    |_/src
        |_main.rs
```
_It also creates a `git` repository with a `.gitignore` file if the parent directory does not already have one. It is possible to control the repository using the `--VCS <VCS>` switch. We can even specify (none)._

The tool has created a `Cargo.toml` file and a `src` directory with a `main.rs` source file.

- #### "Cargo.toml"
This is essntially the project manifest with configuration information about the project and its dependencies, like `package.json` in _NPM_. Unlike _NPM_ though _Cargo_ uses the `TOML` (Tom's Obvious, Minimal Language - By Tom Preston-Werner) syntax instead of `JSON`. This kind of resmbles `INI` files from back in the days, and is essentially a structured list of Key-Value pairs. Our `toml` file would look like - 
```toml
[package]
name = "hello_cargo"
version = "0.1.0"
authors = ["<name>"]
edition = "2018"

[dependencies]
```
At this stage we do not have any dependencies, so that would be empty.

- #### "src" (directory)
Inside the `src` directory `cargo` would have created a `main.rs` file with a simple program to print _"Hello, world!"_ (exactly like what we wrote in the previous section).  

`cargo` expects all source code to exist within the `src` directory. The top-level project directory is for configuration, license and readme etc.

If we started a project without `cargo` all we have to do to convert it is to copy all the source code to and `src` sub-directory and create a `Cargo.toml` file at the top-level.

### Build with `cargo`
With the project and code in place we can use `cargo` to build it for us by using the `build` command from the project directory -
```console
$ cargo build
   Compiling hello_cargo v0.1.0 (<working directory>\hello_cargo)
    Finished dev [unoptimized + debuginfo] target(s) in 4.30s
```
This will create an executable in a `./target/debug` directory. It will also create other files such as `.pdb` for debug etc. We can run our executable from the terminal -
```console
$ ./target/debug/hello_cargo
Hello, world!
```

By default it `cargo` will build for _debug_ mode, which means it has debug symbol files and is not optimized. When the project is ready for **release** we can build with the `--release` switch/option and this will do an optimized build for release environments. The comiplation/build time for **release** mode takes longer but the binary is optmized to execute faster. The **release** build would end up in the `./target/release` directory.

### Run with `cargo`
We can _build_ and _run_ the project in a single command using `run` -
```console
$ cargo run
   Compiling hello_cargo v0.1.0 (<working directory>\hello_cargo)
    Finished dev [unoptimized + debuginfo] target(s) in 1.36s
     Running `target\debug\hello_cargo.exe`
Hello, world!
```
We can see the code built, and the output from the executed binary.

### Check with `cargo`
`cargo` provides a way to **check** if everything is alright woith our code and is good enough for _build_, without _actually creating executables_. To do this use the `check` command -
```console
$ cargo check
    Checking hello_cargo v0.1.0 (<working directory>\hello_cargo)
    Finished dev [unoptimized + debuginfo] target(s) in 0.41s
```
This does everything needed to build without actually creating the executable file, and since there is no I/O it is significantly faster. It is common practice to periodically doing a `cargo check` as we progress with coding to ensure that everything is in a _build ready_ state. 

## Basic Programming Elements
In this section we shall cover the basic programming constructs in _Rust_ that allow us to declare variable, assign values, basic data types, create functions etc.  These are the building blocks of most programming languages, and here we shall see how they are expressed in _Rust_. 

### Comments
_Rust_ has the same syntax as _C/C++_ for basic comments - 
- Line comments - start with `//`
- Block comments - enclose in `/* .. */`  
    - However as a convention, the _Rust community_ seems to encourage using line comments (`//`) for multi-line comments (like using `#` on comment line in _Python_).
- Documentation comments - start with `///`
    - This is used to create HTML documentation from the code using the `rustdoc` tool OR `cargo doc` command if it is a crate (the latter just runs `rustdoc` behind the scene). 
    - Documentation comments support _Markdown_ notation for formatting.

    ```rust
    // This is a single line comment
    // This is another single line comment

    /* This is a block
    - comment */

    /// ## Documentation Comments
    /// **This should generate some documentation**
    /// It supports _Markdown_ notation
    ```
    Executing the documentation tool generates HTMl documentation in the `/doc` directory.

### Variables
- #### Immutable variables
    Since _Rust_ is designed with concurrency in mind so variables are immutable by defualt. We can declare a variable using the `let` keyword.
    ```rust
    let x = 23;
    ```
    Now the _name_ `x` is bound to the _value_ `23`. If we try to change this the compiler will throw an error -
    ```
    let x = 23;
    x = 45; // this will fail at compile time
    ```
    ->
    ```console
    error[E0384]: cannot assign twice to immutable variable `x`
    --> variables.rs:8:5
    |
    7 |     let x = 22;
    |         - first assignment to `x`
    8 |     x = 45;
    |     ^^^^^^ cannot assign twice to immutable variable
    ```
- #### Mutable variables
    We can explicitly declare variables to be mutable when the need arises using the `let mut` syntax.
    ```rust
    let mut x = 21;
    x += 2; // increments x to 23
    ```

- #### Constants
    Constants like immutable variables are _names_ bound to _values_, however they are semantically different in the following ways - 
    - A constant can only be assigned a "_constant value_". I.e. it cannot be something that is computed at runtime. This is just like in `C/C++`.
    - Constants unlike variables can be assigned outside functions, and is often at _global_ or _module_ scope.
    - In _Rust_ the type of a constant has to be specified and cannot be left out to be inferred like in the case of variables.
    ```rust
    const UPPER_LIM: u32 = 100;
    let i = UPPER_LIM;
    ```
    
- #### Shadowing
    _Rust_ allows us to re-declare a variable within the same scope. This is not usually found in other statically typed languages. We can declare a variable and bind it to a vlaue -
    ```rust
    let x = 23;
    ```
    And within the same scope we can them do something like -
    ```rust
    let x = "hello";
    ```
    Now the variable `x` is bound to a new value (`hello`) and the old instance of `x` is _shadowed_. In reality, even though we use the same variable name `x` it is infact a different variable with a different memory location (unlike a `mut` variable where a new _value_ is assigne dto the same _address_).  
    We can claerly see how the memory address changes with shadowing - 
    ```rust
    let a = 2.3;
    println!("Address of a = {:p}", &a); 
    // address of variable named 'a' = 0x1dc7affba8

    let a = 19;
    println!("Address of a = {:p}", &a); 
    // address of a different variable named 'a' = 0x1dc7affc04

    let a = "alpha";
    println!("Address of a = {:p}", &a); 
    // address of yet another variable named 'a' = 0x1dc7affc58
    ```
    With a mutable variable we can see the same address after assighing a new value -
    ```rust
    let mut b = 23;
    println!("Address of b = {:p}", &b);
    // address of variable named 'b' = 0x5c74cffbc4

    b = 101;
    println!("Address of b after mutation = {:p}", &b);
    // address of variable named 'b' after changing value = 0x5c74cffbc4
    ```

### Basic Types
_Rust_ is a _statically typed_ language which implies that the _type_ of all variables should be known at compile time. This is in contrast to _dynamically typed_ languages such as _Python_ or _JavaScript_.  
The basic data types in _Rust_ are categorized as **Scalar types** and **Coumpound types**.

#### Scalar Types
A _Scalar Type_ represents a **single value**, and _Rust_ has four of these - **integers**, **floating-point numbers**, **boolean** and **characters**. This is similar to _C/C++_.

- **Integer**  
_Integer_ type represents number values without fractional component. The different integer types in _Rust_ based on size are -  

    |  Length  |  Signed  |  Unsigned |
    |----------|----------|-----------|
    |   8-bit  |    i8    |    u8     |
    |  16-bit  |   i16    |   u16     |
    |  32-bit  |   i32    |   u32     |
    |  64-bit  |   i64    |   u64     |
    | 128-bit  |   i128   |  u128     |
    |   arch   |   isize  |  usize    |

    These integer types differ in their bit-size, and each can have a _signed_ or _unsigned_ variant.  
    If we do not specify any type the default integer type is `i32`.  
    The `arch` type automatically applie the size depending on the processor architecture word-size - 32-bit or 64-bit.  
    The range of each integer type would be -
    - for signed:  `-2^(n-1)` to `2^(n-1) - 1`  
        so for `i32` we can have values from `-2,147,483,648` to `2,147,483,647`
    - for unsigned: `0` to `(2^n)-1`  
        so for `u32` we can have values from `0` to `4,294,967,295`
    
        **Overflow**  
        If we assign a value exceeding the range of a type _Rust_ will complain and throw a _"panic" / error_ while compiling if it is in _"debug"_ mode. However if it is in _"release"_ mode it will not complain at compile time and we can get unexpected behaviour at runtime, specifically the value will wrap around (_two's complement wrapping_).
        ```rust
        let mut a: u8 = 255;
        println!("'a' initial value = {0}", a);
        // 'a' initial value = 255
        a += 1;
        println!("'a' + 1 = {0}", a);
        // debug mode - thread 'main' panicked at 'attempt to add with overflow',
        // release mode - 'a' + 1 = 0 
        ```
        
        **Formats**  
        We can represent integer literals in different formats :
        |  Format   |  Example    |
        |-----------|-------------|
        |  Decimal  | 9_123_345   |
        |  Hex      | 0xff        |
        |  Octal    | 0o77        |
        |  Binary   | 0b1101_0001 |
        |  Byte(u8) | b'A'        |

- **Floating-Point**  
For numbers with decimal points _Rust_ has two _floating-point_ types - `f32` and `f64`. |The default in _Rust_ is `f64` because in modern processors 64 operations are nearly as fast as 32 bit and gives more precision.  
    ```rust
    let b = 3.3; // f64 - default floating point type
    let c: f32 = 5.0; // f32
    ```

    **Numeric Operations**  
    _Rust_ supports the standard numeric operations such as `+`, `-`, `/`, `%`.  
    Unlike some other languages _Rust_ is very strict in type checking and **will not do implicit casting** with numeric operations if we mix up _float_ and _integer_ types.  

- **Boolean**  
A `bool` type can take either of two values `true` or `false`. It takes _1 Byte_ in memory.
    ```rust
    let f = true;
    println!("Opposite of {0} is {1}", f, !f);
    //Opposite of true is false
    ```

- **Character**  
_Rust_ `char` data type allows us to represent characters. They are four bytes in size and uses Unicode UTF-8 encoding. Their values can range from `U+0000` to `U+D7FF` and `U+E000` to `U+10FFFF`. Like most other languages `char` literals are enclosed in single quotes ('').  
    ```rust
    let g = 'A';
    let h = '\u{41}';   // unicode value of 'A' in Hex
    let i = '\u{03A3}'; // unicode value of Greek Zigma in Hex
    println!("{0}, {1}, {2}", g, h, i); 
    //A, A, Σ
    ```
    Since `char` is 4 byte data type they are different from the way we are used to dealing with them in ASCII and we shall examine later about how _String stores UTF-8 encoded text_.

#### Compound Types
A _Compound Type_ can contain/group multiple values into one. _Rust_ has two primitive _compound types_ - **Tuples** and **Arrays**

- **Tuples**  
A _tuple_ is a grouping together of multiple values into one collection. It can group together values of _different types_. Once declared it is fixed in size and cannot grow. The values can be changed if it is declared mutable though.  
_Tuple_ literals are decalred by enclosing the values in prantheses separated by commas - 
    ```rust
    let rec = ("Alan", 1001, 75.3);
    ```
    If we wish to decalre the types of the vlaues within the declaration we can do so by specifying them in parantheses as a type for the variable - 
    ```rust
    let rec: (&'static str, i32, f32) = ("Alan", 1001, 75.3);
    ```
    _Note: the `&'static str` stands for a `slice` of `str` type with `ststic` lifetime, i.e. `string literal`._
    
    To access the elements of a _tuple_ we can use either the _dot syntax (.\<index>)_ OR _destructuring_.
    ```rust
    let mut rec = ("Alan", 1001, 78.6);
    // accessing the elements using .<index> and changing it
    rec.1 = 1002;
    println!("Record is {:?}", rec);
    // Record is ("Alan", 1002, 78.6)

    // accessing using destructuring
    let (name, _ , weight) = rec;
    println!("{0} weighs {1} Kg!", name, weight);
    // Alan weighs 78.6 Kg!
    ```
    _Note: `'_'` is a placeholder for values we don't care for while destructuring. This is same as in most other programming languages._

- **Arrays**  
_Arrays_ in _Rust_ are homegenous collection of items (i.e. the values are of the same type). However unlike many other languages, _arrays_ in _Rust_ are **fixed size**, so they cannot grow or shrink.  
Arrays are declared by enclosing the values in square brackets seperated by commas - 
    ```rust
    let scores = [87, 67, 48, 56, 73];
    ```
    They can have their type specified -
    ```rust
    let vow: [char; 5] = ['a', 'e', 'i', 'o', 'u'];
    // note type declaration uses ';'
    ```
    There is a shorthand for initializing an array if we want it to all have the same value -
    ```rust
    let base = [0; 5]
    // SAME AS - let base = [0, 0, 0, 0, 0]
    ```
    Accessing elements of an array uses the standard syntax of `'<array_name>[<index>]'` -
    ```rust
    let mut scores = [87, 67, 48, 56, 73];
    // access the first element & modify it (mut array)
    scores[0] = 92;
    // using len() method to get the length
    scores[scores.len() - 1] = 78; 
    print!("New scores are {:?}", scores);
    // New scores are [92, 67, 48, 56, 78]
    ```
    If we attempt to access and element outside the range of the array, _Rust_ will throw an _'index out of bounds..'_ error at runtime.

The difference between a **tuple** and an **array** is the intented purpose for each. A **tuple** is menat to be used as a coumpond type for passing a set set of values around - as arguments to functions or return them. Whereas an **array* is used as an iterable collection of values.
```rust
// array of scores
let scores = [87, 67, 48, 56, 73];
let mut sum = 0;
// using 'for' to iterate over the array
for i in scores.iter(){
    sum += i;
}
println!("Total score = {0}", sum);
```

### Functions

### Control Flow