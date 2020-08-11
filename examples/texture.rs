use gamelib::*;

struct Texture {
    model: Option<Model>,
}

impl Texture {
    fn new() -> Self {
        Self { model: None }
    }
}

impl Game for Texture {
    fn init(&mut self, context: &mut Context) {
        self.model = Some(context.load_texture("examples/textures/ch.png"));
    }

    fn render(&mut self, canvas: &mut Canvas) {
        let model = self.model.as_ref().unwrap();

        canvas.clear(0.1, 0.1, 0.1);
        canvas.fit(1.0, 1.0);
        canvas.model(model, (-0.5, -0.5), (1.0, 1.0), (0.0, 0.0, 0.0));
    }
}

fn main() {
    run_game(Texture::new());
}
