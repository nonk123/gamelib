use gamelib::*;

struct Simple {
    angle: f32,
}

impl Simple {
    fn new() -> Self {
        Self { angle: 0.0 }
    }
}

impl Game for Simple {
    fn configure(&self, config: &mut GameConfig) {
        config.title = "Simple".into();
    }

    fn update(&mut self, context: &mut Context) {
        self.angle += std::f32::consts::PI * context.delta;
    }

    fn render(&mut self, canvas: &mut Canvas) {
        let dist = 0.5;
        let size = 0.1;

        let x = self.angle.cos() * dist;
        let y = self.angle.sin() * dist;

        canvas.clear(0.0, 0.0, 0.0, 1.0);
        canvas.rect_center((x, y), (size, size), (1.0, 0.0, 0.0));
    }
}

fn main() {
    run_game(Simple::new());
}
