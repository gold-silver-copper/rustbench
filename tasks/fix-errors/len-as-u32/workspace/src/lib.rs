/// Number of words in a chat message (stored as u32 in the stats table).
pub fn word_count(msg: &str) -> u32 {
    msg.split_whitespace().count()
}
