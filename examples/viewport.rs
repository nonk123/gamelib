use gamelib::*;

struct Viewport;

impl Game for Viewport {
    fn render(&mut self, canvas: &mut Canvas) {
        canvas.clear(0.1, 0.1, 0.1);

        canvas.fit(2.0, 2.0);

        canvas.rect((-1.0, -1.0), (1.0, 1.0), (1.0, 0.0, 0.0));
        canvas.rect((0.0, -1.0), (1.0, 1.0), (0.0, 1.0, 0.0));
        canvas.rect((-1.0, 0.0), (1.0, 1.0), (0.0, 0.0, 1.0));
        canvas.rect((0.0, 0.0), (1.0, 1.0), (1.0, 1.0, 0.0));
    }
}

fn main() {
    run_game(Viewport);
}
