use std::io;

fn main() {

    let mut input_array: String = String::new();

    let mut input_k: String = String::new();

    println!("Enter the elements of the array separated by spaces");
    io::stdin().read_line(&mut  input_array).expect("Failed to read line");

    println!("Enter the number of positions to rotate (k)");
    io::stdin().read_line(&mut input_k).expect("Failed to read line");

    let arr: Vec<i32> = input_array.split_whitespace().map(|s| s.parse().expect("Invalid Input")).collect();

    let k:usize = input_k.trim().parse().expect("Invalid input");

    let mut rotated_array = arr.clone();

    rotate_array(&mut rotated_array, k);

    println!("Original array: {:?}", arr);
    println!("Rotated Array: {:?}", rotated_array);

}

fn rotate_array(arr: &mut [i32], k: usize) {
   let n = arr.len();

   if n==0 || k ==0 {
    return;
   }
   let k = k%n;

   arr[0..k].reverse();
   arr[k..].reverse();
   arr.reverse();
}