extern crate piston_window;

use piston_window::*;
use graphics::image::{Image, ImageFormat};
use std::fs::File;
use std::path::Path;

struct Player {
  pos_x : i16,
  pos_y : i16,
}

fn main() {
  let window : PistonWindow = WindowSettings::new( "Asteroidy", ( 800, 600 ) )
    .exit_on_esc( true )
    .build()
    .unwrap_or_else( |e| { panic!( "Failed to build PistonWindow: {}", e) } );

  let player_load = File::open( "player.png" );

  // Load the player texture here.
  let player = Image::load( player_load, ImageFormat::PNG );

  for mut e in window {
    e.set_max_fps( 60 );
    e.draw_2d( |_c, g| {
      clear( [0.5, 1.0, 0.5, 1.0], g );
    });
  }
}