mod profile;

use profile::Profile;

fn main() {
    // Read inputs
    let mut input1 = String::new();
    std::io::stdin().read_line(&mut input1).expect("Failed to read line");
    let username1 = input1.trim().to_string();

    let mut input2 = String::new();
    std::io::stdin().read_line(&mut input2).expect("Failed to read line");
    let nickname1 = input2.trim().to_string();

    let mut input3 = String::new();
    std::io::stdin().read_line(&mut input3).expect("Failed to read line");
    let username2 = input3.trim().to_string();

    // TODO: Create first profile with username1 and nickname1 (Some)
    let profile1: Profile = Profile::new(username1, Some(nickname1));
    // TODO: Create second profile with username2 and no nickname (None)
    let profile2: Profile = Profile::new(username2, None);
    // TODO: Print display_name and formatted_nickname for first profile
    // Use {:?} for formatted_nickname since it returns an Option
    println!("Display name: {}",profile1.display_name());
    println!("Formatted nickname: {:?}",profile1.formatted_nickname());
    // TODO: Print display_name and formatted_nickname for second profile
    println!("Display name: {}",profile2.display_name());
    println!("Formatted nickname: {:?}",profile2.formatted_nickname());

}
