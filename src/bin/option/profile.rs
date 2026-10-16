// TODO: Define a public Profile struct with:
// - username: String
// - nickname: Option<String>
#[derive(Debug)]
pub struct Profile {
    // TODO: Add fields here
    username: String,
    nickname: Option<String>
}

impl Profile {
    // TODO: Implement new constructor
    // Takes a username (String) and an optional nickname (Option<String>)
    pub fn new(username: String, nickname: Option<String>) -> Self {
        // TODO: Create and return a new Profile
        Profile {
            username,
            nickname
        }
    }

    // TODO: Implement display_name method
    // Returns the nickname if it exists, or the username as fallback
    // Hint: Use unwrap_or or unwrap_or_else
        // TODO: Return nickname if Some, otherwise return username
    pub fn display_name(&self) -> String {
        self.nickname
            .clone()
            .unwrap_or_else(|| self.username.clone())
    }

    // TODO: Implement formatted_nickname method
    // Returns Option<String> with nickname wrapped in brackets like "[CoolNick]"
    // Returns None if there's no nickname
    // Hint: Use map to transform the Option value
    pub fn formatted_nickname(&self) -> Option<String> {
        // TODO: Use map to transform nickname into bracketed format
        self.nickname
            .as_ref()
            .map(|s| format!("[{}]", s))
    }
}
