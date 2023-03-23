
use ggez;
use ggez::{
    Context,
    GameResult, 
    GameError,
    graphics,
    event,
};

const PADDLE_HEIGHT:f32 = 85.;
const PADDLE_WIDTH:f32 = 18.;

struct MainState {}

impl MainState {
    pub fn new() -> Self {
        MainState {  }
    }
}

impl  event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> Result<(), GameError> {
        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> Result<(), GameError> {
        let mut canvas = graphics::Canvas::from_frame(ctx, graphics::Color::BLACK);

        let paddle = graphics::Rect::new(30., 230., PADDLE_WIDTH, PADDLE_HEIGHT);
        let paddle_mesh = graphics::Mesh::new_rectangle(ctx, 
                graphics::DrawMode::fill(), paddle, graphics::Color::BLUE)?;

        canvas.draw(&paddle_mesh, graphics::DrawParam::default());

        canvas.finish(ctx)?;
        Ok(())
    }
}

fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("Pong", "Sinan Gencoglu");
    let (ctx, event_loop) = cb.build()?; // (?): build can fail
    ctx.gfx.set_window_title("PONG");

    let state = MainState::new();
    event::run(ctx, event_loop, state);
}
