extern crate piston_window; 
extern crate cgmath;

type Vector2 = cgmath::Vector2<f32>;
type Pos2 = cgmath::Point2<f32>;

use piston_window::*;

trait Draw {
  fn draw( &self, w : &PistonWindow, r : RenderArgs );
}

struct Game;

impl Game {
  fn update( &mut self, dt : f32 ) {

  }
}

struct Player {
  graphics : Rectangle,
  pos : Pos2,
  vel : Vector2,
}

impl Player {
  fn new( graphics : Rectangle
          , pos : Pos2, vel : Vector2 ) -> Player {
    Player {
      graphics : graphics,
      pos : pos,
      vel : vel,
    }
  }
  
}

impl Draw for Player {
  fn draw( &self, w : &PistonWindow, r : RenderArgs ) {
    w.draw_2d( |ctx, gfx| {
      rectangle( [ 1.0, 0.0, 0.0, 1.0 ]
               , [ self.pos.x as f64, self.pos.y as f64, 10.0, 10.0 ]
               , ctx.transform
               , gfx );
    } );
  }
}

fn main() {
  let mut window : PistonWindow = WindowSettings::new( "Asteroidy", ( 800, 600 ) )
    .exit_on_esc( true )
    .build()
    .unwrap_or_else( |e| { panic!( "Failed to build PistonWindow: {}", e) } );

  window.set_max_fps( 60 );
  window.set_ups( 120 );

  let mut player = Player::new( Rectangle::new( [1.0, 0.0, 0.0, 1.0] )
                                , Pos2::new( 100.0, 100.0 )
                                , Vector2::new( 5.0, 5.0) );

  for e in window {
    
    if let Some( ra ) = e.render_args() {
      e.draw_2d( |_c, g| {
        clear( [0.0, 0.0, 0.0, 0.0], g );
      }); 

      player.draw( &e, ra );
    }

    if let Some( button ) = e.press_args() {
      match button {
        Button::Keyboard( Key::W ) => player.vel.y -= 5.0,
        Button::Keyboard( Key::A ) => player.vel.x -= 5.0,
        Button::Keyboard( Key::S ) => player.vel.y += 5.0,
        Button::Keyboard( Key::D ) => player.vel.x += 5.0,
        _ => {},

      }
    }
  }
}
