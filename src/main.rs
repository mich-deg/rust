fn main() {
    println!("Hello, world!");

    /* Printing Outputs */

    println!("The value of the constant is {}", 10);

    println!("First name {}, Last name {}", "John", "Doe");

    println!("\n I love {2}, using {1} and {0}", "JavaScript", "Rust", "coding");

    println!("{language} is a language empowering everyone to build reliable and efficient software", language="Rust");
 
    /* Variables in Rust */

    let x = 15;
    println!("The value of the variable x = {}", x);

    /* Scalar Data Types 

        Integer: 
            - signed: i8, i16, i32, i64 -2
            - unsigned (only +ve number ): u8, u16, u32 

        Floats
            -f32, f64

     */

    println!("The maximum number in i8 is equal to {}", std::i8::MAX);
    println!("The maximum number in u8 is equal to {}", std::u8::MAX);

    /* Scalar Data Types 

        Floats
            -f32, f64

     */
    let z = 3.65;

    println!("The maximum number in f32 is {}", std::f32::MAX);

    /* Scalar Data Types 

        Boolean 

     */
    let status = false;
    println!("The values of some of our variables are {:?}", (x, z, status));

    /* Scalar Data Types 

        Characters

     */
     
     let c1 = 'A';
     let c2 = '\u{288A}';
     
     println!("The value of C1 = {} and C2 = {} ", c1, c2);

}
