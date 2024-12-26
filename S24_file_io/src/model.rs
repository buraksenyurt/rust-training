use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct Game<'a> {
    pub id: u32,
    pub name: &'a str,
    pub release_year: u16,
    pub category: &'a str,
    pub score: f32,
}

impl<'a> Game<'a> {
    pub fn new(id: u32, name: &'a str, release_year: u16, category: &'a str, score: f32) -> Self {
        Game {
            id,
            name,
            release_year,
            category,
            score,
        }
    }
}

impl<'a> Display for Game<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{},{},{},{},{}",
            self.id, self.name, self.release_year, self.category, self.score
        )
    }
}
