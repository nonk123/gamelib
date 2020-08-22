use gamelib::*;

struct Texture;

impl Game for Texture {
    fn render(&mut self, canvas: &mut Canvas, context: &mut Context) {
        canvas.clear(0.1, 0.1, 0.1);
        canvas.size(1.0, 1.0);
        canvas.fit();
        context.render("examples/textures/ch.png").commit(canvas);
    }
}

fn main() {
    run_game(Texture);
}
