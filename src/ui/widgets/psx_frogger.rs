/// PSX-Style Frogger Game (1997 PlayStation aesthetic)
///
/// Authentic recreation of the PlayStation 1 Frogger with:
/// - "Polygonal origami" blocky graphics
/// - Vivid PSX-era color palette
/// - Multiple themed levels (Lily Islands, Toxic, Desert, Jungle)
/// - PSX gameplay mechanics (collect flies, rescue baby frogs, time limits)
/// - Smooth animations with bounce/elastic easing

use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Widget},
};
use std::time::{Duration, Instant};

// Game constants
const GAME_WIDTH: usize = 60;
const GAME_HEIGHT: usize = 18;
const GOAL_ROW: i32 = 0;
const WATER_START: i32 = 1;
const WATER_END: i32 = 8;
const MEDIAN_ROW: i32 = 9;
const ROAD_START: i32 = 10;
const ROAD_END: i32 = 16;
const START_ROW: i32 = 17;

// PSX color palette (vivid, limited colors)
const PSX_FROG_GREEN: Color = Color::Rgb(0, 255, 0);        // Bright neon green
const PSX_WATER_BLUE: Color = Color::Rgb(0, 100, 255);      // Bright blue
const PSX_LOG_BROWN: Color = Color::Rgb(139, 69, 19);       // Brown
const PSX_ROAD_GRAY: Color = Color::Rgb(80, 80, 80);        // Gray
const PSX_CAR_RED: Color = Color::Rgb(255, 0, 0);           // Bright red
const PSX_LILYPAD_PINK: Color = Color::Rgb(255, 105, 180);  // Hot pink
const PSX_TOXIC_GREEN: Color = Color::Rgb(57, 255, 20);     // Toxic green
const PSX_DESERT_SAND: Color = Color::Rgb(238, 203, 173);   // Sand
const PSX_JUNGLE_GREEN: Color = Color::Rgb(34, 139, 34);    // Forest green

/// Level themes matching PSX Frogger stages
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LevelTheme {
    /// Retro/Classic lily pads (starting levels)
    LilyIslands,
    /// Toxic waste stage with barrels and slippery pipes
    ToxicWaste,
    /// Desert stage with snakes and beetles
    Desert,
    /// Jungle stage with hippos and vines
    Jungle,
}

impl LevelTheme {
    pub fn name(&self) -> &str {
        match self {
            LevelTheme::LilyIslands => "LILY ISLANDS",
            LevelTheme::ToxicWaste => "TOXIC WASTE ZONE",
            LevelTheme::Desert => "SCORCHING DESERT",
            LevelTheme::Jungle => "WILD JUNGLE",
        }
    }

    pub fn primary_color(&self) -> Color {
        match self {
            LevelTheme::LilyIslands => PSX_LILYPAD_PINK,
            LevelTheme::ToxicWaste => PSX_TOXIC_GREEN,
            LevelTheme::Desert => PSX_DESERT_SAND,
            LevelTheme::Jungle => PSX_JUNGLE_GREEN,
        }
    }

    pub fn water_char(&self) -> char {
        match self {
            LevelTheme::LilyIslands => '≈',
            LevelTheme::ToxicWaste => '☢',  // Toxic symbol
            LevelTheme::Desert => '∿',      // Desert sand waves
            LevelTheme::Jungle => '≋',      // Jungle water
        }
    }

    pub fn obstacle_char(&self, is_front: bool) -> char {
        match self {
            LevelTheme::LilyIslands => if is_front { '◄' } else { '►' },
            LevelTheme::ToxicWaste => if is_front { '◀' } else { '▶' },  // Barrels
            LevelTheme::Desert => if is_front { '◂' } else { '▸' },      // Beetles
            LevelTheme::Jungle => if is_front { '◃' } else { '▹' },      // Animals
        }
    }
}

/// Game state
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GameState {
    Playing,
    Won,
    Lost,
    LevelTransition,
}

/// Power-up types (PSX feature)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PowerUp {
    /// Blue fly - extra points
    BlueFly,
    /// Golden fly - extra life
    GoldenFly,
    /// Speed boost
    SpeedBoost,
}

#[derive(Debug, Clone)]
pub struct PowerUpInstance {
    pub x: i32,
    pub y: i32,
    pub kind: PowerUp,
    pub collected: bool,
}

/// Moving obstacle (cars, logs, barrels, etc.)
#[derive(Debug, Clone)]
pub struct Obstacle {
    pub x: i32,
    pub row: i32,
    pub speed: i32,
    pub width: usize,
    pub is_safe: bool, // true for platforms (logs), false for hazards (cars)
}

/// Baby frog to rescue (PSX feature - 5 per level)
#[derive(Debug, Clone)]
pub struct BabyFrog {
    pub x: i32,
    pub rescued: bool,
    pub color: Color, // Green, orange, purple, blue, red
}

/// Main game state
#[derive(Debug, Clone)]
pub struct PSXFroggerGame {
    // Player state
    player_x: i32,
    player_y: i32,
    player_hop_height: f32, // For jump animation (PSX feature)

    // Game state
    lives: i32,
    score: u32,
    state: GameState,
    current_level: usize,
    theme: LevelTheme,

    // Level objects
    obstacles: Vec<Obstacle>,
    power_ups: Vec<PowerUpInstance>,
    baby_frogs: Vec<BabyFrog>,

    // Time tracking
    last_update: Instant,
    update_interval: Duration,
    level_time_left: f32, // PSX has time limits

    // Animation
    animation_frame: usize,
}

impl PSXFroggerGame {
    pub fn new() -> Self {
        let mut game = Self {
            player_x: GAME_WIDTH as i32 / 2,
            player_y: START_ROW,
            player_hop_height: 0.0,
            lives: 3,
            score: 0,
            state: GameState::Playing,
            current_level: 1,
            theme: LevelTheme::LilyIslands,
            obstacles: Vec::new(),
            power_ups: Vec::new(),
            baby_frogs: Vec::new(),
            last_update: Instant::now(),
            update_interval: Duration::from_millis(100),
            level_time_left: 60.0, // 60 seconds per level
            animation_frame: 0,
        };
        game.init_level();
        game
    }

    fn init_level(&mut self) {
        self.obstacles.clear();
        self.power_ups.clear();
        self.baby_frogs.clear();
        self.player_x = GAME_WIDTH as i32 / 2;
        self.player_y = START_ROW;
        self.level_time_left = 60.0;

        // Set theme based on level
        self.theme = match self.current_level {
            1..=2 => LevelTheme::LilyIslands,
            3..=4 => LevelTheme::ToxicWaste,
            5..=6 => LevelTheme::Desert,
            _ => LevelTheme::Jungle,
        };

        // Create 5 baby frogs to rescue (PSX feature)
        let colors = [
            Color::Rgb(0, 255, 0),   // Green
            Color::Rgb(255, 165, 0), // Orange
            Color::Rgb(160, 32, 240),// Purple
            Color::Rgb(0, 0, 255),   // Blue
            Color::Rgb(255, 0, 0),   // Red
        ];

        for (i, &color) in colors.iter().enumerate() {
            self.baby_frogs.push(BabyFrog {
                x: ((i + 1) * 12) as i32,
                rescued: false,
                color,
            });
        }

        // Create obstacles based on difficulty
        let speed_mult = 1 + (self.current_level / 2);

        // Water obstacles (logs/platforms)
        for row in WATER_START..=WATER_END {
            let count = 2 + (row % 3);
            let speed = if row % 2 == 0 { 1 * speed_mult as i32 } else { -1 * speed_mult as i32 };
            let width = 4 + (row % 3);

            for i in 0..count {
                self.obstacles.push(Obstacle {
                    x: (i * 15 + row * 3) as i32,
                    row,
                    speed,
                    width: width as usize,
                    is_safe: true,
                });
            }
        }

        // Road obstacles (cars/hazards)
        for row in ROAD_START..=ROAD_END {
            let count = 3 + (row % 2);
            let speed = if row % 2 == 0 { 2 * speed_mult as i32 } else { -2 * speed_mult as i32 };
            let width = 3 + (row % 2);

            for i in 0..count {
                self.obstacles.push(Obstacle {
                    x: (i * 12 + row * 2) as i32,
                    row,
                    speed,
                    width: width as usize,
                    is_safe: false,
                });
            }
        }

        // Add power-ups (flies)
        for i in 0..3 {
            self.power_ups.push(PowerUpInstance {
                x: 10 + i * 20,
                y: 5 + i * 3,
                kind: if i == 1 { PowerUp::GoldenFly } else { PowerUp::BlueFly },
                collected: false,
            });
        }
    }

    pub fn update(&mut self) {
        if self.state != GameState::Playing {
            return;
        }

        let now = Instant::now();
        let delta = now.duration_since(self.last_update).as_secs_f32();

        if delta < self.update_interval.as_secs_f32() {
            return;
        }

        self.last_update = now;
        self.animation_frame = (self.animation_frame + 1) % 60;

        // Update time limit (PSX feature)
        self.level_time_left -= delta;
        if self.level_time_left <= 0.0 {
            self.die();
            return;
        }

        // Update hop animation
        if self.player_hop_height > 0.0 {
            self.player_hop_height -= 0.3;
            if self.player_hop_height < 0.0 {
                self.player_hop_height = 0.0;
            }
        }

        // Update obstacles
        for obstacle in &mut self.obstacles {
            obstacle.x += obstacle.speed;

            // Wrap around
            if obstacle.speed > 0 && obstacle.x > GAME_WIDTH as i32 {
                obstacle.x = -(obstacle.width as i32);
            } else if obstacle.speed < 0 && obstacle.x + (obstacle.width as i32) < 0 {
                obstacle.x = GAME_WIDTH as i32;
            }
        }

        // Check if player on safe platform in water
        if self.player_y >= WATER_START && self.player_y <= WATER_END {
            if let Some(platform) = self.find_obstacle_at(self.player_x, self.player_y, true) {
                self.player_x += platform.speed;

                // Check if moved off screen
                if self.player_x < 0 || self.player_x >= GAME_WIDTH as i32 {
                    self.die();
                    return;
                }
            } else {
                // Fell in water!
                self.die();
                return;
            }
        }

        // Check collision with hazards
        if self.player_y >= ROAD_START && self.player_y <= ROAD_END {
            if self.find_obstacle_at(self.player_x, self.player_y, false).is_some() {
                self.die();
                return;
            }
        }

        // Check power-up collection
        for powerup in &mut self.power_ups {
            if !powerup.collected && (powerup.x - self.player_x).abs() <= 1
                && powerup.y == self.player_y {
                powerup.collected = true;
                match powerup.kind {
                    PowerUp::BlueFly => self.score += 50,
                    PowerUp::GoldenFly => {
                        self.lives += 1;
                        self.score += 100;
                    }
                    PowerUp::SpeedBoost => {
                        // Temporary speed boost
                        self.score += 25;
                    }
                }
            }
        }

        // Check baby frog rescue
        if self.player_y == GOAL_ROW {
            for frog in &mut self.baby_frogs {
                if !frog.rescued && (frog.x - self.player_x).abs() <= 2 {
                    frog.rescued = true;
                    self.score += 200;

                    // Check if all rescued
                    if self.baby_frogs.iter().all(|f| f.rescued) {
                        self.win_level();
                    }

                    // Return player to start
                    self.player_x = GAME_WIDTH as i32 / 2;
                    self.player_y = START_ROW;
                    break;
                }
            }
        }
    }

    pub fn move_player(&mut self, dx: i32, dy: i32) {
        if self.state != GameState::Playing {
            return;
        }

        let new_x = self.player_x + dx;
        let new_y = self.player_y + dy;

        // Check bounds
        if new_x < 0 || new_x >= GAME_WIDTH as i32 || new_y < 0 || new_y >= GAME_HEIGHT as i32 {
            return;
        }

        self.player_x = new_x;
        self.player_y = new_y;

        // Trigger hop animation (PSX feature)
        if dy != 0 {
            self.player_hop_height = 1.0;
        }

        // Award points for forward progress
        if dy < 0 {
            self.score += 10;
        }
    }

    fn find_obstacle_at(&self, x: i32, y: i32, is_safe: bool) -> Option<&Obstacle> {
        self.obstacles.iter().find(|obs| {
            obs.row == y
                && obs.is_safe == is_safe
                && x >= obs.x
                && x < obs.x + obs.width as i32
        })
    }

    fn die(&mut self) {
        self.lives -= 1;
        if self.lives <= 0 {
            self.state = GameState::Lost;
        } else {
            self.player_x = GAME_WIDTH as i32 / 2;
            self.player_y = START_ROW;
            self.player_hop_height = 0.0;
        }
    }

    fn win_level(&mut self) {
        self.current_level += 1;
        if self.current_level > 8 {
            self.state = GameState::Won;
        } else {
            self.state = GameState::LevelTransition;
            // Will init next level on next update
        }
    }

    pub fn reset(&mut self) {
        self.lives = 3;
        self.score = 0;
        self.current_level = 1;
        self.state = GameState::Playing;
        self.player_hop_height = 0.0;
        self.animation_frame = 0;
        self.init_level();
    }

    pub fn advance_to_next_level(&mut self) {
        if self.state == GameState::LevelTransition {
            self.state = GameState::Playing;
            self.init_level();
        }
    }

    // Getters
    pub fn lives(&self) -> i32 { self.lives }
    pub fn score(&self) -> u32 { self.score }
    pub fn state(&self) -> &GameState { &self.state }
    pub fn current_level(&self) -> usize { self.current_level }
    pub fn time_left(&self) -> f32 { self.level_time_left }
    pub fn theme(&self) -> &LevelTheme { &self.theme }
}

impl Default for PSXFroggerGame {
    fn default() -> Self {
        Self::new()
    }
}

impl Widget for &PSXFroggerGame {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // PSX-style blocky border
        let block = Block::default()
            .borders(Borders::ALL)
            .title(format!(" ▓▒░ {} - LEVEL {} ░▒▓ ", self.theme.name(), self.current_level))
            .border_style(Style::default().fg(self.theme.primary_color()).add_modifier(Modifier::BOLD));

        let inner = block.inner(area);
        block.render(area, buf);

        // Render HUD
        let hud_y = inner.y;
        let hud_line = format!(
            "LIVES: {:02} │ SCORE: {:06} │ TIME: {:.1}s │ FROGS: {}/5",
            self.lives,
            self.score,
            self.level_time_left,
            self.baby_frogs.iter().filter(|f| f.rescued).count()
        );

        buf.set_line(
            inner.x + (inner.width.saturating_sub(hud_line.len() as u16)) / 2,
            hud_y,
            &Line::from(Span::styled(
                hud_line,
                Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD),
            )),
            inner.width,
        );

        let game_y_offset = hud_y + 2;

        // Handle game state messages
        match self.state {
            GameState::Won => {
                let msg = "★★★ YOU CONQUERED ALL LEVELS! ★★★";
                let y = game_y_offset + (GAME_HEIGHT / 2) as u16;
                buf.set_line(
                    inner.x + (inner.width.saturating_sub(msg.len() as u16)) / 2,
                    y,
                    &Line::from(Span::styled(
                        msg,
                        Style::default().fg(PSX_FROG_GREEN).add_modifier(Modifier::BOLD),
                    )),
                    inner.width,
                );
                return;
            }
            GameState::Lost => {
                let msg = "╳╳╳ GAME OVER ╳╳╳";
                let y = game_y_offset + (GAME_HEIGHT / 2) as u16;
                buf.set_line(
                    inner.x + (inner.width.saturating_sub(msg.len() as u16)) / 2,
                    y,
                    &Line::from(Span::styled(
                        msg,
                        Style::default().fg(PSX_CAR_RED).add_modifier(Modifier::BOLD),
                    )),
                    inner.width,
                );
                return;
            }
            GameState::LevelTransition => {
                let msg = format!("▓▒░ LEVEL {} COMPLETE! ░▒▓", self.current_level - 1);
                let y = game_y_offset + (GAME_HEIGHT / 2) as u16;
                buf.set_line(
                    inner.x + (inner.width.saturating_sub(msg.len() as u16)) / 2,
                    y,
                    &Line::from(Span::styled(
                        msg,
                        Style::default().fg(PSX_FROG_GREEN).add_modifier(Modifier::BOLD),
                    )),
                    inner.width,
                );
                return;
            }
            GameState::Playing => {}
        }

        // Calculate centering
        let x_offset = inner.x + (inner.width.saturating_sub(GAME_WIDTH as u16)) / 2;

        // Render game field (PSX blocky polygon style)
        for row in 0..GAME_HEIGHT {
            let y = game_y_offset + row as u16;
            if y >= inner.y + inner.height {
                break;
            }

            let mut line_spans = Vec::new();

            match row as i32 {
                GOAL_ROW => {
                    // Goal row with baby frogs
                    for x in 0..GAME_WIDTH {
                        let mut drawn = false;

                        // Draw baby frogs
                        for frog in &self.baby_frogs {
                            if !frog.rescued && (x as i32 - frog.x).abs() <= 1 {
                                let char = if x as i32 == frog.x { '◉' } else { '○' };
                                line_spans.push(Span::styled(
                                    char.to_string(),
                                    Style::default().fg(frog.color).add_modifier(Modifier::BOLD),
                                ));
                                drawn = true;
                                break;
                            } else if frog.rescued && (x as i32 - frog.x).abs() == 0 {
                                line_spans.push(Span::styled("✓", Style::default().fg(Color::Rgb(255, 215, 0))));
                                drawn = true;
                                break;
                            }
                        }

                        if !drawn {
                            let water_char = self.theme.water_char();
                            line_spans.push(Span::styled(
                                water_char.to_string(),
                                Style::default().fg(self.theme.primary_color()),
                            ));
                        }
                    }
                }
                WATER_START..=WATER_END => {
                    // Water with platforms (PSX blocky style)
                    let mut row_chars = vec![self.theme.water_char(); GAME_WIDTH];
                    let mut platform_mask = vec![false; GAME_WIDTH];

                    // Draw platforms
                    for obs in &self.obstacles {
                        if obs.row == row as i32 && obs.is_safe {
                            for i in 0..obs.width {
                                let x = (obs.x + i as i32).rem_euclid(GAME_WIDTH as i32) as usize;
                                if x < GAME_WIDTH {
                                    // PSX blocky platform
                                    row_chars[x] = if i == 0 || i == obs.width - 1 { '▓' } else { '▒' };
                                    platform_mask[x] = true;
                                }
                            }
                        }
                    }

                    for (x, ch) in row_chars.iter().enumerate() {
                        let color = if platform_mask[x] {
                            PSX_LOG_BROWN
                        } else {
                            self.theme.primary_color()
                        };
                        line_spans.push(Span::styled(
                            ch.to_string(),
                            Style::default().fg(color).add_modifier(Modifier::BOLD),
                        ));
                    }
                }
                MEDIAN_ROW => {
                    // Safe median (PSX blocky grass)
                    for x in 0..GAME_WIDTH {
                        let char = if x % 3 == 0 { '▓' } else if x % 3 == 1 { '▒' } else { '░' };
                        line_spans.push(Span::styled(
                            char.to_string(),
                            Style::default().fg(PSX_JUNGLE_GREEN).add_modifier(Modifier::BOLD),
                        ));
                    }
                }
                ROAD_START..=ROAD_END => {
                    // Road with hazards (PSX blocky cars)
                    let mut row_chars = vec!['·'; GAME_WIDTH];
                    let mut hazard_mask = vec![false; GAME_WIDTH];

                    // Draw hazards
                    for obs in &self.obstacles {
                        if obs.row == row as i32 && !obs.is_safe {
                            for i in 0..obs.width {
                                let x = (obs.x + i as i32).rem_euclid(GAME_WIDTH as i32) as usize;
                                if x < GAME_WIDTH {
                                    // PSX blocky hazard
                                    row_chars[x] = if i == 0 {
                                        self.theme.obstacle_char(true)
                                    } else if i == obs.width - 1 {
                                        self.theme.obstacle_char(false)
                                    } else {
                                        '▓'
                                    };
                                    hazard_mask[x] = true;
                                }
                            }
                        }
                    }

                    for (x, ch) in row_chars.iter().enumerate() {
                        let color = if hazard_mask[x] {
                            match self.theme {
                                LevelTheme::ToxicWaste => PSX_TOXIC_GREEN,
                                _ => PSX_CAR_RED,
                            }
                        } else {
                            PSX_ROAD_GRAY
                        };
                        line_spans.push(Span::styled(
                            ch.to_string(),
                            Style::default().fg(color).add_modifier(Modifier::BOLD),
                        ));
                    }
                }
                START_ROW => {
                    // Starting zone (PSX blocky)
                    for x in 0..GAME_WIDTH {
                        let char = if x % 2 == 0 { '▓' } else { '▒' };
                        line_spans.push(Span::styled(
                            char.to_string(),
                            Style::default().fg(PSX_JUNGLE_GREEN).add_modifier(Modifier::BOLD),
                        ));
                    }
                }
                _ => {
                    line_spans.push(Span::raw(" ".repeat(GAME_WIDTH)));
                }
            }

            buf.set_line(x_offset, y, &Line::from(line_spans), GAME_WIDTH as u16);
        }

        // Render power-ups (flies)
        for powerup in &self.power_ups {
            if !powerup.collected {
                let px = x_offset + powerup.x as u16;
                let py = game_y_offset + powerup.y as u16;
                if py < inner.y + inner.height && px < inner.x + inner.width {
                    let (char, color) = match powerup.kind {
                        PowerUp::BlueFly => ('✦', Color::Cyan),
                        PowerUp::GoldenFly => ('★', Color::Rgb(255, 215, 0)), // Gold color
                        PowerUp::SpeedBoost => ('◆', Color::Magenta),
                    };
                    let char_str = char.to_string();
                    buf.set_string(
                        px, py, &char_str,
                        Style::default().fg(color).add_modifier(Modifier::BOLD),
                    );
                }
            }
        }

        // Render player (PSX frog with hop animation)
        if self.player_y >= 0 && self.player_y < GAME_HEIGHT as i32 {
            let player_x = x_offset + self.player_x as u16;
            let player_y = game_y_offset + self.player_y as u16;

            // Apply hop animation offset
            let hop_offset = (self.player_hop_height * -1.0) as i16;
            let final_y = (player_y as i16 + hop_offset).max(game_y_offset as i16) as u16;

            if final_y < inner.y + inner.height && player_x < inner.x + inner.width {
                let char = if self.player_hop_height > 0.5 { '▲' } else { '●' };
                buf.set_string(
                    player_x, final_y, &char.to_string(),
                    Style::default()
                        .fg(PSX_FROG_GREEN)
                        .bg(Color::Black)
                        .add_modifier(Modifier::BOLD),
                );
            }
        }

        // Instructions
        let instructions = "←↑↓→: Move | R: Restart | ESC: Quit | Rescue all 5 baby frogs!";
        let inst_y = inner.y + inner.height.saturating_sub(1);
        buf.set_line(
            inner.x + (inner.width.saturating_sub(instructions.len() as u16)) / 2,
            inst_y,
            &Line::from(Span::styled(instructions, Style::default().fg(Color::Gray))),
            inner.width,
        );
    }
}
