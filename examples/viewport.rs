use gamelib::*;

struct Viewport;

impl Game for Viewport {
    fn render(&mut self, canvas: &mut Canvas, context: &mut Context) {
        canvas.clear(0.1, 0.1, 0.1);

        canvas.fit(2.0, 2.0);

        let mut colors = vec![
            (0.0, 0.0, 1.0),
            (1.0, 1.0, 0.0),
            (1.0, 0.0, 0.0),
            (0.0, 1.0, 0.0),
        ];

        colors.reverse();

        let mut rect = |x: f32, y: f32| {
            context
                .render("rect")
                .translate(x, y)
                .shade_tup(colors.pop().unwrap())
                .commit(canvas);
        };

        rect(-0.5, -0.5);
        rect(0.5, -0.5);
        rect(-0.5, 0.5);
        rect(0.5, 0.5);
    }
}

fn main() {
    run_game(Viewport);
}
