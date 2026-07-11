<!--markdownlint-disable-->

# Anatomy of a Rust Program

main is the entry point of rust programs, like most languages.

functions are wrapped in {}



# Compilation and Execution

rustc <file>.rs is used to **compile** the file


After compilation rust outputs a binary that we can use to run  by ./<binary>


# Cargo

Cargo is Rust's build system and package manager


we can create a new project using cargo using
```Rust
cargo new <project name>
```


cargo  new creates a Cargo.toml file, as well as a git repo and .gitignore


cargo.toml is where we list our dependencies and project configuration


cargo expects our source file to live in the **src** directory


## Building and Running with Cargo

We can use Cargo build to build our code

Default build is a debug build so it will live in **./target/debug/hello_cargo**


**cargo run* is used to compile and run the code all in one command


**cargo check** is used to check our code compiles but does not produce a executable


When our project is ready we can use **cargo build --release** to compile it with optimizations. This will output the executable in the **target/release** folder instead of target/debug
# Macros



# imports

imports in rust are the **use** keyword


macros are called with a ! at the endo of the name


# Result

Result is a enumeration that can be one of multiple possible states. We call each state a variant


Values of the Result type have methods. An instance of Result has a expect method that you can call. If this instance of result is an err, expect will crash the program and display the message that you passed to expect


```Rust
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");


```

if we dont call expect() the program will still compile but we will get a warning


# Crates

crates are packages in rust

we add these to the dependencies section of **cargo.toml

we then run cargo build to build our project with the dependencies



## Updating Crates

When we want to update Crates, cargo provides the **cargo update** command which will ignore the Cargo.Lock file and find all the latest versions that fit our specifications

# Match expression


A match expression is  Rust's switch. A match consists of Arms. An arm is a pattern to match against


Rust takes the value given to match and looks thru each arm's pattern
