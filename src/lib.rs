#![allow(clashing_extern_declarations)]
#![warn(missing_docs)]

//! You should export a tick function with the following signature:
//! ```
//! #[no_mangle]
//! pub extern "C" fn tick(tick: u32) {}
//! ```

pub(crate) mod bindings {
    /// The base creates new spirits once it has enough energy.
    /// If your base if destroyed you lose the game.
    pub mod base;
    /// Mappings for the JavaScript console.
    pub mod console;
    /// Yare.io is a RTS game played by executing code.
    pub mod game;
    /// Render circles and lines on the map.
    /// First set the colour and then draw a shape.
    pub mod graphics;
    /// Outposts are objects that can be captured by energizing them.
    /// Outposts will shoot at random energies within their range.
    pub mod outpost;
    /// Information about players in the game.
    pub mod player;
    /// Position is given as an x and y coordinate on an Euclidean plane.
    pub mod position;
    /// Spirits are the game's units. They carry energy which is used for
    /// creating new spirits, fighting, and capturing outposts.
    pub mod spirit;
    /// Stars are sources of energy.
    pub mod star;
}

#[cfg(not(feature = "headless"))]
pub use bindings::*;

#[cfg(feature = "headless")]
mod headless {}

#[cfg(feature = "headless")]
pub use headless::*;
