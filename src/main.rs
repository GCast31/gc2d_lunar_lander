use game::Game;

mod game;
mod lander;

fn main() {

    gc2d::gc2d::Gc2d::new().run(Game::new()).unwrap();

}
