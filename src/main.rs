// fmt::Debug and derive(Debug) example
// use std::fmt;

// #[derive(Debug)] // Automatically implements Debug for the struct
// struct Point {
//     x: i32,
//     y: i32,
// }

// // Manually implementing Debug for a struct
// struct Circle {
//     radius: f64,
// }

// impl fmt::Debug for Circle {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "Circle with radius: {}", self.radius)
//     }
// }

// fn main() {
//     let point: Point = Point { x: 10, y: 20 };
//     let circle: Circle = Circle { radius: 5.0 };

//     // Using the Debug trait to print the structs
//     println!("{:?}", point);   // Output: Point { x: 10, y: 20 }
//     println!("{:?}", circle);  // Output: Circle with radius: 5
// }

// fmt::Debug Example
// use std::fmt;

// struct Point {
//     x: i32,
//     y: i32,
// }

// impl fmt::Debug for Point {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "Point || x-> {}, y-> {} ||", self.x, self.y)
//     }
// }

// fn main() {
//     let origin: Point = Point { x: 0, y: 0 };
//     println!("The origin is: {:?}", origin);
// }

// fmt::Display example
// use std::fmt;

// struct Point {
//     x: i32,
//     y: i32,
// }

// impl fmt::Display for Point {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "({}, {})", self.x, self.y)
//     }
// }

// fn main() {
//     let point: Point = Point { x: 1, y: 2 };
//     println!("{}", point); // Output: (1, 2)
// }

// use std::fmt;

// // Define a Point struct with two fields
// struct Point {
//     x: i32,
//     y: i32,
// }

// // Implement fmt::Display for the Point struct
// impl fmt::Display for Point {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         // User-friendly format
//         write!(f, "Point({}, {})", self.x, self.y) // Display output
//     }
// }

// // Implement fmt::Debug for the Point struct (can also derive)
// impl fmt::Debug for Point {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         // Detailed debug format
//         write!(f, "Point({}, {})", self.x, self.y) // Debug output
//     }
// }

// fn main() {
//     let point = Point { x: 1, y: 2 };

//     // Using Display
//     println!("Display: {}", point); // Output: Display: Point(1, 2)

//     // Using Debug
//     println!("Debug: {:?}", point); // Output: Debug: Point { x: 1, y: 2 }
// }

// macros

// This is a simple macro named `say_hello`.
// macro_rules! say_hello {
//     // `()` indicates that the macro takes no argument.
//     () => {
//         // The macro will expand into the contents of this block.
//         println!("Hello!")
//     };
// }

// fn main() {
//     // This call will expand into `println!("Hello!")`
//     say_hello!()
// }

// macro_rules! vec_of {
//     ($($x:expr),*) => {
//         {
//             println!("Creating a vector with: {:?}", vec![$($x),*]);
//             vec![$($x),*]
//         }
//     };
// }

// fn main() {
//     let my_vec: Vec<i32> = vec_of!(1, 2, 3, 4);
//     println!("{:?}", my_vec); // Outputs: [1, 2, 3, 4]
// }

// macro_rules! vec_of {
//     ($($x:expr),*) => {{
//         let vec = vec![$($x),*];
//         println!("Created vector: {:?}", vec);
//         assert!(vec.iter().all(|&x| x > 0), "All values must be positive");
//         vec
//     }};
// }

// fn main() {
//     let my_vec: Vec<i32> = vec_of!(1, 2, 3, 4); // Works with logging and validation
//         println!("{:?}", my_vec); // Outputs: [1, 2, 3, 4]
// }

// dynamic functions generatiosn using macro

// macro_rules! create_math_functions {
//     ($($name:ident),*) => {
//         $(
//             fn $name(x: i32) -> i32 {
//                 x + 1 // Example: each function adds 1 to the input
//             }
//         )*
//     };
// }

// // Use the macro to create multiple functions
// create_math_functions!(add_one, add_two, add_three);

// fn main() {
//     println!("add_one(5): {}", add_one(5)); // Outputs: add_one(5): 6
//     println!("add_two(5): {}", add_two(5)); // Outputs: add_two(5): 6
//     println!("add_three(5): {}", add_three(5)); // Outputs: add_three(5): 6
// }

// complex examples for macros
// macro_rules! create_configs {
//     ($($p:ident),*) => {
//         $(
//             #[derive(Debug)]
//             struct $p {
//                 host: String,
//                 port: u16,
//                 database: String,
//             }

//             impl $p {
//                 fn new(host: &str, port: u16, database: &str) -> Self {
//                     Self {
//                         host: host.to_string(),
//                         port,
//                         database: database.to_string(),
//                     }
//                 }
//             }
//         )*
//     };
// }

// // Use the macro to create multiple configuration structs
// create_configs!(DevelopmentConfig, ProductionConfig, StagingConfig);

// fn main() {
//     let dev_config: DevelopmentConfig = DevelopmentConfig::new("localhost", 5432, "dev_db");
//     let prod_config: ProductionConfig = ProductionConfig::new("prod.example.com", 5432, "prod_db");
//     let staging_config: StagingConfig =
//     StagingConfig::new("stg.example.com", 5432, "prod_db");

//     println!(
//         "Dev Config: {:?}, Prod Config: {:?}, Staging Config: {:?}",
//         dev_config, prod_config, staging_config
//     );
// }

// #[derive(Debug)]
// enum Mixed {
//     Int(i32),
//     Float(f64),
//     Text(String),
//     FFF(i128),
// }

// fn main() {
//     // Signed Integers
//     let a: i8 = -128;
//     let b: i16 = -32_768;
//     let c: i32 = -2_147_483_648;
//     let d: i64 = -9_223_372_036_854_775_808;
//     let e: i128 = -170_141_183_460_469_231_731_687_303_715_884_105_728;
//     let f: isize = -1;

//     // Unsigned Integers
//     let g: u8 = 255;
//     let h: u16 = 65_535;
//     let i: u32 = 4_294_967_295;
//     let j: u64 = 18_446_744_073_709_551_615;
//     let k: u128 = 340_282_366_920_938_463_463_374_607_431_768_211_455;
//     let l: usize = 1;

//     // Floating Point
//     let m: f32 = 3.14;
//     let n: f64 = 2.718281828459;

//     // char
//     let o: char = 'a';
//     let p: char = 'α';
//     let q: char = '∞';

//     // bool
//     let r: bool = true;
//     let s: bool = false;

//     // Unit Type
//     let unit: () = ();

//     // Print all values
//     println!("i8: {:?}", a);
//     println!("i16: {}", b);
//     println!("i32: {}", c);
//     println!("i64: {}", d);
//     println!("i128: {}", e);
//     println!("isize: {}", f);

//     println!("u8: {}", g);
//     println!("u16: {}", h);
//     println!("u32: {}", i);
//     println!("u64: {}", j);
//     println!("u128: {}", k);
//     println!("usize: {}", l);

//     println!("f32: {}", m);
//     println!("f64: {}", n);

//     println!("char: {}", o);
//     println!("char: {}", p);
//     println!("char: {}", q);

//     println!("bool: {}", r);
//     println!("bool: {}", s);

//     println!("unit: {:?}", unit);

//     let person: (&str, u32, f64) = ("Alice", 30, 5.5);

//     // Accessing tuple elements
//     let name: &str = person.0; // "Alice"
//     let age: u32 = person.1; // 30
//     let height: f64 = person.2; // 5.5

//     // Printing the tuple and its elements
//     println!("Person: {:?}", person);
//     println!("Name: {}, Age: {}, Height: {}", name, age, height);

//     // Defining an array
//     let numbers: [i32; 5] = [1, 2, 3, 4, 5];

//     // Accessing array elements
//     let first: i32 = numbers[0]; // 1
//     let second: i32 = numbers[1]; // 2

//     // Printing the array and its elements
//     println!("Numbers: {:?}", numbers);
//     println!("First: {}, Second: {}", first, second);

//     let student: (&str, [u32; 3]) = ("Bob", [85, 90, 78]);

//     // Accessing tuple elements
//     let name: &str = student.0; // "Bob"
//     let scores: [u32; 3] = student.1; // [85, 90, 78]

//     // Printing the tuple and its elements
//     println!("Student: {}", name);
//     println!("Scores: {:?}", scores);

//     // Array of tuples with mixed data types
//     let mixed_array: [(&str, i32, f64); 3] = [
//         ("Alice", 30, 5.5), // Tuple: (&str, u32, f64)
//         ("Bob", 25, 6.0),
//         ("Charlie", 35, 5.8),
//     ];

//     // Accessing elements
//     for (name, age, height) in mixed_array.iter() {
//         println!("Name: {}, Age: {}, Height: {}", name, age, height);
//     }

//     // Create a Vec to hold mixed types
//     let mut mixed_vec: Vec<Mixed> = Vec::new();

//     // Add different types to the Vec
//     mixed_vec.push(Mixed::Int(42));
//     mixed_vec.push(Mixed::Float(3.14));
//     mixed_vec.push(Mixed::Text(String::from("Hello, Rust!")));
//     print!("{:?}-=--", mixed_vec);
//     // Iterate and print each item
//     for item in &mixed_vec {
//         match item {
//             Mixed::Int(i) => println!("Integer: {}", i),
//             Mixed::Float(f) => println!("Float: {}", f),
//             Mixed::Text(s) => println!("Text: {}", s),
//             Mixed::FFF(s) => println!("FFF: {}", s),
//         }
//     }

//     // let _immutable_binding = 1;
//     let mut mutable_binding = 1;

//     println!("Before mutation: {}", mutable_binding);

//     // Ok
//     mutable_binding += 1;
//     mutable_binding += 1;
//     mutable_binding += 1;
//     mutable_binding += 1;
//     mutable_binding += 1;

//     println!("After mutation: {}", mutable_binding);

//     // Error! Cannot assign a new value to an immutable variable
//     // _immutable_binding += 1;
// }

// Tuples can be used as function arguments and as return values.
// fn reverse(pair: (i32, bool)) -> (bool, i32) {
//     // `let` can be used to bind the members of a tuple to variables.
//     let (int_param, bool_param) = pair;

//     (bool_param, int_param)
// }

// // The following struct is for the activity.
// #[derive(Debug)]
// struct Matrix(f32, f32, f32, f32);

// fn main() {
//     // A tuple with a bunch of different types.
//     let long_tuple: (u8, u16, u32, u64, i8, i16, i32, i64, f32, f64, char, bool) = (1u8, 2u16, 3u32, 4u64,
//                       -1i8, -2i16, -3i32, -4i64,
//                       0.1f32, 0.2f64,
//                       'a', true);

//     // Values can be extracted from the tuple using tuple indexing.
//     println!("Long tuple first value: {}", long_tuple.0);
//     println!("Long tuple second value: {}", long_tuple.1);

//     // Tuples can be tuple members.
//     let tuple_of_tuples: ((u8, u16, u32), (u64, i8), i16) = ((1u8, 2u16, 2u32), (4u64, -1i8), -2i16);

//     // Tuples are printable.
//     println!("tuple of tuples: {:?}", tuple_of_tuples);

//     // But long Tuples (more than 12 elements) cannot be printed.
//     //let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
//     //println!("Too long tuple: {:?}", too_long_tuple);
//     // TODO ^ Uncomment the above 2 lines to see the compiler error

//     let pair = (1, true);
//     println!("Pair is {:?}", pair);

//     println!("The reversed pair is {:?}", reverse(pair));

//     // To create one element tuples, the comma is required to tell them apart
//     // from a literal surrounded by parentheses.
//     println!("One element tuple: {:?}", (5u32,));
//     println!("Just an integer: {:?}", (5u32));

//     // Tuples can be destructured to create bindings.
//     let tuple: (i32, &str, f64, bool) = (1, "hello", 4.5, true);

//     let (a, b, c, d) = tuple;
//     println!("{:?}, {:?}, {:?}, {:?}", a, b, c, d);

//     let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
//     println!("{:?}", matrix);
// }

// use std::mem;

// // This function borrows a slice.
// fn analyze_slice(slice: &[i32]) {
//     println!("First element of the slice: {}", slice[0]);
//     println!("The slice has {} elements", slice.len());
// }

// Arrays and slices
// fn main() {
//     // // Fixed-size array (type signature is superfluous).
//     // let xs: [i32; 5] = [1, 2, 3, 4, 5];

//     // // All elements can be initialized to the same value.
//     // let ys: [i32; 500] = [0; 500];
//     // // Indexing starts at 0.
//     // println!("First element of the array: {}", xs[0]);
//     // println!("Second element of the array: {}", xs[1]);

//     // // `len` returns the count of elements in the array.
//     // println!("Number of elements in array: {}", xs.len());

//     // // Arrays are stack allocated.
//     // println!("Array occupies {} bytes", mem::size_of_val(&xs));

//     // // Arrays can be automatically borrowed as slices.
//     // println!("Borrow the whole array as a slice.");
//     // analyze_slice(&xs);

//     // // Slices can point to a section of an array.
//     // // They are of the form [starting_index..ending_index].
//     // // `starting_index` is the first position in the slice.
//     // // `ending_index` is one more than the last position in the slice.
//     // println!("Borrow a section of the array as a slice.");
//     // analyze_slice(&ys[1 .. 4]);

//     // // Example of empty slice `&[]`:
//     // let empty_array: [u32; 0] = [];
//     // assert_eq!(&empty_array, &[]);
//     // assert_eq!(&empty_array, &[][..]); // Same but more verbose

//     // // Arrays can be safely accessed using `.get`, which returns an
//     // // `Option`. This can be matched as shown below, or used with
//     // // `.expect()` if you would like the program to exit with a nice
//     // // message instead of happily continue.
//     // for i in 0..xs.len() + 1 { // Oops, one element too far!
//     //     match xs.get(i) {
//     //         Some(xval) => println!("{}: {}", i, xval),
//     //         None => println!("Slow down! {} is too far!", i),
//     //     }
//     // }

//     // Out of bound indexing on array causes compile time error.
//     //println!("{}", xs[5]);
//     // Out of bound indexing on slice causes runtime error.
//     //println!("{}", xs[..][5]);

//     let array: [i32; 5] = [1, 2, 3, 4, 5];
//     let slice_from_array: &[i32] = &array[1..4]; // Slices elements 2, 3, and 4
//     println!("{:?}", slice_from_array); // Output: [2, 3, 4]

//     let vec: Vec<i32> = vec![10, 20, 30, 40, 50];
//     let slice_from_vec: &[i32] = &vec[2..5]; // Slices elements 30, 40, and 50
//     println!("{:?}", slice_from_vec); // Output: [30, 40, 50]

// }

// statics

// static GREETING: &str = "Hello, world!";

// static mut COUNTER: i32 = 0;

// fn main() {
//     println!("{}", GREETING);

//     unsafe {
//         COUNTER += 1; // mutation requires unsafe
//         println!("Counter: {}", COUNTER);
//     }
// }

// structures

// An attribute to hide warnings for unused code.
// #![allow(dead_code)]

// #[derive(Debug)]
// struct Person {
//     name: String,
//     age: u8,
// }

// // A unit struct
// struct Unit;

// // A tuple struct
// struct Pair(i32, f32);

// // A struct with two fields
// struct Point {
//     x: f32,
//     y: f32,
// }

// // Structs can be reused as fields of another struct
// struct Rectangle {
//     // A rectangle can be specified by where the top left and bottom right
//     // corners are in space.
//     top_left: Point,
//     bottom_right: Point,
// }

// fn main() {
//     // Create struct with field init shorthand
//     let name: String = String::from("Peter");
//     let age: u8 = 27;
//     let peter: Person = Person { name, age };
//     let cpeter: Person = Person { name:"chaeter".to_string(), age:255 };

//     // Print debug struct
//     println!("{:?}", peter);
//     println!("{:?}", cpeter);

//     // Instantiate a `Point`
//     let point: Point = Point { x: 10.3, y: 0.4 };
//     let another_point: Point = Point { x: 5.2, y: 0.2 };

//     // Access the fields of the point
//     println!("point coordinates: ({}, {})", point.x, point.y);

//     // Make a new point by using struct update syntax to use the fields of our
//     // other one
//     let bottom_right: Point = Point { y: 5.10, ..another_point };

//     // `bottom_right.y` will be the same as `another_point.y` because we used that field
//     // from `another_point`
//     println!("second point: ({}, {})", bottom_right.x, bottom_right.y);

//     // Destructure the point using a `let` binding
//     let Point { x: left_edge, y: top_edge } = point;

//     let _rectangle = Rectangle {
//         // struct instantiation is an expression too
//         top_left: Point { x: left_edge, y: top_edge },
//         bottom_right: bottom_right,
//     };

//     // // Instantiate a unit struct
//     let _unit: Unit = Unit;

//     // // Instantiate a tuple struct
//     let pair: Pair = Pair(1, 0.1);

//     // // Access the fields of a tuple struct
//     println!("pair contains {:?} and {:?}", pair.0, pair.1);

//     // Destructure a tuple struct
//     let Pair(integer, decimal) = pair;

//     println!("pair contains {:?} and {:?}", integer, decimal);
// }

// Create an `enum` to classify a web event. Note how both
// names and type information together specify the variant:
// `PageLoad != PageUnload` and `KeyPress(char) != Paste(String)`.
// Each is different and independent.
// enum WebEvent {
//     // An `enum` variant may either be `unit-like`,
//     PageLoad,
//     PageUnload,
//     // like tuple structs,
//     KeyPress(char),
//     Paste(String),
//     // or c-like structures.
//     Click { x: i64, y: i64 },
// }

// // A function which takes a `WebEvent` enum as an argument and
// // returns nothing.
// fn inspect(event: WebEvent) {
//     match event {
//         WebEvent::PageLoad => println!("page loaded"),
//         WebEvent::PageUnload => println!("page unloaded"),
//         // Destructure `c` from inside the `enum` variant.
//         WebEvent::KeyPress(c) => println!("pressed '{}'.", c),
//         WebEvent::Paste(s) => println!("pasted \"{}\".", s),
//         // Destructure `Click` into `x` and `y`.
//         WebEvent::Click { x, y } => {
//             println!("clicked at x={}, y={}.", x, y);
//         },
//     }
// }

// fn main() {
//     let pressed: WebEvent = WebEvent::KeyPress('x');
//     // `to_owned()` creates an owned `String` from a string slice.
//     let pasted: WebEvent  = WebEvent::Paste("my text".to_owned());
//     let click: WebEvent   = WebEvent::Click { x: 20, y: 80 };
//     let load: WebEvent    = WebEvent::PageLoad;
//     let unload: WebEvent  = WebEvent::PageUnload;

//     inspect(pressed);
//     inspect(pasted);
//     inspect(click);
//     inspect(load);
//     inspect(unload);
// }

// Define an enum with more operations
// enum VeryVerboseEnumOfThingsToDoWithNumbers {
//     Add,
//     Subtract,
//     Multiply,
//     Divide,
// }

// // Create a type alias
// type Operations = VeryVerboseEnumOfThingsToDoWithNumbers;

// // Function to perform the operation
// fn perform_operation(op: Operations, a: f64, b: f64) -> Option<f64> {
//     match op {
//         Operations::Add => Some(a + b),
//         Operations::Subtract => Some(a - b),
//         Operations::Multiply => Some(a * b),
//         Operations::Divide => {
//             if b != 0.0 {
//                 Some(a / b)
//             } else {
//                 None // Handle division by zero
//             }
//         }
//     }
// }

// fn main() {
//     // Using the type alias
//     let op1: VeryVerboseEnumOfThingsToDoWithNumbers = Operations::Add;
//     let op2: VeryVerboseEnumOfThingsToDoWithNumbers = Operations::Subtract;
//     let op3: VeryVerboseEnumOfThingsToDoWithNumbers = Operations::Multiply;
//     let op4: VeryVerboseEnumOfThingsToDoWithNumbers = Operations::Divide;

//     let a: f64 = 10.0;
//     let b: f64 = 5.0;

//     // Perform operations and print results
//     if let Some(result) = perform_operation(op1, a, b) {
//         println!("10 + 5 = {}", result);
//     }

//     if let Some(result) = perform_operation(op2, a, b) {
//         println!("10 - 5 = {}", result);
//     }

//     if let Some(result) = perform_operation(op3, a, b) {
//         println!("10 * 5 = {}", result);
//     }

//     if let Some(result) = perform_operation(op4, a, b) {
//         println!("10 / 5 = {}", result);
//     } else {
//         println!("Cannot divide by zero!");
//     }
// }

// enum VeryVerboseEnumOfThingsToDoWithNumbers {
//     Add,
//     Subtract,
//     Multiply,
//     Divide,
// }

// impl VeryVerboseEnumOfThingsToDoWithNumbers {
//     fn run(&self, x: i32, y: i32) -> Option<i32> {
//         match self {
//             Self::Add => Some(x + y),
//             Self::Subtract => Some(x - y),
//             Self::Multiply => Some(x * y),
//             Self::Divide => {
//                 if y != 0 {
//                     Some(x / y)
//                 } else {
//                     None // Handle division by zero
//                 }
//             }
//         }
//     }
// }

// fn main() {
//     let operation: VeryVerboseEnumOfThingsToDoWithNumbers = VeryVerboseEnumOfThingsToDoWithNumbers::Divide;
//     let x: i32 = 10;
//     let y: i32 = 5;

//     if let Some(result) = operation.run(x, y) {
//         println!("Result: {}", result);
//     } else {
//         println!("Error: Division by zero!");
//     }
// }

// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// impl Rectangle {
//     // Method to calculate area
//     fn area(&self) -> u32 {
//         return self.width * self.height;
//     }

//     // Method to check if it is a square
//     fn is_square(&self) -> bool {
//         return self.width == self.height;
//     }

//     fn represent(&self) {
//         print!("This is a Rectangle structure with width and height")
//     }
// }

// fn main() {
//     let rect: Rectangle = Rectangle {
//         width: 10,
//         height: 5,
//     };

//     println!("Area: {}", rect.area());
//     println!("Is square: {}", rect.is_square());
//     rect.represent();
// }

// Define a trait for shapes
// trait Shape {
//     fn area(&self) -> f64; // Method to calculate area
//     fn perimeter(&self) -> f64; // Method to calculate perimeter
// }

// // Define a struct for Rectangle
// struct Rectangle {
//     width: f64,
//     height: f64,
// }

// // Implement the Shape trait for Rectangle
// impl Shape for Rectangle {
//     fn area(&self) -> f64 {
//         self.width * self.height // Area of rectangle
//     }
//     fn perimeter(&self) -> f64 {
//         2.0 * (self.width + self.height) // Perimeter of rectangle
//     }
// }

// // Define a struct for Circle
// struct Circle {
//     radius: f64,
// }

// // Implement the Shape trait for Circle
// impl Shape for Circle {
//     fn area(&self) -> f64 {
//         std::f64::consts::PI * self.radius.powi(2) // Area of circle
//     }

//     fn perimeter(&self) -> f64 {
//         2.0 * std::f64::consts::PI * self.radius // Circumference of circle
//     }
// }

// fn main() {
//     let rect: Rectangle = Rectangle {
//         width: 10.0,
//         height: 5.0,
//     };

//     let circle: Circle = Circle { radius: 3.0 };

//     // Using the Shape trait methods
//     println!("Rectangle area: {}", rect.area());
//     println!("Rectangle perimeter: {}", rect.perimeter());

//     println!("Circle area: {}", circle.area());
//     println!("Circle perimeter: {}", circle.perimeter());
// }

// An attribute to hide warnings for unused code.
// #![allow(dead_code)]

// enum Stage {
//     Beginner,
//     Advanced,
//     Noob,

// }

// enum Role {
//     Student,
//     Teacher,
// }
// trait Exe {
//     fn execute(&self) {}
// }
// impl Exe for Role {
//     fn execute(&self) {
//         match self {
//             Role::Student => println!("Students are acquiring knowledge!"),
//             Role::Teacher => println!("Teachers are spreading knowledge!"),
//         }
//     }
// }

// impl Exe for Stage {
//     fn execute(&self) {
//         match self {
//             Stage::Beginner => println!("Beginner are acquiring knowledge!"),
//             Stage::Advanced => println!("Advanced are spreading knowledge!"),
//             Stage::Noob => println!("Noob are spreading knowledge!"),

//         }
//     }
// }
// fn main() {
//     use crate::Stage::{Advanced,Noob};
//     // Equivalent to `Stage::Advanced`.
//     let advance: Stage = Advanced;
//     let noob: Stage = Noob;

//     let beginner: Stage = Stage::Beginner;

//     // Equivalent to `Role::Student`.
//     let student: Role = Role::Student;
//     let teacher: Role = Role::Teacher;

//     // Calling the execute method on the role
//     student.execute();
//     teacher.execute();

//     advance.execute();
//     beginner.execute();
//     noob.execute();
// }

// An attribute to hide warnings for unused code.
// #![allow(dead_code)]

// // enum with implicit discriminator (starts at 0)
// enum Number {
//     Zero,
//     One,
//     Two,
//     Three,
//     Fiv,
// }

// // enum with explicit discriminator
// enum Color {
//     Red = 0xff0000,
//     Green = 0x00ff00,
//     Blue = 0x0000ff,
// }

// fn main() {
//     // `enums` can be cast as integers.
//     println!("zero is {}", Number::Zero as i32);
//     println!("one is {}", Number::One as i32);
//     println!("two is {}", Number::Fiv as i32);

//     println!("roses are #{:06x}", Color::Red as i32);
//     println!("violets are #{:06x}", Color::Blue as i32);
// }

// Suppress all warnings from casts which overflow.

// use crate::List::*;

// #[derive(Debug)]
// enum List {
//     // Cons: Tuple struct that wraps an element and a pointer to the next node
//     Fat(u32, Box<List>),
//     // Nil: A node that signifies the end of the linked list
//     Nil,
// }

// // Methods can be attached to an enum
// impl List {
//     #![allow(dead_code)]
//     // Create an empty list
//     fn new() -> List {
//         // `Nil` has type `List`
//         Nil
//     }

//     // Consume a list, and return the same list with a new element at its front
//     fn prepend(self, elem: u32) -> List {
//         // `Cons` also has type List
//         Fat(elem, Box::new(self))
//     }

//     fn append(self, elem: u32) -> List {
//         match self {
//             // If the list is empty, return a new list with the element
//             Nil => Fat(elem, Box::new(Nil)),
//             // If the list is not empty, recursively call append on the tail
//             Fat(head, tail) => Fat(head, Box::new(tail.append(elem))),
//         }
//     }

//     // Return the length of the list
//     fn len(&self) -> u32 {
//         // `self` has to be matched, because the behavior of this method
//         // depends on the variant of `self`
//         // `self` has type `&List`, and `*self` has type `List`, matching on a
//         // concrete type `T` is preferred over a match on a reference `&T`
//         // after Rust 2018 you can use self here and tail (with no ref) below as well,
//         // rust will infer &s and ref tail.
//         // See https://doc.rust-lang.org/edition-guide/rust-2018/ownership-and-lifetimes/default-match-bindings.html
//         match *self {
//             // Can't take ownership of the tail, because `self` is borrowed;
//             // instead take a reference to the tail
//             Fat(_, ref tail) => 1 + tail.len(),
//             // Base Case: An empty list has zero length
//             Nil => 0,
//         }
//     }

//     // Return representation of the list as a (heap allocated) string
//     fn stringify(&self) -> String {
//         match *self {
//             Fat(head, ref tail) => {
//                 // `format!` is similar to `print!`, but returns a heap
//                 // allocated string instead of printing to the console
//                 format!("{}, {}", head, tail.stringify())
//             }
//             Nil => {
//                 format!("Nil")
//             }
//         }
//     }
// }

// fn main() {
//     // Create an empty linked list
//     let mut list: List = List::new();
//     // Prepend some elements
//     list = list.append(1);
//     list = list.append(2);
//     print!("888888888888888____________{:?}___________\n", list);
//     list = list.append(3);
//     list = list.append(4);

//     // Show the final state of the list
//     println!(
//         "-----------------------linked list has length: {}",
//         list.len()
//     );
//     println!("{}", list.stringify());
// }

// fn main() {
//     let an_integer: u32 = 1u32;
//     let a_boolean: bool = true;
//     let unit: () = ();

//     // copy `an_integer` into `copied_integer`
//     let mut copied_integer: u32 = an_integer;
//     copied_integer+=1;
//     println!("Coppied integer: {:?}", copied_integer);
//     println!("An integer: {:?}", an_integer);

//     println!("A boolean: {:?}", a_boolean);
//     println!("Meet the unit value: {:?}", unit);

//     // The compiler warns about unused variable bindings; these warnings can
//     // be silenced by prefixing the variable name with an underscore
//     let _unused_variable: u32 = 3u32;

//     let _noisy_unused_variable: u32 = 2u32;
//     // FIXME ^ Prefix with an underscore to suppress the warning
//     // Please note that warnings may not be shown in a browser
// }

// fn main() {
//     let _immutable_binding: i32 = 1;
//     let mut mutable_binding: i32 = 1;

//     println!("Before mutation: {}", mutable_binding);

//     // Ok
//     mutable_binding += 1;
//     mutable_binding += 1;

//     println!("After mutation: {}", mutable_binding);

//     // Error! Cannot assign a new value to an immutable variable
//     // _immutable_binding += 1;
// }

// fn main() {
//     let x: i32 = 42;
//     let r: *const i32 = &x;
//     unsafe {
//         println!("{:?}", *r); // Dereferencing a raw pointer
//     }
// }

// fn main() {
//     // Suffixed literals, their types are known at initialization
//     let x: u8 = 1u8;
//     let y: u32 = 2u32;
//     let z: f32 = 3f32;

//     // Unsuffixed literals, their types depend on how they are used
//     let i: i32 = 1;
//     let f: f64 = 1.0;

//     // `size_of_val` returns the size of a variable in bytes
//     println!("size of `x` in bytes: {}", std::mem::size_of_val(&x));
//     println!("size of `y` in bytes: {}", std::mem::size_of_val(&y));
//     println!("size of `z` in bytes: {}", std::mem::size_of_val(&z));
//     println!("size of `i` in bytes: {}", std::mem::size_of_val(&i));
//     println!("size of `f` in bytes: {}", std::mem::size_of_val(&f));
// }
// fn first<T>(slice: &[T]) -> &T {
//     &slice[0] // Return a reference to the first element
// }
// `NanoSecond`, `Inch`, and `U64` are new names for `u64`.

// fn main() {
//     let my_str: &str = "hello";
//     let my_string: String = String::from(my_str);

// }

// use std::convert::Into;

// #[derive(Debug)]
// struct Number {
//     value: i128,
// }

// impl Into<Number> for i128 {
//     fn into(self) -> Number {
//         Number { value: self }
//     }
// }

// fn main() {
//     let int: i128 = 56;
//     // Try removing the type annotation
//     let num: Number = int.into();
//     println!("My number is {:?}", num);
// }

// use std::convert::From;

// #[derive(Debug)]
// struct Number {
//     value: i32,
// }

// // Define `From`
// impl From<i32> for Number {
//     fn from(item: i32) -> Self {
//         Number { value: item }
//     }
// }

// fn main() {
//     let int: i32 = 5;
//     // use `Into`
//     let num: Number = int.into();
//     println!("My number is {:?}", num);
// }
// struct MyStruct {
//     value: i32,
// }

// impl From<i32> for MyStruct {
//     fn from(value: i32) -> Self {
//         MyStruct { value }
//     }
// }
// fn take_my_struct(my_struct: MyStruct) {
//     println!("Received MyStruct with value: {}", my_struct.value);
// }

// fn main() {
//     let my_struct: MyStruct = MyStruct::from(20);
//     take_my_struct(my_struct);

//     // Using Into
//     let value: i32 = 30;
//     let my_struct_from_value: MyStruct = value.into();
//     take_my_struct(my_struct_from_value);
// }

// fn main() {
//     let n = 5;

//     if n < 0 {
//         print!("{} is negative", n);
//     } else if n > 0 {
//         print!("{} is positive", n);
//     } else {
//         print!("{} is zero", n);
//     }

//     let big_n: i32 =
//         if n < 10 && n > -10 {
//             println!(", and is a small number, increase ten-fold");

//             // This expression returns an `i32`.
//             10 * n
//         } else {
//             println!(", and is a big number, halve the number");

//             // This expression must return an `i32` as well.
//             n / 2
//             // TODO ^ Try suppressing this expression with a semicolon.
//         };
//     //   ^ Don't forget to put a semicolon here! All `let` bindings need it.

//     println!("{} -> {}", n, big_n);
// }

// fn main() {
//     let mut count: u32 = 0u32;

//     println!("Let's count until infinity!");

//     // Infinite loop
//     loop {
//         count += 1;

//         if count == 3 {
//             println!("three");

//             // Skip the rest of this iteration
//             continue;
//         }

//         println!("{}", count);

//         if count == 5 {
//             println!("OK, that's enough");

//             // Exit this loop
//             break;
//         }
//     }
// }

// #![allow(unreachable_code, unused_labels)]

// fn main() {
//     'x1: loop {
//         println!("Entered the outer loop");

//         'x2: loop {
//             println!("Entered the inner loop");
//             break 'x1;
//         }

//         println!("This point will never be reached");
//     }

//     println!("Exited the outer loop");
// }

// fn main() {
//     let mut counter = 0;

//     let result = loop {
//         counter += 1;

//         if counter == 10 {
//             break counter * 2;
//         }
//     };
//     print!("{}", result);
//     // assert_eq!(result, 203);
// }

// fn main() {
//     // A counter variable
//     let mut n: i32 = 1;

//     // Loop while `n` is less than 101
//     while n < 110 {
//         if n % 15 == 0 {
//             println!("fizzbuzz");
//         } else if n % 3 == 0 {
//             println!("fizz");
//         } else if n % 5 == 0 {
//             println!("buzz");
//         } else {
//             println!("{}", n);
//         }

//         // Increment counter
//         n += 1;
//     }
// }

// fn main() {
//     // `n` will take the values: 1, 2, ..., 100 in each iteration
//     for n in 1..=101 {
//         print!("{}\n", n)
//     }
// let names: Vec<&str> = vec!["Bob", "Frank", "Ferris"];

// for name in names.iter() {
//     match name {
//         &"Ferris" => println!("There is a rustacean among us!"),
//         // TODO ^ Try deleting the & and matching just "Ferris"
//         _ => println!("Hello {}", name),
//     }
// }

// println!("names: {:?}", names);

// for name in names.into_iter() {
//     match name {
//         "Ferris" => println!("There is a rustacean among us!"),
//         _ => println!("Hello {}", name),
//     }
// }

// let mut names: Vec<&str> = vec!["Bob", "Frank", "Ferris"];
// println!("xxxxxxx: {:?}", names);
// for name in names.iter_mut() {
//     *name = match name {
//         &mut "Ferris" => "There is a rustacean among us!",
//         _ => "Hello",
//     }
// }
// println!("names: {:?}", names);

// println!("names: {:?}", names);
// FIXME ^ Comment out this line
// }

// fn main() {
//     let number: i32 = 13;
//     // TODO ^ Try different values for `number`

//     println!("Tell me about {}", number);
//     match number {
//         // Match a single value
//         1 => println!("One!"),
//         // Match several values
//         2 | 3 | 5 | 7 | 11 => println!("This is a prime"),
//         // TODO ^ Try adding 13 to the list of prime values
//         // Match an inclusive range
//         13..=19 => println!("A teen"),
//         // Handle the rest of cases
//         _ => println!("Ain't special"),
//         // TODO ^ Try commenting out this catch-all arm
//     }

//     let boolean: bool = true;
//     // Match is an expression too
//     let binary: i32 = match boolean {
//         // The arms of a match must cover all the possible values
//         false => 0,
//         true => 1,
//         // TODO ^ Try commenting out one of these arms
//     };

//     println!("{} -> {}", boolean, binary);
// }
// #[allow(dead_code)]
// enum Temperature {
//     Celsius(i32),
//     Fahrenheit(i32),
// }

// fn main() {
//     let temperature = Temperature::Celsius(35);
//     // ^ TODO try different values for `temperature`

//     match temperature {
//         Temperature::Celsius(t) if t > 30 => println!("{}C is above 30 Celsius", t),
//         // The `if condition` part ^ is a guard
//         Temperature::Celsius(t) => println!("{}C is equal to or below 30 Celsius", t),

//         Temperature::Fahrenheit(t) if t > 86 => println!("{}F is above 86 Fahrenheit", t),
//         Temperature::Fahrenheit(t) => println!("{}F is equal to or below 86 Fahrenheit", t),
//     }
// }

// fn main() {
//     let triple = (0, -2, 3);
//     // TODO ^ Try different values for `triple`

//     println!("Tell me about {:?}", triple);
//     // Match can be used to destructure a tuple
//     match triple {
//         // Destructure the second and third elements
//         (0, y, z) => println!("First is `0`, `y` is {:?}, and `z` is {:?}", y, z),
//         (1, ..) => println!("First is `1` and the rest doesn't matter"),
//         (.., 2) => println!("last is `2` and the rest doesn't matter"),
//         (3, .., 4) => println!("First is `3`, last is `4`, and the rest doesn't matter"),
//         // `..` can be used to ignore the rest of the tuple
//         _ => println!("It doesn't matter what they are"),
//     }
// }

// fn main() {
//     // Try changing the values in the array, or make it a slice!
//     let array = [1, -2, 6];

//     match array {
//         // Binds the second and the third elements to the respective variables
//         [0, second, third] =>
//             println!("array[0] = 0, array[1] = {}, array[2] = {}", second, third),

//         // Single values can be ignored with _
//         [1, _, third] => println!(
//             "array[0] = 1, array[2] = {} and array[1] was ignored",
//             third
//         ),

//         // You can also bind some and ignore the rest
//         [-1, second, ..] => println!(
//             "array[0] = -1, array[1] = {} and all the other ones were ignored",
//             second
//         ),
//         // The code below would not compile
//         // [-1, second] => ...

//         // Or store them in another array/slice (the type depends on
//         // that of the value that is being matched against)
//         [3, second, tail @ ..] => println!(
//             "array[0] = 3, array[1] = {} and the other elements were {:?}",
//             second, tail
//         ),

//         // Combining these patterns, we can, for example, bind the first and
//         // last values, and store the rest of them in a single array
//         [first, middle @ .., last] => println!(
//             "array[0] = {}, middle = {:?}, array[2] = {}",
//             first, middle, last
//         ),
//     }
// }

// `allow` required to silence warnings because only
// one variant is used.
// #[allow(dead_code)]
// enum Color {
//     // These 3 are specified solely by their name.
//     Red,
//     Blue,
//     Green,
//     // These likewise tie `u32` tuples to different names: color models.
//     RGB(u32, u32, u32),
//     HSV(u32, u32, u32),
//     HSL(u32, u32, u32),
//     CMY(u32, u32, u32),
//     CMYK(u32, u32, u32, u32),
// }

// fn main() {
//     let colors: Vec<Color> = vec![
//         Color::Red,
//         Color::Blue,
//         Color::Green,
//         Color::RGB(255, 0, 0),
//         Color::HSV(0, 100, 100),
//         Color::HSL(0, 100, 50),
//         Color::CMY(0, 255, 255),
//         Color::CMYK(0, 255, 255, 0),
//     ];
//     // TODO ^ Try different variants for `color`
//     println!("What colors are there?");
//     // Match on each color in the vector
//     for color in colors {
//         match color {
//             Color::Red => println!("The color is Red!"),
//             Color::Blue => println!("The color is Blue!"),
//             Color::Green => println!("The color is Green!"),
//             Color::RGB(r, g, b) => println!("Red: {}, Green: {}, Blue: {}!", r, g, b),
//             Color::HSV(h, s, v) => println!("Hue: {}, Saturation: {}, Value: {}!", h, s, v),
//             Color::HSL(h, s, l) => println!("Hue: {}, Saturation: {}, Lightness: {}!", h, s, l),
//             Color::CMY(c, m, y) => println!("Cyan: {}, Magenta: {}, Yellow: {}!", c, m, y),
//             Color::CMYK(c, m, y, k) => println!("Cyan: {}, Magenta: {}, Yellow: {}, Key (Black): {}!", c, m, y, k),
//         }
//     }
// }

// fn main() {
//     let user_input: String = String::from("hello, world!");

//     // Take ownership of the user input to process it
//     let processed_input: String = process_input(user_input);
//     println!("Processed input: {}", processed_input);
// }

// fn process_input(input: String) -> String {
//     input.to_uppercase() // Takes ownership and transforms it
// }
// fn main() {
//     // Assign a reference of type `i32`. The `&` signifies there
//     // is a reference being assigned.
//     let reference: &i32 = &4;

//     match reference {
//         // If `reference` is pattern matched against `&val`, it results
//         // in a comparison like:
//         // `&i32`
//         // `&val`
//         // ^ We see that if the matching `&`s are dropped, then the `i32`
//         // should be assigned to `val`.
//         &val => println!("Got a value via destructuring: {:?}", val),
//     }

//     // To avoid the `&`, you dereference before matching.
//     match *reference {
//         val => println!("Got a value via dereferencing: {:?}", val),
//     }

//     // What if you don't start with a reference? `reference` was a `&`
//     // because the right side was already a reference. This is not
//     // a reference because the right side is not one.
//     let _not_a_reference: i32 = 3;

//     // Rust provides `ref` for exactly this purpose. It modifies the
//     // assignment so that a reference is created for the element; this
//     // reference is assigned.
//     let ref _is_a_reference = 3;

//     // Accordingly, by defining 2 values without references, references
//     // can be retrieved via `ref` and `ref mut`.
//     let value: i32 = 5;
//     let mut mut_value: i32 = 6;

//     // Use `ref` keyword to create a reference.
//     match value {
//         ref r => println!("Got a reference to a value: {:?}", r),
//     }

//     // Use `ref mut` similarly.
//     match mut_value {
//         ref mut m => {
//             // Got a reference. Gotta dereference it before we can
//             // add anything to it.
//             *m += 10;
//             println!("We added 10. `mut_value`: {:?}", m);
//         },
//     }
// }

// fn main() {
//     struct Foo {
//         x: (u32, u32),
//         y: u32,
//     }

//     // Try changing the values in the struct to see what happens
//     let foo: Foo = Foo { x: (1, 2), y: 3 };

//     match foo {
//         Foo { x: (1, b), y } => println!("First of x is 1, b = {},  y = {} ", b, y),

//         // you can destructure structs and rename the variables,
//         // the order is not important
//         Foo { y: 2, x: i } => println!("y is 2, i = {:?}", i),

//         // and you can also ignore some variables:
//         Foo { y, .. } => println!("y = {y}, we don't care about x"),
//         // this will give an error: pattern does not mention field `x`
//         //Foo { y } => println!("y = {}", y),
//     }

//     let faa: Foo = Foo { x: (1, 2), y: 3 };

//     // You do not need a match block to destructure structs:
//     let Foo { x: x0, y: y0 } = faa;
//     println!("Outside: x0 = {x0:?}, y0 = {y0}");

//     // Destructuring works with nested structs as well:
//     struct Bar {
//         foo: Foo,
//     }

//     let bar: Bar = Bar { foo: faa };
//     let Bar {
//         foo: Foo {
//             x: nested_x,
//             y: nested_y,
//         },
//     } = bar;
//     println!("Nested: nested_x = {nested_x:?}, nested_y = {nested_y:?}");
// }

// struct Person {
//     name: String,
//     age: u32,
// }

// impl Person {
//     // Method to return fields as a vector of tuples
//     fn data(self) -> Vec<(String, String)> {
//         vec![
//             ("name".to_string(), self.name),
//             ("age".to_string(), self.age.to_string()), // Convert age to String
//         ]
//     }
// }

// fn main() {
//     let person: Person = Person {
//         name: String::from("Alice"),
//         age: 30,
//     };
//     // Print key-value pairs using a for loop
//     for (key, value) in person.data() {
//         println!("{}: {}", key, value);
//     }
// }

// struct User {
//     username: String,
//     email: String,
// }

// enum Login {
//     Success(User),
//     Failure(String),
// }

// fn main() {
//     let login: Login = Login::Success(User {
//         username: String::from("user123"),
//         email: String::from("user@example.com"),
//     });

//     if let Login::Success(User { username, email }) = login {
//         println!("Logged in as: {}, Email: {}", username, email);
//         // Output: Logged in as: user123, Email: user@example.com
//     }
// }

// fn main() {
//     let x: i32 = 10; // x is the owner, stored on the stack
//     let y: &i32 = &x; // y borrows x immutably
//     let z: &&i32 = &y; // z borrows y immutably

//     // Print values
//     println!("Value of x: {}", x);
//     println!("Value of y (borrowed): {}", y);
//     println!("Value of z (double borrowed): {}", z);

//     // Print memory addresses
//     println!("Memory address of x: {:p}", &x);
//     println!("Memory address of y (borrowed): {:p}", y);
//     println!("Memory address of z (double borrowed): {:p}", &z);
// }

// fn main() {
//     let mut x: i32 = 10; // x is mutable

//     // Create a mutable reference to x
//     let y: &mut i32 = &mut x; // y borrows x mutably

//     // Mutate x through y
//     *y = 20; // Now x is 20

//     let z: &i32 = &y; // z borrows the value of y immutably
//     // Create an immutable reference to y

//     // Accessing through z
//     println!("Value of y (borrowed): {}", y); // This will also print 20
//     println!("Value of z (double borrowed): {}", *z); // This will also print 20
// }

// fn main() {
//     let mut x: i32 = 5;

//     // Immutable borrow
//     let y1: &i32 = &x; // First immutable borrow
//     let y2: &i32 = &x; // Second immutable borrow
//     let y3: &i32 = &x; // Second immutable borrow

//     println!("y1: {}, y2: {}, y3: {}", y1, y2, y3); // Both immutable borrows can be used

//     // Mutable borrow
//     let z: &mut i32 = &mut x; // Mutable borrow
//     *z += 1; // Modify x through the mutable reference

//     *z += 1; // Modify x through the mutable reference

//     x += 1;
//     x += 1;
//     x += 1;
//     x += 1;
//     x += 1;
//     x += 1;
//     // println!("y1: {}, y2: {}", y1, y2); // This would cause a compile error!
//     // Cannot use y1 or y2 here because of the mutable borrow
//     println!("x after mutation: {}", x); // Safe to use x now
// }

// struct Person {
//     name: String,
//     age: u32,
// }

// impl Person {

//     fn birthday(&mut self) {
//         self.age += 1;
//     }

//     fn display(&self) {
//         println!("Name: {}, Age: {}", self.name, self.age);
//     }
// }

// fn main() {
//     // let mut person: Person = Person::new("Alice", 30);
//     let mut person: Person = Person {
//         name: String::from("Alice"),
//         age: 30,
//     };

//     // Immutable borrows
//     let person_ref1: &Person = &person; // First immutable borrow
//     let person_ref2: &Person = &person; // Second immutable borrow

//     person_ref1.display(); // Using the first immutable reference
//     person_ref2.display(); // Using the second immutable reference

//     // Now we want to modify the person, so we need to create a mutable borrow
//     // But first, we need to ensure that we no longer have immutable borrows
//     // We can do this by ending the scope of the immutable borrows

//     {
//         let person_ref3: &mut Person = &mut person; // Mutable borrow
//         person_ref3.birthday(); // Modify the person's age
//     } // person_ref3 goes out of scope here

//     // Now we can safely use the original person again
//     person.display(); // Should show age 31
// }

// #[derive(Debug)]
// struct Person {
//     name: String,
//     age: Option<u32>, // Using Option to allow for an optional age
// }

// impl Person {
//     // Constructor that accepts name and an optional age
//     fn new(name: String, age: Option<u32>) -> Self {
//         Self {
//             name,
//             age: age.or(Some(18)), // Default to Some(18) if age is None
//         }
//     }
//     // Display method to format the output
//     fn display(&self) -> String {
//         let age_display: String = match self.age {
//             Some(age) => age.to_string(),
//             None => "Unknown".to_string(),
//         };
//         format!("Name: {}, Age: {}", self.name, age_display)
//     }
// }

// fn main() {
//     // Create an instance with a specific age
//     let person_with_age: Person = Person::new(String::from("Alice"), Some(30));
//     println!("Person with age: {:?}", person_with_age.display());

//     // Create an instance with the default age of 18
//     let person_with_default_age: Person = Person::new(String::from("Bob"), None);
//     println!(
//         "Person with default age: {:#?}",
//         person_with_default_age.display()
//     );
//     let s: String = String::from("helsssssssssssssssssssssssssssssssssssssssssslo"); // s is valid from this point forward
//     print!("{}", s);
//     print!("{}", s);
//     print!("{}", s);
//     print!("{}", s);
//     print!("{}", s);

//     // do stuff with s
// }

// use std::fmt;

// #[derive(Debug)]
// struct MyString {
//     chars: Vec<char>,
// }

// impl MyString {
//     // Constructor to create a new MyString from a &str
//     fn new<S: Into<String>>(input: S) -> Self {
//         let chars: Vec<char> = input.into().chars().collect();
//         MyString { chars }
//     }

//     // Method to append another MyString
//     fn push(&mut self, other: &MyString) {
//         self.chars.extend(&other.chars);
//     }

//     // Method to get the length of the string
//     fn len(&self) -> usize {
//         self.chars.len()
//     }
// }

// // Implementing the Display trait for MyString
// impl fmt::Display for MyString {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "--->{}<---", self.chars.iter().collect::<String>())
//     }
// }

// fn main() {
//     // Create a new MyString
//     let mut my_string: MyString = MyString::new("Hello");
//     println!("{:?}", my_string); // MyString { chars: ['H', 'e', 'l', 'l', 'o'] }

//     // Append another MyString
//     let other_string: MyString = MyString::new(", world!");
//     my_string.push(&other_string);
//     println!("{:?}", my_string); // MyString { chars: ['H', 'e', 'l', 'l', 'o', ',', ' ', 'w', 'o', 'r', 'l', 'd', '!'] }

//     // Get the length
//     println!("Length: {}", my_string.len()); // Length: 13
//     println!("Converted to String: {:?}", my_string); // Converted to String: Hello, world!
// }

// fn main() {
//     let vec1: Vec<i32> = vec![1, 2, 3];
//     let vec2: Vec<i32> = vec1.clone(); // Explicit deep copy

//     println!("{:?}", vec1); // vec1 is still accessible
//     println!("{:?}", vec2); // vec2 is an independent copy
//     let vec_ref: &Vec<i32> = &vec1; // Borrowing vec1

//     println!("{:?} {:?}", vec1, vec_ref); // Access through reference
// }

// fn main() {
//     let mut arr: [i32; 5] = [1, 2, 3, 4, 5];
//     let slice: &[i32] = &arr[1..4]; // Immutable borrow of a slice

//     // arr[0] = 10;  // This would cause an error because arr is immutably borrowed
//     println!("{:?}", slice);  // Can still access the slice

//     arr[1] = 10;
//     // However, you can mutate the array after the slice is out of scope or if there are no references:
//     println!("{:?}", arr);  // Output: [10, 2, 3, 4, 5]
// }

// fn main() {
//     let mut s: String = String::from("Hello, world");
//     s.push('j');

//     // Immutable slice
//     let slice1: &str = &s[0..5];  // Borrowed as immutable
//     let slice2: &str = &s[7..];   // Another immutable borrow
//     // s.push_str("!");  // Error: cannot mutate while immutably borrowed

//     println!("Slice1: {}, Slice2: {}", slice1, slice2);  // OK: multiple immutable references

//     // Mutable slice (no immutable slices allowed simultaneously)
//     let mut_slice: &mut str = &mut s[0..5];  // Mutable borrow is now possible after the immutable ones are done
//     println!("{}", mut_slice);  // OK: works after immutables are dropped
// }

// fn main() {
//     let mut s: String = String::from("Hello, world");

//     // Create an immutable slice
//     let slice1: &str = &s[0..5]; // "Hello"
//     let slice2: &str = &s[7..];  // "world"

//     println!("Slice1: {}, Slice2: {}", slice1, slice2);

//     // Attempt to create a mutable slice and modify the content
//     let mut_slice: &mut str = &mut s[0..5]; // "Hello"
//     unsafe {
//         mut_slice.as_bytes_mut()[0] = b'J';  // Change 'H' to 'J'
//     }
//     // You can modify individual characters of the mutable slice

//     // Now the original string has been modified
//     println!("Modified string: {}", mut_slice);  // Output: "Jello, world"
// }

// fn main() {
//     let mut arr: [i32; 5] = [1, 2, 3, 4, 5]; // A mutable array
//     let copy_array: [i32; 5] = arr.clone();
//     print!("copied array{:?}", copy_array);
//     // Create a mutable slice that borrows part of the array
//     let slice: &mut [i32] = &mut arr[1..5]; // Mutable borrow of [2, 3, 4]
//     println!("before Slice: {:?}", slice); // Output: [2, 3, 4]

//     // Mutate the elements in the slice
//     slice[0] = 20; // Change the first element of the slice (affects arr[1])
//     slice[2] = 40; // Change the last element of the slice (affects arr[3])

//     // Printing both the slice and the original array to show the mutation
//     println!("Slice: {:?}", slice); // Output: [20, 3, 40]
//     println!("Array: {:?}", arr); // Output: [1, 20, 3, 40, 5]
//     print!("copied after array{:?}", copy_array);

// }

// mod libs {
//     pub mod my_slice; // Declare that the my_slice module is public
// }

// use libs::my_slice::MySlice;

// fn main() {
//     let arr: [i32; 5] = [1, 2, 3, 4, 5];
//     let vec: Vec<i32> = vec![10, 20, 30, 40, 50];
//     arr.own();
//     // Use custom `.slice()` method on array
//     let arr_slice: &[i32] = arr.slice(1, 4);
//     println!("Array Slice: {:?}", arr_slice); // Output: [2, 3, 4]

//     // Use custom `.slice()` method on vector
//     let vec_slice: &[i32] = vec.slice(1, 4);
//     vec.own();

//     println!("Vector Slice: {:?}", vec_slice); // Output: [20, 30, 40]
// }

// fn main() {
//     let m1: String = String::from("Hello");
//     let m2: String = String::from("world");
//     let (m1_again, m2_again) = greet(m1, m2);
//     let s: String = format!("{} {}", m1_again, m2_again);
//     print!("{}",s)
// }

// fn greet(g1: String, g2: String) -> (String, String) {
//     println!("{} {}!", g1, g2);
//     (g1, g2)
// }

// fn main() {
//     let m1: String = String::from("Hello");
//     let m2: String = String::from("world");
//     greet(&m1, &m2); // note the ampersands
//                      // let s: String = format!("{} {}", m1, m2);

//     let mut x: Box<i32> = Box::new(1);
//     let a: i32 = *x; // *x reads the heap value, so a = 1
//     *x += 1; // *x on the left-side modifies the heap value,
//              //     so x points to the value 2

//     let r1: &Box<i32> = &x; // r1 points to x on the stack
//     let b: i32 = **r1; // two dereferences get us to the heap value

//     let r2: &i32 = &*x; // r2 points to the heap value directly
//     let c: i32 = *r2; // so only one dereference is needed to read it
// }

// fn greet(g1: &String, g2: &String) {
//     // note the ampersands
//     println!("{} {}!", g1, g2);
// }
// use std::collections::HashMap;

// #[derive(Debug)]
// struct DynamicStruct {
//     fields: HashMap<String, Value>,
// }

// #[derive(Debug)]
// enum Value {
//     Int(i32),
//     Str(String),
// }

// impl DynamicStruct {
//     // Constructor to create a new instance
//     fn new() -> Self {
//         DynamicStruct {
//             fields: HashMap::new(),
//         }
//     }

//     // Method to set a field dynamically
//     fn set_field(&mut self, key: String, value: Value) {
//         self.fields.insert(key, value);
//     }
// }

// fn main() {
//     // Create an instance of DynamicStruct
//     let mut dynamic_object = DynamicStruct::new();

//     // Dynamically add fields
//     dynamic_object.set_field("age".to_string(), Value::Int(23));
//     dynamic_object.set_field("name".to_string(), Value::Str("Alice".to_string()));

//     println!("{:?}", dynamic_object);
// }

// enum PaymentMethod {
//     CreditCard { number: String, expiry: String },
//     PayPal { email: String },
//     BankTransfer { account_number: String },
// }

// impl PaymentMethod {
//     fn process_payment(self, x: String) {
//         match self {
//             PaymentMethod::CreditCard { number, expiry } => {
//                 println!("Processing credit card payment: {} (expires {} {})", number, expiry, x);
//             }
//             PaymentMethod::PayPal { email } => {
//                 println!("Processing PayPal payment for {} {}", email, x);
//             }
//             PaymentMethod::BankTransfer { account_number } => {
//                 println!("Processing bank transfer to account {} {}", account_number, x);
//             }
//         }
//     }
// }

// fn main() {
//     let payment: PaymentMethod = PaymentMethod::PayPal { email: String::from("user@example.com") };

//     payment.process_payment(String::from("--Done"));
// }

// struct Point {
//     x: f64,
//     y: f64,
// }

// // Implementation block, all `Point` associated functions & methods go in here
// impl Point {
//     // This is an "associated function" because this function is associated with
//     // a particular type, that is, Point.
//     //
//     // Associated functions don't need to be called with an instance.
//     // These functions are generally used like constructors.
//     fn origin() -> Point {
//         Point { x: 0.0, y: 0.0 }
//     }

//     // Another associated function, taking two arguments:
//     fn new(x: f64, y: f64) -> Point {
//         Point { x: x, y: y }
//     }
// }

// struct Rectangle {
//     p1: Point,
//     p2: Point,
// }

// impl Rectangle {
//     // This is a method
//     // `&self` is sugar for `self: &Self`, where `Self` is the type of the
//     // caller object. In this case `Self` = `Rectangle`
//     fn area(&self) -> f64 {
//         // `self` gives access to the struct fields via the dot operator
//         let Point { x: x1, y: y1 } = self.p1;
//         let Point { x: x2, y: y2 } = self.p2;

//         // `abs` is a `f64` method that returns the absolute value of the
//         // caller
//         ((x1 - x2) * (y1 - y2)).abs()
//     }

//     fn perimeter(&self) -> f64 {
//         let Point { x: x1, y: y1 } = self.p1;
//         let Point { x: x2, y: y2 } = self.p2;

//         2.0 * ((x1 - x2).abs() + (y1 - y2).abs())
//     }

//     // This method requires the caller object to be mutable
//     // `&mut self` desugars to `self: &mut Self`
//     fn translate(&mut self, x: f64, y: f64) {
//         self.p1.x += x;
//         self.p2.x += x;

//         self.p1.y += y;
//         self.p2.y += y;
//     }
// }

// // `Pair` owns resources: two heap allocated integers
// struct Pair(Box<i32>, Box<i32>);

// impl Pair {
//     // This method "consumes" the resources of the caller object
//     // `self` desugars to `self: Self`
//     fn destroy(self) {
//         // Destructure `self`
//         let Pair(first, second) = self;

//         println!("Destroying Pair({}, {})", first, second);

//         // `first` and `second` go out of scope and get freed
//     }
// }

// fn main() {
//     let rectangle: Rectangle = Rectangle {
//         // Associated functions are called using double colons
//         p1: Point::origin(),
//         p2: Point::new(3.0, 4.0),
//     };

//     // Methods are called using the dot operator
//     // Note that the first argument `&self` is implicitly passed, i.e.
//     // `rectangle.perimeter()` === `Rectangle::perimeter(&rectangle)`
//     println!("Rectangle perimeter: {}", rectangle.perimeter());
//     println!("Rectangle area: {}", rectangle.area());

//     let mut square: Rectangle = Rectangle {
//         p1: Point::origin(),
//         p2: Point::new(1.0, 1.0),
//     };

//     // Error! `rectangle` is immutable, but this method requires a mutable
//     // object
//     //rectangle.translate(1.0, 0.0);
//     // TODO ^ Try uncommenting this line

//     // Okay! Mutable objects can call mutable methods
//     square.translate(1.0, 1.0);

//     let pair = Pair(Box::new(1), Box::new(2));

//     pair.destroy();

//     // Error! Previous `destroy` call "consumed" `pair`
//     //pair.destroy();
//     // TODO ^ Try uncommenting this line
// }

// fn main() {
//     let outer_var: i32 = 42;

//     // A regular function can't refer to variables in the enclosing environment
//     //fn function(i: i32) -> i32 { i + outer_var }
//     // TODO: uncomment the line above and see the compiler error. The compiler
//     // suggests that we define a closure instead.

//     // Closures are anonymous, here we are binding them to references.
//     // Annotation is identical to function annotation but is optional
//     // as are the `{}` wrapping the body. These nameless functions
//     // are assigned to appropriately named variables.
//     let closure_annotated = |i: i32| -> i32 { i + outer_var };
//     let closure_inferred  = |i     |          i + outer_var  ;

//     // Call the closures.
//     println!("closure_annotated: {}", closure_annotated(1));
//     println!("closure_inferred: {}", closure_inferred(1));
//     // Once closure's type has been inferred, it cannot be inferred again with another type.
//     //println!("cannot reuse closure_inferred with another type: {}", closure_inferred(42i64));
//     // TODO: uncomment the line above and see the compiler error.

//     // A closure taking no arguments which returns an `i32`.
//     // The return type is inferred.
//     let one = || 1;
//     println!("closure returning one: {}", one());

// }

// fn main() {
//     let mut names: Vec<&str> = vec!["Charlie", "Alice", "Bobz"];

//     // Sort names by the last character
//     names.sort_by(|a, b| a.chars().last().cmp(&b.chars().last()));
//     println!("Sorted by last character: {:?}", names); // Output: ["Alice", "Charlie", "Bob"]
// }

// fn create_multiplier(factor: i32) -> Box<dyn Fn(i32) -> i32> {
//     Box::new(move |x| x * factor)
// }

// fn main() {
//     let double: Box<dyn Fn(i32) -> i32> = create_multiplier(2);
//     let triple: Box<dyn Fn(i32) -> i32> = create_multiplier(3);

//     println!("Double 5: {}", double(5)); // Output: 10
//     println!("Triple 5: {}", triple(5)); // Output: 15
// }

// fn main() {
//     use std::mem;

//     let color: String = String::from("green");

//     // A closure to print `color` which immediately borrows (`&`) `color` and
//     // stores the borrow and closure in the `print` variable. It will remain
//     // borrowed until `print` is used the last time.
//     //
//     // `println!` only requires arguments by immutable reference so it doesn't
//     // impose anything more restrictive.
//     let print = || println!("`color`: {}", color);

//     // Call the closure using the borrow.
//     print();

//     // `color` can be borrowed immutably again, because the closure only holds
//     // an immutable reference to `color`.
//     let _reborrow: &String = &color;
//     print();

//     // A move or reborrow is allowed after the final use of `print`
//     let _color_moved = color;

//     let mut count = 0;
//     // A closure to increment `count` could take either `&mut count` or `count`
//     // but `&mut count` is less restrictive so it takes that. Immediately
//     // borrows `count`.
//     //
//     // A `mut` is required on `inc` because a `&mut` is stored inside. Thus,
//     // calling the closure mutates `count` which requires a `mut`.
//     let mut inc = || {
//         count += 1;
//         println!("`count`: {}", count);
//     };

//     // Call the closure using a mutable borrow.
//     inc();

//     // The closure still mutably borrows `count` because it is called later.
//     // An attempt to reborrow will lead to an error.
//     // let _reborrow = &count;
//     // ^ TODO: try uncommenting this line.
//     inc();

//     // The closure no longer needs to borrow `&mut count`. Therefore, it is
//     // possible to reborrow without an error
//     let _count_reborrowed: &mut i32 = &mut count;

//     // A non-copy type.
//     let movable: Box<i32> = Box::new(3);

//     // `mem::drop` requires `T` so this must take by value. A copy type
//     // would copy into the closure leaving the original untouched.
//     // A non-copy must move and so `movable` immediately moves into
//     // the closure.
//     let consume = || {
//         println!("`movable`: {:?}", movable);
//         mem::drop(movable);
//     };

//     // `consume` consumes the variable so this can only be called once.
//     consume();
//     // consume();
//     // ^ TODO: Try uncommenting this line.
// }

// A function which takes a closure as an argument and calls it.
// <F> denotes that F is a "Generic type parameter"
// fn apply<F>(f: F) where
//     // The closure takes no input and returns nothing.
//     F: FnOnce() {
//     // ^ TODO: Try changing this to `Fn` or `FnMut`.

//     f();
// }

// // A function which takes a closure and returns an `i32`.
// fn apply_to_3<F>(f: F) -> i32 where
//     // The closure takes an `i32` and returns an `i32`.
//     F: Fn(i32) -> i32 {

//     f(3)
// }

// fn main() {
//     use std::mem;

//     let greeting: &str = "hello";
//     // A non-copy type.
//     // `to_owned` creates owned data from borrowed one
//     let mut farewell: String = "goodbye".to_owned();

//     // Capture 2 variables: `greeting` by reference and
//     // `farewell` by value.
//     let diary = || {
//         // `greeting` is by reference: requires `Fn`.
//         println!("I said {}.", greeting);

//         // Mutation forces `farewell` to be captured by
//         // mutable reference. Now requires `FnMut`.
//         farewell.push_str("!!!");
//         println!("Then I screamed {}.", farewell);
//         println!("Now I can sleep. zzzzz");

//         // Manually calling drop forces `farewell` to
//         // be captured by value. Now requires `FnOnce`.
//         mem::drop(farewell);
//     };

//     // Call the function which applies the closure.
//     apply(diary);

//     // `double` satisfies `apply_to_3`'s trait bound
//     let double = |x| 2 * x;

//     println!("3 doubled: {}", apply_to_3(double));
//     println!("3 doubled: {}", apply_to_3(double));

// }

// fn main() {
//     panic!("crash and burn");
// }

// fn main() {
//     struct OneTimeExecutor<F: FnOnce()> {
//         func: Option<F>, // Option to allow taking ownership
//     }

//     impl<F: FnOnce()> OneTimeExecutor<F> {
//         fn new(func: F) -> Self {
//             OneTimeExecutor { func: Some(func) }
//         }

//         fn execute(self) {
//             if let Some(f) = self.func {
//                 f(); // Call the closure
//             }
//         }
//     }

//     let executor = OneTimeExecutor::new(|| {
//         println!("This can only be executed once!");
//     });

//     executor.execute(); // Executes the closure
//     // executor.execute(); // Uncommenting this line will cause a compile-time error
// }

// struct OneTimePrinter<F: FnOnce()> {
//     message: F,
// }

// impl<F: FnOnce()> OneTimePrinter<F> {
//     fn new(message: F) -> Self {
//         OneTimePrinter { message }
//     }

//     fn print(self) {
//         (self.message)(); // Call the closure
//     }
// }

// fn main() {
//     let printer = OneTimePrinter::new(|| {
//         println!("This message will print only once!");
//     });

//     printer.print(); // This will print the message
//     // printer.print(); // Uncommenting this line will cause a compile-time error
// }

// use std::fs::File;
// use std::io::ErrorKind;

// fn main() {
//     let greeting_file_result: Result<File, std::io::Error> = File::open("hello.txt");

//     let greeting_file: File = match greeting_file_result {
//         Ok(file) => file,
//         Err(error) => match error.kind() {
//             ErrorKind::NotFound => match File::create("hello.txt") {
//                 Ok(fc) => fc,
//                 Err(e) => panic!("Problem creating the file: {e:?}"),
//             },
//             other_error => {
//                 panic!("Problem opening the file: {other_error:?}");
//             }
//         },
//     };

//     print!("{:?}", greeting_file)
// }

/// The original ErrorKind enum.
// pub enum ErrorKind {
//     /// An entity was not found, often a file.
//     NotFound,
//     /// Some other error variant.
//     InvalidInput,
// }

// use std::io;

// /// A trait to describe error behavior.
// pub trait Error {
//     fn description(&self) -> &'static str;
// }

// /// A custom enum to wrap std::io::ErrorKind.
// #[derive(Debug)]
// pub enum MyErrorKind {
//     IoError(io::ErrorKind),
//     NewError,
// }

// /// Implementing the Error trait for MyErrorKind.
// impl Error for MyErrorKind {
//     fn description(&self) -> &'static str {
//         match self {
//             MyErrorKind::IoError(kind) => match kind {
//                 io::ErrorKind::NotFound => "An entity was not found.",
//                 io::ErrorKind::InvalidInput => "The input provided is invalid.",
//                 _ => "An unknown IO error occurred.",
//             },
//             MyErrorKind::NewError => "A new error occurred.",
//         }
//     }
// }

// /// A combined enum that can represent any kind of error.
// pub enum CombinedError {
//     Kind(MyErrorKind),
// }

// /// Implementing the Error trait for CombinedError.
// impl Error for CombinedError {
//     fn description(&self) -> &'static str {
//         match self {
//             CombinedError::Kind(kind) => kind.description(),
//         }
//     }
// }

// fn main() {
//     let error1 = CombinedError::Kind(MyErrorKind::IoError(io::ErrorKind::NotFound));
//     let error2 = CombinedError::Kind(MyErrorKind::NewError);

//     println!("Error 1: {}", error1.description());
//     println!("Error 2: {}", error2.description());
// }
// use std::fs::File;
// use std::io::ErrorKind;

// fn main() {
//     let greeting_file: File = File::open("hello.txt").unwrap_or_else(|error| {
//         if error.kind() == ErrorKind::NotFound {
//             File::create("hello.txt").unwrap_or_else(|error| {
//                 panic!("Problem creating the file: {error:?}");
//             })
//         } else {
//             panic!("Problem opening the file: {error:?}");
//         }
//     });

//     print!("xxxxxxxxxxxxxxxxxxxxxxxxxxxx{greeting_file:?}xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx");
// }

// fn main() {
//     use std::fs::File;
//     use std::io::{self, Read};

//     fn read_username_from_file() -> Result<String, io::Error> {
//         let mut username_file = File::open("hello.txt")?;
//         let mut username = String::new();
//         username_file.read_to_string(&mut username)?;
//         Ok(username)
//     }

//     let _ = read_username_from_file();
// }

// fn main() {
//     let r: &i32;
//     {
//         let x: i32 = 5;
//         r = &x;
//     }

//     println!("r: {r}");
// }



// fn main() {
//     let x: i32 = 5;            // ----------+-- 'b
//                           //           |
//     let r: &i32 = &x;           // --+-- 'a  |
//                           //   |       |
//     println!("r: {r}");   //   |       |
//                           // --+       |
// }                         // ----------+

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
fn main() {
    let string1: String = String::from("abcd");
    let string2: &str = "xyz";

    let result: &str = longest(string1.as_str(), string2);
    println!("The longest string is {result}");
}