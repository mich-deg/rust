fn main() {

     basic_fn();
     
     function_with_inputs("John", 40_000);

     let first_name = "Doe";
     let salary_info = 50_000;
     function_with_inputs(first_name, salary_info);

    let multiply =  function_io(10, 15);
    println!("The answer of the multiplication is {}", multiply );

    let (multiplication, addition, subtraction) = function_input_with_multiple_output(10, 15);
    println!("M = {}, Add = {}, Sub = {}", multiplication, addition, subtraction);

    /* Code Blocks */

    let full_name = {
     let first_name = "John";
     let last_name = "Doe";
     format!("{} {}", first_name, last_name)
    };

    println!("My full name is {}", full_name);

    /* Inputs from user */

    let mut n = String::new();
    std::io::stdin()
    .read_line(&mut n)
    .expect("failed to read input");
    
    let n:f64 = n.trim().parse().expect("invalid input");
    println!("{:?}", n);

}

fn basic_fn(){
     println!("This is a basic function");
}

fn function_with_inputs(name: &str, salary: i32){
     println!("The name is {} and the salary is {}",name, salary);
}

fn function_io(num1:i32, num2:i32) -> i32 {
     num1 * num2
}

fn function_input_with_multiple_output(num1:i32, num2:i32) -> (i32, i32, i32) {
     (num1*num2, num1+num2, num1-num2)
}