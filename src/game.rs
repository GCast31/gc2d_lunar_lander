
use gc2d::{event::EventLoop, gc2d::Gc2dResult, keyboard::KeyCode, color::Color, graphics::DrawMode};

use crate::lander::Lander;

const IMAGE_SHIP: &str = "assets/images/ship.png";
const IMAGE_ENGINE: &str = "assets/images/engine.png";

pub struct Game {
    screen_width: f32,
    screen_height: f32,

    lander: Lander,
}

impl Game {
    pub fn new() -> Self {
        Self {
            screen_height: 0.,
            screen_width: 0.,
            lander: Lander::default(),
        }
    }
}

impl EventLoop for Game {

    fn load(&mut self, gc2d: &mut gc2d::gc2d::Gc2d, _audio_manager: &mut gc2d::audio::AudioManager) -> Gc2dResult<()> {

        gc2d.window.set_title("Formation Gamecodeur.com - Lunar Lander (with Gc2d)");

        gc2d.graphics.new_image(IMAGE_SHIP).unwrap();
        gc2d.graphics.new_image(IMAGE_ENGINE).unwrap();

        self.screen_height = gc2d.graphics.get_height();
        self.screen_width = gc2d.graphics.get_width(); 

        self.lander.image = String::from(IMAGE_SHIP);
        self.lander.image_engine = String::from(IMAGE_ENGINE);

        self.lander.x = self.screen_width / 2. - gc2d.graphics.get_image_width(self.lander.image.as_str()) / 2.;
        self.lander.y = self.screen_height / 2. - gc2d.graphics.get_image_height(self.lander.image.as_str()) / 2.;
        self.lander.angle = 270.;
        self.lander.speed = 3.;

        self.lander.engine_x = 
            (gc2d.graphics.get_image_width(self.lander.image_engine.as_str()) - 
             gc2d.graphics.get_image_width(self.lander.image.as_str())
            ) / 2.;

        self.lander.engine_y =  
            (gc2d.graphics.get_image_height(self.lander.image_engine.as_str()) - 
             gc2d.graphics.get_image_height(self.lander.image.as_str())
            ) / 2.;

        Ok(())
    }

    fn update(&mut self, gc2d: &mut gc2d::gc2d::Gc2d, dt: f32, _audio_manager: &mut gc2d::audio::AudioManager) -> Gc2dResult<()> {
        

        self.lander.vy += 0.6 * dt;

        if gc2d.keyboard.is_down(KeyCode::Right) {
            self.lander.angle += 90. * dt as f64;
            if self.lander.angle > 360. {
                self.lander.angle = 0.;
            }
        }
        if gc2d.keyboard.is_down(KeyCode::Left) {
            self.lander.angle += -90. * dt as f64;
            if self.lander.angle < 0. {
                self.lander.angle = 360.;
            }
        }
        if gc2d.keyboard.is_down(KeyCode::Up) {
            self.lander.engine_enable = true;  

            let force_x = self.lander.angle.to_radians().cos() as f32 * (self.lander.speed * dt);
            self.lander.vx += force_x;

            let force_y = self.lander.angle.to_radians().sin() as f32 * (self.lander.speed * dt);
            self.lander.vy += force_y;


        } else {
            self.lander.engine_enable = false;
        }

        let limit_velocity = |v: f32, max: f32| -> f32 {
            if v.abs() > max {
                if v.is_sign_positive() {
                    max
                } else {
                    -max
                }
            } else {
                v
            }
        };

        self.lander.vx = limit_velocity(self.lander.vx, 1.);
        self.lander.vy = limit_velocity(self.lander.vy, 1.);

        self.lander.x += self.lander.vx;
        self.lander.y += self.lander.vy;


        Ok(())
    }

    fn draw(&mut self, gc2d: &mut gc2d::gc2d::Gc2d, _fonts: &mut gc2d::fonts::FontsManager, _dt: f32) -> Gc2dResult<()> {
        gc2d.graphics.draw(
            &self.lander.image.as_str(), 
            None, 
            self.lander.x , 
            self.lander.y, 
            self.lander.angle, 
        );

        if self.lander.engine_enable {
            gc2d.graphics.draw(
                &self.lander.image_engine.as_str(), 
                None, 
                self.lander.x - self.lander.engine_x, 
                self.lander.y - self.lander.engine_y, 
                self.lander.angle
            );
        }

        Ok(())
    }

}
