fn main() {
     /* Rust ownership
          Each value in Rust has an owner.
          There can only be one owner at a time.
          When the owner goes out of scope, the value will be dropped.
      */

     let  x = 30;
     let  y = x;
     println!("The value of x {} and the value of y {}", x, y);

     let s1 = String::from("hello");
     let s2 = &s1; // borrowing and referencing
     let s3 = s1.clone(); // cloning
     println!("The value of s1 {} and the value of s2 {} or s3 {}", s1, s2, s3);

     let num_vec1 = vec![2,4,6,8,3,5,7,9];
     let num_vec2 = &num_vec1; // borrowing and referencing
     println!("The value of first vector is {:?} and the second vector is {:?}", num_vec1, num_vec2);

     let num_vec2 = num_vec1.clone(); // cloning 
     println!("The value of first vector is {:?} and the second vector is {:?}", num_vec1, num_vec2);

     /* ownership functions */

     let stack_num = 32;
     let mut heap_num = vec![4,5,6,9,3];

     stack_function(stack_num);
     println!("The stack variable is copied and the original value was {} ", stack_num);

     heap_function(&mut heap_num);
     println!("The value of the vector outside the function is {:?}", heap_num)

}



fn stack_function(mut stack_num: i32)
{
     stack_num = 56;
     println!("The copied value of the variable has been updated to {}", stack_num);
}

fn heap_function(var: &mut Vec<i32>){
     var.push(35);
     println!("The value of the vector inside the function is {:?}", var)
}