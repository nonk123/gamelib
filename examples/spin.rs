use gamelib::*;

struct Spin(f32);

impl Game for Spin {
    fn update(&mut self, context: &mut Context) {
        self.0 += context.delta * std::f32::consts::PI;
    }

    fn render(&mut self, canvas: &mut Canvas, context: &mut Context) {
        canvas.clear(0.0, 0.0, 0.0);
        canvas.fit(1.0, 1.0);
        context
            .render("rect")
            .rotate(self.0)
            .scale(0.5, 0.5)
            .shade(0.0, 1.0, 0.5)
            .commit(canvas);
    }
}

fn main() {
    run_game(Spin(0.0));
}
