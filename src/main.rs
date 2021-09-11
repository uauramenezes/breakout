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
    direction: f32,
    speed: f32,
}

impl Player {
    fn get_direction(&mut self) -> f32 {
        if is_key_down(KeyCode::Left) || is_key_down(KeyCode::A) {
            return -1.0;
        }
        if is_key_down(KeyCode::Right) || is_key_down(KeyCode::D) {
            return 1.0;
        }
        0.0
    }

    fn wall_collision(&mut self) {
        if self.x <= 0.0 {
            self.x = 0.0;
        }
        if self.x + self.w > screen_width() {
            self.x = screen_width() - self.w;
        }
    }
}

struct Ball {
    x: f32,
    y: f32,
    r: f32,
    x_direction: f32,
    y_direction: f32,
    speed: f32,
}

impl Ball {
    fn move_self(&mut self, player: &Player) {
        if (self.x - self.r < 0.0 && self.x_direction < 0.0)
            || (self.x + self.r > screen_width() && self.x_direction > 0.0)
            || (((self.x - self.r <= player.x + player.w && self.x >= player.x + player.w)
                || (self.x >= player.x && self.x - self.r < player.x))
                && self.y + self.r >= player.y
                && self.y - self.r <= player.y + player.h
                && self.y_direction > 0.0)
        {
            self.x_direction = -self.x_direction;
        }
        if (self.y - self.r <= 0.0)
            || (self.y_direction > 0.0
                && self.x >= player.x
                && self.y + self.r >= player.y
                && self.x <= player.x + player.w
                && self.y - self.r <= player.y + player.h)
        {
            self.y_direction = -self.y_direction;
        }

        self.y += self.speed * self.y_direction * get_frame_time();
        self.x += self.speed * self.x_direction * get_frame_time();
    }
}

fn play_game(mut play: bool) -> bool {
    if is_key_down(KeyCode::Space) && !play {
        play = true;
    }
    if is_key_down(KeyCode::Escape) && play {
        play = false;
    }

    play
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut player = Player {
        x: screen_width() / 2.0 - 40.0,
        y: screen_height() - 25.0,
        w: 80.0,
        h: 8.0,
        direction: 0.0,
        speed: 350.0,
    };

    let mut ball = Ball {
        x: screen_width() / 2.0,
        y: screen_height() - 25.0 - 8.0,
        r: 8.0,
        x_direction: -1.0,
        y_direction: -1.0,
        speed: 250.0,
    };

    let mut play = false;

    loop {
        clear_background(BLACK);

        draw_rectangle(player.x, player.y, player.w, player.h, WHITE);
        draw_circle(ball.x, ball.y, ball.r, WHITE);

        play = play_game(play);

        if play {
            ball.move_self(&player);
        }

        if ball.y > screen_height() {
            play = false;
            ball.x = screen_width() / 2.0;
            ball.y = screen_height() - 25.0 - 8.0;
            player.x = screen_width() / 2.0 - 40.0;
        }

        player.wall_collision();
        player.direction = player.get_direction();
        player.x += player.speed * player.direction * get_frame_time();

        next_frame().await
    }
}
