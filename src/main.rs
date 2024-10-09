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

use crate::List::*;

enum List {
    // Cons: Tuple struct that wraps an element and a pointer to the next node
    That(u32, Box<List>),
    // Nil: A node that signifies the end of the linked list
    Nil,
}

// Methods can be attached to an enum
impl List {
    // Create an empty list
    fn new() -> List {
        // `Nil` has type `List`
        Nil
    }

    // Consume a list, and return the same list with a new element at its front
    fn prepend(self, elem: u32) -> List {
        // `Cons` also has type List
        That(elem, Box::new(self))
    }

    // Return the length of the list
    fn len(&self) -> u32 {
        // `self` has to be matched, because the behavior of this method
        // depends on the variant of `self`
        // `self` has type `&List`, and `*self` has type `List`, matching on a
        // concrete type `T` is preferred over a match on a reference `&T`
        // after Rust 2018 you can use self here and tail (with no ref) below as well,
        // rust will infer &s and ref tail.
        // See https://doc.rust-lang.org/edition-guide/rust-2018/ownership-and-lifetimes/default-match-bindings.html
        match *self {
            // Can't take ownership of the tail, because `self` is borrowed;
            // instead take a reference to the tail
            That(_, ref tail) => 1 + tail.len(),
            // Base Case: An empty list has zero length
            Nil => 0,
        }
    }

    // Return representation of the list as a (heap allocated) string
    fn stringify(&self) -> String {
        match *self {
            That(head, ref tail) => {
                // `format!` is similar to `print!`, but returns a heap
                // allocated string instead of printing to the console
                format!("{}, {}", head, tail.stringify())
            }
            Nil => {
                format!("Nil")
            }
        }
    }
}

fn increment_box(b: &mut Box<i32>) {
    **b += 1; // Increment the value inside the Box
}

fn double_box(b: &mut Box<i32>) {
    **b *= 2; // Double the value inside the Box
}

fn fouble_box(b: &mut Box<i32>) {
    **b *= 4; // Fouble the value inside the Box
}
fn main() {
    // Create an empty linked list
    let mut list: List = List::new();

    // Prepend some elements
    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);

    // Show the final state of the list
    println!("linked list has length: {}", list.len());
    println!("{}", list.stringify());

    let mut b: Box<i32> = Box::new(10); // Create a mutable Box
    println!("XXXXX: {:?}", b); // Prints 22

    println!("Original value: {}", b); // Prints 10

    // First mutable borrow
    increment_box(&mut b);
    println!("After increment: {}", b); // Prints 11

    // Second mutable borrow
    double_box(&mut b);
    println!("After doubling: {}", b); // Prints 22

    fouble_box(&mut b);
    println!("After foubling: {}", b); // Prints 22
    println!("SSSS: {}", *b); // Prints 22


}
