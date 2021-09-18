use macroquad::prelude::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "Pong".to_owned(),
        window_width: 480,
        window_height: 640,
        fullscreen: false,
        window_resizable: false,
        ..Default::default()
    }
}

#[derive(Clone, Copy)]
struct Brick {
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    color: Color,
}

impl Brick {
    fn draw(&self) {
        draw_rectangle(self.x, self.y, self.w, self.h, self.color);
    }

    // TODO: Add collision
    fn collision(&self, ball: &Ball) -> bool {
        if (ball.x >= self.x
            && ball.x <= self.x + self.w
            && ball.y + ball.r >= self.y
            && ball.y - ball.r <= self.y + self.h)
            || (((ball.x - ball.r <= self.x + self.w
                && ball.x >= self.x + self.w
                && ball.x_direction > 0.0)
                || (ball.x + ball.r >= self.x
                    && ball.x - ball.r <= self.x
                    && ball.x_direction < 0.0))
                && ball.y - ball.r <= self.y + self.h
                && ball.y + ball.r >= self.y)
        {
            return true;
        }
        false
    }
}

#[derive(Clone)]
struct Wall {}

impl Wall {
    fn new() -> Vec<Brick> {
        let mut wall: Vec<Brick> = Vec::new();

        let color = vec![RED, GREEN, BLUE, YELLOW, VIOLET, ORANGE];
        for i in 0..6 {
            for j in 0..5 {
                let c = i + j;
                let c = match c {
                    6 => 0,
                    7 => 1,
                    8 => 2,
                    9 => 3,
                    _ => i + j,
                };
                let x = (i * 80 + 15) as f32;
                let y = ((j + 1) * 30) as f32;
                wall.push(Brick {
                    x: x,
                    y: y,
                    w: 50.0,
                    h: 15.0,
                    color: color[c],
                });
            }
        }

        wall
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
    fn new() -> Player {
        Player {
            x: screen_width() / 2.0 - 40.0,
            y: screen_height() - 25.0,
            w: 80.0,
            h: 8.0,
            direction: 0.0,
            speed: 350.0,
        }
    }

    fn draw(&self) {
        draw_rectangle(self.x, self.y, self.w, self.h, WHITE);
    }

    fn move_self(&mut self) {
        self.direction = self.get_direction();
        self.x += self.speed * self.direction * get_frame_time();
    }
    fn get_direction(&mut self) -> f32 {
        if is_key_down(KeyCode::Left) || is_key_down(KeyCode::A) {
            return -1.0;
        }
        if is_key_down(KeyCode::Right) || is_key_down(KeyCode::D) {
            return 1.0;
        }
        0.0
    }

    fn border_collision(&mut self) {
        if self.x <= 0.0 {
            self.x = 0.0;
        }
        if self.x + self.w > screen_width() {
            self.x = screen_width() - self.w;
        }
    }

    fn restart(&mut self) {
        self.x = screen_width() / 2.0 - 40.0;
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

fn get_random_number() -> f32 {
    let random_number = rand::gen_range(1, 3);
    let random = match random_number {
        1 => -1.0,
        2 => 1.0,
        _ => 0.0,
    };
    random
}

impl Ball {
    fn new() -> Ball {
        Ball {
            x: screen_width() / 2.0,
            y: screen_height() - 25.0 - 8.5,
            r: 8.0,
            x_direction: get_random_number(),
            y_direction: -1.0,
            speed: 15.0,
        }
    }

    fn draw(&self) {
        draw_circle(self.x, self.y, self.r, WHITE);
    }

    fn move_self(&mut self, player: &Player, brick: &Brick) {
        if (self.x - self.r < 0.0 && self.x_direction < 0.0)
            || (self.x + self.r > screen_width() && self.x_direction > 0.0)
            || (((self.x - self.r <= player.x + player.w && self.x >= player.x + player.w)
                || (self.x + self.r >= player.x && self.x <= player.x))
                && self.y + self.r >= player.y
                && self.y - self.r <= player.y + player.h
                && self.y_direction > 0.0)
            || (((self.x - self.r <= brick.x + brick.w
                && self.x >= brick.x + brick.w
                && self.x_direction < 0.0)
                || (self.x + self.r >= brick.x
                    && self.x - self.r <= brick.x
                    && self.x_direction > 0.0))
                && self.y - self.r <= brick.y + brick.h
                && self.y + self.r >= brick.y)
        {
            self.x_direction = -self.x_direction;
        }
        if (self.y - self.r <= 0.0)
            || (self.y_direction > 0.0
                && self.x >= player.x
                && self.y + self.r >= player.y
                && self.x <= player.x + player.w
                && self.y - self.r <= player.y + player.h)
            || (self.x >= brick.x
                && self.y + self.r >= brick.y
                && self.x <= brick.x + brick.w
                && self.y - self.r <= brick.y + brick.h)
        {
            self.y_direction = -self.y_direction;
        }

        self.y += self.speed * self.y_direction * get_frame_time();
        self.x += self.speed * self.x_direction * get_frame_time();
    }
    fn restart(&mut self) {
        self.x = screen_width() / 2.0;
        self.y = screen_height() - 25.0 - 8.5;
        self.x_direction = get_random_number();
        self.y_direction = -1.0;
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
    let mut wall = Wall::new();
    let mut player = Player::new();

    let mut ball = Ball::new();

    let mut play = false;

    loop {
        clear_background(BLACK);

        ball.draw();
        player.draw();
        player.move_self();
        player.border_collision();

        play = play_game(play);

        for (index, brick) in wall.clone().iter().enumerate() {
            brick.draw();
            if play {
                ball.move_self(&player, &brick);
            }
            let brick_collision = brick.collision(&ball);
            if brick_collision {
                wall.remove(index);
            }
        }

        if ball.y > screen_height() {
            play = false;
            ball.restart();
            player.restart();
            wall = Wall::new();
        }

        if wall.len() == 0 {
            play = false;
        }

        next_frame().await
    }
}
