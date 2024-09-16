use std :: collections::HashMap;
use std :: io;
fn main() {
let mut expenses: HashMap<String, f32> = HashMap::new();
loop {
let mut input = String::new();
println!("Enter expense (or 'exit' to quit): ");
io::stdin().read_line(&mut input).unwrap();
let input = input.trim();


if input == "exit" {
break;
}
let parts: Vec<&str> =  input.split_whitespace().collect();
if parts.len() != 2 {
println!("Please enter in the format: <name> <amount>");
continue;
}
let name = parts[0].to_string();
let amount: f32 = parts[1].parse().unwrap();
expenses.insert(name,amount);
}

println!("Your expenses: {:?}",expenses);
}