use gamelib::*;

struct Camera {
    seconds: f32,
    camera_index: usize,
}

impl Camera {
    fn new() -> Self {
        Self {
            seconds: 0.0,
            camera_index: 0,
        }
    }
}

impl Game for Camera {
    fn update(&mut self, context: &mut Context) {
        self.seconds += context.delta;

        let interval = 1.5;

        while self.seconds >= interval {
            self.seconds -= interval;
            self.camera_index += 1;

            if self.camera_index == 4 {
                self.camera_index = 0;
            }
        }
    }

    fn render(&mut self, canvas: &mut Canvas, context: &mut Context) {
        let positions = vec![(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)];

        let colors = vec![
            (1.0, 0.0, 0.0),
            (0.0, 1.0, 0.0),
            (0.0, 0.0, 1.0),
            (1.0, 1.0, 0.0),
        ];

        canvas.clear(0.1, 0.1, 0.1);
        canvas.bottom_left();
        canvas.fit();

        let (x, y) = positions[self.camera_index];
        canvas.look_at(x + 1.0, y + 1.0);

        for i in 0..4 {
            context
                .render("rect")
                .translate_tup(positions[i])
                .shade_tup(colors[i])
                .commit(canvas);
        }
    }
}

fn main() {
    run_game(Camera::new());
}
