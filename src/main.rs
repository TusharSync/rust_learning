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
fn reverse(pair: (i32, bool)) -> (bool, i32) {
    // `let` can be used to bind the members of a tuple to variables.
    let (int_param, bool_param) = pair;

    (bool_param, int_param)
}

// The following struct is for the activity.
#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

fn main() {
    // A tuple with a bunch of different types.
    let long_tuple: (u8, u16, u32, u64, i8, i16, i32, i64, f32, f64, char, bool) = (1u8, 2u16, 3u32, 4u64,
                      -1i8, -2i16, -3i32, -4i64,
                      0.1f32, 0.2f64,
                      'a', true);

    // Values can be extracted from the tuple using tuple indexing.
    println!("Long tuple first value: {}", long_tuple.0);
    println!("Long tuple second value: {}", long_tuple.1);

    // Tuples can be tuple members.
    let tuple_of_tuples: ((u8, u16, u32), (u64, i8), i16) = ((1u8, 2u16, 2u32), (4u64, -1i8), -2i16);

    // Tuples are printable.
    println!("tuple of tuples: {:?}", tuple_of_tuples);

    // But long Tuples (more than 12 elements) cannot be printed.
    //let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
    //println!("Too long tuple: {:?}", too_long_tuple);
    // TODO ^ Uncomment the above 2 lines to see the compiler error

    let pair = (1, true);
    println!("Pair is {:?}", pair);

    println!("The reversed pair is {:?}", reverse(pair));

    // To create one element tuples, the comma is required to tell them apart
    // from a literal surrounded by parentheses.
    println!("One element tuple: {:?}", (5u32,));
    println!("Just an integer: {:?}", (5u32));

    // Tuples can be destructured to create bindings.
    let tuple = (1, "hello", 4.5, true);

    let (a, b, c, d) = tuple;
    println!("{:?}, {:?}, {:?}, {:?}", a, b, c, d);

    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("{:?}", matrix);
}