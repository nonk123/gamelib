use gamelib::*;

struct Simple {
    angle: f32,
    vel: f32,
}

impl Simple {
    fn new() -> Self {
        Self {
            angle: 0.0,
            vel: 0.5,
        }
    }
}

impl Game for Simple {
    fn configure(&self, config: &mut GameConfig) {
        config.title = "Simple".into();
    }

    fn update(&mut self, context: &mut Context) {
        self.angle += std::f32::consts::PI * self.vel * context.delta;

        if context.was_pressed(KeyCode::Space) {
            self.vel = -self.vel;
        }
    }

    fn render(&mut self, canvas: &mut Canvas, context: &mut Context) {
        let dist = 0.5;
        let size = 0.15;

        let x = self.angle.cos() * dist;
        let y = self.angle.sin() * dist;

        canvas.clear(0.0, 0.0, 0.0);
        canvas.size(1.0, 1.0);
        canvas.fit();
        context
            .render("rect")
            .translate(x - size / 2.0, y - size / 2.0)
            .scale(size, size)
            .rotate(self.angle)
            .shade(1.0, 0.0, 0.0)
            .commit(canvas)
    }
}

fn main() {
    run_game(Simple::new());
}
