use gamelib::{run_game, Canvas, Game};

struct Simple {
    angle: f32,
    dist: f32,
    size: f32,
}

impl Simple {
    fn new() -> Self {
        Self {
            angle: 0.0,
            dist: 200.0,
            size: 30.0,
        }
    }
}

impl Game for Simple {
    fn tick(&mut self, canvas: &mut Canvas, delta: f32) {
        self.angle += std::f32::consts::PI * delta;

        let x = self.angle.cos() * self.dist;
        let y = self.angle.sin() * self.dist;

        canvas.rect_center(x, y, self.size, self.size);
    }
}

fn main() {
    run_game(Simple::new());
}
