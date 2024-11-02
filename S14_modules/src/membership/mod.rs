pub mod member_management;

struct Member {
    id: u32,
    full_name: String,
    level: u8,
}

impl Member {
    pub fn new(id: u32, full_name: String, level: u8) -> Self {
        Member {
            id,
            full_name,
            level,
        }
    }
    fn get_bonus() {}
}
