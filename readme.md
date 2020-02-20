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
`cargo` provides a way to **check** if everything is alright with our code and is good enough for _build_, without _actually creating executables_. To do this use the `check` command -

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
    Since _Rust_ is designed with concurrency in mind so variables are immutable by default. We can declare a variable using the `let` keyword.
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
    _Rust_ allows us to re-declare a variable within the same scope. This is not usually found in other statically typed languages. We can declare a variable and bind it to a value -
    ```rust
    let x = 23;
    ```
    And within the same scope we can them do something like -
    ```rust
    let x = "hello";
    ```
    Now the variable `x` is bound to a new value (`hello`) and the old instance of `x` is _shadowed_. In reality, even though we use the same variable name `x` it is in-fact a different variable with a different memory location (unlike a `mut` variable where a new _value_ is assigned to the same _address_).  
    We can clearly see how the memory address changes with shadowing - 
    
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
    With a mutable variable we can see the same address after assigning a new value -
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
The basic data types in _Rust_ are categorised as **Scalar types** and **Compound types**.

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
    The `arch` type automatically applies the size depending on the processor architecture word-size - 32-bit or 64-bit.  
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
  _Tuple_ literals are declared by enclosing the values in parentheses separated by commas - 

    ```rust
    let rec = ("Alan", 1001, 75.3);
    ```
    If we wish to decalre the types of the vlaues within the declaration we can do so by specifying them in parentheses as a type for the variable - 
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
_Arrays_ in _Rust_ are homogeneous collection of items (i.e. the values are of the same type). However unlike many other languages, _arrays_ in _Rust_ are **fixed size**, so they cannot grow or shrink.  
Arrays are declared by enclosing the values in square brackets separated by commas - 
  
    ```rust
    let scores = [87, 67, 48, 56, 73];
    ```
    They can have their type specified -
    ```rust
    let vow: [char; 5] = ['a', 'e', 'i', 'o', 'u'];
    // note type declaration uses ';'
    ```
    There is a shorthand for initialising an array if we want it to all have the same value -
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

The difference between a **tuple** and an **array** is the intended purpose for each. A **tuple** is menat to be used as a coumpond type for passing a set set of values around - as arguments to functions or return them. Whereas an **array* is used as an iterable collection of values.
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
In _Rust_ functions are a fundamental unit of code. Execution starts with the special _entry point_ function `main`. Unlike many other object-oriented languages we do not need to wrap the function in a `class` or `object`.  
We declare a function using the `fn` keyword followed by the _function name_, then _parameters_ within parentheses, an optional _return type_ and finally the _body_ of the function within curly braces -

  ```rust
    // function with no parameters
    fn say_hello(){
        println!("Hola!");
    }

    // function with parameters
    fn say_hello_to(name: &str){
        // paramter type is 'str slice'
        println!("Hello {0}!", name);
    }

    // function with params & return type
    // param type = slice of array
    // return type = i32
    fn get_mean(nums: &[i32]) -> i32{
        let mut sum = 0;
        for i in nums.iter(){
            sum += i;
        }
        return sum / nums.len() as i32;
    }
  ```
_NOTE: The convention in Rust is to use `snake_case` (smaller case words separated by '\_') for function names._  
_Rust_ function names have to be unique within its scope as it does not support _function overloading_ directly.  
In the last example we could have the last line of the function as -

```rust
fn get_mean(nums: &[i32]) -> i32{
    ...
    sum / nums.len() as i32
    // Note: last line is an expression (no ;)
}
```
In _Rust_ the last expression in the function body is implicitly treated as the return value. We do not explicitly need to use the `return` keyword unless we wish to return early.

#### Statements vs Expressions
A set of instructions that can be used as the RHS of an assignment is an _Expression_, it implicitly evaluates to some value.  
A _Statement_ on the other hand is simply a set of instructions and cannot be treated like a value.  
This is a familiar concept from many other languages (such as - Ruby, Python, C#).  

```rust
    let root = {
        let r = rslt as f32;
        r.sqrt() // Note: no ;
    };
    //  'r.sqrt()' gets assigned to 'root'
    println!("Root = {0}", root);
}
```
In _Rust_ the last line of an expression should NOT have a semicolon (;), whereas statements should always end with semicolon.

### Control Flow
The basic control flow constructs are _conditional branching_ using the `if .. else` expression and _looping_ commands such as `loop`, `while` and `for`.    
- **Conditional Flow (`if..else`)**  
    The `if..else` conditional flow construct is like most other languages -
    ```rust
    if root > 5.0 {
            println!("{0} is too Big!", root);
    }
    else if root > 3.0 {
            println!("{0} seems alright.", root);
    }
    else {
            println!("{0} is too Low!", root);
    }
    ```
    In _Rust_ `if..else` is an _expression_, so we can use it for conditional assignment (lie we would use the _ternary operator_ in _C_) - 
    ```rust
    let comment = if root > 5.0 {
        "Too Big!" // Note - NO ;
    }
    else if root > 3.0 {
        "Seems alright." // Note - NO ;
    }
    else {
        "Too Low!" // Note - NO ;
    };
    // Note - expressin value should not end in semicolon
    println!("{}", comment);
    ```
- **Looping (`loop`)**  
    The `loop` keyword tells _Rust_ to keep repeating a block of code again and again (unless something causes it to `break` out of it).
    ```rust
    let mut i = 1;
    loop{
        print!("{},", i);
        i += 1;
        if i > 10 {
            break;
        }
    };
    // repeats the above block 10 times
    // 1,2,3,4,5,6,7,8,9,10,
    ```
    The `loop` construct can be used as an expression. It returns the value (or expression) after the `break` keyword -
    ```rust
    //loop as an expression
    let (mut i, mut sum) = (0, 0);
    let rslt = loop{
        i += 1;
        sum += i * i;
        if i > 10{
            break sum; // 'sum' is returned from the loop
        }
    };
    // 'rslt' will now have the value of 'sum'
    println!("{}", rslt);
    // 506
    ```
- **Conditional Looping (`while`)**  
    For looping based on a condition the normal construct to use is the `while` loop. This is a basic looping construct found in almost all programming languages and is the same in _Rust_ -
    
    ```rust
    let mut j = 1;
    let mut fact = 1;
    while j <= 3 {
        // executes the body as long as 
        // the condition above is true
        fact *= j;
        j += 1;
    }
    println!("Factorial = {0}", fact);
    ```
- **Looping over Iterables (`for .. in`)**  
    `for .. in` is the construct in _Rust_ that implements iteration over iterable collections -
    
    ```rust
    let scores = [67, 84, 47, 56, 78];
    let mut mean_score = 0;
    for s in &scores{ // use 'slice` or '.iter()'
        mean_score += s;
    }
    mean_score /= scores.len();
    println!("Mean score = {0}", mean_score);
    // Mean score = 66
    ```
    This approach is preferred over using `while` loops for iterating collections as it uses the collections own (intended) iteration behaviour and we do not concern with bounds and indexing etc.  
    Even for repeating code a certain number of times the idiomatic way in _Rust_ is to use `for .. in` with a _Range_ -
    
    ```rust
    let mut fact = 1;
    for i in 1..10{ // using range 1..10
        fact *= i;
    }
    println!("Factorial = {0}", fact);
    // Factorial = 362880
    ```
### "Ownership" in Rust
_Rust_ provides memory management without a _garbage collector_ (unlike .Net or Java). Its **"ownership"** mechanism enables it to be deterministic about variable lifetimes and therefore make **"memory safety guarantees"**.

- **Ownership**  
  All language runtime have to manage the memory they use at runtime, and they take different approaches to do this.  

    - Some of them rely on _generational garbage collection_ (_GC_) that periodically looks for and cleans up unused memory - _'.Net'_, _'Java'_ for example.  
    - Another approach is _automatic reference counting_ (_ARC_) that tries to keep track of all the references to an allocation and removes it when that count reaches zero - this is the approach taken by _'Swift'_  
    - Yet other languages put the onus on the programmer to allocate and free up memory - this is the model in _'C'_.  

    _Rust_ does not take dynamic memory deallocation approach like _GC_ or _ARC_, it is actually "similar" to the _manual_ model of _C_ that have code to explicitly deallocate heap objects. The difference is that in _Rust_ the compiler is capable of injecting that deallocation into the code at compile time. This is possible because its strict "ownership" model, which makes the lifetime of objects in memory deterministic at compile time.  
    _Rust_ keeps track of which part of code is using what data on the _Heap_, minimising the amount of duplicate data on the _Heap_, and cleaning up unused data.  It takes a similar approach to _Heap_ memory as it does with _Stack_, in that as soon as the _"Owner"_ of the memory goes out of scope it can claim the memory back.

- **Rules of "Ownership"**  
"Ownership" in _Rust_ is based on 3 simple rules -
    - Each **value** has a variable that is called its **owner**
    - There can only be **one owner** at a time
    - When the **owner** goes out of **scope** the **value** will be **dropped**

    _Note: Rust is "block scoped"_  -  
    ```rust
    {
        let x = "Hello"; // x scope starts
        ... // x is valid here
    } // x goes out of scope here
    ```
    In order to illustrate the behaviour of _ownership_ we need a more complex data type than the simple _scalar_ types as they are allocated on the **stack** since they have limited ad predetermined size. _string literals_ will not do either as they are compiled into the binary and have known fixed size.  
    We need a data type that needs _allocation_ on the **heap** such as the **String** type. Unlike _string literals_ the `String` types is able to store a varying amount of text (i.e. it can grow or shrink). Whilst there is a lot to be said of `String`, for our purposes now we shall limit ourselves to aspects that are relevant to memory management.
  
  Two convenient ways to create a string is using its static methods `new` and `from`, as shown below -
  
  ```rust
  let x = String::new(); // empty String
  println!("Value of x = {}", x);
  // Value of x = 
  
  let y = String::from("Hola"); // from string literal
  println!("Value of y = {}", y);
  // Value of y = Hola
  ```
  
  `Srtring` gives us methods to modify the text in place -
  
  ```rust
  let mut y = String::from("Hola");
  println!("Value of y = {}", y);
  // Value of y = Hola
  
  y.push_str(" amigo!");
  println!("Value of y = {}", y);
  // Value of y = Hola amigo!
  ```
  
  _Note: How we now have to make the variable `y` 'mutable'_
  
  When we need to allocate memory on the _Heap_ we must request it from the **OS**, and we do that when we call `String::from`. When the variable that _owns_ the data in this memory (the variable `y` in our case) goes out of scope _Rust_ injects a method call to a method called `drop`. This is similar to `destructor` in _C++_, the writer of `String` can have cleanup code here. _Rust_ will ensure this method gets called automatically when "ownership" ends.
  
  _Note: In C++ this pattern of cleaning up resources at the end of an object's lifetime is called **RAII** or 'Resource Acquisition Is Initialisation'_.  
  
- **Variable assignment - "Move or Copy"**  
  We saw that when the variable that is bound to the data goes out of scope the memory for that data is freed. What happens if there are more than one variable bound to the data? Let us see how this might work with the following setup -

  - Assign an `i32` to a variable `a`
  - Re-assign the data to another variable `b`
  - Pass 'b' to a custom function `show` to print it to _std I/O_
    - _there is a reason why we are using this function. It helps simulate ownership transfer situation, as we shall see as we proceed._
  - Finally call `show` on the original variable `a` again

  ```rust
  // "copy" semantics
  fn main() {
      let a: i32 = 23;
      // assign 'a' to 'b'
      let b = a;
      show(b);    // 23
      // now show 'a'
      show(a);    // 23
  }
  // custom function to print value
  fn show<A: std::fmt::Display>(p: A){
      println!("{}", p);
  }
```
  
So far so good, now let us try to do this with a `String` data type instead of `i32`
  
  ```rust
  // "move" semantics
  fn main() {
      let a: String = String::from("hello");
      // assign 'a' to 'b'
      let b = a;
      show(b);    // hello
      
      // now show 'a'
      show(a);    // Error!
  }
  /*
  error[E0382]: use of moved value: `a`
   --> src/main.rs:8:10
    |
  2 |     let a: String = String::from("hello");
    |         - move occurs because `a` has type `std::string::String`, which does not implement the `Copy` trait
  3 |     // assign 'a' to 'b'
  4 |     let b = a;
    |             - value moved here
  ...
  8 |     show(a);    // 23
    |          ^ value used here after move
*/
  ```

  We get an error, the compiler is panics and and is trying to tell us that we are trying to use the variable `a` after the data has been **moved**, and the "move" occurred at `let b = a`. It goes on to explain that the "move" happened because `String` does not implement the `Copy` trait.

  So as we can see the compiler error is quite descriptive and goes it great detail explaining the scenario. This is one of _Rust's_ philosophy and even though it is quite strict it can be a helpful friend.

  - So what happened here is that with `String` (unlike the `i32` value) since the data is allocated on the _Heap_ when a new variable points to the same data, the ownership gets transferred to the new variable and the old variable is no longer associated with the data value and when we attempt to use the invalidated variable _Rust_ will trow an error.

  - Since there is only one owner (`b` in this case) _Rust_ can make a deterministic `free` of the data it points to in memory when `b` goes out of scope (or end of its lifetime).
  
  - In the case of the `i32` it was a simple data type with a fixed size known at compile time and therefore their data is placed on the _Stack_. When we assign it to a new variable _Rust_ makes a **copy** of that data in memory and binds the new variable with that, leaving the old data and variable in tact. When the function call associated with the _Stack_ is complete the _Stack_ frame is removed with all the data in it.
  - With dynamic data such as `String` we cannot use the _Stack_, they are allocated on the _Heap_. Allocation and deallocation on the _Heap_ is expensive and can become a significant performance bottleneck. _Rust_ does not implicitly create _deep copies_. 
  - So the approach _Rust_ has taken to memory management on the _Heap_ is to **move** ownership whenever some data on the _Heap_ is assigned to another variable or passed to some function as an argument (in the later case the function parameter gets the **ownership**).
  - The mechanism that is used the language level is the `Copy` **trait** (a _trait_ is like an interface, we shall learn more about them later). Simple scalar types such as Integer, Float, Double, Boolean, Character and Tuples of these can be placed on the _Stack_, and they have the `Copy` trait. That way when another variable/parameter needs them it gets a copy of the data (similar to _"Pass by Value"_ in _C++_). We can have our custom types implement types implement the `Copy` trait, and then _Rust_ will treat it with the **copy semantics** just like it does for the builtin scalar types. However if our type has the `Drop` trait, then it will not allow us to implement the `Copy` trait on that type. As we have seen `drop` is for cleanup behaviour when the owner goes out of scope. It does not make sense to have both traits.
  - 
  
- **Variable assignment - "Clone"**  
  If we do want to create a deep copy of the data on the _Heap_, we can do that for objects that support the `clone` method.

```rust
fn main() {
    let a: String = String::from("hello");
    // clone 'a' to 'b'
    let b = a.clone();
    show(b);    // hello
    
    // now show 'a'
    show(a);    // hello
}
```

Here `b` gets a **cloned** copy of the string `hello` and it leaves `a` intact with individual ownership to the separate data copies in memory. 

- **Function calls and Ownership**  
  **Ownership** and memory management comes into concern not only during _variable assignment_, the same semantics apply when we invoke functions passing the data around.

  - **Passing data to a Function**

    Passing a variable to a function call as an argument will result in an **ownership move or copy**, depending on the data type (just like assigning to a variable). In this case it is technically an assignment to the function's formal parameter. We can demonstrate that in our code snippet with a small rearrangement -

    ```rust
    fn main() {
        let a: i32 = 23;
        
        // call a function with 'a' as argument
        show(a);    // hello
        
        // now assign 'a' to 'b'
        let b = a; // 23
    }
    ```

    Passing an `i32` as argument to a function results in a **copy**.

    ```rust
    fn main() {
        let a: String = String::from("hello");
        
        // calls a function with 'a' as argument
        show(a);    // hello
        
        // now assign 'a' to 'b'
        let b = a; // Error!
    }
    /*
    error[E0382]: use of moved value: `a`
     --> src/main.rs:8:13
      |
    2 |     let a: String = String::from("hello");
      |         - move occurs because `a` has type `std::string::String`, which does not implement the `Copy` trait
    ...
    5 |     show(a);    // hello
      |          - value moved here
    ...
    8 |     let b = a; // Error!
      |             ^ value used here after move
    */
    ```

    Passing a `String` (_Heap_) data as argument to a function, and then trying to use that same variable again results in the same `E0382` error. This is because, with the function call the **ownership** of the `String` data has **moved** to the function's parameter (`p: A` in our case).

  - **Returning data from a Function**

    Returning a value from a function can also transfer **ownership**. If the return value from a function is assigned to a variable, then that variable gets the **ownership** of that data. The lifetime of the data will now be extended beyond the scope of the function. Return can be used to take back **ownership** of data passed to a function. We can modify our previous code snippet to work correctly using this technique-

    ```rust
    fn main() {
        // make the variable mutable
        let mut a: String = String::from("hello");
        
        // calls a function with 'a' as argument
        // and assign teh return back to the original variable
        a = show(a);    // hello
        
        // now assign 'a' to 'b'
        let b = a; // now it works!
    }
    
    fn show<A: std::fmt::Display>(p: A) -> A{
        println!("{}", p);
        p // returning the parameter value
    }
    ```

    Now when we use the variable `a` after passing it to the `show` function it works without giving an error. This is because in this case the function `show` returns the value it received as parameter back to the caller which gets assigned again to the same variable (`a`). However to make this work we had to do quite a bit of jugglery -

    - The variable `a` has to be made _mutable_ with the `mut` keyword, so that it can be reassigned with the return from the function `show`.
    - We have to make the function invocation into an expression on the _RHS_ of an assignment.
    - The function `show` itself has to be modified to have a return type (`-> A`).
    - The value of the parameter `p` has to to be returned from the function (as the last line of the function without a `;`)

    Setting up the code in this way to take back ownership can get complicated, neither is it always practical. Next we shall see better way to do this **References** and **Borrowing**.

- **References**
- **Slices**

