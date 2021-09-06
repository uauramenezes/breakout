use macroquad::prelude::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "Pong".to_owned(),
        fullscreen: false,
        window_height: 500,
        window_resizable: false,
        ..Default::default()
    }
}

enum Side {
    Left,
    Right,
}

struct Player {
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    color: Color,
}

impl Player {
    fn draw(&self) {
        draw_rectangle(self.x, self.y, self.w, self.h, self.color)
    }

    fn moves(&mut self, up: KeyCode, down: KeyCode) {
        if is_key_down(up) {
            self.y -= 5.0
        }
        if is_key_down(down) {
            self.y += 5.0
        }
    }
}

fn new_player(side: Side) -> Player {
    let x = match side {
        Side::Left => 11.0,
        Side::Right => screen_width() - 11.0 - 8.0,
    };
    Player {
        x: x,
        y: screen_height() / 2.0 - 80.0 / 2.0,
        w: 8.0,
        h: 80.0,
        color: WHITE,
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut left_player = new_player(Side::Left);
    let mut right_player = new_player(Side::Right);

    loop {
        clear_background(BLACK);

        left_player.draw();
        right_player.draw();

        left_player.moves(KeyCode::W, KeyCode::S);
        right_player.moves(KeyCode::Up, KeyCode::Down);

        next_frame().await
    }
}
