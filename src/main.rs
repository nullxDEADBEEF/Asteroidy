extern crate piston_window;
extern crate cgmath;

use std::ops::*;

type Vec2 = cgmath::Vector2<f32>;
type Pos2 = cgmath::Point2<f32>;

use piston_window::*;

trait Draw {
    fn draw<E: GenericEvent>(&self, e: &E, w: &mut PistonWindow);
}

const SPEED: f32 = 100.0;
const ACCELERATION: f32 = 75.0;

const ROCKET_SPEED: f32 = 1000.0;
const ROCKET_SIZE: f32 = 5.0;

const PLAYER_SIZE: f32 = 10.0;

struct Player {
    graphics: Vec<[f64; 2]>,
    pos: Pos2,
    heading: Vec2,
    vel: Vec2,
    angle: f32,
    rockets: Vec<Rockets>,
}

// functions regarding the player
impl Player {
    fn new(graphics: Vec<[f64; 2]>, pos: Pos2, vel: Vec2) -> Player {
        Player {
            graphics,
            pos,
            heading: Vec2::new(0.0, 0.0),
            vel,
            angle: 1.34,
            rockets: Vec::new(),
        }
    }

    fn update_movement(&mut self, dt: f32, controller_input: ControllerInput) {
        self.heading = if controller_input.move_forward {
            Vec2::new(self.angle.cos(), self.angle.sin()).mul(ACCELERATION)
        } else {
            Vec2::new(0.0, 0.0)
        };

        if controller_input.turn_left {
            self.angle -= 1.4 * dt;
        }

        if controller_input.turn_right {
            self.angle += 1.4 * dt;
        }

        self.vel = self.vel.add(&self.heading.mul(dt));

        self.vel.y = self.vel.y.min(SPEED).max(-SPEED);
        self.vel.x = self.vel.x.min(SPEED).max(-SPEED);

        self.pos.y += self.vel.y * dt;
        self.pos.x += self.vel.x * dt;


    }

    fn update_collision(&mut self, w: &PistonWindow) {
        // screen wrap checking
        if self.pos.x < 0.0 {
            self.pos = Pos2::new(w.size().width as f32 - PLAYER_SIZE, self.pos.y);
        }

        if self.pos.x > w.size().width as f32 - PLAYER_SIZE {
            self.pos = Pos2::new(0.0, self.pos.y);
        }

        if self.pos.y < 0.0 {
            self.pos = Pos2::new(self.pos.x, w.size().height as f32 - PLAYER_SIZE);
        }

        if self.pos.y > w.size().height as f32 - PLAYER_SIZE {
            self.pos = Pos2::new(self.pos.x, 0.0);
        }
    }

    fn check_player(&mut self) {}

    fn update(&mut self, dt: f32, controller_input: ControllerInput, w: &PistonWindow) {
        self.update_movement(dt, controller_input);
        self.update_collision(w);
        self.check_player();
    }
}

impl Draw for Player {
    fn draw<E: GenericEvent>(&self, e: &E, w: &mut PistonWindow) {
        w.draw_2d(e, |ctx, gfx| {
            let transform = ctx.transform
                .trans(self.pos.x as f64, self.pos.y as f64)
                .rot_rad(self.angle as f64);

            polygon([1.0, 1.0, 1.0, 1.0],
                    self.graphics.as_slice(),
                    transform,
                    gfx);
        });
    }
}

#[derive(Clone, Copy)]
struct ControllerInput {
    move_forward: bool,
    turn_left: bool,
    turn_right: bool,
    shoot: bool,
}

impl ControllerInput {
    fn new() -> ControllerInput {
        ControllerInput {
            move_forward: false,
            turn_left: false,
            turn_right: false,
            shoot: false,
        }
    }
}

macro_rules! key_status {
    ( $event: expr, $conin: expr, $($keyname: pat => $keyfield: ident),+ ) => {
      if let Some(key) = $event.press_args() {
        match key {
          $(
            Button::Keyboard($keyname) => $conin.$keyfield = true,
          )+
          _ => {}
        }
      }

      if let Some(key) = $event.release_args() {
        match key {
          $(
            Button::Keyboard($keyname) => $conin.$keyfield = false,
          )+
          _ => {}
        }
      }
    }
}

#[derive(Debug)]
struct Rockets {
    graphics: Vec<[f64; 2]>,
    pos: Pos2,
    vel: Vec2,
    angle: f32,
}

impl Rockets {
    fn new(graphics: Vec<[f64; 2]>, pos: Pos2, vel: Vec2) -> Rockets {
        Rockets {
            graphics,
            pos,
            vel,
            angle: 1.34,
        }
    }

    fn update_movement(&mut self, dt: f32, heading: Vec2) {
        self.vel.x += heading.x + ROCKET_SPEED * dt;
        self.vel.y += heading.y + ROCKET_SPEED * dt;

        self.pos.y += self.vel.y * dt;
        self.pos.x += self.vel.x * dt;
    }

    fn update(&mut self, dt: f32, heading: Vec2) {
        self.update_movement(dt, heading);
    }
}

impl Draw for Rockets {
    fn draw<E: GenericEvent>(&self, e: &E, w: &mut PistonWindow) {
        w.draw_2d(e, |ctx, gfx| {
            let transform = ctx.transform
                .trans(self.pos.x as f64, self.pos.y as f64)
                .rot_rad(self.angle as f64);

            polygon([1.0, 0.0, 0.0, 1.0],
                    self.graphics.as_slice(),
                    transform,
                    gfx)
        });
    }
}

fn main() {
    // no need for &'static since 1.17
    const TITLE: &str = "Asteroidy - v0.2.11";

    let mut game_window: PistonWindow =
        WindowSettings::new(TITLE, [800, 600])
            .exit_on_esc(true)
            .build()
            .unwrap_or_else(|e| panic!("Failed to build PistonWindow: {}", e));

    game_window.set_max_fps(60);
    game_window.set_ups(120);

    let player_poly = vec![[-10.0, -8.0], [10.0, 0.0], [-10.0, 8.0]];
    let rocket_poly = vec![[-5.0, -4.0], [5.0, 0.0], [-5.0, 4.0]];

    let mut player = Player::new(player_poly, Pos2::new(100.0, 100.0), Vec2::new(0.0, 0.0));

    // handle press/release, update and render args
    let mut controller_input = ControllerInput::new();


    while let Some(e) = game_window.next() {
        key_status!( &e, controller_input
               , Key::W     => move_forward
               , Key::A     => turn_left
               , Key::D     => turn_right
               , Key::Space => shoot );

        if let Some(ua) = e.update_args() {
            player.update(ua.dt as f32, controller_input, &game_window);
            for rocket in &mut player.rockets {
                rocket.update(ua.dt as f32, player.heading);
            }

            player
                .rockets
                .retain(|rocket| {
                            !(rocket.pos.x < 0.0 ||
                              rocket.pos.x > game_window.size().width as f32 - ROCKET_SIZE ||
                              rocket.pos.y < 0.0 ||
                              rocket.pos.y > game_window.size().height as f32 - ROCKET_SIZE)
                        });
        }

        if let Some(_) = e.render_args() {
            game_window.draw_2d(&e, |_c, g| { clear([0.0, 0.0, 0.0, 1.0], g); });
            player.draw(&e, &mut game_window);
            if controller_input.shoot {
                let rocket = Rockets::new(rocket_poly.clone(),
                                          Pos2::new(player.pos.x, player.pos.y),
                                          Vec2::new(0.0, 0.0));

                player.rockets.push(rocket);
            }
            for rocket in &player.rockets {
                rocket.draw(&e, &mut game_window);
            }
        }
    }
}
