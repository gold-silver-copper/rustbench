use crate::teams::{PER_TEAM, TEAMS};

/// Seats for people who watch but don't play.
pub const SPECTATOR_SLOTS: usize = 2;

/// Everyone who can be in a lobby at once.
pub const MAX_PLAYERS: usize = TEAMS * PER_TEAM + SPECTATOR_SLOTS;

pub fn has_room(joined: usize) -> bool {
    joined < MAX_PLAYERS
}
