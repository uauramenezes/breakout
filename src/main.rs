use macroquad::prelude::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "Pong".to_owned(),
        window_width: 450,
        window_height: 650,
        fullscreen: false,
        window_resizable: false,
        ..Default::default()
    }
}

struct Player {
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    dir: f32,
    speed: f32,
}

struct Ball {
    x: f32,
    y: f32,
    r: f32,
    dir_x: f32,
    dir_y: f32,
    speed: f32,
}

impl Ball {
    fn move_self(&mut self, play: bool) {
        self.speed = match play {
            true => 150.0,
            false => 0.0,
        };

        if self.x - self.r <= 0.0 || self.x + self.r >= screen_width() {
            self.dir_x = -self.dir_x;
        }
        if self.y - self.r <= 0.0 || self.y + self.r >= screen_height() {
            self.dir_y = -self.dir_y;
        }

        self.y += self.speed * self.dir_y * get_frame_time();
        self.x += self.speed * self.dir_x * get_frame_time();
    }
}

fn get_player_dir() -> f32 {
    if is_key_down(KeyCode::Left) || is_key_down(KeyCode::A) {
        return -1.0;
    }
    if is_key_down(KeyCode::Right) || is_key_down(KeyCode::D) {
        return 1.0;
    }
    return 0.0;
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut player = Player {
        x: screen_width() / 2.0 - 40.0,
        y: screen_height() - 25.0,
        w: 80.0,
        h: 8.0,
        dir: 0.0,
        speed: 350.0,
    };

    let mut ball = Ball {
        x: screen_width() / 2.0,
        y: screen_height() - 25.0 - 8.0,
        r: 8.0,
        dir_x: -1.0,
        dir_y: -1.0,
        speed: 0.0,
    };

    let mut play = false;

    loop {
        clear_background(BLACK);

        draw_rectangle(player.x, player.y, player.w, player.h, WHITE);
        draw_circle(ball.x, ball.y, ball.r, WHITE);

        if is_key_down(KeyCode::Space) && !play {
            play = true;
        }

        ball.move_self(play);

        player.dir = get_player_dir();
        player.x += player.speed * player.dir * get_frame_time();

        next_frame().await
    }
}
