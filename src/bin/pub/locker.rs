
pub struct Locker {
    code: u32,
    contents: String,
}

impl Locker {
    // Create a new locker
    pub fn new(code: u32, contents: String) -> Locker {
        Locker {
            code,
            contents,
        }
    }

    // Change the secret code
    pub fn set_code(&mut self, new_code: u32) {
        self.code = new_code;
    }

    // Return contents only if the code is correct
    pub fn get_contents(&self, attempted_code: u32) -> String {
        if attempted_code == self.code {
            self.contents.clone()
        } else {
            String::from("Access denied")
        }
    }
}
