mod keyvalue;

use keyvalue::KeyValue;

fn main() {
    // Read five inputs
    let mut input1 = String::new();
    std::io::stdin().read_line(&mut input1).expect("Failed to read line");
    let string_key = input1.trim().to_string();

    let mut input2 = String::new();
    std::io::stdin().read_line(&mut input2).expect("Failed to read line");
    let int_value: i32 = input2.trim().parse().expect("Failed to parse");

    let mut input3 = String::new();
    std::io::stdin().read_line(&mut input3).expect("Failed to read line");
    let replacement_int: i32 = input3.trim().parse().expect("Failed to parse");

    let mut input4 = String::new();
    std::io::stdin().read_line(&mut input4).expect("Failed to read line");
    let int_key: i32 = input4.trim().parse().expect("Failed to parse");

    let mut input5 = String::new();
    std::io::stdin().read_line(&mut input5).expect("Failed to read line");
    let string_value = input5.trim().to_string();

    // TODO: Create a KeyValue<String, i32> pair using string_key and int_value
    let mut key_value: KeyValue<String, i32> = KeyValue::new(string_key, int_value);
    // TODO: Print the key and value in format: "Key: {key}, Value: {value}"
    println!("Key: {}, Value: {}",key_value.key(), key_value.value());
    // TODO: Update the value using set_value with replacement_int
    key_value.set_value(replacement_int);
    // TODO: Print the updated value in format: "Updated value: {value}"
    println!("Updated value: {}",key_value.value());
    // TODO: Create a KeyValue<i32, String> pair using int_key and string_value
    let string_input: KeyValue<i32, String> = KeyValue::new(int_key, string_value);
    // TODO: Print this pair's key and value in format: "Key: {key}, Value: {value}"
    println!("Key: {}, Value: {}",string_input.key(),string_input.value());
}
