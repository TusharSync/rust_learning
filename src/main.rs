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

// fn main() {
//     let mut arr: [i32; 5] = [1, 2, 3, 4, 5];
//     print!("before slice array======={arr:?}\n");
//     {
//         let slice: &mut [i32] = &mut arr[1..4]; // Take a mutable slice of the array

//         // Print the original slice
//         println!("Original slice: {:?}", slice); // Output: [2, 3, 4]

//         // Modify the elements through the slice
//         slice[0] = 20; // arr[1]
//         slice[1] = 30; // arr[2]

//         // Print the updated slice
//         println!("Updated slice: {:?}", slice); // Output: [20, 30, 4]
//     } // Slice goes out of scope here

//     // Print the modified original array
//     println!("Modified original array: {:?}", arr); // Output: [1, 20, 30, 4, 5]
// }

// fn main() {
//     fn sum_odd_numbers(up_to: u32) -> u32 {
//         let mut acc: u32 = 0;
//         for i in 0..up_to {
//             // Notice that the return type of this match expression must be u32
//             // because of the type of the "addition" variable.
//             let addition: u32 = match i%2 == 1 {
//                 // The "i" variable is of type u32, which is perfectly fine.
//                 true => i,
//                 // On the other hand, the "continue" expression does not return
//                 // u32, but it is still fine, because it never returns and therefore
//                 // does not violate the type requirements of the match expression.
//                 false => continue,
//             };
//             acc += addition;
//         }
//         acc
//     }
//     println!("Sum of odd numbers up to 9 (excluding): {}", sum_odd_numbers(9));
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
//     let print = |x:String| println!("{}", x);

//     // Call the closure using the borrow.
//     print("tushar".to_string());

//     // `color` can be borrowed immutably again, because the closure only holds
//     // an immutable reference to `color`.
//     let _reborrow: &String = &color;
//     print("tushccar".to_string());

//     // A move or reborrow is allowed after the final use of `print`
//     let _color_moved: String = color;

//     let mut count: i32 = 0;
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
//     let _count_re_borrowed: &mut i32 = &mut count;

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
//     let double = |x: i32| 2 * x;

//     println!("3 doubled: {}", apply_to_3(double));
// }

// A function which takes a closure as an argument and calls it.
// <F> denotes that F is a "Generic type parameter"
// fn apply<F>(f: F) where
//     // The closure takes no input and returns nothing.
//     F: FnOnce() {
//     // ^ TODO: Try changing this to `Fn` or `FnMut`.

//     f();
// }
// fn apply<F>(f: F)
// where
//     F: FnOnce(),
// {
//     f();
// }

// // A function which takes a closure and returns an `i32`.
// fn apply_to<T, F>(f: F, value: T) -> T
// where
//     F: Fn(T) -> T,
// {
//     f(value)
// }

// fn main() {
//     use std::mem;

//     let greeting = "hello";
//     // A non-copy type.
//     // `to_owned` creates owned data from borrowed one
//     let mut farewell = "goodbye".to_owned();

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
//     let double = |x: i32| 2 * x;

//     println!("3 doubled: {}", apply_to(double,3));
// }

// fn apply_fn<F>(f: F)
// where
//     F: Fn(i32) -> i32,
// {
//     let result: i32 = f(10);
//     println!("Fn result: {}", result);
// }

// fn main() {
//     let add_one = |x| x + 1; // A closure that adds one to its input
//     apply_fn(add_one); // Can be called multiple times
//     apply_fn(add_one); // Can be called multiple times
//     apply_fn(add_one); // Can be called multiple times
//     apply_fn(add_one); // Can be called multiple times
//     apply_fn(add_one); // Can be called multiple times

// }

// fn apply_fn_once<F>(f: F)
// where
//     F: FnMut(i32) -> i32,
// {
//     let result: i32 = f(10);
//     println!("FnOnce result: {}", result);
// }

// fn main() {
//     let mut s: String = String::from("Hello");
//     let consume_string = |x: i32| {
//         s.push_str("test");
//         println!("Consuming: {}", s); // Takes ownership of `s`
//         x + 1
//     };
//     apply_fn_once(consume_string); // Can only be called once
// }

// TODO: remove this when you're done with your implementation.

// fn main() {
//     let matrix: [[i32; 3]; 3] = [
//         [101, 102, 103], // <-- the comment makes rustfmt add a newline
//         [201, 202, 203],
//         [301, 302, 303],
//     ];

//     println!("matrix: {:#?}", matrix);
// }

// fn apply_function<T>(mut f: T)
// where
//     T: FnMut(),
// {
//     f();
// }

// fn main() {
//     // let mut counter: i32 = 0;
//     let mut counter: i32 = 0;

//     let increment = || {
//         counter += 1;
//         println!("Counter: {}", counter);
//         println!("Counter: {}", counter);
//         println!("Counter: {}", counter);
//         println!("Counter: {}", counter);
//         println!("Counter: {}", counter);
//         println!("Counter: {}", counter);
//     };

//     apply_function(increment); // Counter: 1
// }

// fn apply_function<T>(mut f: T)
// where
//     T: FnMut(),
// {
//     f();
// }

// fn main() {
//     // let mut counter: i32 = 0;
//     let mut counter: i32 = 0;

//     let increment = || {
//         counter += 1;
//         counter += 1;
//         counter += 1;
//         counter += 1;
//         counter += 1;
//         counter += 1;
//         counter += 1;
//         println!("Counter: {}", counter);
//         println!("Counter: {}", counter);
//         println!("Counter: {}", counter);
//         println!("Counter: {}", counter);
//         println!("Counter: {}", counter);
//         println!("Counter: {}", counter);
//     };

//     apply_function(increment); // Counter: 1

// }

// fn apply_once<F>(mut f: F)
// where
//     F: FnMut(),
// {
//     f();
// }

// fn main() {
//     let mut name: String = String::from("Alice");

//     let greet = || {
//         name.push_str("string");
//         name.push_str("string");
//         name.push_str("string");
//         name.push_str("string");
//         name.push_str("string");
//         name.push_str("string");
//         name.push_str("string");

//         println!("Hello, {}", name);
//         // drop(name);
//     };

//     apply_once(greet); // Consumes `name` and prints "Hello, Alice"
//     // After this call, `name` cannot be used anymore
//     // println!("{}", name); // This would cause a compile error
// }

// fn apply_fn<T>(f: T)
// where
//     T: FnOnce(),
// {
//     f();
// }

// fn main() {
//     let greeting: String = String::from("Hello");
//     println!("{}", greeting);

//     let say_hello = || {
//         println!("{}", greeting);
//     };

//     apply_fn(say_hello); // Prints "Hello"
//     apply_fn(say_hello); // Prints "Hello" again
//     apply_fn(say_hello); // Prints "Hello" again
//     apply_fn(say_hello); // Prints "Hello" again
//     apply_fn(say_hello); // Prints "Hello" again
//     apply_fn(say_hello); // Prints "Hello" again
//     apply_fn(say_hello); // Prints "Hello" again
//     apply_fn(say_hello); // Prints "Hello" again
//     apply_fn(say_hello); // Prints "Hello" again
//     apply_fn(say_hello); // Prints "Hello" again
//     apply_fn(say_hello); // Prints "Hello" again
//     apply_fn(say_hello); // Prints "Hello" again
//     apply_fn(say_hello); // Prints "Hello" again
//     apply_fn(say_hello); // Prints "Hello" again
//     apply_fn(say_hello); // Prints "Hello" again
//     apply_fn(say_hello); // Prints "Hello" again
// }

// use std::convert::TryFrom;
// use std::convert::TryInto;

// #[derive(Debug, PartialEq)]
// struct EvenNumber(i32);

// impl TryFrom<i32> for EvenNumber {
//     type Error = ();

//     fn try_from(value: i32) -> Result<Self, Self::Error> {
//         if value % 2 == 0 {
//             Ok(EvenNumber(value))
//         } else {
//             Err(())
//         }
//     }
// }

// fn main() {
//     // TryFrom

//     assert_eq!(EvenNumber::try_from(8), Ok(EvenNumber(8)));
//     assert_eq!(EvenNumber::try_from(5), Err(()));

//     // TryInto

//     let result: Result<EvenNumber, ()> = 8i32.try_into();
//     assert_eq!(result, Ok(EvenNumber(8)));
//     let result: Result<EvenNumber, ()> = 5i32.try_into();
//     assert_eq!(result, Err(()));
// }

// use std::convert::TryInto;

// use std::convert::TryInto;
// use std::fmt;

// #[derive(Debug)]
// struct MyNumber(u8);

// #[derive(Debug)]
// struct ConversionError;

// impl fmt::Display for ConversionError {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "Conversion error")
//     }
// }

// impl TryInto<MyNumber> for i32 {
//     type Error = ConversionError;

//     fn try_into(self) -> Result<MyNumber, Self::Error> {
//         if self < 0 || self > u8::MAX as i32 {
//             Err(ConversionError)
//         } else {
//             Ok(MyNumber(self as u8))
//         }
//     }
// }

// fn main() {
//     let number: i32 = 10;

//     // Attempt to convert i32 to MyNumber
//     let result: Result<MyNumber, _> = number.try_into();

//     match result {
//         Ok(my_number) => println!("Conversion successful: {:?}", my_number),
//         Err(err) => println!("Conversion failed: {}", err),
//     }
// }

// fn main() {
//     println!("Find the sum of all the numbers with odd squares under 1000");
//     let upper = 1000;

//     // Imperative approach
//     // Declare accumulator variable
//     let mut acc = 0;
//     // Iterate: 0, 1, 2, ... to infinity
//     for n in 0.. {
//         // Square the number
//         let n_squared = n * n;

//         if n_squared >= upper {
//             // Break loop if exceeded the upper limit
//             break;
//         } else if is_odd(n_squared) {
//             // Accumulate value, if it's odd
//             acc += n_squared;
//         }
//     }
//     println!("imperative style: {}", acc);

//     // Functional approach
//     let sum_of_squared_odd_numbers: u32 =
//         (0..).map(|n| n * n)                             // All natural numbers squared
//              .take_while(|&n_squared| n_squared < upper) // Below upper limit
//              .filter(|&n_squared| is_odd(n_squared))     // That are odd
//              .sum();                                     // Sum them
//     println!("functional style: {}", sum_of_squared_odd_numbers);
// }

// A module named `my_mod`

// mod my_mod {
//     // Items in modules default to private visibility.
//     fn private_function() {
//         println!("called `my_mod::private_function()`");
//     }

//     // Use the `pub` modifier to override default visibility.
//     pub fn function() {
//         println!("called `my_mod::function()`");
//     }

//     // Items can access other items in the same module,
//     // even when private.
//     pub fn indirect_access() {
//         print!("called `my_mod::indirect_access()`, that\n> ");
//         private_function();
//     }

//     // Modules can also be nested
//     pub mod nested {
//         pub fn function() {
//             println!("called `my_mod::nested::function()`");
//         }

//         #[allow(dead_code)]
//         fn private_function() {
//             println!("called `my_mod::nested::private_function()`");
//         }

//         // Functions declared using `pub(in path)` syntax are only visible
//         // within the given path. `path` must be a parent or ancestor module
//         pub(in crate::my_mod) fn public_function_in_my_mod() {
//             print!("called `my_mod::nested::public_function_in_my_mod()`, that\n> ");
//             public_function_in_nested();
//         }

//         // Functions declared using `pub(self)` syntax are only visible within
//         // the current module, which is the same as leaving them private
//         pub(self) fn public_function_in_nested() {
//             println!("called `my_mod::nested::public_function_in_nested()`");
//         }

//         // Functions declared using `pub(super)` syntax are only visible within
//         // the parent module
//         pub(super) fn public_function_in_super_mod() {
//             println!("called `my_mod::nested::public_function_in_super_mod()`");
//         }
//     }

//     pub fn call_public_function_in_my_mod() {
//         print!("called `my_mod::call_public_function_in_my_mod()`, that\n> ");
//         nested::public_function_in_my_mod();
//         print!("> ");
//         nested::public_function_in_super_mod();
//     }

//     // pub(crate) makes functions visible only within the current crate
//     pub(crate) fn public_function_in_crate() {
//         println!("called `my_mod::public_function_in_crate()`");
//     }

//     // Nested modules follow the same rules for visibility
//     mod private_nested {
//         #[allow(dead_code)]
//         pub fn function() {
//             println!("called `my_mod::private_nested::function()`");
//         }

//         // Private parent items will still restrict the visibility of a child item,
//         // even if it is declared as visible within a bigger scope.
//         #[allow(dead_code)]
//         pub(crate) fn restricted_function() {
//             println!("called `my_mod::private_nested::restricted_function()`");
//         }
//     }
// }

// mod my_module; // Declare the module

// mod my {
//     // A public struct with a public field of generic type `T`
//     pub struct OpenBox<T> {
//         pub contents: T,
//     }

//     // A public struct with a private field of generic type `T`
//     pub struct ClosedBox<T> {
//         contents: T,
//     }

//     impl<T> ClosedBox<T> {
//         // A public constructor method
//         pub fn new(contents: T) -> ClosedBox<T> {
//             ClosedBox {
//                 contents: contents,
//             }
//         }
//     }
// }
// fn function() {
//     println!("called `function()`");
// }

// fn main() {
//     my_module::public_function(); // Access the public function
//     // Modules allow disambiguation between items that have the same name.
//     function();
//     my_mod::function();

//     // Public items, including those inside nested modules, can be
//     // accessed from outside the parent module.
//     my_mod::indirect_access();
//     my_mod::nested::function();
//     my_mod::call_public_function_in_my_mod();

//     // pub(crate) items can be called from anywhere in the same crate
//     my_mod::public_function_in_crate();

//     // Public structs with public fields can be constructed as usual
//     let open_box: my::OpenBox<&str> = my::OpenBox { contents: "public information" };

//     // and their fields can be normally accessed.
//     println!("The open box contains: {}", open_box.contents);

//     // Public structs with private fields cannot be constructed using field names.
//     // Error! `ClosedBox` has private fields
//     //let closed_box = my::ClosedBox { contents: "classified information" };
//     // TODO ^ Try uncommenting this line

//     // However, structs with private fields can be created using
//     // public constructors
//     let _closed_box: my::ClosedBox<&str> = my::ClosedBox::new("classified information");

//     // and the private fields of a public struct cannot be accessed.
//     // Error! The `contents` field is private
//     //println!("The closed box contains: {}", _closed_box.contents);
//     // TODO ^ Try uncommenting this line
// }

// fn recursive_function(n: u64) {
//     println!("Recursion level: {}", n);
//     recursive_function(n + 1); // No base case, leading to infinite recursion
// }

// fn main() {
//     recursive_function(1); // Start recursion at level 1
// }

// use std::thread;

// fn recursive_function(n: u64) {
//     println!("Recursion level: {}", n);
//     recursive_function(n + 1); // Infinite recursion will eventually overflow
// }

// fn main() {
//     let stack_size: usize = 64 * 1024; // Set stack size to 64 KB (just an example)
//     let builder: thread::Builder = thread::Builder::new().stack_size(stack_size);

//     let handler: Result<thread::JoinHandle<()>, std::io::Error> = builder.spawn(|| {
//         recursive_function(1); // Start the recursive function in this thread
//     });

//     match handler {
//         Ok(h) => {
//             let _ = h.join(); // Wait for the thread to finish (or overflow)
//         }
//         Err(e) => {
//             println!("Thread encountered an error: {:?}", e);
//         }
//     }
// }

// fn main() {
//     let mut x: Box<i32> = Box::new(1);
//     let a: i32 = *x; // *x reads the heap value, so a = 1
//     print!("{a:?}\n");
//     *x += 1; // *x on the left-side modifies the heap value,
//              //     so x points to the value 2
//     print!("{a:?}\n");

//     let r1: &Box<i32> = &x; // r1 points to x on the stack
//     let b: i32 = **r1; // two dereferences get us to the heap value
//     print!("{b:?}bbb\n");

//     let r2: &i32 = &*x; // r2 points to the heap value directly
//     let c: i32 = *r2; // so only one dereference is needed to read it
//     print!("{c:?}\n");
// }

// fn return_a_string() -> String {
//     let s: String = String::from("Hello world");
//     s
// }

// fn main(){
//     let mut x: String = return_a_string();
//     let y=x.clone();
//     let z=x.clone();
//     x.push_str("syz");
//     print!("{x:?}");
//     print!("{y:?}");
//     print!("{z:?}");

// }

// fn main() {
//     // let x = 5; // `x` is stored on the stack
//     // let y = x; // `y` is a copy of `x` (x is copied because integers implement the `Copy` trait)
//     // let z = x;
//     // println!("x = {}, y = {} z={z}", x, y); // both `x` and `y` are valid and accessible

//     let x: i32 = 10;  // `x` is on the stack
//     let y: &i32 = &x;  // `y` is an immutable reference to `x`
//     println!("x = {}, y = {}", x, y); // both `x` and `y` are valid
// }

// fn main() {
//     let s1: String = String::from("Hello"); // `s1` is a String stored on the heap
//     let s2: String = s1;                    // Ownership of the heap data is moved from `s1` to `s2`

//     // println!("{}", s1);          // Error! `s1` is no longer valid
//     println!("{}", s2);              // `s2` is now the owner of the heap data
// }

// fn main() {
//     let s1: String = String::from("Rust");  // Heap allocation
//     let s2: &String = &s1;                   // Immutable reference to `s1`
//     println!("s1 = {}, s2 = {}", s1, s2); // Both `s1` and `s2` can be accessed
// }
// struct BankAccount {
//     name: String,
//     balance: f64,
// }

// impl BankAccount {
//     // Method to read the balance (immutable reference)
//     fn get_balance(&self) -> f64 {
//         println!("{} has {}", self.name, self.balance);  // Use `{}` for normal formatting
//         self.balance
//     }

//     // Method to deposit money (mutable reference)
//     fn deposit(&mut self, amount: f64) {
//         self.balance += amount;
//     }

//     // Method to withdraw money (mutable reference)
//     fn withdraw(&mut self, amount: f64) -> bool {
//         if self.balance >= amount {
//             self.balance -= amount;
//             true
//         } else {
//             false
//         }
//     }
// }

// fn main() {
//     let mut account: BankAccount = BankAccount {
//         name: String::from("Alice"),
//         balance: 1000.0,
//     };

//     // Multiple components can read the balance concurrently (immutable reference)
//     let balance_reader1: &BankAccount = &account;  // Immutable reference to account
//     let balance_reader2: &BankAccount = &account;  // Another immutable reference
//     println!("Balance Reader 1 sees: ${}", balance_reader1.get_balance());
//     println!("Balance Reader 2 sees: ${}", balance_reader2.get_balance());

//     // You cannot modify the balance while it’s being read:
//     // account.deposit(500.0); // This will cause a compile-time error because immutable borrows exist

//     // After the immutable references are done, we can modify the account
//     account.deposit(500.0);  // Now we can modify the balance
//     println!("New balance after deposit: ${}", account.get_balance());

//     account.withdraw(300.0);
//     println!("New balance after withdrawal: ${}", account.get_balance());
// }

// fn main() {
//     // let colors: Vec<&str> = vec!["blue", "red", "green"];
//     // // method 1: access vector elements using vector index
//     // println!("first color = {}", colors[0]);
//     // println!("second color = {}", colors[1]);
//     // println!("third color = {}", colors[2]);
//     // // method 2: access vector elements using get() method and vector index
//     // println!("first color = {:?}", colors.get(7));
//     // println!("second color = {:?}", colors.get(1).unwrap());
//     // println!("third color = {:?}", colors.get(2).unwrap());
//     // mutable vector
//     let v: Vec<i32> = vec![2, 4, 6, 8, 10];
//     let mut even_numbers: Vec<i32> = vec![2, 4, 6, 8, 10];

//     println!("original vector = {:?}", v);

//     // push values at the end of the vector
//     even_numbers.push(12);
//     even_numbers.push(14);

//     println!("changed vector = {:?}", v);
//     println!("changed even vector = {:?}", even_numbers);
//     // remove value from the vector in its second index
//     even_numbers.remove(22);
// }

// fn main() {
//     let colors: Vec<&str> = vec!["blue", "red", "green"];

//     let mut index: i32 = 0;

//     // loop through a vector to print its index and value
//     for color in colors {
//         println!("Index: {} -- Value: {}", index, color);
//         index = index + 1;
//     }

//     let v: Vec<i32> = Vec::new();
//     print!("{v:?}")
// }

// fn main() {
//     // let v: Vec<i32> = vec![1, 2, 3, 4, 5];

//     // // let does_not_exist = &v[1];
//     // let does_not_exist = v.get(1);

//     // print!("{does_not_exist:?}");
//     // print!("{v:?}");
//     // let mut v = vec![1, 2, 3, 4, 5];

//     // let first = &v[0];

//     // println!("The first element is: {first}");
//     // v.push(6);

//     // println!("The v element is: {v:?}");
//     let mut v: Vec<i32> = vec![100, 32, 57];
//     for i in &mut v {
//         // *i += 50;
//         // *i += 50;
//         // *i += 50;
//         // *i += 50;
//         // *i += 50;
//         // *i += 50;
//         print!("{i:?}\n")
//     }
//     print!("{v:?}\n");
// }

// #![allow(unused)]
// fn main() {
//     use std::ptr;
//     use std::mem;

//     let v: Vec<usize> = vec![1, 2, 3];

//     // Prevent running `v`'s destructor so we are in complete control
//     // of the allocation.
//     let mut v: mem::ManuallyDrop<Vec<usize>> = mem::ManuallyDrop::new(v);

//     // Pull out the various important pieces of information about `v`
//     let p: *mut usize = v.as_mut_ptr();
//     let len: usize = v.len();
//     let cap: usize = v.capacity();

//     unsafe {
//         // Overwrite memory with 4, 5, 6
//         for i in 0..len {
//             ptr::write(p.add(i), 4 + i);
//         }

//         // Put everything back together into a Vec
//         let rebuilt: Vec<usize> = Vec::from_raw_parts(p, len, cap);
//         assert_eq!(rebuilt, [4, 5, 6]);
//     }
// }

// fn main() {
//     use std::collections::HashMap;

//     let mut scores: HashMap<String, i32> = HashMap::new();

//     scores.insert(String::from("Blue"), 10);
//     scores.insert(String::from("Yellow"), 50);
//     scores.insert(String::from("Yellow1"), 50);
//     scores.insert(String::from("Yellow2"), 50);
//     scores.insert(String::from("Yellow3"), 50);
//     scores.insert(String::from("Yellow4"), 504);
//     scores.entry("key3".to_string()).or_insert(30);
//     scores.entry("key3".to_string()).or_insert(30);

//     print!("before------{scores:?}\n");

//     for (key, value) in &scores {
//         println!("{}: {}", key, value);
//     }
//     let filtered_map: HashMap<_, _> = scores
//     .iter()
//     .filter(|&(_key, &value)| value > 10)
//     // .map(|(key, &value)| (*key, value)) // Deref to copy the key-value pair
//     .collect();

//     print!("{filtered_map:?}\n");
//     print!("{scores:?}\n");

//     print!("{:?}\n", scores.len());
// }

// use std::collections::HashMap;

// fn main() {
//     let mut map: HashMap<i32, i32> = HashMap::new();
//     map.insert(1, 10);
//     map.insert(2, 20);
//     map.insert(3, 5);
//     print!("{map:?}");
//     let filtered_map: HashMap<_, _> = map
//         .iter()
//         .filter(|&(_key, &value)| value > 10)
//         .map(|(&key, &value)| (key, value)) // Copying the integers
//         .collect();

//     println!("{:?}", filtered_map); // Output: {1: 10, 2: 20}
// }

// #![allow(unused)]
// fn main() {
//     use std::collections::HashSet;
//     // Type inference lets us omit an explicit type signature (which
//     // would be `HashSet<String>` in this example).
//     let mut books = HashSet::new();

//     // Add some books.
//     books.insert("A Dance With Dragons".to_string());
//     books.insert("To Kill a Mockingbird".to_string());
//     books.insert("The Odyssey".to_string());
//     books.insert("The Great Gatsby".to_string());

//     // Check for a specific one.
//     if !books.contains("The Winds of Winter") {
//         println!("We have {} books, but The Winds of Winter ain't one.",
//                  books.len());
//     }

//     print!("{books:?}");
//     // Remove a book.
//     books.remove("The Odyssey");

//     // Iterate over everything.
//     for book in &books {
//         println!("{book}");
//     }
// }

// #![allow(unused)]
// fn main() {
//     use std::collections::HashSet;
//     #[derive(Hash, Eq, PartialEq, Debug)]
//     struct Viking {
//         name: String,
//         power: usize,
//     }

//     let mut vikings: HashSet<Viking> = HashSet::new();

//     vikings.insert(Viking { name: "Einar".to_string(), power: 9 });
//     vikings.insert(Viking { name: "Einar".to_string(), power: 9 });
//     vikings.insert(Viking { name: "Olaf".to_string(), power: 4 });
//     vikings.insert(Viking { name: "Harald".to_string(), power: 8 });
//     print!("{vikings:?}");
//     // // Use derived implementation to print the vikings.
//     // for x in &vikings {
//     //     println!("{x:?}");
//     // }
// }

// use std::collections::HashSet;

// fn main() {
//     let set1: HashSet<_> = [1, 2, 3].iter().cloned().collect();
//     let set2: HashSet<_> = [3, 4, 5].iter().cloned().collect();
//     let union: HashSet<_> = set1.union(&set2).cloned().collect();
//     let intersection: HashSet<_> = set1.intersection(&set2).cloned().collect();
//     let difference: HashSet<_> = set1.difference(&set2).cloned().collect();
//     let symmetric_difference: HashSet<_> = set1.symmetric_difference(&set2).cloned().collect();
//     print!("{union:?}union\n");
//     print!("{intersection:?}intersection\n");
//     print!("{difference:?}difference\n");
//     print!("{symmetric_difference:?}symmetric_difference\n");
//     let mut set_clone: HashSet<i32> = set1.clone();
//     // set_clone.clear();
//     print!("{set_clone:?}\n");

//     for value in set_clone.iter() {
//         println!("{}---------------", value);
//     }

//     for value in set_clone.drain() {
//         println!("{}", value);
//     }

//     let mut set: HashSet<i32> = HashSet::new();
//     set.insert(1);
//     set.insert(2);
//     set.insert(3);

//     // Using get method and storing the value in a second variable
//     if let Some(&value) = set.get(&2) {
//         let second_variable: i32 = value; // Store the value in a second variable
//         let third_variable: i32 = value; // Store the value in a second variable
//         println!("Found: {}", second_variable);
//         println!("Found: {}", third_variable);

//     } else {
//         println!("Not found");
//     }
// }

// Defining a declarative macro using `macro_rules!`
// macro_rules! mainu {
//     // Base case: single argument, just return it
//     ($x:expr) => {
//         $x
//     };

//     // Recursive case: compare two or more arguments
//     ($x:expr, $($rest:expr),+) => {
//         // Compare the current element with the result of the rest
//         std::cmp::min($x, mainu!($($rest),+))
//     };
// }

// fn main() {
//     let result1: i32 = mainu!(3);
//     let result2: i32 = mainu!(5, 2, 8, 1, 4);

//     println!("The minimum of 3 is: {}", result1);
//     println!("The minimum of (5, 2, 8, 1, 4) is: {}", result2);
// }
// #[derive(Debug)]
// struct Node {
//     value: i32,
//     next: Option<Box<Node>>,
// }

// #[derive(Debug)]
// struct MyLinkedList {
//     head: Option<Box<Node>>,
// }

// impl MyLinkedList {
//     // Create a new empty LinkedList
//     fn new() -> Self {
//         MyLinkedList { head: None }
//     }

//     // Add a new node at the beginning of the list
//     fn push(&mut self, value: i32) {
//         let new_node: Box<Node> = Box::new(Node {
//             value,
//             next: self.head.take(),
//         });
//         self.head = Some(new_node);
//     }

//     // Print all elements in the list
//     fn print_list(&self) {
//         let mut current_node: &Option<Box<Node>> = &self.head;
//         while let Some(node) = current_node {
//             print!("{} -> ", node.value);
//             current_node = &node.next;
//         }
//         println!("None");
//     }

//      // Get the node at the specified index and return its value and the address of the next node
//     fn get_node(&self, index: usize) -> Option<(&i32, Option<&Box<Node>>)> {
//         let mut current: &Option<Box<Node>> = &self.head;
//         let mut current_index = 0;

//         while let Some(ref node) = current {
//             if current_index == index {
//                 return Some((&node.value, node.next.as_ref()));
//             }
//             current_index += 1;
//             current = &node.next;
//         }

//         // If the index is out of range, return None
//         None
//     }
// }

// fn main() {
// let mut list: MyLinkedList = MyLinkedList::new();

// // Adding elements to the list
// list.push(10);
// list.push(20);
// list.push(30);
// list.push(40);
// // list.push(50);
// // list.push(60);

// // Printing the list
// list.print_list(); // Output: 30 -> 20 -> 10 -> None

//  // Testing the get_node method
//  if let Some((value, next)) = list.get_node(0) {
//     println!("Value at index 0: {}, Next address: {:?}", value, next); // Output: Value at index 0: 30, Next address: Some(Node { value: 20, next: Some(Node { value: 10, next: None }) })
// } else {
//     println!("Index 0 not found");
// }

// if let Some((value, next)) = list.get_node(1) {
//     println!("Value at index 1: {}, Next address: {:?}", value, next); // Output: Value at index 1: 20, Next address: Some(Node { value: 10, next: None })
// } else {
//     println!("Index 1 not found");
// }

// if let Some((value, next)) = list.get_node(2) {
//     println!("Value at index 2: {}, Next address: {:?}", value, next); // Output: Value at index 2: 10, Next address: None
// } else {
//     println!("Index 2 not found");
// }
//     use std::collections::LinkedList;

//     // 1. Create a new LinkedList
//     let mut list: LinkedList<i32> = LinkedList::new();
//     // 2. Adding elements
//     list.push_front(1); // Adds 1 at the front: List = [1]
//     list.push_back(2); // Adds 2 at the end:   List = [1, 2]
//     list.push_back(3); // Adds 3 at the end:   List = [1, 2, 3]
//     list.push_front(0); // Adds 0 at the front: List = [0, 1, 2, 3]
//     print!("{list:?}");

//     // 3. Accessing elements without removing them
//     if let Some(front) = list.front() {
//         println!("Front element: {}", front); // Output: Front element: 0
//     }

//     if let Some(back) = list.back() {
//         println!("Back element: {}", back); // Output: Back element: 3
//     }

//     // 4. Removing elements
//     let first: Option<i32> = list.pop_front(); // Removes and returns the front element (0), List = [1, 2, 3]
//     println!("Removed front element: {:?}", first); // Output: Removed front element: Some(0)

//     let last: Option<i32> = list.pop_back(); // Removes and returns the last element (3), List = [1, 2]
//     println!("Removed back element: {:?}", last); // Output: Removed back element: Some(3)

//     // 5. Checking length and if the list is empty
//     println!("Current length: {}", list.len()); // Output: Current length: 2
//     println!("Is the list empty? {}", list.is_empty()); // Output: Is the list empty? false

//     // 6. Iterating over the list
//     println!("Current list values:");
//     for value in &list {
//         println!("{}", value); // Output: 1, 2
//     }

//     // 7. Clear the list
//     list.clear();
//     println!("List cleared. Is the list empty now? {}", list.is_empty()); // Output: List cleared. Is the list empty now? true

// }

// #[derive(Debug)]
// pub struct List<T> {
//     head: Link<T>,
// }

// type Link<T> = Option<Box<Node<T>>>;

// #[derive(Debug)]
// struct Node<T> {
//     elem: T,
//     next: Link<T>,
// }

// impl<T> List<T> {
//     pub fn new() -> Self {
//         List { head: None }
//     }

//     pub fn push(&mut self, elem: T) {
//         let new_node: Box<Node<T>> = Box::new(Node {
//             elem: elem,
//             next: self.head.take(),
//         });

//         self.head = Some(new_node);
//     }

//     pub fn pop(&mut self) -> Option<T> {
//         self.head.take().map(|node: Box<Node<T>>| {
//             self.head = node.next;
//             node.elem
//         })
//     }

//     pub fn peek(&self) -> Option<&T> {
//         self.head.as_ref().map(|node: &Box<Node<T>>| &node.elem)
//     }
// }

// impl<T> Drop for List<T> {
//     fn drop(&mut self) {
//         let mut cur_link: Option<Box<Node<T>>> = self.head.take();
//         while let Some(mut boxed_node) = cur_link {
//             cur_link = boxed_node.next.take();
//         }
//     }
// }

// fn main() {
//     let mut list: List<i32> = List::new();

//     // // Populate list
//     list.push(1);
//     list.push(2);
//     list.push(3);
//     list.push(4);
//     list.push(5);
//     // print!("{list:?}");
//     print!("{:?}",list.peek())
// }

// #![allow(unused)]
// fn main() {
//     use std::collections::BinaryHeap;

//     // Type inference lets us omit an explicit type signature (which
//     // would be `BinaryHeap<i32>` in this example).
//     let mut heap: BinaryHeap<i32> = BinaryHeap::new();

//     // We can use peek to look at the next item in the heap. In this case,
//     // there's no items in there yet so we get None.
//     assert_eq!(heap.peek(), None);

//     // Let's add some scores...
//     heap.push(1);
//     heap.push(5);
//     heap.push(2);
//     // heap.push(7);
//     // heap.push(8);
//     // heap.push(9);
//     // heap.push(99);
//     // heap.push(45);
//     // heap.push(65);
//     // heap.push(3);
//     // heap.push(5);
//     // heap.push(51);
//     // heap.push(50);
//     // heap.push(21);
//     // heap.push(27);
// // 99, 65, 51, 45, 5, 50, 27, 1, 7, 3, 5, 2, 9, 8, 21
//     print!("{:?}\n", heap);
//     // a random order.
//     for x in &heap {
//         println!("{x}");
//     }

//     // We can clear the heap of any remaining items.
//     heap.clear();

//     // The heap should now be empty.
//     assert!(heap.is_empty())
// }

// #[derive(Debug)]
// pub struct MinHeap {
//     data: Vec<i32>,
// }

// impl MinHeap {
//     /// Creates a new, empty MinHeap
//     pub fn new() -> Self {
//         MinHeap { data: Vec::new() }
//     }

//     /// Returns the parent index of a given child index
//     fn parent_index(&self, index: usize) -> usize {
//         (index - 1) / 2
//     }

//     /// Returns the left child index of a given parent index
//     fn left_child_index(&self, index: usize) -> usize {
//         2 * index + 1
//     }

//     /// Returns the right child index of a given parent index
//     fn right_child_index(&self, index: usize) -> usize {
//         2 * index + 2
//     }

//     /// Inserts a new element into the heap
//     pub fn insert(&mut self, value: i32) {
//         // Add the value to the end of the array
//         self.data.push(value);

//         // Heapify up from the last element
//         let mut index: usize = self.data.len() - 1;
//         while index > 0 {
//             let parent = self.parent_index(index);
//             if self.data[index] < self.data[parent] {
//                 self.data.swap(index, parent);
//             }
//             index = parent;
//         }
//     }

//     /// Retrieves the minimum element without removing it
//     pub fn peek(&self) -> Option<&i32> {
//         self.data.first()
//     }

//     /// Extracts and returns the minimum element from the heap
//     pub fn extract_min(&mut self) -> Option<i32> {
//         if self.data.len() == 0 {
//             return None;
//         }

//         let min = self.data.swap_remove(0);

//         if !self.data.is_empty() {
//             // Heapify down from the root element
//             let mut index = 0;
//             let last_index = self.data.len() - 1;

//             loop {
//                 let left: usize = self.left_child_index(index);
//                 let right: usize = self.right_child_index(index);

//                 let mut smallest: usize = index;

//                 if left <= last_index && self.data[left] < self.data[smallest] {
//                     smallest = left;
//                 }

//                 if right <= last_index && self.data[right] < self.data[smallest] {
//                     smallest = right;
//                 }

//                 if smallest != index {
//                     self.data.swap(index, smallest);
//                     index = smallest;
//                 } else {
//                     break;
//                 }
//             }
//         }

//         Some(min)
//     }
// }

// fn main() {
//     let mut heap: MinHeap = MinHeap::new();
//     heap.insert(10);
//     heap.insert(3);
//     heap.insert(5);
//     heap.insert(1);
//     heap.insert(55);
//     heap.insert(2);

//     print!("{:?}\n", heap);
//     println!("Min element: {:?}", heap.peek()); // Should output: 1

//     println!("Extract Min: {:?}", heap.extract_min()); // Should output: 1
//     println!("Min element after extraction: {:?}", heap.peek()); // Should output: 3

//     heap.insert(2);
//     println!("Min element after inserting 2: {:?}", heap.peek()); // Should output: 2
// }

// Define a generic Stack struct
// #[derive(Debug)]
// struct Stack<T> {
//     elements: Vec<T>,
// }

// impl<T> Stack<T> {
//     // Create a new, empty stack
//     fn new() -> Self {
//         Stack {
//             elements: Vec::new(),
//         }
//     }

//     // Push an element onto the stack
//     fn push(&mut self, item: T) {
//         self.elements.push(item);
//     }

//     // Pop an element off the stack
//     fn pop(&mut self) -> Option<T> {
//         self.elements.pop()
//     }

//     // Peek at the top element without removing it
//     fn peek(&self) -> Option<&T> {
//         self.elements.last()
//     }

//     // Check if the stack is empty
//     fn is_empty(&self) -> bool {
//         self.elements.is_empty()
//     }

//     // Get the size of the stack
//     // fn size(&self) -> usize {
//     //     self.elements.len()
//     // }
// }

// fn main() {
//     // Create a new stack of integers
//     let mut stack: Stack<i32> = Stack::new();

//     // Push elements onto the stack
//     stack.push(1);
//     stack.push(2);
//     stack.push(3);
//     print!("{:?}",stack);
//     // Peek at the top element
//     if let Some(top) = stack.peek() {
//         println!("Top element is: {}", top);
//     }

//     // Pop elements off the stack
//     while let Some(top) = stack.pop() {
//         println!("Popped element: {}", top);
//     }

//     // Check if the stack is empty
//     if stack.is_empty() {
//         println!("The stack is empty!");
//     }
// }

// use std::io::{self, Write};

// fn main() {
//     print!("Enter your name: ");
//     // Make sure to flush stdout to immediately print the prompt
//     io::stdout().flush().expect("Failed to flush");

//     // Create a mutable string to store the input
//     let mut input: String = String::new();

//     // Read the input from the standard input (stdin)
//     io::stdin()
//         .read_line(&mut input)
//         .expect("Failed to read line");

//     // Remove the trailing newline character
//     let input = input.trim();

//     println!("Hello, {}!", input);
// }
// use rpassword::read_password;

// use std::io::{self, Write};

// fn main() {
//     // Function to read input from the user
//     fn read_input(prompt: &str) -> String {
//         print!("{}", prompt);
//         // Ensure the prompt is printed immediately
//         io::stdout().flush().expect("Failed to flush stdout");

//         // Create a mutable string to hold the user input
//         let mut input: String = String::new();

//         // Read input from stdin and handle errors
//         io::stdin()
//             .read_line(&mut input)
//             .expect("Failed to read line");

//         // Remove any trailing whitespace or newlines
//         input.trim().to_string()
//     }

//     // Ask for the first name
//     let first_name: String = read_input("Enter your first name: ");

//     // Ask for the email
//     let email: String = read_input("Enter your email: ");

//     print!("Enter your password: ");
//     io::stdout().flush().expect("Failed to flush stdout");
//     let password: String = read_password().expect("Failed to read password");

//     // Display the collected information (omit or handle this securely in real-world applications)
//     println!("\nCollected Information:");
//     println!("First Name: {}", first_name);
//     println!("Email: {}", email);
//     println!("Password: {}", password); // In a real scenario, avoid printing passwords
// }

// use std::process::Command;

// fn main() {
//     // Spawn a separate process to run an external command (e.g., `ls` to list files)
//     let output: std::process::Output = Command::new("ls") // Use "dir" if you're on Windows
//         .arg("-l") // Adding an argument to the command
//         .output()
//         .expect("Failed to execute command");

//     // Check if the command was successful
//     if output.status.success() {
//         let stdout: std::borrow::Cow<'_, str> = String::from_utf8_lossy(&output.stdout);
//         println!("Command output:\n{}", stdout);
//     } else {
//         let stderr: std::borrow::Cow<'_, str> = String::from_utf8_lossy(&output.stderr);
//         eprintln!("Command failed:\n{}", stderr);
//     }
// }

// use std::thread;
// use std::time::Duration;

// fn main() {
//     let mut handles: Vec<thread::JoinHandle<()>> = vec![];

//     // Create 5 worker threads
//     for i in 0..5 {
//         // Spawn a thread and push the handle into the vector
//         let handle: thread::JoinHandle<()> = thread::spawn(move || {
//             // Simulate processing work with a sleep
//             println!("Thread {} is processing a task...", i);
//             thread::sleep(Duration::from_secs(10));
//             println!("Thread {} has completed the task.", i);
//         });

//         // Store the handle so we can wait for the thread to finish later
//         handles.push(handle);
//     }

//     // Wait for all threads to complete
//     for handle in handles {
//         handle.join().expect("Failed to join thread");
//     }

//     println!("All threads have completed their tasks.");
// }

// use std::process::Command;
// use std::thread;
// use std::time::Duration;

// fn main() {
//     let mut handles: Vec<thread::JoinHandle<()>> = vec![];

//     // Create threads to simulate concurrent processing
//     for i in 0..3 {
//         let handle: thread::JoinHandle<()> = thread::spawn(move || {
//             // Simulate doing some work
//             println!("Thread {} is processing data...", i);
//             thread::sleep(Duration::from_secs(2));
//             println!("Thread {} finished processing data.", i);

//             // Let's assume that after processing, we want to call an external command
//             if i == 2 {
//                 // Example: Running an external command like `echo`
//                 let output: std::process::Output = Command::new("echo")
//                     .arg(format!("Hello from thread {}", i))
//                     .output()
//                     .expect("Failed to execute external command");

//                 if output.status.success() {
//                     let stdout: std::borrow::Cow<'_, str> = String::from_utf8_lossy(&output.stdout);
//                     println!("Thread {}: External command output:\n{}", i, stdout);
//                 } else {
//                     let stderr = String::from_utf8_lossy(&output.stderr);
//                     eprintln!("Thread {}: External command failed:\n{}", i, stderr);
//                 }
//             }
//         });

//         handles.push(handle);
//     }

//     // Wait for all threads to complete
//     for handle in handles {
//         handle.join().expect("Failed to join thread");
//     }

//     println!("All threads have completed their tasks.");
// }

// use std::process::{Command, Stdio};
// use std::thread;
// use std::time::Duration;
// // use std::io::{self, Write};

// fn spawn_threads_in_process() {
//     let handles: Vec<_> = (1..=3).map(|i| {
//         thread::spawn(move || {
//             for j in 1..=5 {
//                 println!("Thread {}: iteration {}", i, j);
//                 thread::sleep(Duration::from_millis(500)); // Simulate some work
//             }
//         })
//     }).collect();

//     // Wait for all threads to finish
//     for handle in handles {
//         handle.join().unwrap();
//     }
// }

// fn main() {
//     // Creating a new process using std::process::Command
//     let mut child: std::process::Child = Command::new("echo")
//         .arg("Starting a new process with threads inside it...")
//         .stdout(Stdio::inherit())
//         .spawn()
//         .expect("Failed to start process");

//     // Wait for the process to finish
//     let _ = child.wait().expect("Failed to wait on process");

//     // Now, simulate multiple threads inside this process
//     println!("Spawning threads in the main process:");
//     spawn_threads_in_process();

//     println!("All threads have finished their execution.");
// }

// use std::sync::{Arc, Mutex};
// use std::thread;
// use std::time::Duration;

// // Represents the number of seats in the theater
// const NUM_SEATS: usize = 10;

// fn main() {
//     // Create a shared vector of seats, all initially unbooked (false)
//     let seats: Vec<bool> = vec![false; NUM_SEATS];
//     let shared_seats: Arc<Mutex<Vec<bool>>> = Arc::new(Mutex::new(seats));

//     // Create threads simulating users trying to book seats
//     let mut handles: Vec<thread::JoinHandle<()>> = vec![];

//     for user_id in 1..=5 {
//         // Clone the shared Arc<Mutex> to pass into the thread
//         let seats_clone: Arc<Mutex<Vec<bool>>> = Arc::clone(&shared_seats);

//         let handle: thread::JoinHandle<()> = thread::spawn(move || {
//             for attempt in 1..=3 {
//                 // Try to book a seat
//                 let mut seats: std::sync::MutexGuard<'_, Vec<bool>> = seats_clone.lock().expect("Failed to acquire lock");

//                 if let Some(index) = seats.iter_mut().position(|&mut booked| !booked) {
//                     // Book the seat if available
//                     seats[index] = true;
//                     println!("User {} successfully booked seat {} on attempt {}", user_id, index + 1, attempt);
//                     break;
//                 } else {
//                     println!("User {} found no available seats on attempt {}", user_id, attempt);
//                     thread::sleep(Duration::from_millis(100)); // Wait before retrying
//                 }
//             }
//         });

//         handles.push(handle);
//     }

//     // Wait for all threads to complete
//     for handle in handles {
//         handle.join().expect("Failed to join thread");
//     }

//     // Print final seat bookings
//     let final_seats: std::sync::MutexGuard<'_, Vec<bool>> = shared_seats.lock().expect("Failed to acquire lock");
//     println!("Final seat bookings: {:?}", final_seats);
// }

// with lock
// use std::sync::{Arc, Mutex};
// use std::thread;

// fn main() {
//     // Create a shared counter with initial value 0
//     let counter: Arc<Mutex<i32>> = Arc::new(Mutex::new(0));

//     // Create multiple threads
//     let mut handles: Vec<thread::JoinHandle<()>> = vec![];

//     for _ in 0..10 {
//         let counter_clone: Arc<Mutex<i32>> = Arc::clone(&counter);
//         let handle: thread::JoinHandle<()> = thread::spawn(move || {
//             // Lock the mutex to gain access to the counter
//             let mut num: std::sync::MutexGuard<'_, i32> = counter_clone.lock().unwrap();
//             *num += 1;
//         });
//         handles.push(handle);
//     }

//     // Wait for all threads to complete
//     for handle in handles {
//         handle.join().unwrap();
//     }

//     // Print the final value of the counter
//     println!("Final counter value: {}", *counter.lock().unwrap());
// }

// use std::sync::{Arc, Mutex};
// use std::thread;

// fn main() {
//     let counter: Arc<Mutex<i32>> = Arc::new(Mutex::new(0));

//     let mut handles: Vec<thread::JoinHandle<()>> = vec![];

//     for _ in 0..10 {
//         let counter_clone: Arc<Mutex<i32>> = Arc::clone(&counter);

//         // Spawn a new thread
//         let handle: thread::JoinHandle<()> = thread::spawn(move || {
//             // Directly access the counter without locking
//             let _num: i32 = *counter_clone.lock().unwrap(); // This works correctly

//             // Uncomment the following line to see the error
//             // *counter_clone += 1;  // Compilation error: cannot dereference Arc<Mutex>
//         });

//         handles.push(handle);
//     }

//     for handle in handles {
//         handle.join().unwrap();
//     }

//     println!("Final counter value: {}", *counter.lock().unwrap());
// }

// #![allow(unused)]
// fn main() {
//     use std::collections::BTreeMap;

//     // type inference lets us omit an explicit type signature (which
//     // would be `BTreeMap<&str, &str>` in this example).
//     let mut movie_reviews: BTreeMap<&str, &str> = BTreeMap::new();

//     // review some movies.
//     movie_reviews.insert("Office Space", "Deals with real issues in the workplace.");
//     movie_reviews.insert("Pulp Fiction", "Masterpiece.");
//     movie_reviews.insert("The Godfather", "Very enjoyable.");
//     movie_reviews.insert("The Blues Brothers", "Eye lyked it a lot.");
//     print!("{movie_reviews:?}");
    // // check for a specific one.
    // if !movie_reviews.contains_key("Les Misérables") {
    //     println!(
    //         "We've got {} reviews, but Les Misérables ain't one.",
    //         movie_reviews.len()
    //     );
    // }

    // // oops, this review has a lot of spelling mistakes, let's delete it.
    // movie_reviews.remove("The Blues Brothers");

    // // look up the values associated with some keys.
    // let to_find: [&str; 2] = ["Up!", "Office Space"];
    // for movie in &to_find {
    //     match movie_reviews.get(movie) {
    //         Some(review) => println!("{movie}: {review}"),
    //         None => println!("{movie} is unreviewed."),
    //     }
    // }
    // let getreview: Option<&&str> = movie_reviews.get("Pulp Fiction");

    // // The base string to concatenate the review if it exists
    // let mut base_string: String = "Movie Review: ".to_string();

    // match getreview {
    //     Some(&review) => {
    //         // Concatenate the review to the base string
    //         base_string.push_str(review);
    //     }
    //     None => {
    //         // Optionally, you can add a default message for not found cases
    //         base_string.push_str("None");
    //     }
    // }

    // // Look up the value for a key (will panic if the key is not found).
    // println!("Movie review: {}", movie_reviews["Pulp Fiction"]);
    // println!("Movie xxxxxxxxxxxxxxxxxxxxreview: {:?}", base_string);

    // // iterate over everything.
    // for (movie, review) in &movie_reviews {
    //     println!("{movie}: \"{review}\"");
    // }
// }

use std::collections::BTreeSet;

fn main() {
    let mut set = BTreeSet::new();
    
    // Inserting elements
    set.insert(10);
    set.insert(20);
    set.insert(30);
    set.insert(5);
    
    // Attempting to insert a duplicate
    let duplicate_insertion: bool = set.insert(10); // Will return false since 10 already exists
    print!("{duplicate_insertion}");
    // Removing an element
    set.remove(&20);
    
    // Checking for membership
    if set.contains(&10) {
        println!("Set contains 10");
    } else {
        println!("Set does not contain 10");
    }
    
    // Iterating over the set
    for element in &set {
        println!("{}", element);
    }
}

