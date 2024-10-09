<!-- What is Rust? -->
Rust is a systems programming language focused on speed, memory safety, and parallelism. It was designed to provide high performance while preventing common programming errors, particularly those related to memory management, which can lead to vulnerabilities and crashes. Rust is increasingly used for applications where performance and reliability are crucial, such as web servers, operating systems, and game engines.

Memory Safety
- at the level of c and c++

Zero-Cost Abstractions
    - rust gives high level features without implementation of abstractions like other language like js, python etc.

Debugging in Rust
    #[derive(Debug)]
        fmt::Display
             - used for normal display(with formate) and it does not take {:?} into println.
        
        fmt::Debug
            - Used for developer friendly display(with formate) and it takes {:?} into println.
        
    
Dead-code
    - unused functions, structures etc can be skipped by #[allow(dead_code)] line above it for not compiling it


macro_rules
    - Macro rules are used for define own macros.
    Difference between macro and function
        - macros will generate before compile time and it is dynamic.
        - functions are executed at runtime

Primitives
    Signed integers: i8, i16, i32, i64, i128 and isize (pointer size)
    Unsigned integers: u8, u16, u32, u64, u128 and usize (pointer size)
    Floating point: f32, f64
    char Unicode scalar values like 'a', 'α' and '∞' (4 bytes each)
    bool either true or false
    The unit type (), whose only possible value is an empty tuple: ()

Compound Types
    Tuples
    Arrays
    Mutability
    Inference

Structures
Enums
    Type aliases
    c-like enums 
    cast
    Testcase: linked-list

Attributes
    - used for conditional compilation


match cases in rust
    - Some and None

impl and trait

<!-- use keyword (pending------------------------------------------------) -->


variables ownership, scop and borrowing 
    - borrowing a single variable acrose multiple functions with mutable and immutable aproaches.

Box - Smart  pointer