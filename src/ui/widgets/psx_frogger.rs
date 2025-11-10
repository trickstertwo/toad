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

// 3D rendering constants for "polygonal origami" style
const USE_3D_RENDERING: bool = true; // Enable PSX-style 3D graphics
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

// Shading colors for 3D effect
const SHADE_DARK: Color = Color::Rgb(20, 20, 20);
const SHADE_MID: Color = Color::Rgb(60, 60, 60);
const SHADE_LIGHT: Color = Color::Rgb(120, 120, 120);

/// 3D Block rendering for "polygonal origami" PSX style
///
/// Uses Unicode block characters to create faceted, low-poly 3D look:
/// - Full blocks (█) for solid surfaces
/// - Shaded blocks (▓ ▒ ░) for depth gradients
/// - Half blocks (▀ ▄) for top/bottom faces
/// - Quarter blocks (▌ ▐) for left/right faces
#[derive(Debug, Clone, Copy)]
struct Block3D {
    /// Top face character
    top: char,
    /// Front face character
    front: char,
    /// Side face character
    side: char,
    /// Shading level (0=darkest, 3=brightest)
    shade: u8,
}

impl Block3D {
    /// Create a 3D block with isometric shading
    fn new(top: char, front: char, side: char, shade: u8) -> Self {
        Self { top, front, side, shade }
    }

    /// Full cube (solid block)
    fn cube() -> Self {
        Self::new('▀', '█', '▌', 2)
    }

    /// Platform/log (elongated horizontal block)
    fn platform() -> Self {
        Self::new('▀', '▓', '▒', 2)
    }

    /// Car/vehicle (low profile block)
    fn vehicle() -> Self {
        Self::new('▄', '█', '▐', 1)
    }

    /// Water surface (animated)
    fn water(frame: usize) -> char {
        match frame % 4 {
            0 => '≈',
            1 => '∼',
            2 => '≈',
            3 => '~',
            _ => '≈',
        }
    }

    /// Get shading color based on shade level and base color
    fn shade_color(&self, base: Color) -> Color {
        match self.shade {
            0 => Self::darken_color(base, 0.3),
            1 => Self::darken_color(base, 0.6),
            2 => base,
            3 => Self::lighten_color(base, 1.3),
            _ => base,
        }
    }

    fn darken_color(color: Color, factor: f32) -> Color {
        match color {
            Color::Rgb(r, g, b) => Color::Rgb(
                (r as f32 * factor) as u8,
                (g as f32 * factor) as u8,
                (b as f32 * factor) as u8,
            ),
            _ => color,
        }
    }

    fn lighten_color(color: Color, factor: f32) -> Color {
        match color {
            Color::Rgb(r, g, b) => Color::Rgb(
                ((r as f32 * factor).min(255.0)) as u8,
                ((g as f32 * factor).min(255.0)) as u8,
                ((b as f32 * factor).min(255.0)) as u8,
            ),
            _ => color,
        }
    }
}

/// 3D Frog rendering with polygonal origami style
#[derive(Debug, Clone)]
struct Frog3D {
    /// Body segment (blocky geometric shapes)
    body: Vec<(char, i8, i8)>, // (char, x_offset, y_offset)
    /// Current hop animation phase (0-1)
    hop_phase: f32,
}

impl Frog3D {
    fn new() -> Self {
        Self {
            // PSX-style blocky frog made of geometric shapes
            body: vec![
                ('●', 0, 0),   // Body center
                ('▲', 0, -1),  // Head
                ('◄', -1, 0),  // Left leg
                ('►', 1, 0),   // Right leg
                ('▼', 0, 1),   // Back legs
            ],
            hop_phase: 0.0,
        }
    }

    fn update_hop(&mut self, hop_height: f32) {
        self.hop_phase = hop_height;
    }

    /// Get frog character based on animation state
    fn get_char(&self, part_idx: usize) -> char {
        if part_idx < self.body.len() {
            // Rotate body parts during hop for 3D effect
            if self.hop_phase > 0.5 {
                match part_idx {
                    0 => '◆', // Rotated body
                    1 => '▲',
                    _ => self.body[part_idx].0,
                }
            } else {
                self.body[part_idx].0
            }
        } else {
            '●'
        }
    }
}

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

    // 3D rendering (polygonal origami style)
    frog_3d: Frog3D,
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
            frog_3d: Frog3D::new(),
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

        // Update 3D frog animation
        self.frog_3d.update_hop(self.player_hop_height);

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
                    // Water with 3D platforms (polygonal origami style)
                    let water_char = Block3D::water(self.animation_frame);
                    let mut row_chars = vec![water_char; GAME_WIDTH];
                    let mut platform_mask = vec![None; GAME_WIDTH];

                    // Draw platforms with 3D effect
                    for obs in &self.obstacles {
                        if obs.row == row as i32 && obs.is_safe {
                            for i in 0..obs.width {
                                let x = (obs.x + i as i32).rem_euclid(GAME_WIDTH as i32) as usize;
                                if x < GAME_WIDTH {
                                    // Create 3D platform block with depth
                                    let block = Block3D::platform();
                                    if i == 0 {
                                        // Front edge (darker)
                                        row_chars[x] = block.front;
                                        platform_mask[x] = Some(1);
                                    } else if i == obs.width - 1 {
                                        // Back edge (lighter)
                                        row_chars[x] = block.side;
                                        platform_mask[x] = Some(3);
                                    } else {
                                        // Top surface
                                        row_chars[x] = block.top;
                                        platform_mask[x] = Some(2);
                                    }
                                }
                            }
                        }
                    }

                    for (x, ch) in row_chars.iter().enumerate() {
                        let (color, bold) = if let Some(shade) = platform_mask[x] {
                            // 3D shaded platform
                            let base_color = PSX_LOG_BROWN;
                            let shaded = Block3D::new(' ', ' ', ' ', shade).shade_color(base_color);
                            (shaded, true)
                        } else {
                            // Animated water
                            (self.theme.primary_color(), false)
                        };
                        let style = if bold {
                            Style::default().fg(color).add_modifier(Modifier::BOLD)
                        } else {
                            Style::default().fg(color)
                        };
                        line_spans.push(Span::styled(ch.to_string(), style));
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
                    // Road with 3D hazards (polygonal origami style)
                    let mut row_chars = vec!['·'; GAME_WIDTH];
                    let mut hazard_mask = vec![None; GAME_WIDTH];

                    // Draw hazards with 3D vehicle blocks
                    for obs in &self.obstacles {
                        if obs.row == row as i32 && !obs.is_safe {
                            for i in 0..obs.width {
                                let x = (obs.x + i as i32).rem_euclid(GAME_WIDTH as i32) as usize;
                                if x < GAME_WIDTH {
                                    // 3D vehicle with depth
                                    let vehicle = Block3D::vehicle();
                                    if i == 0 {
                                        // Front bumper (bright highlight)
                                        row_chars[x] = self.theme.obstacle_char(true);
                                        hazard_mask[x] = Some(3);
                                    } else if i == obs.width - 1 {
                                        // Rear (darker shadow)
                                        row_chars[x] = self.theme.obstacle_char(false);
                                        hazard_mask[x] = Some(1);
                                    } else {
                                        // Body (mid-tone)
                                        row_chars[x] = vehicle.front;
                                        hazard_mask[x] = Some(2);
                                    }
                                }
                            }
                        }
                    }

                    for (x, ch) in row_chars.iter().enumerate() {
                        let (color, bold) = if let Some(shade) = hazard_mask[x] {
                            // 3D shaded vehicle
                            let base = match self.theme {
                                LevelTheme::ToxicWaste => PSX_TOXIC_GREEN,
                                _ => PSX_CAR_RED,
                            };
                            let shaded = Block3D::new(' ', ' ', ' ', shade).shade_color(base);
                            (shaded, true)
                        } else {
                            // Road surface
                            (PSX_ROAD_GRAY, false)
                        };
                        let style = if bold {
                            Style::default().fg(color).add_modifier(Modifier::BOLD)
                        } else {
                            Style::default().fg(color)
                        };
                        line_spans.push(Span::styled(ch.to_string(), style));
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

        // Render player (3D polygonal origami frog)
        if self.player_y >= 0 && self.player_y < GAME_HEIGHT as i32 {
            let base_x = x_offset + self.player_x as u16;
            let base_y = game_y_offset + self.player_y as u16;

            // Apply hop animation offset
            let hop_offset = (self.player_hop_height * -1.0) as i16;

            if USE_3D_RENDERING {
                // Render multi-part 3D frog
                for (idx, &(_, dx, dy)) in self.frog_3d.body.iter().enumerate() {
                    let part_x = (base_x as i32 + dx as i32) as u16;
                    let part_y = ((base_y as i16 + dy as i16 + hop_offset).max(game_y_offset as i16)) as u16;

                    if part_y < inner.y + inner.height && part_x < inner.x + inner.width {
                        let char = self.frog_3d.get_char(idx);
                        // Highlight effect during hop (brighter color)
                        let color = if self.player_hop_height > 0.3 {
                            Block3D::new(' ', ' ', ' ', 3).shade_color(PSX_FROG_GREEN)
                        } else {
                            PSX_FROG_GREEN
                        };
                        buf.set_string(
                            part_x, part_y, &char.to_string(),
                            Style::default()
                                .fg(color)
                                .bg(Color::Black)
                                .add_modifier(Modifier::BOLD),
                        );
                    }
                }
            } else {
                // Fallback 2D rendering
                let final_y = (base_y as i16 + hop_offset).max(game_y_offset as i16) as u16;
                if final_y < inner.y + inner.height && base_x < inner.x + inner.width {
                    let char = if self.player_hop_height > 0.5 { '▲' } else { '●' };
                    buf.set_string(
                        base_x, final_y, &char.to_string(),
                        Style::default()
                            .fg(PSX_FROG_GREEN)
                            .bg(Color::Black)
                            .add_modifier(Modifier::BOLD),
                    );
                }
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
