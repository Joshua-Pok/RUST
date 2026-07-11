<!--markdownlint-disable-->


# Declaring Variables

We declare in rust using the **let** keywod


By default all varuavkes in rust are **immutable by default**


IF we want a variable to be mutable we need to add the **mut** keyword


```Rust
let mut x = 5;

```


# Declaring Constants

We declare constants with the **const** keyword


constants cannot be used with the **mut** keyword for obvious reasons


Generally constants names are fully capitalized


# Shadowing


Shadowing is when we declare a new variable with the same name as a previous variable. > [!TIP]
>
Rustaceans say that the first variable is **shadowed** by the second. Which means that second variable is what the compiler will see when using the name of the variable


Eg:

```Rust
fn main(){
let x = 5;
let x = x + 1; //x = 6

{
let x = x * 2;
println!("The value of x in the inner scope is {x}"); // x = 12
}

println!("The value of x is: {x}";) //x == 6
}



```


Difference between shadowing ant mut is that we will get a compile time error if we try to reassign this variable without the **let** keyword


The other difference is that we are effectively creating a new variable again when we use the **let** keyword


# Data Types


## Integer Types in Rust

Integer types is just whole number


we can declare  integer type of various sizees, in multiples of 8 bits, i for signed, u for unsigned


signed 64 bits would be i64

unsigned 64 bits would be u64

Signed numbers are stored in 2's complement'



# Tuple

Tuples are a general way of grouping together a number of values with a variety of types into one compound type

eg:

```Rust

let tup: (i32, f64, u8) = (500, 6.4, 1);
```


We access values in the tuple with (.) dot notation

eg
```Rust
let five_hundred = tup.0
```



