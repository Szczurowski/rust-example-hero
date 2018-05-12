use std::fmt;

#[derive(Serialize, Deserialize, Debug)]
pub struct Hero {
    pub id: Option<i32>,
    pub name: String,
    pub identity: String,
    pub hometown: String,
    pub age: i32
}

impl fmt::Display for Hero {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "name:{}", self.name)
    }
}