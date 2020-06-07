# Rust Programming - Overview
_Rust_ is a `multi paradigm systems programming language` developed at _Mozilla_ Research and open sourced under _MIT License_ and _Apache License 2.0_.  
It was originally designed by _Graydon Hoare_ with contributions added on by others including _Brendan Eich_.

It is a language designed with **safety**, **speed**, and **concurrency** as primary importance. It is a _low level language_ and doesn't use _Garbage Collection_, however it brings together different programming paradigms such as `procedural`, `object-oriented`, `concurrent actor`, `functional`, and `meta-programming`.

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
- `rustup` - **installation** and **version management** tool
  
    - _Rust_ has a rapid development and 6 week release cycle, so this tool is essential to manage the versions
    - To update _Rust_ simply d
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
`main` can take parameters or be empty.

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
This is essentially the project manifest with configuration information about the project and its dependencies, like `package.json` in _NPM_. Unlike _NPM_ though _Cargo_ uses the `TOML` (Tom's Obvious, Minimal Language - By Tom Preston-Werner) syntax instead of `JSON`. This kind of resembles `INI` files from back in the days, and is essentially a structured list of Key-Value pairs. Our `toml` file would look like - 
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

By default it `cargo` will build for _debug_ mode, which means it has debug symbol files and is not optimised. When the project is ready for **release** we can build with the `--release` switch/option and this will do an optimised build for release environments. The compilation/build time for **release** mode takes longer but the binary is optimised to execute faster. The **release** build would end up in the `./target/release` directory.

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
    ```rust
    let x = 23;
    x = 45; // this will fail at compile time
    ```
    
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
    If we wish to declare the types of the values within the declaration we can do so by specifying them in parentheses as a type for the variable - 
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
    There is a shorthand for initialising an array if we want it to all have the same value 
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

    The difference between a **tuple** and an **array** is the intended purpose for each. A **tuple** is meant to be used as a compound type for passing a set set of values around - as arguments to functions or return them. Whereas an **array** is used as an iterable collection of values.
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
## "Ownership" in Rust
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

    _Note: Rust is "block scoped"_ -
    
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
  
    So far so good, now let us try to do this with a `String` data type instead of `i32` - 
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

    The problem with passing our `String` variable as an argument to the `show` function is that when we do that the function parameter `p` gets **ownership** of the data value pointed by the variable. After that if we try to use the 	original variable _Rust_ will complain. So we need a mechanism to pass/assign a variable without transferring **ownership**. This is what **references** allow us to do. We represent a **reference** by prefixing the variable/argument with **`&`**. Let us modify our code listing to work with a **reference** and see how it works -

    ```rust
    fn main() {
        let a: String = String::from("hello");
        
        // borrow a reference to 'a'
        show(&a);    // hello
        
        // now assign 'a' to 'b'
        let b = a; // 'a' is still available
    }
    
    fn show<A: std::fmt::Display>(p: &A){
        // 'p' has a reference to the value
        // but no ownership
        println!("{}", p);
    }
    ```

    The changes we made are:

    - When we invoke the function we pass in a **reference** to the variable instead of the variable (`show(&a)`)
    - The function definition itself has a modification in the parameter type, the parameter is defined as a **reference** type (`p: &A`)

    With this change the original variable `a` still retains the **ownership** while the parameter `p` can get a reference to the value that it can use as `read-only`.

    A **reference** unlike **raw pointer** enforces certain rules about referencing/borrowing that help avoid inadvertent mutations to the value. 

    To see this let us try to setup a function that tries to modify our string using a **borrow** - 

    ```rust
    fn main() {
        // declare and init a String variable 
        let a = String::from("Alpha");
        
        // pass 'a' as reference to the function
        change_str(&a); // Error
        
        println!("{}", a); 
        
        
    }
    
    // specify the parameter as reference to String
    fn change_str(text: &String){
        text.push_str(" Beta!");
    }
    
    /*
    Compiling playground v0.0.1 (/playground)
    error[E0596]: cannot borrow `*text` as mutable, as it is behind a `&` reference
      --> src/main.rs:15:5
       |
    14 | fn change_str(text: &String){
       |                     ------- help: consider changing this to be a mutable reference: `&mut std::string::String`
    15 |     text.push_str(" Beta!");
       |     ^^^^ `text` is a `&` reference, so the data it refers to cannot be borrowed as mutable
    */
    ```

    The compiler complains that the data cannot be borrowed as **"mutable"** because it is behind a **normal reference**.

    If we need to modify a value it is possible to get a **mutable reference**   by prefixing it with the **`mut`** keyword, as shown -

    ```rust
    fn main() {
        // declare the variable as mutable
        let mut a = String::from("Alpha");
        
        // borrow 'a' as a 'mutable reference' (&mut)
        change_str(&mut a);
        
        println!("{}", a); // Alpha Beta!
        
        
    }
    
    // specify the parameter type as 'mutable reference' (&mut)
    fn change_str(text: &mut String){
        text.push_str(" Beta!");
    }
    ```

    Now we are able to change the data via the **mutable reference**, and in order to achieve that we had to do a few things explicitly -

    	- Declare the variable `a` as mutable
    	- Borrow as **mutable reference** using `&mut` when calling the function
    	- Define the function parameter as **mutable reference** (`&mut String`)

    As we can see trying to do state mutation requires us to make it explicit in the code and at all points that might impact it.

    Even though we can obtain **mutable references** there are certain restrictions imposed that prevent us from resulting in **concurrency errors** (such as **race conditions** and **dead locks**).

    - One restriction is that we can have only one active **mutable reference** at a time in the same scope. For example we could try to get two mutable references as below -

      ```rust
      fn main() {
          // declare and init a String variable 
          let mut a = String::from("Alpha");
          
          // get two mutable references
          let r1 = &mut a;
          let r2 = &mut a;   
      }
      ```

      So far it seems _Rust_ has allowed us to get two **mutable references**, however if we modify the code slightly to try and use the earlier **reference** we will get a compile time error -

      ```rust
      fn main() {
          // declare and init a String variable 
          let mut a = String::from("Alpha");
          
          // get two mutable references
          let r1 = &mut a;
          let r2 = &mut a;
          
          // Try to print (use) 1st refernce
          println!("r1 = {}", r1); // Error!
      }
      /*
      error[E0499]: cannot borrow `a` as mutable more than once at a time
        --> src/main.rs:7:14
         |
      6  |     let r1 = &mut a;
         |              ------ first mutable borrow occurs here
      7  |     let r2 = &mut a;
         |              ^^^^^^ second mutable borrow occurs here
      ...
      10 |     println!("r1 = {}", r1); // Error!
         |                         -- first borrow later used here
      */
      ```

      This way we cannot end up inadvertently modifying the same data from two places.

    - Another restriction is that we cannot have a **mutable reference** while holding active **immutable references**. This make sense as the code that uses the **immutable references** would not expect the data to be modified inadvertently, which could happen if you can have active **mutable reference** at the same time. We can see that _Rust_ will give an error if we try -

      ```rust
      fn main() {
          // declare and init a String variable 
          let mut a = String::from("Alpha");
          
          // get an immutable reference 
          let r1 = &a;
          
          // get a mutable reference 
          let r2 = &mut a;
          
          // Try to print (use) 1st refernce
          println!("r1 = {}", r1); // Error!
      }
      /*
      error[E0502]: cannot borrow `a` as mutable because it is also borrowed as immutable
        --> src/main.rs:9:14
         |
      6  |     let r1 = &a;
         |              -- immutable borrow occurs here
      ...
      9  |     let r2 = &mut a;
         |              ^^^^^^ mutable borrow occurs here
      ...
      12 |     println!("r1 = {}", r1); // Error!
         |                         -- immutable borrow later used here
      */
      ```

    - The third restriction is that **references must be valid**. In languages like _C/C++_ which allow manipulating raw pointers, it is possible to have **dangling pointers**, or pointers to deallocated or invalid memory locations. _Rust_ on the other hand ensure that it does not end up with **dangling pointers/references**, by making sure that the **data** will not go out of scope before the **reference** goes out of scope. It is a simple but effective approach.

      We can try to simulate this with a function that creates a string and returns a **reference** to it -

      ```rust
      fn main() {
      
          let x = dangling_ref();
          
          // Try to print 'x'
          println!("x = {}", x); // Error!
      }
      
      
      fn dangling_ref() -> &String{
          // initialize a local string data
          let s = String::from("Hello");
          // return reference to it
          &s
      }
      /*
      rror[E0106]: missing lifetime specifier
        --> src/main.rs:10:22
         |
      10 | fn dangling_ref() -> &String{
         |                      ^ help: consider giving it a 'static lifetime: `&'static`
         |
         = help: this function's return type contains a borrowed value, but there is no value for it to be borrowed from
      */
      ```

      The function returns a **borrowed reference** to `s` which is a local variable, bu that goes out of scope once the function is over, so there is nothing to **refer** to for the returned **reference**. _Rust_ preemptively stops us from doing such things.

      If we did want to return a value from the function we can return it as a **value with ownership**, in which case the **ownership** gets transferred to the variable outside the function -

      ```rust
      fn main() {
      
          let x = not_dangling_ref();
          
          // Try to print 'x'
          println!("x = {}", x); // x = Hello
      }
      
      
      fn not_dangling_ref() -> String{
          // initialize a local string data
          let s = String::from("Hello");
          // return value with ownership
          s
      }
      ```

      Now `x` will have **ownership** of the `String` data `Hello`.

    To summarise the **rules of reference** are:

    - At a given time we can have only **one mutable reference** or any number of **immutable references**. We cannot have more that one **mutable reference** or combine **immutable references** with **mutable** ones.
    - A **reference** must always point to **valid data**

- **Slices**

    Another form of reference/data type that does NOT have ownership is **slices**. They come into picture when working with collections (especially strings). A **slice** is a reference to a **contiguous range** within a **collection** rather than the whole collection. So whilst a normal _string reference_ (`&String`) is a **reference** to the whole `String` data, a _string slice_ (`&str`) is a **reference** to a specific range of the string data. We can see a few examples of how to specify slices, and how the _range start and end specifiers_ behave (it is very similar to other languages such as `Python`).
    
    ```rust
    fn main() {
    
        let s = String::from("Hello World!");
        
        // string slice from 0 to 5 chars
        let s1 = &s[0..5];
        println!("{:?}", s1);// Hello
        
        // starting from 0 can omit 0
        let s2 = &s[..5];
        println!("{:?}", s2);// Hello
        
        // ending in len can omit the range-end
        let s3 = &s[6..];
        println!("{:?}", s3);// World!
    
        // range-end specifier is end-index + 1
        // so to get 6th char to 3 chars after 6th
        let s4 = &s[6..6+3]; // &s[6..9]
        println!("{:?}", s4);// Wor
    }
    ```
    
    Typically when we pass _strings_ or _vectors_ around (or return them from functions), we tend to use **slices**. This way we do not transfer **ownership** and also we get a more _flexible reference_ to the underlying data. Let us look at an example that can return the first word of a string text -
    
    ```rust
    fn main() {
    
        let s = String::from("Hello World!");
        
        let f = first_word(&s);
        println!("first word in {} = '{}'", s, f);
        // first word in Hello World! = 'Hello'
    
    }
    
    fn first_word(text: &String) -> &str{ // returns string slice
       // access underlying String bytes
       let bytes = text.as_bytes();
       // iterate through the bytes and check for space
       // destructure iterator item to index=i and ref to item=&byt
       for (i, &byt) in bytes.iter().enumerate(){
           if byt == b' ' {
            // return slice till 'i'
               return &text[..i]
           }
       }
       
       // if reached here return full string as slice
       &text[..]
    }
    ```
    
    Our function `first_word` returns a **slice** of the `String` passed in. Under the hood this is just a _memory safe pointer_ to part of the `String` data. They follow the same rules of **borrowing** as regular references. So for example if in the above example we tried to modify the original `String` _Rust_ will prevent us -
    
    ```rust
    fn main() {
    
        let mut s = String::from("Hello World!");
        
        let f = first_word(&s);
        
        // try clearing the string here
        s.clear(); // Error - cannot do mutable borrow
        
        println!("first word in {} = '{}'", s, f);
        // first word in Hello World! = 'Hello'
    
    }
    /*
    error[E0502]: cannot borrow `s` as mutable because it is also borrowed as immutable
     --> src/main.rs:7:5
      |
    5 |     let f = first_word(&s);
      |                        -- immutable borrow occurs here
    6 |     
    7 |     s.clear();
      |     ^^^^^^^^^ mutable borrow occurs here
    8 |     
    9 |     println!("first word in {} = '{}'", s, f);
      |                                            - immutable borrow later used here
    */
    ```
    
    Here we tried to clear(empty) the `String` '`s`' after we returned it to the variable '`f`' (which is an immutable **slice** to the `String` data '`s`'). Further more we are using this **immutable** borrow **slice** '`f`' in the `println!` macro later. If this is allowed, then _Rust_ cannot ensure data integrity (or memory safety). If we are not using '`f`' (the **slice**)  then it is fine and we are allowed to mutate the original `String 's'`.
    
    ```rust
    fn main() {
    
        let mut s = String::from("Hello World!");
        
        let f = first_word(&s);
        
        // try clearing the string here
        s.clear(); // this works now
        
        // we are not refering to 'f' later
        
        println!("now the String is '{}'", s); 
        // now the String is ''
    }
    ```
    
    Though _Rust_ will give a _warning_ that the "variable 'f' is unused".
    
    **Mutable Slices >>** 
    
    Like regular **references** , **slices** can also be mutable. In fact we can use a **mutable slice** to examine how it relates to the original collection. If we modify our code example to return a **mutable slice**, we can see that modifying the **slice** modifies the **original String** -
    
    ```rust
    fn main() {
    
        let mut s = String::from("Hello World!");
        
        let f = first_word(&mut s);
        println!("The slice 'f' is '{}'", f);
        //The slice 'f' is 'Hello'
        
        // then modify the mutable slice to be uppercase
        f.make_ascii_uppercase();
        
        println!("Now the original String is '{}'", s);
        // Now the original String is 'HELLO World!'
    
    }
    
    fn first_word(text: &mut String) -> &mut str{ // returns mutable string slice
       // access underlying String bytes
       let bytes = text.as_bytes();
       // iterate through the bytes and check for space
       // destructure iterator item to index=i and ref to item=&byt
       for (i, &byt) in bytes.iter().enumerate(){
           if byt == b' ' {
            // return slice till 'i'
               return &mut text[..i]
           }
       }
       
       // if reached here return full string as slice
       &mut text[..]
    }
    ```
    
    Of course everything is still consistent with the rules of **borrowing**, for example if we tried to do another **mutable borrow** in the same scope we would get an error-
    
    ```rust
    fn main() {
    
        let mut s = String::from("Hello World!");
        
        let f = first_word(&mut s);
        println!("The slice 'f' is '{}'", f);
        //The slice 'f' is 'Hello'
        
        // try to get another mutabel slice
        let g = first_word(&mut s); // Error!
        
        // then modify the mutable slice to be uppercase
        f.make_ascii_uppercase();
        
        println!("Now the original String is '{}'", s);
        // Now the original String is 'HELLO World!'
    
    }
    /*
    error[E0499]: cannot borrow `s` as mutable more than once at a time
      --> src/main.rs:9:24
       |
    5  |     let f = first_word(&mut s);
       |                        ------ first mutable borrow occurs here
    ...
    9  |     let g = first_word(&mut s);
       |                        ^^^^^^ second mutable borrow occurs here
    ...
    12 |     f.make_ascii_uppercase();
       |     - first borrow later used here
    */
    ```
    
    So **slices** give us the flexibility to work with collections/string whilst ensuring memory safety and helping us avoid situations in code that can lead to data related inconsistencies. It is the idiomatic way to work with strings in _Rust_. 
    
    **String Literals >>**
    
    In fact **string literals** in _Rust_ are accessed as **slices**. For optimisation reasons _Rust_ embeds **string literals** into the binary an **immutable slice** reference to it within the code. 
    
    ```rust
    let a = "This is a string literal";
    
    let b = a;
    ```
    
    In the above code snippet the variable '`a`' is of the type `&str` (an **immutable string slice**) and when we assign that variable '`b`', it's type too will be `&str`. Now both the **slices** will point to the same embedded character sequence in the binary (which the underlying memory representation for string literals).
    
    This means that in general, experienced _Rust_ developers would use **string slice** instead of **String reference** as parameter/return types. This gives the flexibility to borrow from `String` or _string literals_. We would write our now familiar example as -
    
    ```rust
    fn main() {
    
        let s = String::from("Alpha Centauri");
        
        let f = first_word(&s);// takes String
        println!("{}", f);
        // Alpha
        
        let g = first_word("Beta Centauri"); // takes string literal
        println!("{}", g);
        // Beta
    }
    
    // accept &str
    fn first_word(text: &str) -> &str{ // returns string slice
       // access underlying String bytes
       let bytes = text.as_bytes();
       // iterate through the bytes and check for space
       // destructure iterator item to index=i and ref to item=&byt
       for (i, &byt) in bytes.iter().enumerate(){
           if byt == b' ' {
            // return slice till 'i'
               return &text[..i]
           }
       }
       
       // if reached here return full string as slice
       &text[..]
    }
    ```
    
    **Slices to other Data-Types >>**
    
    Whilst **slices** are very natural for `String` data, we can have **slices** to any data-type in general. If we wish to refer to/deal with part of an integer vector we could have a **slice** to that -
    
    ```rust
     // an array of numbers (i32)
    let nums = [3, 2, 4, 1, 6, 5, 9];
    
    // a slice of the i32 array or (&[i32])
    let sn = &nums[..3];
    println!("{:?}", sn); // [3, 2, 4]
    ```
    
    Here the type of the variable '`sn`' is **`&[i32]`**. This the general type representation for a **slice**. Even for a **string slice** the representation of **`&str`** is just syntactic sugar for **`&[string]`**.
    
    If we wanted to write a function to take an integer array and return the sum, we would do something like -
    
    ```rust
    fn main() {
        
        // an array of numbers (i32)
        let nums = [3, 2, 4, 1, 6, 5, 9];
        
        // sum of whole array
        let t1 = sum_nums(&nums[..]);
        println!("Sum = {}", t1); // Sum = 30
        
        // sum of half the array
        let t2 = sum_nums(&nums[..nums.len()/2]);
        println!("Sum = {}", t2); // Sum = 9
    }
    
    // fn accepts a slice of an array of i32
    fn sum_nums(ns: &[i32]) -> i32{
        let mut s = 0;
        for i in ns{
            s += i;
        }
        s
    }
    ```
    
    Since our function accepts a **slice** of `i32` array, we can pass in parts of the original array/vector. In the same way we can have **slice** to collections of any any other data-types (even user defined data types).
    
    With a good understanding of **memory handling**, **references**, **borrowing**, and **slices** we can try to write a nontrivial piece of code, by implementing an _in-place quicksort algorithm_  -
    
    ```rust
    fn main() {
        
        // an array of numbers (i32)
        let mut nums = [3, 2, 4, 1, 6, 5, 9];
        //let mut nums = [2, 4, 6, 3, 1, 5];
        //let mut nums = [2, 1];
        
        // call 'qsort' with a mutable reference
        qsort(&mut nums);
        println!("{:?}", nums);
        // sorted array 
    }
    
    // in-place quicksort, takes a mutable slice of i32
    fn qsort(ns: &mut [i32]){
        let l = ns.len();
        if l > 1{
            let mut pi = 0; // find pivot
            let mut si = pi + 1;
            while si < l {
                // scan from pivot and find smaller elements
                if ns[si] < ns[pi]{
                    let t = ns[si];
                    let mut j = si;
                    // shift from pivot right by 1
                    while j > pi{
                        ns[j] = ns[j-1];
                        j -= 1;
                    }
                    // swap smaller to pivot
                    ns[pi] = t;
                    pi += 1;
                }
                si += 1;
            }
            // recursive quicksort left and right of pivot
            qsort(&mut ns[..pi]); // qsort the left (smaller) side
            qsort(&mut ns[pi+1..]); // qsort the right (greater) side
        }
    }
    
    ```
    
    The algorithm itself is just a standard _quick-sort_ algorithm using recursive calls, however the interesting thing from a _Rust_ perspective is how we declare the parameter as a **mutable slice** (`&mut i32`) and we make **mutable borrows**.
    
    Next set to explore how to define and operate with custom data-types.

## Custom Data-Types

In _Rust_ **Structs** and **Enums** are the building blocks for creating our own "custom-types" in our program domain. They are respectively the **product** and **sum** _algebraic data types (ADTs)_ in _Rust_.

### Structs

A **struct** (or **structure**) is a data-type that allows us to package together multiple data elements together into one named type. An instance of a **struct** will be a composite of **"all"** its individual elements (or members) - I.e. it is a **product data-type**. _(the cardinality of a **struct** type will be the product of the cardinality of all its member types)_. **Structs** is a familiar programming concept in other classical language such as _Pascal_ and _C_.

Another **product type** we have already seen is the **tuple** type. Unlike a **tuple** though the members of a **struct** and defined and accessed with _associative names_ and not their position. This makes **structs** more flexible and natural to model domain types.

#### Defining Structs

We can define a **struct** using the `struct` keyword followed by the _name_ and then its members with data types separated by commas -

```rust
struct User{
    user_id: String,
    email_id: String,
    logged_in: bool
}
```

Here we have defined a **structure** to represent a `User`. _Note: how the members/fields of the structure have names with 'Snake_Case', this is the recommended style in Rust and in fact the compiler will give us a warning if we don't follow it._

#### Instantiating Structs

When we need instances of the **struct** we can create them using the **name** of the **struct** followed by the fields and concrete values for each _(Note that there is no `new` keyword like we use for classes in say C++/C#/Java. In that regard this is more similar to Kotlin in syntax)._ Also note that the actual order of the fields does not matter -

```rust
let u1 = User{
    email_id: String::from("albert.einstein@acme.com"),
    logged_in: false,
    user_id: String::from("R1002")
};
```

Note how the syntax for instantiating a **struct** is very similar to defining it (we even use the `:` to separate the name and the value). We simply specify the **value** instead of the **type** of the field.

> Note how in the **struct** `User` we specify the type of the fields `email_id` and `user_id` as owned type `String` instead of the slice type `&str`. This is because the instance of the **struct** should **"own all its data"** and for that data to be valid for as long as the **struct instance** is valid.
>
> It is possible to use **references** and **slices** as field types, but that will need the use of _**lifetime specifiers**_ that can ensure that the data referenced is valid as long as the **struct instance** is valid.
>
> We shall revisit this once we learn about **lifetime** later.

#### Accessing Members

We access **struct fields** using the "dot notation" (`<instance_name>.<field_name>`) syntax just like most other languages.

```rust
let u1 = User{
    email_id: String::from("albert.einstein@acme.com"),
    logged_in: false,
    user_id: String::from("R1002")
};
// access 'u1' fields using '.'
println!("User {} is {}loggedin.", u1.user_id, if !u1.logged_in {"Not "} else {""});
// User R1002 is Not loggedin.
```

#### Mutable Structs

If we want to change the value of a field after it is crated we can do that by making the **struct** _**mutable**_. Note that we can only make the whole **struct** mutable (or not), we cannot make individual fields mutable (which makes sense if we want deterministic control on that behaviour) -

```rust
let mut u1 = User{
    email_id: String::from("albert.einstein@acme.com"),
    logged_in: false,
    user_id: String::from("R1002")
};
// change the 'logged_in' status
u1.logged_in = true;
println!("User {} is {}loggedin.", u1.user_id, if !u1.logged_in {"Not "} else {""});
// User R1002 is loggedin.
```

Now we have a **mutable struct** and we can change the value of `logged_in` to `true`.

#### Field Init Shorthand Syntax

Specifying the name and the value for each field can get quite verbose and tedious. Luckily _Rust_ provides a shortcut syntax to specify the fields using variables(or function parameters) directly as long as the variable/parameter identifiers have the same name as the fields -

```rust
// varibales have same name as struct feilds
let email_id = String::from("albert.einstein@acme.com");
let user_id = String::from("R1002");
let logged_in = false;

let mut u1 = User{
    email_id,
    logged_in,
    user_id
};
```

This is especially handy with functions that create **structs** with passed in values -

```rust
fn main() {
	// call function to get User instance
    let mut u1 = get_user(String::from("albert.einstein@acme.com")
                        , String::from("R1002"));
    
    u1.logged_in = true;
    println!("User {} is {}loggedin.", u1.user_id, if !u1.logged_in {"Not "} else {""});
    // User R1002 is loggedin.
}

// function parameters have same name as struct feilds
fn get_user(email_id: String, user_id: String) -> User{
    User{
        email_id,
        user_id,
        // we can even combine with explicit field name 
        logged_in: false
    }
}
```

#### Struct Update Syntax - Instances from other Instances

If we want to create a **struct** instance using values of another instance we might try something like -

```Rust
fn main() {
    // declare first istance c1 of car
    let c1 = Car{
                make: String::from("Ford"),
                plate: String::from("CA 2341"),
                year: 2014
            };
    
    // create c2 with values of c1
    let c2 = Car{
        		// use c1 values individually
                make: c1.make,
                year: c1.year,
                plate: String::from("TX G023"),
            };   
}
// struct for car
struct Car{
    make: String,
    plate: String,
    year: u16
}

```

Again _Rust_ provides some syntactic sugar to make this type of declarations easier (using the **struct update syntax** `..`) -

```rust
fn main() {
    let c1 = Car{
                make: String::from("Ford"),
                plate: String::from("CA 2341"),
                year: 2014
            };
    
    // create c2 with values of c1
    let c2 = Car{
                plate: String::from("TX G023"),
                // copy remianing values from c1
                ..c1
            };
}
```

Now all fields that are not explicitly specified will be copied over from `c1.` Note that the **struct update syntax** construct (`..c1` in our case) should come at the **end** of the instance declaration. If we put `..c1` anywhere else in the middle of at the beginning it will not compile.

#### Tuple Structs

_Rust_ provides something called **tuple structs**, which is something between a **struct** and a **tuple**. We can define a **struct** with the body of a **tuple**, i.e. the members/fields are not named. The syntax uses the `struct` keyword but the body looks like that of a **tuple** 

```rust
// define tupple structs
struct Colour(u16, u16, u16);
struct Point(u16, u16, u16);
// Colour and Point have same shape

// declare x is of the type 'Point'
let x: Point;

// instantiate a tuple struct 'Point'
x = Point(101, 203, 132);
```

However if we try to assign a `Colour` to the variable `x` it will not compile -

```rust
// instantiate a tuple struct 'Colour' to variable 'x'
x = Colour(101, 203, 132);
/*
error[E0308]: mismatched types
  --> src/main.rs:12:9
   |
12 |     x = Colour(101, 203, 132);
   |         ^^^^^^^^^^^^^^^^^^^^^ expected struct `main::Point`, found struct `main::Colour`
*/
```

To access the elements we can use the same **`.<index>`** or **destructuring** just like with normal **tuples**.

```rust
// define tupple structs
struct Point(u16, u16, u16);

// instantiate a tuple struct 'Point'
let x = Point(101, 203, 132);

// access tuple struct field with '.'
println!("the 'x' axis = {}", x.0);

// destructuring to access fields
let Point(_, y, _) = x; // Note the struct type quialifier on the LHS
println!("the 'y' axis = {}", y);
```

Note that when **destructuring** a **tuple struct** we have to prefix the **left hand side** with the type of the **struct**. In this case we use `let Point(_, y, _) = x`. If we do not specify the **struct** type on the left, then _Rust_ will give an error stating that we cannot assign a **struct** to a **tuple**.

#### Unit-Like Structs

We can define **structs** without any  fields (called unit-like structs). These are similar to `()`, the **`unit`** type. The reason why we might need them is when we wish to implement a type for some **trait**, bu the type itself does not need to store any data within it (sort of like _static types_ implementing an interface in some OOP languages). We shall see example of this later when we cover **`traits`**.

#### An Example of using Structs

Let us progress through a simple example of working with **structs** and see how it introduces the need of other related concepts -

```rust
struct Rect{
    width: u32,
    height: u32
};

let a = Rect{
    width: 120,
    height: 90
};
```

We have a **struct** `Rect` that represents a rectangle and then we declare a variable `a` of the type `Rect`. Now suppose we need a function that calculates the area of the rectangle we could do something like -

```rust
// define a function that borrows a 'Rect' and returns the area
fn area(rect: &Rect) -> u32{
    rect.width * rect.height
}

// invoke the function to get the area
println!("Area is {}", area(&a));
// Area is 10800
```

So we used the function to calculate the area and print it. Now let us try to print the "`Rect`" instance itself-

```rust
// try prining the rectangle -default formatter
println!("Rect is {}", a); // error
/*
error[E0277]: `main::Rect` doesn't implement `std::fmt::Display`
  --> src/main.rs:20:28
   |
20 |     println!("Rect is {}", a);
   |                            ^ `main::Rect` cannot be formatted with the default formatter
   ....
*/

// try debug formatter
println!("Rect is {:?}", a); // error
/*
error[E0277]: `main::Rect` doesn't implement `std::fmt::Debug`
  --> src/main.rs:20:30
   |
20 |     println!("Rect is {:?}", a);
   |                              ^ `main::Rect` cannot be formatted using `{:?}`
   |
   = help: the trait `std::fmt::Debug` is not implemented for `main::Rect`
   = note: add `#[derive(Debug)]` or manually implement `std::fmt::Debug`
   = note: required by `std::fmt::Debug::fmt`
*/
```

What _Rust_ is complaining about is the fact that it does not know how to display our **struct**. The `println!` macro relies on methods of underlying traits that the type has to implement in order to display it. In this case it is saying that our type `Rect` does not implement the `std::fmt::Display` trait or the `std::fmt::Debug` trait. This is kind of like `toString()` method in languages like _Java, C#_ etc. One way to work around this is to let _Rust_ inject this trait for us (it can do this for simple types), by using the **"compiler pragma"** **`#[derive(Debug)]`**. Now _Rust_ does the work of implementing the trait for us (this is very similar to **`deriving (Show)`** in _Haskell_ to implement the `Show` typeclass).

```rust
// compiler pragma to derive 'Debug' trait
#[derive(Debug)]
struct Rect{
    width: u32,
    height: u32
};

let a = Rect{
    width: 120,
    height: 90
};

// try prining the rectangle
println!("Rect is {:?}", a);
// Rect is Rect { width: 120, height: 90 }
```

Now we can use the debug formatter `{:?}` to print the rectangle, of course if we want to display it differently then we have to do our own custom implementation (which we will see later when we learn **traits**).

#### Method Syntax

Methods in _Rust_ are just like in other OOP languages, they are functions that belong to an instance of a type (a **struct**, **enum** or **trait** in the case of _Rust_). They are defined within the context of the type and have access to its member data and other methods. The first parameter in a method is **`self`** (or typically a reference to it). This is just like **`this`** being the first parameter of methods in most languages, though the syntax implicitly handles that in many of them. In _Rust_ we have to explicitly declare **`self`** (or its reference) as the **first parameter**, we can then use it within the method body to refer to instance that the method was invoked on. 

We can modify our example code to implement the `area` function as a method -

```rust
fn main() {
    #[derive(Debug)]
    struct Rect{
        width: u32,
        height: u32
    };
    
    // implementation section for 'Rect`
    impl Rect{
        // method takes '&self' as first param
        fn area(&self) -> u32{
            // access instance data using 'self'
            self.width * self.height
        }
    }

    let a = Rect{
        width: 120,
        height: 90
    };
    
    // call 'area()'with method syntax
    println!("Area is {:?}", a.area());
    // Area is 10800
}
```

The things to note here are:

- The method/s of the **struct** are defined in an implementation section (**`impl`**). _This might look familiar to people who have worked with "Pascal" language which has separate `interface` and `implementation` sections_. _Rust_ allows us to have multiple implementation blocks for the same type. There is no reason to do it in this example, bu the syntax supports this. It has it uses when we work with **generics** and **traits** which we shall see later.
- The method is defined within the **`impl`** section just like we write a function, with the difference that the first parameter is a reference to the instance that the method is invoked on (**`&self`**) .
  - Note that we do not have to specify the type of the parameter here as that is implied within the **`impl`** section for `Rect`.
  - Here we used an **immutable borrow** of self for the first parameter (**`&self`**), this is because we just want to read the instance values. This parameter can take **ownership** (**`self`**) or be a **mutable borrow** (**`&mut self`**). We might use the **mutable borrow** if we wish to modify the value of the instance. Take **ownership** of self from the caller would be very rare. 
- Within the method we use the **`self.<member>`** syntax to access the data/members of the instance. This is just like using **`this`** in _Java, C#_ etc.
  - In _Rust_ we access members of a reference/pointer directly with the **"`.`"** operator, unlike in _C/C++_ where we have to use **"`->`"** operator. _Rust_ automatically does the **dereferencing** behind the scene for us based on the context.
- When we want to invoke the method we just use the familiar **"`.`"** syntax (`a.area()`). The method `area()` will be invoked passing in `&a` as the first argument.

#### Additional Parameters

When we have additional parameters we simply specify them as we would with a normal function, but after the first (**`self`**) parameter -

```rust
// implementation section for 'Rect`
impl Rect{
    // method takes '&self' as first param
    fn area(&self) -> u32{
        // access instance data using 'self'
        self.width * self.height
    }
	// method takes another Rect as a param
    fn can_hold(&self, other: &Rect) -> bool{
        other.width <= self.width && other.height <= self.height
    }
}

let a = Rect{
    width: 120,
    height: 90
};

let b = Rect{
    width: 120,
    height: 85
};

// call method passing in explicit arguent `&b`
println!("'a' can contain 'b' is {}", a.can_hold(&b));
// 'a' can contain 'b' is true
```

#### Associated Functions

We can use implementation section (**`impl`**) to define functions that are not "methods", i.e. they do not have an implicit **`self`** parameter, and access to an instance. We have used a few of these functions by now such as `string::from`. This tells us that the `from()` function here is associated with the `string` type, but not with any particular instance of `string`. In fact they are typically used to create instances of the **struct** (_factory functions_) . _Associated Functions_ are like _Static Methods_ in traditional OOP languages like _Java & C#_.

In our example we can define an _associated function_ to create a "square" -

```rust
// implementation section for 'Rect`
impl Rect{
    // method takes '&self' as first param
    fn area(&self) -> u32{
        // access instance data using 'self'
        self.width * self.height
    }
    
    // assocaited function - create square
    fn square(side: u32) -> Rect{
        Rect{
            width: side,
            height: side
        }
    }
}
//...
// get a square using the assocaited function in Rect
let s = Rect::square(78);
println!("Area of {:?} is {}", s, s.area());
// Area of Rect { width: 78, height: 78 } is 6084
```

We specified an _associated function_ (`square`) within the **`impl`** section of `Rect` and then we invoked it using `Rect::square()`. We use the scope resolution operator `::` to access the _associated function_ because it is _"namespaced"_ by the **struct**.

### Enums

**Enums** allow us to define types by _enumerating_ its _"variants"_. It is the **"sum" algebraic data type (ADT)** in _Rust_ (semantically similar to other functional languages such as _F#_ and _Haskell_).

#### Defining and using Enums

When we want to define a type that can express _"variations"_  of a common type, a **sum type** is an ideal choice in languages that support **ADT**. In traditional OOP languages we might use more heavy weight constructs such as **interfaces** or **inheritance** to define a family of classes. Languages that support **ADT** provide a leaner and more natural seeming construct with the **"sum type"**. _FP_ languages use this pervasively - a few examples in _Haskell_ -

```haskell
data Bool = False | True
-- Bool can have values False or True (its variants)

data Either a b = Left a | Right b
-- Either has Left of type parameter 'a' or Right of type parameter 'b'

data DivisionResult = DivisionByZero | Success Double
-- DivisonResult can be DivisionByZero or Success with the result

```

In such languages we rely heavily on **pattern matching** and **destructuring** to disambiguate and access the variants. In this regard _Rust_ is very similar.

let us take a look at how to define an **enum** in _Rust_ using an example of _"IP address"_ which can be _"IPv4"_ or _"IPv6"_ (its variants). 

```rust
enum IpAddress{
        IpV4,
        IpV6
    };
```

We can define one using the **`enum`** keyword followed by the _variants_ within the body (curly braces). We can now declare variables of `IpAddress` and create instances of it -

```rust
let ip IpAddress;
ip1 = Address::IpV4; // use :: for enum variant

let ip2 = IpAddress::IpV6; // use :: for enum variant
```

This gives us the ability to specify different types of IP addresses but the address itself, i.e. it does not specify any data. In _Rust_ we can specify **data members** for each of the **enum variants**. In this case we could do something like -

```rust
enum IpAddress{
    IpV4(u8, u8, u8, u8), // data/shape of IpV4 variant
    IpV6(String) // data/shape of IpV6 variant
};

// IpV4 octets as numbers
let home = IpAddress::IpV4(127, 0, 0, 1);

// IpV6 as string
let loopback = IpAddress::IpV6(String::from("::1"));
```

We can pass these variables to functions that take the **enum** as parameter -

```rust
fn main() {
    
    // IpV4 octets as numbers
    let home = IpAddress::IpV4(127, 0, 0, 1);
    
    // IpV6 as string
    let loopback = IpAddress::IpV6(String::from("::1"));
    
    ping(home); // ping IpV4
    // packets received at IpV4(127, 0, 0, 1)
    
    ping(loopback); // ping IpV6
    // packets received at IpV6("::1")
}

#[derive(Debug)]
enum IpAddress{
        IpV4(u8, u8, u8, u8),
        IpV6(String)
    }

fn ping(host: IpAddress){
    println!("packets received at {:?}", host);
}
```

As we can see, this is a powerful concept. It allows us to specify whole family of related types with a simple construct. In fact the _Rust standard library_ uses **enums** (along with **structs**) to define builtin types for IP address -

```rust
// struct to describe IPv4 address feilds
struct Ipv4Addr{
    //----
}
// struct to describe IPv6 address feilds
struct Ipv6Addr{
    //----
}
// IpAddr  as enum of struct variants
enum IpAddr{
    V4(Ipv4Addr),
    V6(Ipv6Addr)
}
```

**Enums** can contain any type of data - strings, numeric types, structs or even other enums. It is quite a flexible and powerful **ADT** for describing our domain types.

```rust
enum Message{
    Quit, // no associated data
    Move {x: i32, y: i32}, // anonymous struct
    Write(String), // tuple struct
    ChangeColor(u16, u16, u16) // tuple struct
    
}
```

#### Methods and Associated functions

Just like **structs** we can define **methods** and **associated functions** for **enums** in the **`impl`** section.

```rust
impl Message{
    // method to call a message
    fn call(&self){
        // do something with &self data
    }
    
    // associated function to raise a quit message
    fn raise_quit() -> Message{
        Message::Quit
    }
}
```

#### The "Option" enum

One builtin **enum** that we will encounter pervasively in _Rust_ would be the **`Option`** enum. As a conscious decision, _Rust_ dose not have a **"`null`"** type, like most other languages. Whilst the concept of **`null`** to indicate the absence of a value is good, the implementation in the mainstream languages leads to many areas of potential bugs. The main reason is that the handling of values that can be **`null`** (the **"`null check`"**) is left to the developer to explicitly cater to in code. Furthermore when this possible absence of value is not described in the type, they cannot be guarded for at compile time. 

The **`Option`** enum in _Rust_ encapsulates the notion of a type that can have some value or have nothing. This is exactly like the **`Maybe`** monad in _Haskell_ or _TypeScript_. The _standard library_ defines it as -

```rust
enum Option<T>{
    Some(T),
    None
}
```

This defines **`Option`** as an **enum** with a _generic type parameter_ **`T`** and having two _variants_, **`Some`** that can hold a value of type **`T`** Or a **`None`** which indicates the absence of any value.

The **`Option`** type is so common that it is included wit the prelude, we do not have to explicitly bring it into scope. Additionally the _variants_ **`Some`** and **`None`** can be used directly without having to qualify it with the **enum** all the time.

```Rust
let a = Some(23);
let b = Some(String::from("Hello"));
let c: Option<i32> = None;
let d = None::<i32>;

println!("a = {:?}", a); // a = Some(23)
println!("b = {:?}", b); // b = Some("Helo")
println!("c = {:?}", c); // c = None
println!("d = {:?}", d); // d = None
```

- In the case of `a` and `b` variables which has a **`Some`**, we can directly specify the value and the _type parameter_ `T` is inferred from the value.
- In the case of `c` and `d` which is a **`None`** we have to explicitly specify the _type parameter_ somewhere, for `c` it is the type specification and for `d` it uses the **"turbo fish operator"** (**`::<i32>`**)

#### Pattern Matching with enums

After we define and declare our **enums,** we need some way to access the value within. This is what **pattern matching** can help us with. The **`match`** expression is a control flow construct which when used with **enums** can "match" a _branch_ of code to execute depending on the _variant,_ an also "bind" a variable to the data inside the _variant_ which can then be used in the code branch. 

Let us use an example to understand the syntax and capabilities of **`match`** when working with **enums** -

```rust
fn main() {
    
    // enum to represent network Protocol Data Unit
    enum PDU{
        Data,
        Segments,
        Packets,
        Frames,
        Bits
    }
    
    // fn takes a pdu and prints nw layer
    fn get_nw_layer(pdu: &PDU){
        let nw_layer: &str;
        // match statement branching code based on PDU variant
        match pdu{
            PDU::Data => nw_layer = "Application",
            PDU::Segments => nw_layer = "Transport",
            PDU::Packets => nw_layer = "Internet",
            PDU::Frames => nw_layer = "Data Link",
            PDU::Bits => nw_layer = "Physical"
        }
        println!("{}", nw_layer);
    }
    
    // instantiate a PDU
    let du = PDU::Packets;
    
    get_nw_layer(&du);
    // Internet
}
```

In this example we have used **`match`** like a **`switch case`** statement to branch based on which _variant_ of the **enum** (`PDU`) it is. Accordingly it sets a variable with the respective **string literal**, which then gets printed.

Since **`match`** is an expression in _Rust_ (like in _Haskell_), we can (and should) write it as -

```rust
// fn takes a pdu and prints nw layer
fn get_nw_layer(pdu: &PDU){
    // match expression branching code based on PDU variant
    let nw_layer = match pdu{
        PDU::Data => "Application",
        PDU::Segments => "Transport",
        PDU::Packets => "Internet",
        PDU::Frames => "Data Link",
        PDU::Bits => "Physical"
    };
    println!("{}", nw_layer);
}
```

**`match`** expression returns a value that gets assigned to the variable directly. 

We can make the code _cleaner_ by just returning the application layer value as a string literal and print it outside the function. This way it becomes a  _"pure function"_.

```rust
// fn takes a pdu and returns a nw layer
fn get_nw_layer(pdu: &PDU) -> &str{
    match pdu{
        PDU::Data => "Application",
        PDU::Segments => "Transport",
        PDU::Packets => "Internet",
        PDU::Frames => "Data Link",
        PDU::Bits => "Physical"
    }
}

// instantiate a PDU
let du = PDU::Packets;

println!("{}", get_nw_layer(&du));
// Internet
```

Now let us look at how to access and work with the data within the **enum variants** - 

```rust
fn main() {
    // enum to represent shape
    #[derive(Debug)]
    enum Shape{
        Triangle {b: u32, h: u32},
        Rectangle {l: u32, b:u32},
        Square {s: u32},
        Circle {r: u32}
    }
    
    fn area(shp: &Shape) -> f32{
        match shp {
            // match to Traingle and bind to variables 'b' & 'h'
            Shape::Triangle{b, h}=> 0.5 * (*b as f32) * (*h as f32),
            // match to Rectangle and bind to variables 'l' & 'b'
            Shape::Rectangle{l, b}=> (*l as f32) * (*b as f32),
            // match to Square and bind to variable 's'
            Shape::Square{s}=> (*s as f32) * (*s as f32),
            // catch all
            _ => 0.0
        }
    }
    
    let s1 = Shape::Triangle{b: 51, h: 43};
    
    println!("area of {:?} is {}", s1, area(&s1));
    // area of Triangle { b: 51, h: 43 } is 1096.5
}

```

Here we have an **enum** of different shapes and function that takes a reference to a shape and calculates the area. In the `area` function we use a **`match`** expression to find a _variant_ that the `shp`  parameter matches with and, also **bind** the _match variables_ with the _data_ of the _variant_. 

- `Shape::Triangle{b, h}` - The data of the parameter `shp` is bound to the variables "`b`" and "`h`", which can then be used in the body of the expression branch.

- One thing to note with **structs** is that the **variable names** that we bind in the **match** has to be same as the **field names** in the **struct definition**. So we will get an error if we try something like -

  ```rust
  // match to Square and try bind to variable 'a'
  Shape::Square{a}=> (*a as f32) * (*a as f32)
  
  // Error
  /*
  error[E0026]: variant `main::Shape::Square` does not have a field named `a`
    --> src/main.rs:19:27
     |
  19 |             Shape::Square{a}=> (*a as f32) * (*a as f32),
     |                           ^
     |                           |
     |                           variant `main::Shape::Square` does not have this field
     |                           help: a field with a similar name exists: `s`
  *
  ```

- If it is a **tuple** on the other hand, there is no **field name**, so we can use any name for the **bind variables**. Which means if `Shape::Square` was a **tuple** we could write -

  ```rust
  // match to Square and bind to variable 'a'
  Shape::Square(a)=> (*a as f32) * (*a as f32)
  ```

- Note also that since _Rust_ is strongly typed we have to **explicitly cast** the **`u32`** values to **`f32`** to do the computation.

- Another thing to note is that with the **variables** we are having to **"dereference"** them. If we try using the variables directly we will get an error -

  ```rust
  // match to Traingle and bind to variables 'b' & 'h'
  // use variables 'b' and 'h' dierctly
  Shape::Triangle{b, h}=> 0.5 * (b as f32) * (h as f32)
  // Error!
  /*
  error[E0606]: casting `&u32` as `f32` is invalid
    --> src/main.rs:15:43
     |
  15 |             Shape::Triangle{b, h}=> 0.5 * (b as f32) * (h as f32),
     |                                           ^-^^^^^^^^
     |                                           ||
     |                                           |help: dereference the expression: `*b`
     |                                           cannot cast `&u32` as `f32`
  
  error[E0606]: casting `&u32` as `f32` is invalid
    --> src/main.rs:15:56
     |
  15 |             Shape::Triangle{b, h}=> 0.5 * (b as f32) * (h as f32),
     |                                                        ^-^^^^^^^^
     |                                                        ||
     |                                                        |help: dereference the expression: `*h`
     |                                                        cannot cast `&u32` as `f32`
  */
  ```

  _Rust_ complains because "`b`" and "`h`" are **references to `u32`** (`&u32`) , therefore we cannot use the value it points to without **dereferencing** it. The reason the **bound variables** are **references** and not direct **values** is that we have written our function to **borrow** a **reference** to `Shape`, which means the **bound variables** are also **references** to the  `Shape` data (the variables do not have **ownership** of the data, which makes sense).

  For demonstration, if we change our function to take **ownership** instead of **borrow**, the **bound variables** will also have the direct data -

  ```rust
  // 'Shape' parameter takes ownership
  fn area(shp: Shape) -> f32{
      match shp {
          // match variables 'b' & 'h' have direct access to value
          Shape::Triangle{b, h}=> 0.5 * (b as f32) * (h as f32),
          // match variables 'l' & 'b' have direct access to value
          Shape::Rectangle{l, b}=> (l as f32) * (b as f32),
          // match variable 'a' has direct access to value
          Shape::Square(a)=> (a as f32) * (a as f32),
          // catch all
          _ => 0.0
      }
  }
  
  let s1 = Shape::Triangle{b: 51, h: 43};
  
  // pass in the shape variable 's1' with ownership
  println!("area is {}", area(s1));
  // area is 1096.5
  ```

  Of course passing **ownership** is not a good practice and seldom ever required. In practice we would almost always stick to **borrowing reference** and **dereferencing** the variables in the **match branch**.

- The last case in out **match expression** is the "**catch all pattern**" (`_ => 0.0`). Like most other languages, the **match expression** is **exhaustive** in its case check. This means that _Rust_ will force us to ensure that all possible _variants_ are considered. If we have missed out any the compiler will output an error an specify the ones we have missed.

  ```rust
  fn area(shp: Shape) -> f32{
      match shp {
          Shape::Triangle{b, h}=> 0.5 * (b as f32) * (h as f32),
          Shape::Rectangle{l, b}=> (l as f32) * (b as f32),
          Shape::Square(a)=> (a as f32) * (a as f32),
       	// missed Circle and no 'catch-all'
      }
  }
  /*
  error[E0004]: non-exhaustive patterns: `Circle { .. }` not covered
    --> src/main.rs:13:15
     |
  5  | /     enum Shape{
  6  | |         Triangle {b: u32, h: u32},
  7  | |         Rectangle {l: u32, b:u32},
  8  | |         Square (u32),
  9  | |         Circle {r: u32}
     | |         ------ not covered
  10 | |     }
     | |_____- `main::Shape` defined here
  ...
  13 |           match shp {
     |                 ^^^ pattern `Circle { .. }` not covered
     |
     = help: ensure that all possible cases are being handled, possibly by adding wildcards or more match arms
  */
  ```

  This helps prevent many potential bugs. Ideally we should handle each and every case, and if we are really interested only in a subset of the cases then we can ignore the others by using a **catch all** pattern (**`_`**). The "**`_`**" identifier is a placeholder for a **pattern** or **variable** we are not interested in. This is similar to many other languages such as _Haskell_ or _TypeScript_.

Now we can see how to extract the value from the **`Option`** enum we saw earlier -

```rust
fn main() {
    let x: Option<i32>;
    
    x = Some(23);
    
    let dbl = match x {
        // extract data in 'x' to bound varaible 'i'
        Some(i) => i * 2,
        // return 0 for None
        None => 0
    };
    
    println!("{}", dbl);
    // 46
}

```

Simply use the **`match`** expression to match for the _variants_ **`Some`** and **`None`**, and bind the data within **`Some`** to the branch variable. This is a very common pattern in languages that use **ADTs**.

#### The "`if let`" construct with enums

Whenever we are interested only in **one case** of a set of **matches** and we wish to ignore the rest, the **`if let`** syntax provides a less verbose way to do it -

```rust
fn main() {
    // enum to represent shape
    enum Shape{
        Triangle {b: u32, h: u32},
        Rectangle {l: u32, b:u32},
        Square {s: u32},
        Circle {r: u32}
    }
    
    let s1 = Shape::Circle{r: 13};
    
    // 'if let` syntax to handle single case of circle
    if let Shape::Circle{r} = s1 {
        println!("Circle has radius {}", r);
    }
}
```

So to handle the case of matching to a single case (_variant_), we use an **`if`** statement with a **`let`** binding that allows us to **match** for that single case and handle it with the **`if`** body.

- Note how we have gone from an **"expression form"** withe **match** to a **"statement form"** with the **if let**.
- Also whilst this is convenient for handling **single  match** cases, we lost the ability for _Rust_ to do **exhaustive** check for us.

If we wanted to do something special for the cases that are **not** the **single case**, then we have an **`else`** part to the **if let .. else`** - 

```rust
// 'if let` syntax to handle single case of circle
if let Shape::Circle{r} = s1 {
    println!("Circle has radius {}", r);
} else {
    // handle if Not Circle
    println!("This is not a circle!");
}
```

Again notice how now it is a very **imperative** style code with a **statement form**, as opposed to an **expression form** we would get with **match expressions**. So whilst it can help with getting less verbose code sometimes, choose wisely before using it considering that we prefer safety and expressiveness first.

## Modularity - Packages, Crates, Modules & Paths

As our code-base grows big or when "programming in the large" as it is sometimes referred to, the ability to effectively organise and structure your code becomes vital. We should design our code with software architecture such as modularity, cohesion, loose coupling, encapsulation, reuse etc. Every programming language provides some mechanism for modularity.

_Rust_ has a number of features to enable organisation and modularity of code, these are collectively called the **module system** - **modules**, **paths**, **use**, **crates** and **packages**. We shall examine each of these in detail.

### Modules

A **module** is a **logical unit** of organising code into one **cohesive** unit. The different elements of code such as **structs**, **enums**, **functions**, **constants** that we think belong together typically go into one **module**. We can also **nest** modules within them creating an tree like hierarchy. In that sense of encapsulating and code organisation it is similar to **namespaces** in _C#_ (though with some important differences as well).

Beyond this **cohesive** encapsulation they also serve as the mechanism for controlling **visibility**  (or **"privacy"** as per the _Rust_ documentation). By default everything in a **module** is **private**. If we wish to make anything public we have to explicitly do so by qualifying it with the **`pub`** keyword.

Let us take a look at using **modules** to organise some code -

```rust
// outer module for a bank
mod bank{
    // nested module for accounts
    mod accounts{
        struct Account{
            number: u64,
            amount: f32
        }

        impl Account{
            fn create(number: u64) -> Account{
                Account{
                    number: number,
                    amount: 0.0
                } 
            }

            fn deposit(&mut self, amt: f32){
                self.amount += amt;  
            }

            fn withdraw(&mut self, amt: f32) -> bool {
                if amt <= self.amount{
                    self.amount -= amt;
                    return true
                }
                return false  
            }
        }
    }
	// module for customer
    mod customers{
        struct Customer{
            id: String,
            name: String,
            address: Address,
            // account: Account // this line will not compile now!
        }
        
        struct Address{
            house: String,
            street: String,
            city: String,
            postcode: String
        }
    }

}
```

We have created a **`bank`** module with nested modules for **`accounts`** and **`customers`**. Now everything is neatly organised into its logical groups. We shall use this example going forward to build on the concepts.

Now let us try using this model from the main function -

```rust
mod bank{    
    mod accounts{

        struct Account{
            number: u64,
            amount: f32
        }
        //...
    }
    
    mod customers{
        //...
    }
    
}

fn main() {
    // create an account
    let acc = Account::create(100120013001); // Error!
    
}
/*
error[E0433]: failed to resolve: use of undeclared type or module `Account`
  --> src/main.rs:54:15
   |
54 |     let acc = Account::create(100120013001);
   |               ^^^^^^^ use of undeclared type or module `Account`
*/
```

This gives an error saying we are trying to use an **"undeclared "** type or module. The name `Account` does not exist the place we are trying to use it. In order to do that we have to bring it into **scope**, by using the **`use`** keyword (this is similar to "`import`" in _Java_, _Python_, _ES6_ etc. or "`using`" in _C#_).  We specify the module or item we want to bring into scope using its **path**, which can be an "**absolute path**" in which case it starts from the "**crate root**" (we shall cover crates shortly), or a "**relative path**" which means it starts from the same level in the hierarchy as we are trying to use it from. This is exactly like **files system paths** in this respect. Of course as is the convention, we use the **scope resolution operator** (`::`)  to refer to items in inner scope. 

The syntax would look like -

```rust
 // bring Account into scope with absolute path
    use crate::bank::accounts::Account;
// OR
 // bring Account into scope with realtive path
    use bank::accounts::Account;
```

Whether to use **absolute** or **relative** path depends on the context of the code, and how we perceive the module will move around in time. If the module definition and the code that uses it remain in the same hierarchy relative to each other, then use a **relative path**, else  it might be better to use **absolute path**.

OK, so let us now correct our code above by bringing the `Account` type into scope -

```rust
fn main() {
    // bring Account into scope with realtive path
    use bank::accounts::Account;
    
    // create an account
    let acc = Account::create(100120013001); // Still Error!
    
}
/*
error[E0603]: module `accounts` is private
  --> src/main.rs:54:15
   |
54 |     use bank::accounts::Account;
   |               ^^^^^^^^ this module is private
...
*/
```

Now _Rust_ complains that `accounts` is **private** (since everything in a module is **private** by default). If we need to have access to that we would have to explicitly make it **public** using the **`pub`** keyword. Note that if we we cannot just make the nested `accounts` module **`pub`**, but all the internal types and functions that we need to to access from outside as well. So to make our coed work, we would have to do something like -

```rust
mod bank{
    // make module 'accounts' public
    pub mod accounts{
		// make struct 'Account' public
        pub struct Account{
            number: u64,
            amount: f32
        }
        
        impl Account{
            // make assocaited function public
            pub fn create(number: u64) -> Account{
                Account{
                    number: number,
                    amount: 0.0
                } 
            }
            // make method public
            pub fn deposit(&mut self, amt: f32){
                self.amount += amt;  
            }
            // make method public
            pub fn withdraw(&mut self, amt: f32) -> bool {
                if amt <= self.amount{
                   self.amount -= amt;
                   return true
                }
                return false  
            }
        }
        
    }
    
    mod customers{
        //...
    }
    
}

fn main() {
    // bring Account into scope with realtive path
    use bank::accounts::Account;
    
    // create a mutable account
    let mut acc = Account::create(100120013001);
    
    // deposit some money
    acc.deposit(2000.0);   
}
```

Now this compiles without any error. We have brought the type `Account` that we want into scope using the **`use`** keyword. We have also made all the internal types, functions and methods that we need public with the **`pub`** qualifier.

#### Making structs and enums public

So far so good, we seemed to have created an account and deposited some money into it. Now let us try to print out the amount  -

```rust
fn main() {
    // bring Account into scope with realtive path
    use bank::accounts::Account;
    
    // create a mutable account
    let mut acc = Account::create(100120013001);
    
    // deposit some money
    acc.deposit(2000.0);
    
    println!("Account balance is {}", acc.amount); // Error!
}
/*
error[E0616]: field `amount` of struct `bank::accounts::Account` is private
  --> src/main.rs:62:39
   |
62 |     println!("Account balance is {}", acc.amount);
   |                                       ^^^^^^^^^^
*/
```

 This happened because whilst we declared the **struct** `Account` as **public**, we did not say anything about its members (`number`, `amount`). With a product type like **struct**, it is not just sufficient to make the **struct** public but also the members we want to access from outside. 

```rust
mod bank{
    // public module accounts
    pub mod accounts{
		// public struct Account
        pub struct Account{
            number: u64,
            // public member 'amount'
            pub amount: f32
        }
        // ...       
    }
    
    // ...
}

fn main() {
    // bring Account into scope with realtive path
    use bank::accounts::Account;
    
    // create an account
    let mut acc = Account::create(100120013001);
    
    // deposit some money
    acc.deposit(2000.0);
    
    println!("Account balance is {}", acc.amount);
    // Account balance is 2000    
}

```

Now the member `amount` is accessible from outside.

With a sum type like **enum** however this is not the case, if the **enum** is **public** its members are **public** as well. This makes sense because an **enum** value **"is"** essentially one of its members, whereas a **struct** contains (or **"has"**) its members. So if we had an enum in our module we could make it public and its members would be public implicitly -

```rust
mod bank{
    // public module accounts    
    pub mod accounts{
        // public enum 'AccType'
        #[derive(Debug)]
        pub enum AccType {
            // members are also become public
            Current,
            Savings,
            Investment
        }
     	// ...   
    }
    
}

fn main() {
    // bring Account into scope with realtive path
    use bank::accounts::Account;
    
    // create an account
    let mut acc = Account::create(100120013001);
    
    // deposit some money
    acc.deposit(2000.0);
    
    println!("Account balance is {}", acc.amount);
    // Account balance is 2000
    
    // access and print an AccType
    println!("{:?}", bank::accounts::AccType::Investment);
    // Investment
}
```

Because the **enum**  `AccType` is **public** so are all its _variants_.

Notice how we used the `AccType` **enum** directly without bringing it into scope with the **`use`** keyword. That is because we used the **fully qualified name** with the entire **path** specified while referring to it. In one-off scenarios like this it may be fine, but it can get get quite unwieldy pretty soon. Not to mention the nightmare of having to change everywhere in code if the **path** changes sometime.

#### Multiple Paths

Sometimes we may need to bring a **number** of **nested paths** into scope and there can be a lot of duplication in the path specifiers. _Rust_ provides a shorthand using a **path list** syntax, where we can specify the common part of the paths outside followed by **`::`** and then the **list** of paths that differ inside curly braces **`{`** **`}`** - 

```rust
// bring Account into scope
use bank::accounts::Account;
// bring AccType into scope
use bank::accounts::AccType;

// can be written as

// bring Account and AccType into scope
use bank::accounts::{Account, AccType};
```

#### The Glob operator (`*`)

If there are a large number of items to import from a module we can use the **`*`** syntax. This will bring **all public** members of the module into scope - 

```rust
// bring all accessible members into scope
use bank::accounts::*;
```

#### Import with Alias (`as`)

When we import something into scope with **`use`** we can give it an **alias** using the "**`as`**" keyword. Then we can use the alias within the referring code instead of  the original name.

This is often used for reasons such as **name resolution conflict** (another entity with the same name already exists in that scope), or if the original name is too long or can be renamed to give better meaning in the used context.

```rust
// import with alias
use bank::accounts:: AccType as Type;
// use alias
println!("{:?}", Type::Investment);
```

#### Re-exporting with "`pub use`"

With the **"`pub use`"** syntax we can re-export what we bring into scope. This is typically done when we wish to create a more friendly pubic API (a facade) for our model. This gives us the freedom to choose an internal module hierarchy that is efficient for our purposes as library writer, but expose a simpler public facade that is more intuitive to the consumer of our module.

```rust
mod bank{
   	// nested module accounts   
    pub mod accounts{
        #[derive(Debug)]
        pub enum AccType {
            Current,
            Savings,
            Investment
        }
        
        //  ...
        
    }
    // re-export AccType at this level
    pub use accounts::AccType as AccountType;
    
    mod customers{
        // ...
    }
    
}

fn main() {
    // import the re-exported definition
    use bank::AccountType;
    
    println!("{:?}", AccountType::Investment);
    // Investment    
}

```

Note how we used a simpler **path**, the nesting and hierarchy of the modules within the crate is of no use to me as a consumer.

Most of these concepts related to **`use`** and **paths** are similar to **import** in other languages such as _Python_ and _ES6_. With a good idea of the logical **code organisation**, **scope** and **visibility** management, we can now look at **crates** and **packages**.

### Crates

A **crate** is the unit of compilation in _Rust_. It is analogous to **assembly** in _.Net_ or **jar** in _Java_. A **crate** can be compiled into a **library** or a **binary **(executable). We shall now see how we can move our **`bank`** module into a **crate** and consume it from another one. To build a deeper understanding of what is going on, we shall do this by hand using the **rustc** compiler only. In practice however we would always use **cargo** (the build and package manager) to do this. 

We shall start off with our source structure looking like -

```bash
demo_crate
	|___ main.rs
	|___ banking.rs
```

We have two source files, with the code of the **`bank`** module we wrote moved into the **`banking.rs`** file, and a **`main.rs`** that will try to consume it. They are both in the same directory.

- The first thing to do is to **build** **`banking.rs`** into a **library crate** as that will be referenced by the **`main.rs`**. By default the **rustc** compiler will produces a **binary crate**, we can use the **`--crate-type=lib`** flag to override this behaviour.

  ```bash
  $ rustc --crate-type=lib banking.rs
  $ ls
  banking.rs  libbanking.rlib  main.rs
  ```

  We can see that **rustc** now created a **library crate** with the name **`libbanking.rlib`** (it prefixes the filename with **`lib`** and gives a **`.rlib`** extension). If we wish to specify a different name, we can do that with the **`--crate-name=`** flag. For example if we try to name it **finance**, we would get **`libfinance.rlib`** instead.

  ```bash
  $ rustc --crate-type=lib --crate-name=finance banking.rs
  $ ls
  banking.rs  libfinance.rlib  main.rs
  ```

  Let us just revert to keep it to **banking**.

- At this point we have a **library crate** (**`libbanking.rlib`**) with the name **`banking`** and some nested modules in it. Based on the code we wrote the hierarchy tree should look like -

  ```bash
  banking (Crate)
      |____bank (top most Module)
          |____ accounts (nested Module)
          |		|____ Account (Struct)
          |		|____ AccType (Enum)
          |____ customers (nested Module)
              		|____ Customer (Struct)
              		|____ Address (Struct)
  ```

- Now we need a way to **reference** this **library** in our **`main.rs`**, bring the modules into scope and use the types/functions within it. We **reference** the library using the **`extern crate <crate-name>`** syntax. This will **link** the **library** to our source, and also **import** its contents (preserving its internal hierarchy and visibility) under a single **top level module** with the name of the **library**. So our **`main.rs`** code would look like - 

  ```rust
  // reference external crate
  extern crate banking;
  
  fn main(){
      // bring struct into scope using full path
      // top level module name is same as library ("banking")
      use banking::bank::accounts::Account;
  	// use struct from the library
      let mut acc = Account::create(100120013001);
  
      acc.deposit(2000.0);
  
      println!("account balance is {}", acc.amount);
  }
  ```

- Next we need to compile the **`main.rs`** into a **binary crate**. Since the compiler needs to **link** this to another **crate**, we have to specify that as a **command line flag** to the **rustc** compiler using the **`--extern <crate-name>=<file-name>`** flag.

  ```bash
  $ rustc --extern banking=libbanking.rlib main.rs
  $ ls
  banking.rs  libbanking.rlib  main  main.rs
  ```

  We compiled **`main.rs`** into a **binary crate** (**`main`**), specifying the **external library** with name **`banking`** which is the file-name **`libbanking.rlib`**.

- Now we can execute our **binary crate** and see if we get the output we expected -

  ```bash
  $ ./main
  account balance is 2000
  ```

  So the **`main`** **crate** was able to **link** to a **library crate**, **import** the **modules** and use it.

This should give us a good understanding of what a **crate** is and how they are related to **modules**, how to **reference**, **link** and **use** them. As mentioned before though, in practice we would never deal with **crates** directly in this fashion, rather we would use the **cargo** tool to manage it all for us. And that brings us to the concept of **packages**.

### Packages

A **package** is a container of one or more crates, with some manifest information about its version, dependencies, how to build it etc. It is also the unit of code distribution. _Rust_ comes with a **package manager** tool called **Cargo**, that essentially manages the life-cycle of a _Rust_ project. We would typically use **cargo** to do the following:

- Create a new project (**package**), with some scaffolding.
- Declare and manage dependencies with other **packages**.
- Integrate with the **package repository** (**`crates.io`**)
- Run **unit tests**
- Build and distribute our **packages**

Whilst we shall see more about **cargo** as we go along, we shall focus on their **unit of software** which we call **packages**. A **package** has the following properties:

- It should contain a **`cargo.toml`** file that describes the **package**, its dependencies and how to build it.
- It should contain **at least** one **crate**.
- It can contain as many **binary crates** as needed.
- However it can have only  a maximum of **one** **library crate**.
- It can have a combination of **binary and library crates**, but still only one **library crate** at most.

To get a feel of how to work with **packages** and **cargo**, let us take out **banking example** and move it all (including the **main**) into a **binary crate** within one **package**, and let us start and configure the project using **cargo**.

```bash
$ cargo new hello_package
     Created binary (application) `hello_package` package
```

Now we should have a _Rust_ project withe following file system structure - 

```bash
hello_pacakage
    ├── Cargo.toml
    └── src
        └── main.rs
```

The **`Cargo.toml`** file would look like -

```bash
[package]
name = "hello_package"
version = "0.1.0"
authors = ["<git user name>"]
edition = "2018"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
```

It uses the the **TOML (Tom's Obvious Markup Language)** markup syntax, and looks very similar to old school **INI** files. As of now we can see the **package name**, **semantic versioning** and the fact there are no **dependencies**. Additionally it has created a **`src`** directory and a simple **`main.rs`** with just an entry-point and a **`println!`**.

> We can observe the similarity with other programming platforms such as **Node.js**, where **npm** is the **package manager** and **package.json** is the manifest file equivalent to our **`Cargo.toml`**.

To continue with our example, let us move all our code into **`main.rs`**. 

```rust
// hello_package/src/main.rs
mod bank{
    pub mod accounts{
        // ...
        pub struct Account{
            number: u64,
            pub amount: f32
        }
        
        impl Account{
            //...
        }
    }
        
    mod customers{
        // ...
    }
}

fn main() {
	// use the local module in the same file
    use bank::accounts::Account;

    let mut acc = Account::create(100120013001);

    acc.deposit(2000.0);

    println!("account balance is {}", acc.amount);
}

```

We can now **check** or **build** it with **cargo** -

```bash
$ cargo build
```

This will create **build** and crate a **crate** in a nested **`target`** folder - 

```bash
hello_package
    ├── Cargo.lock
    ├── Cargo.toml
    ├── src
    │   └── main.rs
    └── target
        └── debug
            ├── build
            ├── deps
            │   ├── ...
            ├── examples
            ├── hello_package   // --- the binary crate ---
            ├── hello_package.d
            └── incremental

```

As we can see **cargo** does quite a few things behind the scene like creating a **Cargo.lock** file and a **target** directory with  multiple sub directories, and finally the **crate** itself with the name of the **package**. So far this seems simple enough. 

##### Split Modules into Independent Files

As our code base increases, we might want to organise our modules into their own source files. In our example we can now move the **bank** module to its own file. To do this we create a **file** in the same **`src`** directory with the **same name** as the **module**, and move all our **`bank`** module code into that -

```bash
hello_package
    ├── Cargo.lock
    ├── Cargo.toml
    ├── src
        ├── bank.rs (code of bank module)
        └── main.rs
....
```

```rust
// hello_package/src/bank.rs
mod bank{
    pub mod accounts{
        // ...
        pub struct Account{
            // ...
        }
    }    
    mod customers{
        // ...
    }
}
```



Next in **`main`** where we try to use the **`bank`** module we need to let _Rust_ know that it can load it from a different file. To do that we add a line in **`main.rs`** -

```rust
// ';' after module name => load from a file 'bank.rs'
mod bank;

fn main() {

    // import 'Account' into scope
    use bank::accounts::Account;

    let mut acc = Account::create(100120013001);

    acc.deposit(2000.0);

    println!("account balance is {}", acc.amount);
}
```

When we use a **`;`** after the **`mod bank`** (instead of the **module body** in **`{}`**) then _Rust_ will **load** the contents of the **module** from another **file with the same name** as the **module**.

Now if we try to **build**/**check** our package -

```bash
$ cargo check
Checking hello_package v0.1.0 (/.../hello_package)
error[E0432]: unresolved import `bank::accounts`
 --> src/main.rs:7:15
  |
7 |     use bank::accounts::Account;
  |               ^^^^^^^^ could not find `accounts` in `bank`

```

We get an **error!** It says that **`main.rs`** cannot find **`bank::Accounts`**. The reason for this is the **structure** of our **`bank.rs`** source code. We have an **outer module** with the name **`bank`** that **contains** the **inner modules**. When we move that to its own **file** with the name **`bank`**, and then **load** it into **`main.rs`**, the **whole content** of the **file** gets **loaded** , but also gets **enclosed** in a **module** with the same name as the **file**.

So in this case the actual **module** hierarchy wold be -

```bash
  bank
    |--bank
        |--accounts
            |--Account
```

This means we have to change the **use** statement to -

```rust
 use bank::bank::accounts::Account;
```

But when we try this we get another error!

```bash
Checking hello_package v0.1.0 (/.../hello_package)
error[E0603]: module `bank` is private
 --> src/main.rs:7:15
  |
7 |     use bank::bank::accounts::Account;
```

By now we should know that this is because in our **`bank.rs`** source code we did not make the **`bank`** module public, so we can do that now -

```rust
// hello_package/src/bank.rs
pub mod bank{
    pub mod accounts{
        // ...
        pub struct Account{
            // ...
        }
    }    
    mod customers{
        // ...
    }
}
```

Now this will **build** successfully and we can run the **package** -

```bash
$ ./target/debug/hello_package
account balance is 2000
```

Note that we still have only **one crate** (`hello_package`), but 2 **source files** with the **module** moved into that -

```bash
|---------|
| bank.rs |-----|
|---------|     |
                |
              (Load)
                |
|---------|     |                          |---------------|
| main.rs |-----|--------(Compile)-------> | hello_package |  (Crate)
|---------|                                |---------------|
```

In this the **`main.rs`** is called the **crate root**, that is what the _Rust_ compiler starts from and makes the **root module** of our **crate**.

In this example we had a slight confusion with the **path** of the **`bank`** module. Because of the additional wrapping created by the separate file, we had to use a path like -

```rust
 use bank::bank::accounts::Account;
```

We could make it more clearer by giving a **different name** for our file, we could have called it **`banking.rs`** instead. 

```bash
hello_package
    ├── Cargo.toml
    ├── src
        ├── banking.rs (code of bank module)
        └── main.rs
....
```

Then we would change the **references** and **import** within the **`main.rs`** -

```rust
// ';' after module name => load from a file 'banking.rs'
mod banking;

fn main() {

    // import 'Account' into scope
    use banking::bank::accounts::Account;
	// ...
}
```

Another way we could address this would be to avoid the **top level** (**`bank`**) module specification in the **`banking.rs`** file. Since by moving it into a **separate file**, we get an **enclosing** module (**banking**) automatically, we could just directly have **`accounts`** and **`customers`** at the **top level**. Then we could import it more succinctly like -

```rust
use banking::accounts::Account
```

We could do away with the extra covering.

##### Separate into Multiple Crates

Continuing along this line we could next move our **banking** module into its own **crate** as a **library** and use it within the **main** (**binary crate**). It will be similar to how we have done it directly using **`rustc`**, but this time we will do it as a **package** using **`cargo`** (as it would normally be done).

In fact to turn our simple solution above (which has the **`banking`** module in its own file **`banking.rs`**) into one where the **`banking`** module is in its own **library crate** , there is _nothing_ we have to change in the **project structure**. There are only **two** changes we have to make - one is to modify is the **`Cargo.toml`** file to instruct **cargo** that there is a **library crate** and a **binary crate**, what we want their **names** to be and the **path** to find the files. So with the same project structure as -

```bash
hello_package
    ├── Cargo.toml
    ├── src
        ├── banking.rs (code of bank module)
        └── main.rs (main uses banking)
....
```

we would modify the **`Cargo.toml`** to -

```bash
[package]
name = "hello_package"
version = "0.1.0"
authors = ["<...>"]
edition = "2018"

[lib]
name = "banking"
path = "src/banking.rs"

[[bin]]
name = "main"
path = "src/main.rs"

[dependencies]

```

We can see a **`[lib]`** section that gives the **name** and **path** of the **library crate** and **source file**. Similarly there is a **`[[bin]]`** section with details of the **`main`** **binary crate**.

Since now **`banking`** is a **separate crate**, we will have to **reference** it as such in our **`main.rs`**, so that would be the only other change that we would have to do -

```rust
// reference separate crate banking
extern crate banking;

fn main() {

    // import 'Account' into scope
    use banking::bank::accounts::Account;

    let mut acc = Account::create(100120013001);
	// ...
}
```

We use the **`extern crate`** to **reference** **`banking`**, everything else remains the same. Now if we build this we would get -

```bash
hello_package
    ├── Cargo.toml
    ├── src
    │   ├── banking.rs
    │   └── main.rs
    └── target
        └── debug
            ├── hello_package (our package)
            ├── hello_package.d
            ├── libbanking.d
            ├── libbanking.rlib (library crate)
            ├── main (binary crate)
            └── main.d
```

_Note: of course there are many other files and directories created for dependency management and as part of the build. But we can ignore those for our purposes here._

Now we can run our package as we did before -

```bash
$ ./target/debug/hello_pcakage
account balance is 2000
```

##### Default filenames and **`bin`** directory

Another way to achieve the same would be to have our **banking library** with the **"default filename"** - **`lib.rs`** and the source for the **binary** in a **`src/bin`** directory. If we stick to this convention, we might not even have to modify the **`Cargo.toml`**. 

```bash
hello_package
    ├── Cargo.toml
    ├── src/
    │   ├── lib.rs
    │   └── bin/
    │       ├── main.rs
```

The use of the **`src/bin`** directory is really useful if there are **multiple binary crates**. However we can always have at-most **one library crate** in our **package**.

As our project gets even bigger we may want more **structure** and **flexibility** in organising our crates. We may have **multiple libraries** that we wish to reuse. **Cargo** offers a _feature_ called **workspaces** to help achieve this. It helps us manage **multiple related packages** in one logical group. We shall examine **workspaces** later.

## Common Collections

The _Rust_ _standard library_ includes a number of useful data structures called **collections**. These **collections** are _composites types_, i.e. they contain multiple values. This is a common feature of almost all programming languages and we can easily see the analogy with _Python_, _C++_, _C#_ etc.

We have already seen the **primitive** composite types - **array** and **tuple**. However they are both **fixed size** collections, i.e. they cannot have elements added or removed dynamically to them. The **collections** from the **standard library** that we shall see next are **dynamic** and the data they point to is stored on the **heap**. Each of the different **collection** type has its own characteristics and intended purpose. Like any programming language, these data structures are essential to writing useful programs.

### Vector 

A **vector** is a _homogeneous_ collection of elements in _contiguous_ memory, that can grow or shrink in size. So this is just like an **array** but its size is not fixed. Some languages call this data structure **list**(_Python_) and some call it **arraylists** (_C#_), it is all the same.

#### Creating a Vector

We create a **vector** using **`Vec::new`** associated function, or the **`vec!`** macro -

```rust
fn main() { 
    let v1: Vec<i32> = Vec::new(); 
    println!("v1 = {:?}", v1);
    // v1 = []
    
    let v2 = vec![10, 20 , 30, 40];
    println!("v2 = {:?}", v2);
    // v2 = [10, 20, 30, 40]
}
```

Note that with **`Vec::new`** we have to specify the **type** of the variable as there are no values that _Rust_ can use to infer it from. The **`vec!`** macro on the other hand can infer the type as it crates and **initialises** the **vector** in the same statement. 

#### Adding and Removing elements

**Vector** provides methods to **add** and **delete** elements -

```rust

fn main() {
    let mut v1: Vec<i32> = Vec::new(); 
    println!("Initial = {:?}", v1);
    // Initial = []
    
    // adding elements
    v1.push(10);
    v1.push(20);
    v1.push(30);
    v1.push(40);
    println!("After push = {:?}", v1);
    // After push = [10, 20, 30, 40]
    
    // deleting elements 
    let i1 = v1.remove(0); // specify index
    println!("After remove = {:?}, elem was {}", v1, i1);
    // After remove = [20, 30, 40], elem was 10
}
```

Note that we have to make the **vector** **mutable**, and then we can use **`push()`** and **`remove()`** methods to add and delete elements. Of course if we try to delete at an index out of bounds then _Rust_ will panic with an error.

#### Dropping a Vector

Like any other data, when a **vector** goes out of scope it gets dropped, and along with it, its elements also get cleaned up. We can see this in action if we make a **wrapper struct** that implements the **`Drop`** trait and override the **`drop()`** method to print something out when it gets dropped, and then make a **vector** of this struct - 

```rust
fn main() {
    // a custom wrapper struct
    struct BoxItem{
       value: i32 
    }
    // overrides 'drop()' to print value
    impl Drop for BoxItem{
        fn drop(&mut self){
            println!("..dropping item {}", self.value);
        }
    }
    
    {
        // create a vector of 'BoxItem`
        let mut vi: Vec<BoxItem> = Vec::new();
        
        // add elements to it
        vi.push(BoxItem{value: 10});
        vi.push(BoxItem{value: 20});
        vi.push(BoxItem{value: 30});
    } // vi goes out of scope and dropped
    // ..dropping item 10
    // ..dropping item 20
    // ..dropping item 30
    // the elements also get dropped
}
```

This makes it visible that when the **vector** is dropped, its content gets cleaned up as well.

#### Accessing elements

There are **two** ways to access elements of a **vector** -

- Using the **index** of the element within square brackets **`[]`**.
- Using the **`get()`** method which takes an **index** and returns an **`Option<&T>`**.

We have to bear in mind that accessing an element from a **vector** has the same **copy** or **borrow** semantics as assignment. As an example let us try to access elements from a **`Vec<i32>`** -

```rust
fn main() {
    let u = vec![10, 20 , 30, 40];
    let u1 = u[0];
    println!("First number is {}", u1);
    // First number is 10    
}
```

This seems to work just fine. Now let us try this with a heap allocated data in a vector **`Vec<String>`** -

```rust
fn main() {
    let v = vec![String::from("Jan"),
            String::from("Feb"),
            String::from("Mar")];
    let v1 = v[0];
    println!("First month is {}", v1); //Error
}
/*
rror[E0507]: cannot move out of index of `std::vec::Vec<std::string::String>`
 --> src/main.rs:6:14
  |
6 |     let v1 = v[0];
  |              ^^^^
  |              |
  |              move occurs because value has type `std::string::String`, which does not implement the `Copy` trait
  |              help: consider borrowing here: `&v[0]`
*/
```

_Rust_ complains because when we try to access the element directly using **`[]`**, it tries to apply **copy semantics** for **ownership** of the element in the **vector**. With **`Vec<i32>`** this was fine because **`i32`** implements the **`Copy`** trait, however with **`Vec<String>`**, **`String`** does not implement **`Copy`**. So the normal way to access an element is to **borrow** it using a **reference**. So we would normally do -

```rust
fn main() {
    let u = vec![10, 20, 30, 40];
    let u1 = &u[0]; // borrow 1st element
    println!("First number is {}", u1);
    // First numbr is 10

    let v = vec![String::from("Jan"),
            String::from("Feb"),
            String::from("Mar")];
    let v1 = &v[0]; // borrow 1st element
    println!("First month is {}", v1);
    // First month is Jan
}
```

With **borrow** the **index** based access works for both types.

Of course we can also do a **mutable borrow** in case we wish to **modify** an element in a **vector** -

```rust
fn main() {
    let mut u = vec![10, 20, 30, 40];
    let u1 = &mut u[0]; // mutable borrow 1st element
    *u1 = 15; // de-reference and modify pointed to value
    println!("First number is {}", &u[0]);
    // First numbr is 15
}
```

Here we do a **mutable borrow** of the first element, **dereference** the access the pointed value and modify it. Indeed we see that the actual first element in the **vector** gets modified.

With the **`get()`** method we get back an **`Option<&T>`** type that we can check and operate on using **pattern matching** -

```rust

fn main() {    
    let v = vec![String::from("Jan"),
            String::from("Feb"),
            String::from("Mar")];
    
    let v1 = v.get(0); // returns an Option<&String>
    match v1{
        Some(first) => println!("First month is {}", first),
        None => println!("Cannot access an invalid element")
    }
    // First month is Jan
}
```

Here **`get()`** returns an **`Option<&String>`** that we can **unpack** using **pattern matching**. If we had tried to access an index that does not exist -

```rust
fn main() {    
    let v = vec![String::from("Jan"),
            String::from("Feb"),
            String::from("Mar")];
    
    let v1 = v.get(100); // try access invalid index
    match v1{
        Some(first) => println!("First month is {}", first),
        None => println!("Cannot access an invalid element")
    }
    // Cannot access an invalid element
}
```

Here the program gracefully handles this scenario without panicking and crashing. Which method we use depends on our particular scenario.

With **`get()`** too the rules of **borrow** and **ownership** will apply. So if we try to **`get()`** an element from a **mutable vector**, add an element to the **vector** and then use the **borrowed** element - 

We get an error. We cannot have a **mutable** and **immutable** **borrow** in the same scope and then try to use one of them. This might seem strange that we cannot **borrow** a **reference** to an **element** and **modify** the **vector** in the same scope. The reason is the underlying implementation of **vector** is such that sometimes when we modify the **vector** , it might not have **contiguous space** in memory and might require it to **reallocate** the whole **vector** to a new space - in which case the **reference** we hold to the element would become invalid. -

```rust
fn main() {
    let mut v = vec![String::from("Jan"),
            String::from("Feb"),
            String::from("Mar")];
            
    let v1 = v.get(0); // get first element
    
    v.push(String::from("Apr")); // add an element to 'v'
    
    // try using the immutable borrow 'v1'
    match v1{
        Some(first) => println!("First month is {}", first),
        None => println!("Cannot access an invalid element")
    }
    // Error
}
/*
error[E0502]: cannot borrow `v` as mutable because it is also borrowed as immutable
  --> src/main.rs:10:5
   |
8  |     let v1 = v.get(0); // get first element
   |              - immutable borrow occurs here
9  |     
10 |     v.push(String::from("Apr")); // add an element to 'v'
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^ mutable borrow occurs here
11 |     
12 |     match v1{
   |           -- immutable borrow later used here
*/
```

So even if take an **immutable borrow** on an **element** we cannot have **mutable borrow** of the **vector** within the same scope and try to use the **immutable** one later. This helps avoid data inconsistency and race conditions.

#### Iterating over a Vector

Just like we did with **array** we can iterate over the elements of a **vector** using the **`for .. in`** loop -

```rust
fn main() {
    let v = [10, 20, 30, 40, 50];
    
    for m in &v{ // 'm' is a borrow because of '&v'
        println!("{}", m);
    }
}
```

_Note_: how we are **borrowing** the element from the **vector** by using **`&v`** and not taking a **copy**. As we have seen before accessing elements (even if we are looping) is better done as a **borrow**.

We can also have also have **mutable borrows** in our loop if we wish to modify the elements - 

```rust
fn main() {
    // mark vector as mutable
    let mut v = [10, 20, 30, 40, 50];
    
    // loop variable is a mutable borrow 
    for m in &mut v{
        // dereference the borrow to access the pointed to element
        *m += 1;
    }
    
    for m in &v{
        print!("{}, ", m);
    }
    // 11, 21, 31, 41, 51,
}
```

The key things to observe are:

- We made a **mutable** borrow from the **vector** by specifying **`for m in &mut v{..`**}.
- We have to **dereference** the loop variable to access the **element** it points to, so that we can change it. When we do simple access of the **reference** to do things like _printing_ or _passing_ it around we don't need to **dereference** it explicitly because _Rust_ handles it behind the scene for us. However if we need to modify the value we will have to **dereference** it access the value it points to.

#### Storing Multiple Types

Since a **vector** can store only elements of a single type, if we have a situation that has variants of types to be handled we can do that using **enums** -

```rust
fn main() {
// define an enum for different shapes
    enum Shape{
        Square(i32),
        Rect(i32, i32),
        Triangle(i32, i32, i32),
        Circle(f64)
    }
    
    // declare a vector of 'Shape'
    let mut shps: Vec<Shape> = Vec::new();
    // add different variants of 'Shape' to it
    shps.push(Shape::Circle(10.13));
    shps.push(Shape::Rect(15, 17));
    shps.push(Shape::Square(23));
    shps.push(Shape::Circle(9.37));
    
    // iterate over the vector
    for s in &shps{
        // pattern match on loop variable
        match s{
            Shape::Square(x) => println!("square with side {}", x),
            Shape::Rect(x, y) => println!("rectangle with sides {}, {}", x, y),
            Shape::Circle(x) => println!("circle with radius {}", x),
            _ => println!("Unhandled shape"),
        }
    }
}
```

Here we are able to handle different kinds of shapes using an **enum** to represent the _variants,_ then operate on that **vector** of **enums**.

### Strings - UTF-8 encoded text

_Rust_ uses two data types to represent textual information:

- The **`str`** _slice_ which represents an immutable sequence of characters. This is used for **string literals** and almost always referenced with a **borrow** (**`&str`**).  The underlying data can be thought of like an **array** of **bytes** embedded into the **binary** or on the **stack** (or even reference to data on the **heap**). **`str`** **slice** is part of the _Rust_ **core** language, it is very memory efficient, safe and rather inflexible.
- The _standard library_ (**`std`**) in _Rust_ provides another, more flexible type called **`String`**, which are allocated on the **heap** and can be **mutable**. This is more closer to the concept of strings from other programming languages, however the similarity ends soon there. Because of the memory trade-off decisions taken by _Rust_, handling **`String`** data in _Rust_ is more complicated than in most other languages (though **`Go`** has similar interface).

When necessary _Rust_ allows us to translate between the types easily, as long as the memory semantics are valid. Under the hood both **`str`** **slice** and **`String`** are a sequence of **bytes** that are **`UTF-8`** encoded. The difference as we have seen is way it is allocated and handled in memory. 

We have already covered the **`str`** **slice** previously, in this section we shall focus more on the **`String`** type. The first thing to learn would be how to create a **`String`**, and we can do that from a **literal** using the **`from()`** associated function, or the **`to_string()`** method on **`str`** slice -

```rust
let x = String::from("ART");
let y = "BANG".to_string();
```

Before we explore further operations on **`String`**, we should try to understand how _Rust_ represents text as **`UTF-8`** under the hood. This will make it clear what we can do what we cannot do and why. Also **`UTF-8`** applies to both **`String`** and **`str`**.

**`UTF-8`** is a **Unicode** encoding scheme that uses **variable byte** length to represent **code points**. A **code point** is a unique _integer_ mapping that represents a particular **character**. In the old days we had **`ASCII`** encoding that used **7** bits, which gave us a range of **`0`** to **`127`** to represent all the **Latin character set** and some **control characters**. As the need for **multilingual** support evolved various **encoding** standards emerged. **Unicode** was the most successful with initial attempts of **`UTF-32`** and **`UTF-16`** which used **fixed byte** sizes of **4** and **2** bytes respectively fell out of favour because of the waste in memory for the existing **`ASCII`** ranges. **`UTF-8`** emerged as the new de-facto standard because it was much more memory efficient and was transparently backward compatible with existing **`ASCII`** encoded data. However it is a little more complicated than the other schemes because it uses **variable size bytes**.

The best way to understand this is with some examples. We can use the **`char()`** and **`bytes()`** methods to examine the underlying data of the **`String`** -

```rust
let x = String::from("ART");
for c in x.chars(){
    print!("{} ", c);
}
// ART

println!();
for c in x.bytes(){
    print!("{} ", c);
}
// 65        82        84
// 01000001  01010010  01010100
// A         R         T
```

This seems straight forward with a **`String`** of 3 characters **`A R T`**, and each character represented with **one byte** and the encoding seems to be the same as **`ASCII`**(**`A`** has **65** and so on). 

Note how in this case the **first bit** of each byte is always **`0`**, this tells us that this is a **single byte** character (and compliant with **`ASCII`**). We can then read the **8 bits** and find the **code point**. In this case we have **code points** **65**, **82**, and **84** - which map to the English (Latin) characters **`A`**,  **`R`**, **`T`** respectively., and so we have our string text.

Suppose instead of the familiar English characters we had some Greek text say - **`φκβ`** -

```rust
let y = "φκβ".to_string();
for c in y.chars(){
    print!("{} ", c);
}
// φ κ β

println!();
for c in y.bytes(){
    print!("{} ", c);
}
// 207       134       206       186       206       178
// 11001111  10000110  11001110  10111010  11001110  10110010
```

Well now even though there are only **3 characters**, we have **6 bytes** in memory to encode them! This is because beyond the originally **`ASCII`** range of characters **`UTF-8`** uses multiple bytes to represent a character (actually its **code point** - the Integer number the character is mapped to). 

In this example the first byte is **`207`** or **`11001111`** in binary. the first **two bits** are **`11`**(this is called the **Byte Order Mark** or **BOM**), which tells us that it is a **2 byte** **code point** and the rest of the bits are part of the **code point**. The second byte from it is **`134`** or **`10000110`** which has **`10`** which tells us that this is a **continuation** byte and the remaining bits are the rest of the **code point** data. So from the first **two** bytes we can extract the **code point** as -

```rust
//First  Byte = BOM ++ Code Point bits
	 11001111 = 11 ++ 001111
//  Next Byte = Cont. Marker ++ Code Point bits
     10000110 = 10 ++ 000110

// => the complete Code Point is 
	 001111 ++ 000110 = 001111000110 = 966 = 0xCF86 = φ
```

If we consider the **BOM** and **Continuation Markers** and extract the rest of the **bits** we can obtain the **code point** for the **`UTF-8`** character. In this case we get the **`966`** in decimal or **`CF86`** in hexadecimal which is the **code point** for the Greek letter **`φ`**. Using this scheme **`UTF-8`** encodes **1,112,064**  **code points** covering a wide range of alphabets from various languages.

Let us look at a more complicated example from Hindi using the _Devanagari_ script -

```rust
let txt = String::from("नमस्ते");   
for c in txt.chars(){
    print!("{} ", c);
}
// न म स ् त े

println!();
for c in txt.bytes(){
    print!("{} ", c);
}
// 224 164 168 224 164 174 224 164 184 224 165 141 224 164 164 224 165 135
```

In this case the Hindi **word** is **`नमस्ते`** (which means greetings). This comprises of the _human readable letters_ - **` "न", "म", "स्", "ते"`**- which is technically called **"grapheme clusters"**.

The **characters** (_Rusts_ **`char`** data type) of this word are - **`न, म, स, ्, त, े`**. Along with the **letters** like **`न`** we also have 2 extra **characters** **` ्`** & **`  े`** which are called **diacritics** used to modify the **pronunciation** and **accent**. So in total we have **6** **characters** each needing a **scalar value** or **code point** to represent it.

Finally we see there are **18** **bytes** to represent this in **`UTF-8`** encoding. **3 bytes** for each of the **6** **code points**. We can try to manually extract the **code point bits** from the **first 3 bytes** -

```rust
// First Byte = BOM ++ Code Point data
224 => 11100000 = 111 ++ 00000
// Next Byte = Cont. Marker ++ Code Point data
164 => 10100100 = 10  ++ 100100
// Next Byte = Cont. Marker ++ Code Point data
168 => 10101000 = 10  ++ 101000

// => Code Point from first 3 bytes
00000 ++ 100100 ++ 101000 = 00001001 00101000 = 2344 = न
```

From the **3** **bytes** we get the **scalar value** of the **code point** as **2344** in decimal which corresponds to the Hindi letter **`न`**. 

Now it is more clear that representing and handling text data is more complicated than it looks when we are aware of the underlying memory layout. Because of the memory safety and performance choices made by _Rust_, as programmers we are exposed to some of this complexity. In most other languages these aspects are abstracted away and the language runtime or the compiler does a lot of heavy memory adjustments behind the scene, but this can come with some performance overheads.

#### String Operations 

With a solid understanding of **`UTF-8`** and how it relates to **`String`** and **`str`** in _Rust_ we can move onto common operations that we can perform with **`String`** types.

##### Updating Strings

Since a **`String`** is a **`Vec<u8>`** it can grow in size and its content can be changed. 

We can use **`push()`** and **`pop()`** to add or remove characters from a string -

```rust
fn main() {
    let mut txt = String::new();
    txt.push('φ');
    txt.push('κ');
    txt.push('β');
    println!("{}", txt);
    // φκβ
    let c = txt.pop(); // pops the last char off
    println!("{}", c.unwrap()); // unwrap option<char>
    // β
    println!("{}", txt); // now 'txt' will be only 2 chars
    // φκ
}
```

It is pretty much resembles standard **`push`** and **`pop`** from any vector. One thing to note is that **`pop`** method returns and  **`Option<T>`** so we have to **`unwrap`** it to get the actual value.

To append a `String` to another `String` we would use the **`push_str`** method. Actually though the method takes an **`&str`** slice and not a **`String`** as parameter, so if we try passing in a **`String`** it will fail -

```rust
fn main() {
    let mut txt = String::from("uno");
   
    let s1 = String::from(" dos");
    txt.push_str(s1);
    println!("{}", txt); // Error
}
/*
error[E0308]: mismatched types
 --> src/main.rs:6:18
  |
6 |     txt.push_str(s1);
  |                  ^^
  |                  |
  |                  expected `&str`, found struct `std::string::String`
  |                  help: consider borrowing here: `&s1`
*/
```

As the error message states, we need to pass in a **`&str`** slice, which we can do simply by "prefixing **`s1`** with a **`&`** to borrow the value", and _Rust_ will coerce it to a **`&str`** slice -

```rust
fn main() {
    let mut txt = String::from("uno");
   
    let s1 = String::from(" dos");
    txt.push_str(&s1); // borrow s1
    println!("{}", txt);
    // uno dos
}
```

To **concatenate** two strings we can use the **`+`** operator or the **`format!`** macro. Let us take an example of each -

```rust
fn main() {
    let s1 = String::from("alpha");
    let s2 = String::from(" beta");
    // concatenate two strings with '+'
    let s3 = s1 + s2; // Error
    println!("{}", s3);
}
/*
error[E0308]: mismatched types
 --> src/main.rs:5:19
  |
5 |     let s3 = s1 + s2; // Error
  |                   ^^
  |                   |
  |                   expected `&str`, found struct `std::string::String`
  |                   help: consider borrowing here: `&s2`
*/
```

Oops! that did not work. Because the **`+`** operator uses the **`add`** method, which expects the second parameter to be a **borrow**, since we do not want to take ownership of the second string (otherwise that would not be available for use in this context after the append operation). So we can correct that as follows -

```rust
let s1 = String::from("alpha");
let s2 = String::from(" beta");
let s3 = s1 + &s2; // s2 is a borrow
println!("{}", s3);
// alpha beta
```

The **`add`** method looks like below (actually it uses generics, but for our purposes we can use the materialised types) - 

```rust
fn add(self, s: &str) -> String { 
    // ..
}
```

Note that the first parameter is **not** a **borrow**, so in effect the method takes **ownership** of the **`self`** instance it is called on and transfers that to the result.  So if we try using the variable bound to the string instance before concatenation then it will not be available - 

```rust
let s1 = String::from("alpha");
let s2 = String::from(" beta");
let s3 = s1 + &s2; // s1 is an ownership transfer
println!("Old string = {}", s1); // Error!
println!("New string = {}", s3);
/*
error[E0382]: borrow of moved value: `s1`
 --> src/main.rs:6:33
  |
3 |     let s1 = String::from("alpha");
  |         -- move occurs because `s1` has type `std::string::String`, which does not implement the `Copy` trait
4 |     let s2 = String::from(" beta");
5 |     let s3 = s1 + &s2; // s1 is an ownership transfer
  |              -- value moved here
6 |     println!("Old string = {}", s1); // Error!
  |                                 ^^ value borrowed here after move
*/
```

The detailed error message says it all!

The second parameter is **`&str`** but we are passing in a **`&String`** argument, this is fine as _Rust_ will coerce this into an **`&str[..]`** slice for us.

If we want to append multiple strings we can do -

```rust
let s1 = String::from("alpha");
let s2 = String::from("beta");
let s3 = String::from("gamma");

let rs = s1 + ", " + &s2 + ", " + &s3;
println!("New string = {}", rs);
// New string = alpha, beta, gamma
```

This is getting unwieldy now, and a better way to do this would be the **`format!`** macro -

```rust
let s1 = String::from("alpha");
let s2 = String::from("beta");
let s3 = String::from("gamma");

let rs = format!("{}, {}, {}", s1, s2, s3);
println!("New string = {}", rs);
// New string = alpha, beta, gamma
```

It is like the **`sprintf`** function in **`C`**. Also **`format!`** macro does **not** take ownership of the variables.

##### Indexing into a String

As we saw in the **UTF-8** section, _Rust_ does not hide the complexity of the underlying binary encoding from the developer. This coupled with the fact that **UTF-8** uses a variable length binary encoding scheme, we cannot just index into the **`String`** which is a **`Vec<u8>`** to get at a character. If we try it _Rust_ will panic with an error -

```rust
let s1 = String::from("alpha");
println!("3rd char is = {}", &s1[2]); // Error!

/*
error[E0277]: the type `std::string::String` cannot be indexed by `{integer}`
 --> src/main.rs:5:35
  |
5 |     println!("3rd char is = {}", &s1[2]);
  |                                   ^^^^^ `std::string::String` cannot be indexed by `{integer}`
  |
  = help: the trait `std::ops::Index<{integer}>` is not implemented for `std::string::String`
*/
```

We have also seen how _Rust_ provides **`chars()`** and **`bytes()`** methods to obtain the **characters** and **bytes** (respectively) as **iterables** from a string.

```rust
let s1 = String::from("alpha");
for c in s1.chars(){
    println!("{}", c); 
}
```

##### Slicing Strings

_Rust_ supports the **slice** operation on strings. We can try to get the characters using a **slice** with some **range**.

```rust
let s1 = String::from("alpha");
println!("1st two chars are '{}'", &s1[0..2]);
// 1st two chars are 'al'
```

However this is risky and can panic with runtime error if the range is not at the right character-bye boundary in the **UTF-8** encoding. For example if try the above with a **3 byte** character -

```rust
let s1 = String::from("नमस्ते");
println!("1st two chars are '{}'", &s1[0..2]); // Error
/*
thread 'main' panicked at 'byte index 2 is not a char boundary; it is inside 'न' (bytes 0..3) of `नमस्ते`'
*/
```

The error message says it all!

#### Summary

Once we are exposed to the underlying layout of string data, it is definitely more complex to handle than in other languages that abstract the complexity away, albeit at the cost of performance and memory trade-off. In _Rust_ we are left to deal with complexity as developers when we handle strings.

### Hash Maps

The **`HashMap<K, V>`** is a standard datastructure that enables us to store **values** against **keys** that can be looked-up in constant time. This is a fundamental computer science datastructure available in other languages and libraries, often called by different names such as dictionary (_Python_), hash (_Ruby_), object (_JS_), map (_Haskell_), or associative arrays (_computer science_).

#### Creating Hash Maps

We can create a **`HashMap`** using the **`new()`** constructor function and then **`insert()`** method to add items to it -

```rust
// explicitly bring HashMap into scope
use std::collections::HashMap;

fn main() {
    // hashmap to keep scores by names
    let mut scrs: HashMap<String, u32> = HashMap::new();
    // insert values
    scrs.insert("Alan".to_string(), 81);
    scrs.insert("Jon".to_string(), 94);
    scrs.insert("Albert".to_string(), 78);
}
```

- The first thing to note is that unlike other collections, **`HashMap`** needs to be explicitly brought into scope with the **`use`** statement as it is not available by default with the prelude. It also has less support from the standard library and do not have **macros** to help create it.
- When we create the **`HashMap`**, if the code cannot determine the values we are going to insert into it then we would have to specify the type of the generic parameters. In this example it would work even if we had not specified the type because we are inserting items just below with **key** as _String_ and **value** as _u32_, so _Rust_ can infer that.
- Like other strongly typed languages _Rust_ **`HashMap`** is a homogeneous collection, i.e  all **keys** have to be on the same type and all **values** have to be of one type.
- Next we use the **`insert`** method to add items to it. If the **key** does **not exist** then it will **add** the item to the **`HashMap`**, however if the **key** **exists** then it will **overwrite/update** the **value**.

Another (more functional) way to create a **`HashMap`** and add values to it is using the **`collect()`** method. It can aggregate from an **iterable** into a **collection**, and depending on the data and the type of the return value it can be a **`HashMap`**. This is best seen with an example -

```rust
use std::collections::HashMap;

fn main() {
    // vector of name and score tuples
    let recs = vec![
        ("Alan".to_string(), 81),
        ("Jon".to_string(), 94),
        ("Albert".to_string(), 78),
    ];
    // collect/aggregate vector of tuples iterable to hash-map
    let scrs: HashMap<String, u32> = recs.into_iter().collect();   
}
```

Often we may want to join two collections into a key-value pair and create a dictionary. We can do that in _Rust_ quite easily like above, with the help of an additional **`zip`()** method. The code for that would look like -

```rust
// vector of names
let names = vec![
    "Alan".to_string(), 
    "Jon".to_string(),
    "Albert".to_string()
];
// vector of scores
let scores = vec![81, 94, 78];

// zip vectors together and then collect to a mash-map
let scrs: HashMap<String, u32> = names.into_iter().zip(
    scores.into_iter()).collect();
// Note: we have to qualify the return type as 'collect' supports other return types as well
```

This is a common pattern handled similarly in languages that support functional style of programming. Here is ho wit would look in _Haskell_ -

```haskell
names = ["Alan", "Jon", "Albert"]
scores = [81, 94, 78]
-- zip and collect to a Map
recs = Map.fromList(zip names scores)
```

#### HashMaps and Ownership

An important aspect of **`HashMaps`** in _Rust_ that is not familiar in other languages is the **ownership** aspect. So in _Rust_ whenever we **insert** an item into a **`HashMap`** (both the **key** and **value**), it gets **copied** (for objects that implement the **copy** trait) or **moved** to the **`HashMap`**. After insert the **`HashMap`** will be owner of the data. The old variables before adding will not be available for use after they are inserted. Also they get cleaned up when the **`HashMap`** gets cleaned up. We can see that with a an example that we have modified to display a message when the item gets dropped -

```rust
use std::collections::HashMap;

fn main() {
    // custom wrapper over int
    struct BoxItem{
        value: i32
    }
    // override 'drop' to display msg when it gets dellocated
    impl Drop for BoxItem{
        fn drop(&mut self){
            println!("... dropping item : {}", self.value);
        }
    }
    // create instances of our wrapper values
    let s1 = BoxItem{value: 81};
    let s2 = BoxItem{value: 94};
    let s3 = BoxItem{value: 78};
    
    // create a new scope
    {
        // add the box-item scores the hash-map
        let items: HashMap<_, _>
                    = vec![
                      ("Alan", s1), 
                      ("Jon", s2), 
                      ("Albert", s3)
                    ].into_iter().collect();
    }
    // nested block scope ends
    println!("Outside nested block!");
}
/*
... dropping item : 94
... dropping item : 81
... dropping item : 78
Outside nested block!
*/
```

From the output we can see that our _`BoxItem`_ values get dropped as soon as the _`items`_ **`HashMap`** goes out of scope when we exit the nested block.

It is possible to insert **references** into a **`HashMap`**, in which case the **ownership** is not moved. However this will mean that the data that the reference points to should remain valid at least as long as the **`HashMap`** is valid. We shall examine this detail when we discuss **Lifetimes**. If we modified our example above to simulate this behaviour -

```rust
use std::collections::HashMap;

fn main() {
    // custom wrapper over int
    struct BoxItem{
        value: i32
    }
    // override 'drop' to display msg when it gets dellocated
    impl Drop for BoxItem{
        fn drop(&mut self){
            println!("... dropping item : {}", self.value);
        }
    }
    
    // create a hash-map 
    let mut items: HashMap<String, &BoxItem> = HashMap::new();
    
    // create a new scope
    {
        // create instances of our wrapper values
        // inside the scope
        let s1 = BoxItem{value: 81};
        let s2 = BoxItem{value: 94};
        let s3 = BoxItem{value: 78};
        
        // add references of box-item scores to the hash-map
        items.insert("Alan".to_string(), &s1); 
        items.insert("Jon".to_string(), &s2);
        items.insert("Albert".to_string(), &s3);
    }
    // the actual values will get dropped here
    println!("Outside nested block!");
    
    // try access the item from the hash-map later
    if let Some(item) = items.get("Jon"){
        println!("Score = {}", item.value);
    }
}
/*
error[E0597]: `s1` does not live long enough
  --> src/main.rs:27:42
   |
27 |         items.insert("Alan".to_string(), &s1); 
   |                                          ^^^ borrowed value does not live long enough
...
30 |     }
   |     - `s1` dropped here while still borrowed
...
35 |     if let Some(item) = items.get("Jon"){
   |                         ----- borrow later used here
   ....
*/
```

The compiler will complain that the **borrowed values get dropped while still being borrowed**, and that we are trying ti use this dropped values later. This way _Rust_ prevents us from shooting ourselves in the foot.

#### Accessing a value

In the above example, we have already seen how to access the **value** for given **key**, using the **`get()`** method. This method checks for an entry with the specified **key** and returns an **`Option<&V>`**. This is because if the item does not exist the return value will be **`None`** and if the value exists then it will be wrapped in **`Some(value)`** which we can unwrap or destructure.

```rust
// access the item from the hash-map using 'get(key)'
if let Some(item) = items.get("Jon"){
    // get() returns an 'Option' type
    println!("Score = {}", item.value);
}
```

#### Iterating over the HashMap

We can easily iterate over a **`HashMap`** to get each of the **key-value** pairs. We can use the same code above to insert values and then just iterate over (a reference of) the **`HashMap`** as an iterable of **key-value** pairs -

```rust
// iterate over key-value pairs in HashMap
for (k, v) in &items{
    println!("{} : {:?}", k, v);
}
/*
Alan : BoxItem { value: 81 }
Jon : BoxItem { value: 94 }
Albert : BoxItem { value: 78 }
*/
```

#### Updating the HashMap

When we **insert** an entry into a **`HashMap`** it has the following behaviour:

- If the **key** does not exist then the **key-value** pair is **added** to the **`HashMap`**
- However if the **key** already exists then the value will be **overwritten** by the new value 

```rust
let mut items: HashMap<String, u32> = HashMap::new();
items.insert("Alan".to_string(), 81); // insert happens
items.insert("Alan".to_string(), 92); // overrite happens
    
println!("{}", items.get("Alan").unwrap());
// 92 - new value
```

This is the typical behaviour for **associative arrays** in most languages.

#### Only Inserting if Key does Not exist

Man times we will want to check if a **key** exists and if not then **insert** otherwise do not do anything. In many languages we would have to do this as two step process. _Rust_ provides us with an efficient and handy way to do this using the **`entry()`** method -  

```rust
items.insert("Alan".to_string(), 81); 
// check for 'Entry' and if not insert
items.entry("Alan".to_string()).or_insert(92);

println!("{}", items.get("Alan").unwrap());
// 81 - old value
```

The **`entry()`** method returns an **`Entry`** **enum** which represents a **value** that might or might not exist. The **`or_insert()`** method of this **`Entry`** then inserts the **new** value if it does **not exist**, OR if it **does exist** then returns a mutable reference that we can chose to modify or ignore as we wish. This is a clever and much for efficient way to handle this scenario.

#### Updating Values in HashMap based on Current Value

Sometimes we want to update the values in a **`HashMap`** based on the existing values. There are different ways to achieve this depending on the use case. We can get a **mutable** borrow and then update that -

```rust
// create a hash-map of score
let mut scores: HashMap<&str, u32> = vec![
                                            ("Alan", 87),
                                            ("Bob", 78),
                                            ("Cathy", 91)
                                        ].into_iter().collect();
// iterate and reduce all scores by 5
// notice mutable borrow of hashmap
for (_, val) in &mut scores{
    *val -= 5; // dereferencing from the borrow
}

println!("{:?}", scores);
// {"Bob": 73, "Alan": 82, "Cathy": 86}
```

Notice how we get a **mutable reference** to the **value** which we have to **dereference** and modify. This changes the **original data** in the **`HashMap`**.

We can also use the **`entry()`** + **`or_insert()`** method to get a **mutable reference** to he **value** for a **specific key** (if it exists) and then modify that -

```rust
// mutable reference to Entry
let cs = scores.entry("Cathy").or_insert(99);
// dereference and modify the original value
*cs = 99;

println!("{:?}", scores);
// {"Bob": 73, "Alan": 82, "Cathy": 99}
```

Note again how we have to **deference** the **mutable reference** to access and modify the **original data** stored in the **`HashMap`**.

## Error Handling

Error handling in _Rust_ takes a slightly different approach than the familiar **exception trow.. try.. catch** pattern that most mainstream language provide. _Rust_ forces the developer to acknowledge the possibility of error and decide how to deal with it explicitly.

In _Rust_  we have two categories of errors:

- **unrecoverable errors** that we handle with the **`panic!`** **macro**, which will terminate the execution.
- **recoverable errors** that we represent with the **`Result<T,E>`** **enum**, which we **unwrap** and determine what to do next.

### Unrecoverable errors with **`panic!`**

These are typically bugs or situations that the code is not designed to handle. In such situations the execution is terminated with a **`panic!`** operation. Normally this also **"unwinds"** the execution stack and cleans up the data that it allocated. Whilst this is a cleaner approach it is also more expensive. The alternative is to **"abort"** immediately and leave it to the OS to do the cleanup. This behaviour can be specified using the **`panic = 'abort'`** flag in the **`profile`** section of the **`cargo.toml`** file. For example if we wanted to **abort** on **panic** in the **release mode** we would do this -

```toml
[profile.release]
panic = 'abort'
```

To make the program **panic** we simply call the **`panic!`** macro with the desired message -

```rust
panic!("..we cant go back from here!!");
```

Often the code can **panic** from other modules or libraries probably because we used it incorrectly in our code. In such salutations we may want to get a stack trace of the call from our code to the point that resulted in the **panic**. The below code should cause a runtime panic as we are trying access an element in a **vector** beyond its boundaries.

```rust
fn main() {
    let arr = vec![1, 2, 3];
    println!("{}", arr[4]);
}
/*
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/demo_error`
thread 'main' panicked at 'index out of bounds: the len is 3 but the index is 4', /rustc/5e1a799842ba6ed4a57e91f7ab9435947482f7d8/src/libcore/slice/mod.rs:2806:10
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
*/
```

This is referring to some library **`../libcore/slice/mod.rs`** which raised the **panic**. If we want to see the full trace we can do that by specifying the **env variable** **`RUST_BACKTRACE=1`** in the **`cargo run`**.

```bash
$ RUST_BACKTRACE=1 cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.02s
     Running `target/debug/demo_error`
thread 'main' panicked at 'index out of bounds: the len is 3 but the index is 4', /rustc/5e1a799842ba6ed4a57e91f7ab9435947482f7d8/src/libcore/slice/mod.rs:2806:10
stack backtrace:
   0: backtrace::backtrace::libunwind::trace
             at /cargo/registry/src/github.com/backtrace-0.3.40/src/backtrace/libunwind.rs:88
   ...
  13: core::panicking::panic_bounds_check
             at src/libcore/panicking.rs:62
  14: <usize as core::slice::SliceIndex<[T]>>::index
             at /rustc/5e1a799842ba6ed4a57e91f7ab9435947482f7d8/src/libcore/slice/mod.rs:2806
  ...
  17: demo_error::main # our code
             at src/main.rs:3
  18: std::rt::lang_start::{{closure}}
             at /rustc/5e1a799842ba6ed4a57e91f7ab9435947482f7d8/src/libstd/rt.rs:67
  ...
  26: main
  27: __libc_start_main
  28: _start
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```

This will show a **trace** of the calls as it happened and somewhere at line **17** in this case we can see our code in the**`src/main.rs`** at line number **3** that started it all.

### Recoverable Errors with **`Result<T,E>`**

To deal with errors that can be understood and dealt with (**recoverable**) we can use the **`Result<T,E>`** **enum**. The generic type parameters **`T`** represents the type of the **"value"** returned and **`E`** represents the type of the **"error"** if it was an error. In this approach we design our code to return the **`Result<T,E>`** enum instead and the calling code can examine this **enum** and depending on the **variant** it can either unwrap the **value** or deal with the **error**. This pattern is typical in _Rust_ and we will see this pervasively in the standard libraries and projects.

The **`Result<T, E>`** **enum** looks like: 

```rust
enum Result<T,E>{
    Ok(T),
    Err(E)
}
```

Let us see how this might be used in practice with an example of trying to open and read from a file:

```rust
use std::fs::File;

fn main() {
    let p = "inputs.txt";
    let f = File::open(p);
}
```

Now **`File::open`** operation can fail or succeed depending on the situation. For example maybe the file does not exist or we might not have permission to read the file. To accommodate this the **`File::open`** function returns a **`Result<std::fs::File, std::io:Error>`** type. The calling code can then decide how to handle this. So we could modify this to pattern match the return value and handle it as shown:

```rust
use std::fs::File;

fn main() {
    let f = File::open("inputs.txt");

    // pattern match on the result
    let f = match f {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };
}
```

Pattern match to check the two variants. If **`OK`** then the **value** it wraps is what we need, else it is **`Err`** and that will hold the **error**.

This is not very helpful though, we may want to check what the **error** is and handle that differently. For example if the fie does not exist we can create it, but if it is some other error we terminate the program with a **`panic!`**.

```rust
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let p = "inputs.txt"; 
    let f = File::open(p);

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind(){
            // nested match for type of error
            ErrorKind::NotFound => match File::create(p){
                // nested match for file creation function
                Ok(new_file) => new_file,
                Err(cr_err) => panic!("Error creating file => {}", cr_err)
            },
            other_error => panic!("Error opening file => {:?}", other_error)
        }
    };
}
```

Here we are doing a set of **nested matches**, first to check the **`ErrorKind`** , and if that is **`NotFound`** then we **create** the file, otherwise we know it is some **other error** and we **`panic!`** and terminate with that **error**. But the **`File::create`** function can have the similar problem that it can fail, so we have another **match** to handle that.

This is a loot of **`match` nesting**. The **`Result<T,E>` enum** gives some methods of its own to handle these scenarios with a lot less code. One is the **`unwrap`** method:

```rust
use std::fs::File;

fn main() {
    let p = "inputs.txt"; 
    let f = File::open(p).unwrap();
}
```

The **`unwrap`** method will do the work of checking the **enum** and unpack the contained value for us, OR if if it is an error then it will call **`panic!` macro** for us with the error message.

If we want to **control** the **error** message then we can use the **`expect`** method instead, which is the same as the **`unwrap`** but we can specify what the message should be for **error**.

```rust
let f = File::open(p).expect("Could not open file {}", p);
```

Another method that **`Result<T,E>`** gives is a **higher order function**  **`unwrap_or_else()`** for the **error handling**, to which we can give a **closure**:

```rust
fn main() {
    let p = "inputs.txt";
    let f = File::open(p).unwrap_or_else(
        // handle error when opening file
        |op_err| {
            if op_err.kind() == ErrorKind::NotFound {
                File::create(p).unwrap_or_else(
                    // handle error on creating file
                    |cr_err| {
                        panic!("Error creating file {}", cr_err)
                    }
                )
            }
            else {
                panic!("Error opening file {}", op_err);
            }
        }
    );
}
```

Note that with this we have more freedom to do what we want in the **error** situation, also we can have **reusable** functions to handle commonly occurring conditions and pass them as the argument to the **HOF** in place of the **closure**.

#### Propagating Errors

When we write **functions** which have code that can cause an error, for example if we are trying to open a file, or make a network connection etc. It is often better to pass-on / propagate that error to the code that is calling our **function**, rather than trying to deal with it ourselves. The calling code often has better context for dealing with the failure appropriately, also that is not really the responsibility of the specific function.

In languages such as _Java_ and _C#_ we **"rethrow"** the error (perhaps with some additional information) and let the first **exception handling** block deal with it. In _Rust_ we design our function to have a return type of **`Result<T, E>`**. let us modify our example above to have a function that reads from a file, and have it called from **`main()`**.

```rust
use std::fs::File;
use std::io;
use std::io::Read;

// return type is Result<T,E> enum
fn read_from_file(path: &str) -> Result<String, io::Error>{
    let f = File::open(path);
    
    let mut f = match f{
        Ok(file) => file,
        Err(error) => return Err(error)
        // if error return will be variant Err with data 'error'
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        // return variant Ok with data 's' 
        Err(error) => Err(error)
        // if error return will be variant Err with data 'error'
    }
}

fn main() {
    let file = "inputs.txt";
    // call the function to read from file
    // check the return value and handle as needed
    match read_from_file(file){
        Ok(txt) => println!("File contents => {}", txt),
        Err(error) => println!("Error reading!! => {}", error)
    }   
}
```

The return value of our **`read_from_file()`** function is of the type **`Result<String, io::Error>`**. Within the body of the function we check for the **`Result<T,E`** of other operations and if it is an **`Err`**, then we pass on/return that **`Err`** variant with its data as the result. If everything goes correctly we return the **`Ok`** variant with the value wrapped in it as the result.

Finally the calling code in **`main`** checks for the **`Result<String, io::Error>`** with a pattern match and decides to print the result value or the error.

##### Shortcut for Propagating Errors (the **`?`** operator)





