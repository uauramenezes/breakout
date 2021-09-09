use macroquad::prelude::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "Pong".to_owned(),
        window_width: 500,
        window_height: 800,
        fullscreen: false,
        window_resizable: false,
        ..Default::default()
    }
}

// TODO: Change game to Breakout

enum Side {
    Left,
    Right,
}

struct Player {
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    dir: f32,
    speed: f32,
    color: Color,
}

impl Player {
    fn draw(&self) {
        draw_rectangle(self.x, self.y, self.w, self.h, self.color)
    }

    fn player_dir(&mut self) {
        if is_key_down(KeyCode::Up) || is_key_down(KeyCode::W) {
            self.dir = -1.0;
        } else if is_key_down(KeyCode::Down) || is_key_down(KeyCode::S) {
            self.dir = 1.0;
        } else {
            self.dir = 0.0;
        }
    }

    fn ai_dir(&mut self) {
        if self.y <= 0.0 || self.y + self.h >= screen_height() {
            self.dir = -self.dir;
        }
    }

    fn move_self(&mut self, side: Side) {
        match side {
            Side::Left => self.player_dir(),
            Side::Right => self.ai_dir(),
        }
        self.y += self.speed * self.dir;
    }
}

fn new_player(side: Side) -> Player {
    let x = match side {
        Side::Left => 11.0,
        Side::Right => screen_width() - 11.0 - 8.0,
    };
    let speed = match side {
        Side::Left => 5.0,
        Side::Right => 3.0,
    };
    Player {
        x: x,
        y: screen_height() / 2.0 - 80.0 / 2.0,
        w: 8.0,
        h: 80.0,
        dir: 1.0,
        speed: speed,
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

        left_player.move_self(Side::Left);
        right_player.move_self(Side::Right);

        next_frame().await
    }
}
