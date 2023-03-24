use ggez;
use ggez::input::keyboard::KeyCode;
use ggez::mint::Point2;
use ggez::{event, graphics, Context, GameError, GameResult};

use rand::{thread_rng, Rng};

const SCREEN_HEIGHT: f32 = 600.;
const SCREEN_WIDTH: f32 = 600.;

const X_OFFSET: f32 = 20.;
const PADDLE_WIDTH: f32 = 12.;
const PADDLE_HEIGHT: f32 = 75.;

const BALL_RADIUS: f32 = 12.;

struct Ball {
    rect: graphics::Rect,
    vel: Point2<f32>,
}

impl Ball {
    fn new() -> Self {
        let mut rng = thread_rng();
        let mut x_vel = rng.gen_range(3.5..=4.5);
        let mut y_vel = rng.gen_range(3.5..=4.5);

        if rng.gen_bool(0.5) {
            x_vel = -1.;
        }

        if rng.gen_bool(0.5) {
            y_vel = -1.;
        }

        Ball {
            rect: graphics::Rect {
                x: SCREEN_WIDTH / 2.0 - BALL_RADIUS / 2.0,
                y: SCREEN_HEIGHT / 2.0 - BALL_RADIUS / 2.0,
                w: BALL_RADIUS,
                h: BALL_RADIUS,
            },
            vel: Point2 { x: x_vel, y: y_vel },
        }
    }
}

struct MainState {
    left_paddle: ggez::graphics::Rect,
    right_paddle: ggez::graphics::Rect,
    ball: Ball,
    left_score: u16,
    right_score: u16,
}

impl MainState {
    pub fn new(_: &mut Context) -> GameResult<MainState> {
        let main_state = MainState {
            left_paddle: graphics::Rect {
                x: X_OFFSET,
                y: SCREEN_HEIGHT / 2.,
                w: PADDLE_WIDTH,
                h: PADDLE_HEIGHT,
            },
            right_paddle: graphics::Rect {
                x: SCREEN_WIDTH - X_OFFSET,
                y: SCREEN_HEIGHT / 2.,
                w: PADDLE_WIDTH,
                h: PADDLE_HEIGHT,
            },
            ball: Ball::new(),
            left_score: 0,
            right_score: 0,
        };
        Ok(main_state)
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> Result<(), GameError> {
        // handle keyboard input
        if ctx.keyboard.is_key_pressed(KeyCode::W) {
            self.left_paddle.y -= 5.; // starting from top left corner, this is why - instead of +
        }
        if ctx.keyboard.is_key_pressed(KeyCode::S) {
            self.left_paddle.y += 5.; // starting from top left corner, this is why - instead of +
        }
        if ctx.keyboard.is_key_pressed(KeyCode::Up) {
            self.right_paddle.y -= 5.; // starting from top left corner, this is why - instead of +
        }
        if ctx.keyboard.is_key_pressed(KeyCode::Down) {
            self.right_paddle.y += 5.; // starting from top left corner, this is why - instead of +
        }

        // move ball
        self.ball.rect.translate(self.ball.vel);

        // handle collision
        if (self.ball.vel.x < 0. && self.ball.rect.overlaps(&self.left_paddle))
            || (self.ball.vel.x > 0. && self.ball.rect.overlaps(&self.right_paddle))
        {
            self.ball.vel.x *= -1.;
        }
        if (self.ball.vel.y < 0. && self.ball.rect.top() < 0.)
            || (self.ball.vel.y > 0. && self.ball.rect.bottom() > SCREEN_HEIGHT)
        {
            self.ball.vel.y *= -1.;
        }
        if self.left_paddle.y < 0. && self.left_paddle.top() < 0. {
            self.left_paddle.y = 0.;
        }
        if self.right_paddle.y < 0. && self.right_paddle.top() < 0. {
            self.right_paddle.y = 0.;
        }
        if self.left_paddle.y > 0. && self.left_paddle.bottom() > SCREEN_HEIGHT {
            self.left_paddle.y = SCREEN_HEIGHT - PADDLE_HEIGHT;
        }
        if self.right_paddle.y > 0. && self.right_paddle.bottom() > SCREEN_HEIGHT {
            self.right_paddle.y = SCREEN_HEIGHT - PADDLE_HEIGHT;
        }

        // handle scores: if ball goes beyond left/right wall
        if self.ball.rect.left() < 0. {
            self.right_score += 1;
            std::thread::sleep(std::time::Duration::from_secs(1));
            self.ball = Ball::new();
        }
        if self.ball.rect.right() > SCREEN_WIDTH {
            self.left_score += 1;
            std::thread::sleep(std::time::Duration::from_secs(1));
            self.ball = Ball::new();
        }

        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> Result<(), GameError> {
        let mut canvas = graphics::Canvas::from_frame(ctx, graphics::Color::BLACK);

        let left_paddle_mesh = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            self.left_paddle,
            graphics::Color::RED,
        )
        .expect("Error creating left paddle mesh");
        let right_paddle_mesh = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            self.right_paddle,
            graphics::Color::RED,
        )
        .expect("Error creating right paddle mesh");

        let ball_mesh = graphics::Mesh::new_rounded_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            self.ball.rect,
            BALL_RADIUS,
            graphics::Color::BLUE,
        )
        .expect("Error creating ball mesh");

        canvas.draw(&ball_mesh, graphics::DrawParam::default());
        canvas.draw(&left_paddle_mesh, graphics::DrawParam::default());
        canvas.draw(&right_paddle_mesh, graphics::DrawParam::default());

        let scoreboard =
            graphics::Text::new(format!("L: {} \t R: {}", self.left_score, self.right_score));

        canvas.draw(
            &scoreboard,
            graphics::DrawParam::default().dest([SCREEN_WIDTH / 2., 10.]),
        );

        canvas.finish(ctx).expect("error in canvas finish");
        Ok(())
    }
}

fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("Pong", "Sinan Gencoglu")
        .window_mode(ggez::conf::WindowMode::default().dimensions(SCREEN_WIDTH, SCREEN_HEIGHT));
    let (mut ctx, event_loop) = cb.build()?; // (?): build can fail
    ctx.gfx.set_window_title("PONG");

    let state = MainState::new(&mut ctx)?;
    event::run(ctx, event_loop, state);
}
