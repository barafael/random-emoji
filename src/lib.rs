use rand::{thread_rng, Rng};

pub fn generate() -> char {
    // overly simple, but works
    char::from_u32(thread_rng().gen_range(0x1F600..=0x1F64F)).unwrap_or('💔')
}
