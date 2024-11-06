// use struct_display_macro::AutoDisplay;

// #[derive(AutoDisplay)]
// struct ImplExample2 {
//     t: i32,
//     other_field: String,
//     additional_field: f64,
//     x:String
// }

// fn main() {
//     let x: ImplExample2 = ImplExample2 {
//         t: 10,
//         other_field: String::from("example"),
//         additional_field: 42.5,
//         x:"test".to_string()
//     };
//     println!("{}", x);
// }


// macro_rules! generate_enum_with_functions {
//     ($enum_name:ident, $($variant:ident),*) => {
//         enum $enum_name {
//             $($variant),*
//         }

//         impl $enum_name {
//             fn as_str(&self) -> &'static str {
//                 match self {
//                     $(Self::$variant => stringify!($variant)),*
//                 }
//             }

//             fn print_variant(&self) {
//                 println!("Enum variant: {}", self.as_str());
//             }
//         }
//     };
// }

// #[allow(dead_code)]
// fn main() {
//     // Generate an enum with the macro
//     generate_enum_with_functions!(MyEnum, Variant1, Variant2, Variant3, Variant4);
//     let variant: MyEnum = MyEnum::Variant2;
//     variant.print_variant();
// }


// macro_rules! impl_debug_display {
//     ($($struct_name:ident),*) => {
//         $(
//             impl std::fmt::Debug for $struct_name {
//                 fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//                     write!(f, "{} {{ id: {}, name: {} }}", stringify!($struct_name), self.id, self.name)
//                 }
//             }

//             impl std::fmt::Display for $struct_name {
//                 fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//                     write!(f, "{}", self.name)
//                 }
//             }
//         )*
//     };
// }

// struct EntityA {
//     id: u32,
//     name: String,
// }

// struct EntityB {
//     id: u32,
//     name: String,
// }

// // Apply the macro
// impl_debug_display!(EntityA, EntityB);

// fn main() {
//     let entity_a: EntityA = EntityA { id: 1, name: String::from("EntityA") };
//     let entity_b: EntityB = EntityB { id: 2, name: String::from("EntityB") };

//     println!("{:?}", entity_a);
//     println!("{:?}", entity_b);
// }


// use builder_macro::Builder;

// #[derive(Builder)]
// struct User {
//     name: String,
//     age: u32,
//     email: String,
// }

// fn main() {
//     let user = User::builder()
//         .name("Alice".to_string())
//         .age("30".parse().unwrap())
//         .email("alice@example.com".to_string())
//         .build()
//         .expect("Failed to build User");

//     println!("User: {:?}", user);
// }

fn main(){
    
}