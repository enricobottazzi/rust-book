# rust-book
Me learning RUST

### Interesting Stuff

- Rust is an ahead-of-time compiled language, meaning you can compile a program and give the executable to someone else, and they can run it even without having Rust installed.
- Use `cargo check` to check for errors without compiling. Use `cargo run` to compile and run the program.
- In Rust, variables are immutable by default, meaning once we give the variable a value, the value won’t change. We can make a variable mutable by adding the mut keyword before the variable name.
- The `&` indicates that this argument is a reference, which gives you a way to let multiple parts of your code access one piece of data without needing to copy that data into memory multiple times.
- What about constants? Like immutable variables, constants are values that are bound to a name and are not allowed to change, but there are a few differences between constants and variables. First, you aren’t allowed to use mut with constants. Constants aren’t just immutable by default—they’re always immutable. You declare constants using the const keyword instead of the let keyword, and the type of the value must be annotated. The last difference is that constants may be set only to a constant expression, not the result of a value that could only be computed at runtime.
- A scalar type represents a single value. Rust has four primary scalar types: integers, floating-point numbers, Booleans, and characters. You may recognize these from other programming languages. Let’s jump into how they work in Rust.
- Compound types can group multiple values into one type. Rust has two primitive compound types: tuples and arrays. 
- Vectors are similar to arrays, but they can grow or shrink in size.
- Return expression do not need a semicolon. If you add a semicolon to the end of an expression, you turn it into a statement, which will then not return a value.
- Ownership is a set of rules that governs how a Rust program manages memory.
- Double free memory problem => Earlier, we said that when a variable goes out of scope, Rust automatically calls the drop function and cleans up the heap memory for that variable. But Figure 4-2 shows both data pointers pointing to the same location. This is a problem: when s2 and s1 go out of scope, they will both try to free the same memory. This is known as a double free error and is one of the memory safety bugs we mentioned previously. Freeing memory twice can lead to memory corruption, which can potentially lead to security vulnerabilities. To ensure memory safety, after the line let s2 = s1, Rust considers s1 as no longer valid. Therefore, Rust doesn’t need to free anything when s1 goes out of scope. Check out what happens when you try to use s1 after s2 is created; it won’t work.
- Copy trait for types that are stored on the stack. => Rust has a special annotation called the Copy trait that we can place on types that are stored on the stack, as integers are (we’ll talk more about traits in Chapter 10). If a type implements the Copy trait, variables that use it do not move, but rather are trivially copied, making them still valid after assignment to another variable. Rust won’t let us annotate a type with Copy if the type, or any of its parts, has implemented the Drop trait
- The double free memory problem also applies to functions. If we try to use a variable after it’s been moved to another variable, we’ll get an error. This is because the variable we’re trying to use has been invalidated by the move => https://rust-book.cs.brown.edu/ch04-01-what-is-ownership.html#ownership-and-functions
-The ownership of a variable follows the same pattern every time: assigning a value to another variable moves it. When a variable that includes data on the heap goes out of scope, the value will be cleaned up by drop unless ownership of the data has been moved to another variable.
- What is the problem of ownership? Let's take this example: 
```rust
fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: String) -> usize {
    let length = s.len(); // len() returns the length of a String

    length
}
```

The problem is that we are trying to use s1 after it has been moved to the calculate_length function. This is not allowed in Rust. Therefore the print will not work. There are two solutions to this problem:

1. Clone the string. Not very efficient and not very elegant. 
```rust
fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(s1.clone());

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: String) -> (usize) {
    let length = s.len(); // len() returns the length of a String

    length
}
```

2. We can use tuples to let the calculate_length function return multiple values. This way, we can return the length of the string and the string itself. Not elegant and also a bit tedious.

```rust
fn main() {
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}
```

3. The solution is to pass a reference to the string instead of the string itself. This way, the string will not be moved and we can still use it after the function call. Ownership is not transferred to the function. The function only borrows the string. This is called borrowing. 

```rust
fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
```

The &s1 syntax lets us create a reference that refers to the value of s1 but does not own it. s1 doesn't get moved to the function and remains valid inside its scope. 


- Mutable and immutable references. References are immutable by default. This will result in a compile error. 

```rust
fn main() {
    let s = String::from("hello");

    change(&s);
}

fn change(some_string: &String) {
    some_string.push_str(", world");
}
```

We can make them mutable by adding the mut keyword. 

```rust
fn main() {
    let mut s = String::from("hello");

    change(&mut s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
```

- One limit to the mutable references is that we can only have one mutable reference to a particular piece of data INSIDE THE SAME SCOPE. This code will not compile. 

```rust 
fn main() {
    let mut s = String::from("hello");

    let r1 = &mut s;
    let r2 = &mut s;

    println!("{}, {}", r1, r2);
}
```

- A similar rule exists for combining mutable and immutable references INSIDE THE SAME SCOPE. This code will not compile. 

```rust
fn main() {
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    let r3 = &mut s; // BIG PROBLEM

    println!("{}, {}, and {}", r1, r2, r3);
}
```

It's very important to understand the REFERENCE SCOPE RULE. A reference's scope starts from where it is introduced and continues through the last time that reference is used. For instance, this code will compile. 

```rust
fn main() {
    let mut s = String::from("hello");

    let r1 = & mut s; // no problem
    println!("{}", r1); // r1 is no longer used after this point

    let r3 = &mut s; // no problem
    println!("{}", r3);
}
```

This one as well.

```rust
fn main() {
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2); // r1 and r2 are no longer used after this point

    // r1 and r2 are no longer used after this point
    let r3 = &mut s; // no problem
    println!("{}", r3);
}
```

The scopes of the immutable references r1 and r2 end after the println! where they are last used, which is before the mutable reference r3 is created. These scopes don’t overlap, so this code is allowed

- Dereferencing. We can use the dereference operator * to access the value that a reference points to. In Rust, references are used to refer to values stored in memory, and dereferencing a reference allows you to access the value that the reference points to.



```rust
fn incr(n: &mut i32) {
    *n += 1;
  }
  fn main() {
    let mut n = 1;
    incr(& mut n);
    println!("{n}");
  }
```

Not always we need to use the dereference operator. 

```rust
fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
``` 

In this case the dereferencing operator is not used to access the length of s. 
s is a reference to a String value, and the len method is automatically available on references to String values. This is because the String type implements the Deref trait, which allows references to String values to be automatically dereferenced to str values.

- The String Slice. Let's start with the string slice; a string slice is a reference to part of a `String`. Slices are of type &str. 

```rust
fn main() {
    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];

    println!("{}, {}", hello, world);
}
```

The slice directly points to the string data stored in the memory. 

All string literals are slices. 

```rust
fn main() {
    let s = "Hello, world!";

    println!("{}", s);
}
```

The type of s here is &str: it’s a slice pointing to that specific point of the binary. This is also why string literals are immutable; &str is an immutable reference. This is different from the String type, which is growable and mutable.

A reference to a String is equivalent to a slice of the entire String. 

```rust
fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}
```

- Structs are similar to tuples, discussed in “The Tuple Type” section, in that both hold multiple related values. Like tuples, the pieces of a struct can be different types. Unlike with tuples, in a struct you’ll name each piece of data so it’s clear what the values mean. A struct is defined by setting a set of `fields` and their `types`. 

```rust
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
```

- To use a struct after we define it, we create an instance of that struct by specifying concrete values for each of the fields. We create an instance by stating the name of the struct. 

```rust
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
```

- If I want an instance of a Struct to be mutable, I need to add the mut keyword before the struct name. 

```rust
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
```

- The debug trait is pretty useful. The dbg! macro takes ownership of an expression, prints the file and line number where the macro is called, and then returns the expression. Sometimes we might want to print a value without taking ownership of it. In this case, we can use the & operator to create a reference to the value.

```rust
fn main() {
    let s = String::from("hello");
    dbg!(&s);
    dbg!(s);
}
```

- Methods can take ownership of self, borrow self immutably as we’ve done here, or borrow self mutably, just as they can any other parameter. We’ve chosen &self here for the same reason we used &Rectangle in the function version: we don’t want to take ownership, and we just want to read the data in the struct, not write to it. If we wanted to change the instance that we’ve called the method on as part of what the method does, we’d use &mut self as the first parameter.

- Creatting getters methods on a struct. Getters are useful because you can make the field private but the method public and thus enable read-only access to that field as part of the type’s public API. 

```rust
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn width(&self) -> u32 {
        self.width
    }
}
```

- Associated functions of a struct don't need to take self as a parameter. These type of functions are often used for constructors that will return a new instance of the struct. 

```rust
impl Rectangle {
    fn square(size: u32) -> Self {
        Self { width: size, height: size }
    }
}
```

- Interesting example to understand mutability in Rust

```rust

struct Point {
  x: i32,
  y: i32
}
impl Point {
  fn get_x(&mut self) -> &mut i32 {
    &mut self.x
  }
}
fn main() {
  let mut p = Point { x: 1, y: 2 };
  let x = p.get_x();
  *x += 1;
  println!("{} {}", *x, p.y);
}
```

This wont compile because get_x mutably borrows all of p, a program cannot use p in any way until x is no longer used. Therefore reading x and p.y in the same line is an ownership error.

In order to fix this, we can simply read x and p.y in two different lines!. 

```rust
struct Point {
    x: i32,
    y: i32
  }
  impl Point {
    fn get_x(&mut self) -> &mut i32 {
      &mut self.x
    }
  }
  fn main() {
    let mut p = Point { x: 1, y: 2 };
    let x = p.get_x();
    *x += 1;
    println!("{}", *x);
    println!("{}", p.y);

  }
```

- Enums give you a way of saying a value is one of a possible set of values. 

```rust
enum IpAddrKind {
    V4,
    V6,
}
```

Now IpAddrKind is a custom data type.

```
let four = IpAddrKind::V4;
let six = IpAddrKind::V6;
```

- We can also use enums as functions that take arguments. 

```rust
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

let home = IpAddr::V4(127, 0, 0, 1);
let loopback = IpAddr::V6(String::from("::1"));
```

- We can also define methods on enums. 

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        // method body would be defined here
    }
}

let m = Message::Write(String::from("hello"));
m.call();

```

- The Option enum is as enum defined in the standard library. The Option type encodes the very common scenario in which a value could be something or it could be nothing.

```rust
enum Option<T> {
    None,
    Some(T),
}
```

You can use it without the Option:: prefix because it is included into Rust’s prelude. 
The <T> syntax is a feature of Rust called generics. It means that the Some variant of the Option enum can hold one piece of data of any type. Based on this type, the type of Option itself will be different. 

```rust
    let some_number = Some(5);
    let some_char = Some('e');

    let absent_number: Option<i32> = None;
```

The type of some_number is Option<i32>. The type of some_char is Option<char>, which is a different type. Rust can infer these types because we’ve specified a value inside the Some variant. For absent_number, Rust requires us to annotate the overall Option type: the compiler can’t infer the type that the corresponding Some variant will hold by looking only at a None value. Here, we tell Rust that we mean for absent_number to be of type Option<i32>.

Everywhere that a value has a type that isn’t an Option<T>, you can safely assume that the value isn’t null.

IT MEANS THAT IF YOU HAVE A VALUE THAT CAN BE NULL, YOU SHOULD USE AN OPTION<T> TO DEFINE IT!


