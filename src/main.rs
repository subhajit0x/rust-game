use ggez::*;

struct State {}

impl ggez::event::EventHandler<GameError> for State {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        Ok(())
    }
}

fn main() {
    let state = State {};
    let c = conf::Conf::new();
    let (ctx, event_loop) = ContextBuilder::new("game", "me")
        .default_conf(c)
        .build()
        .unwrap();

    event::run(ctx, event_loop, state);
}
