// use std::time;

use ggez::graphics::DrawParam;
use ggez::*;

struct State {
    dt: std::time::Duration,

    t: graphics::Text,
    dp: DrawParam,
}

impl ggez::event::EventHandler for State {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        while timer::check_update_time(ctx, 60) {
            // update_game_physics()?;
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        // println!("Hello ggez! dt = {}ms", self.dt.as_millis());
        graphics::clear(ctx, graphics::BLACK);
        // let t = graphics::Text::new(graphics::TextFragment::from(format!(
        //     "Hello ggez! dt = {}ms",
        //     self.dt.as_millis()
        // )));
        // let dp = DrawParam::default(); //.color(graphics::BLACK);
        // graphics::draw(ctx, &t, dp)?;
        self.t.fragments_mut()[0].text = format!(
            "Hello ggez! {}Hz",
            1.0 / (1e-9 * (self.dt.as_nanos() as f64))
        );
        self.dt = timer::delta(ctx);
        graphics::draw(ctx, &self.t, self.dp)?;
        graphics::present(ctx)?;
        timer::yield_now();
        Ok(())
    }

    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut Context,
        _button: event::MouseButton,
        _x: f32,
        _y: f32,
    ) {
    }

    fn mouse_button_up_event(
        &mut self,
        _ctx: &mut Context,
        _button: event::MouseButton,
        _x: f32,
        _y: f32,
    ) {
    }

    fn mouse_motion_event(&mut self, _ctx: &mut Context, _x: f32, _y: f32, _dx: f32, _dy: f32) {}

    fn mouse_wheel_event(&mut self, _ctx: &mut Context, _x: f32, _y: f32) {}

    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        keycode: event::KeyCode,
        _keymods: event::KeyMods,
        _repeat: bool,
    ) {
        if keycode == event::KeyCode::Escape {
            event::quit(ctx);
        }
    }

    fn key_up_event(
        &mut self,
        _ctx: &mut Context,
        _keycode: event::KeyCode,
        _keymods: event::KeyMods,
    ) {
    }

    fn focus_event(&mut self, _ctx: &mut Context, _gained: bool) {}

    fn resize_event(&mut self, _ctx: &mut Context, _width: f32, _height: f32) {}
}

fn main() {
    let state = &mut State {
        dt: std::time::Duration::new(0, 0),
        t: graphics::Text::new(graphics::TextFragment::from(format!(
            "Hello ggez! dt = {}ms",
            0
        ))),
        dp: DrawParam::default(),
    };
    let mut c = conf::Conf::new();
    c.window_setup.title = String::from("hashtorio!").to_owned();

    let (ref mut ctx, ref mut event_loop) = ContextBuilder::new("hello_ggez", "DomNomNom")
        .conf(c)
        .build()
        .unwrap();
    event::run(ctx, event_loop, state).unwrap();
}
