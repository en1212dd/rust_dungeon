use crate::prelude::*;

mod map;
mod state;
mod player;

mod prelude{
    pub use bracket_lib::prelude::*;
    pub const SCREEAN_WIDTH:i32=80;
    pub const SCREEAN_HEIGHT:i32=50;
    pub use crate::map::*;
    pub use crate::player::*;
    pub use crate::state::*;
}

fn main()->BError {
    let context = BTermBuilder::simple80x50()
    .with_title("Dungeon")
    .with_fps_cap(30.0)
    .build()?;
main_loop(context, State::new())
}
