pub fn answer() -> u32 {
    42
}

pub mod bar {
    pub fn question() -> &'static str {
        "the meaning of everything"
    }
}

pub mod bar_defined_as_a_file;