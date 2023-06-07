fn main() {

   /* Tuples 
        - Destructing tuples
        - Nested tuples
   */

   let my_info = ("Salary", 40_000);
   println!("{} is equal to {}", my_info.0, my_info.1);

   let (salary, salary_value) = my_info;
   println!("The values of the individual variables are, {} and {}", &salary, salary_value);

   let nested_tuples = (4, 5.5, (3, 2.5), "Hello");
   let element = nested_tuples.2.0;
   println!("The value of element is {}", element);

   /* Arrays
         - Updating elements
         - Strings and char arrays
         - Slices
    */

   let mut number_array = [4,5,6,8,9];
   println!("{}", number_array[0]);

   println!("{:?}", number_array);

   number_array[4] = 5;
   println!("{:?}", number_array);

   let string_array = ["apple", "orange", "grapes", "banana"];
   println!("{:?}", string_array);

   let subset_array = &number_array[0..=3];
   println!("The subset of the value of the array are {:?}", subset_array);

   println!("Elements in the array are {}", number_array.len());

   println!("The array is occupying {} bytes", std::mem::size_of_val(&number_array));

   let check_index = number_array.get(2);
   println!("{:?}", check_index);


   /* Vectors */

   let mut number_vec = vec![2,3,4,6,7,8,9,3,5,0, 10, 15, 38];
   println!("{}", number_vec[0]);

   number_vec[4] = 40;
   println!("{:?}", number_vec);

   let mut string_array_1 = vec!["apple", "orange", "grapes", "banana"];
   string_array_1[0] = "John Doe";

   let subset_vec = &number_vec[0..6];
   println!("The subset of values of the vector are {:?}", subset_vec);

   println!("Elements in the vector are {}", number_vec.len());

   let check_index = number_vec.get(7);
   println!("{:?}", check_index);

   number_vec.push(50);
   number_vec.push(90);

   println!("The values of the vector are {:?}", number_vec);

   number_vec.remove(5);
   println!("The vector after removing the element at index 5 is {:?}", number_vec);

   println!("The value of 3 exist in the vector {}", number_vec.contains(&3));

}
