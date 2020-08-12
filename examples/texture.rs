use gamelib::*;

struct Texture;

impl Game for Texture {
    fn render(&mut self, canvas: &mut Canvas, context: &mut Context) {
        canvas.clear(0.1, 0.1, 0.1);
        canvas.fit(1.0, 1.0);
        canvas.model(
            context.get_sprite("examples/textures/ch.png"),
            (-0.5, -0.5),
            (1.0, 1.0),
            (0.0, 0.0, 0.0),
        );
    }
}

fn main() {
    run_game(Texture);
}
