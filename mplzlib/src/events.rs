///
/// An event that is triggered when the player lands a place.
///
pub enum EventKind<'a> {
    ///
    /// Nothing happens.
    ///
    None(&'a str),

    ///
    /// Must pay dollars to the bank.
    ///
    PayToBank(&'a str, u32),

    /// Must pay dollars to the other player.
    ///
    /// The third argument is the amount of dollars to pay.
    PayToOther(&'a str, usize, u32),

    ///
    /// Get a profit.
    ///
    Reward(&'a str, u32),

    /// Give a place to the player.
    ///
    /// The second argument is the amount of dollars to pay.
    GivePlace(usize, u32),

    ///
    /// Must move to the designated place.
    ///
    Move(&'a str, usize),

    ///
    /// Get jailed.
    ///
    GetJailed,
}
