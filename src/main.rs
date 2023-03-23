use ggez;
use ggez::{
    GameResult, 
    GameError,
    graphics,
    event,
};

struct MainState {}

impl MainState {
    pub fn new() -> Self {
        MainState {  }
    }
}

impl  event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut ggez::Context) -> Result<(), GameError> {
        Ok(())
    }
    fn draw(&mut self, ctx: &mut ggez::Context) -> Result<(), GameError> {
        let canvas = graphics::Canvas::from_frame(ctx, graphics::Color::BLACK);
        _ = canvas.finish(ctx);
        Ok(())
    }
}

fn main() -> GameResult {
    let ctx = ggez::ContextBuilder::new("Pong", "Sinan Gencoglu");
    let (ctx, event_loop) = ctx.build()?; // (?): build can fail
    ctx.gfx.set_window_title("PONG");

    let state = MainState::new();
    event::run(ctx, event_loop, state);
}
