fn main(){
for number in 1..=3{
	println!("This will run for numbers 1 to 3: {}",
number);
}

//Iterating over an array
let array = [41, 66, 94];
for element in array.iter(){
println!("Array element: {}", element);
   }

}