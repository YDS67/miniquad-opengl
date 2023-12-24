pub const WIDTH: i32 = 1280;
pub const HEIGHT: i32 = 800;
pub const WIDTHF: f32 = 1280.0;
pub const HEIGHTF: f32 = 800.0;
pub const ASPECT: f32 = HEIGHTF/WIDTHF;
pub const MAPSIZE: usize = 256;
pub const PLAYERHEIGHT: f32 = 0.5;
pub const TILESCREENSIZE: f32 = 3.0;
pub const MAPOFFSETX: f32 = WIDTHF / 2.0 - TILESCREENSIZE * (MAPSIZE as f32)/2.0;
//pub const MAPOFFSETX: f32 = 10.0;
pub const PLAYERX0: f32 = 0.0;
pub const PLAYERY0: f32 = 0.0;
pub const PLAYERA0: f32 = PI / 4.0;
pub const PLAYERB0: f32 = 0.0;
pub const FOVXY: f32 = PI/3.0;
pub const FOVZ: f32 = HEIGHT as f32 / WIDTH as f32 * FOVXY;
pub const DELTATIME: f32 = 1.0/60.0;
pub const PLAYERSPEED: f32 = 0.1;
pub const PLAYERSIZE: f32 = 0.5;
pub const PI: f32 = 3.14159;
pub const MAXDRAWDIST: usize = MAPSIZE * 4;
pub const NUMRAYS: usize = 250;
pub const TOLERANCE: f32 = 1e-16;