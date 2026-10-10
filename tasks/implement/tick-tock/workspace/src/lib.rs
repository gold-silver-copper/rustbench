/// Returns the "tick-tock" sequence for the numbers `1..=n`, joined with `,`
/// (no spaces).
///
/// Each number `i` is written as:
/// - `TickTock` if it is a multiple of both 3 and 5,
/// - otherwise `Tick` if it is a multiple of 3,
/// - otherwise `Tock` if it is a multiple of 5,
/// - otherwise `i` itself in decimal.
///
/// Then, if the decimal form of `i` contains the digit `7`, a `!` is appended
/// to what was written (so `7` becomes `7!` and `27` becomes `Tick!`).
/// `n == 0` gives an empty string.
///
/// ```
/// assert_eq!(task::tick_tock(7), "1,2,Tick,4,Tock,Tick,7!");
/// ```
pub fn tick_tock(n: u32) -> String {
    todo!()
}
