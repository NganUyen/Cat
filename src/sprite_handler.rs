use anyhow::{Result, anyhow};
use iced::{
    widget::{Text, Column, Image, Container},
    Element, Length,
};
use log::{info, error};
use rand::seq::SliceRandom;
use rand::Rng;
use std::collections::HashMap;
use std::time::{Duration, Instant};

use crate::comment_generator::Commenter;
use crate::display::Message;

#[derive(Debug, Clone)]
enum Direction {
    Left,
    Right,
}

impl Direction {
    fn to_string(&self) -> String {
        match self {
            Direction::Left => "left".to_string(),
            Direction::Right => "right".to_string(),
        }
    }
    
    fn opposite(&self) -> Self {
        match self {
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

#[derive(Debug, Clone)]
enum AnimationState {
    Idle,
    Transitioning,
}

#[derive(Debug, Clone)]
struct Position {
    x: i32,
}

#[derive(Clone)]
pub struct SpriteController {
    // Sprite và animation
    animation_frames: HashMap<String, Vec<image::DynamicImage>>,
    current_animation: String,
    frame_index: usize,
    max_frame_index: usize,
    
    // Vị trí và di chuyển
    pos: Position,
    direction: Direction,
    animation_state: AnimationState,
    
    // Thời gian và delay
    last_update: Instant,
    idle_delay: u32,
    max_idle_delay: u32,
    
    // Chat và comment
    commenter: Commenter,
    chat_response: Option<String>,
    chat_visible: bool,
    chat_duration: u32,
    chat_max_duration: u32,
    
    // Màn hình và giới hạn
    screen_width: i32,
    
    // Chuyển màn hình
    is_transitioning: bool,
    transition_complete: u32,
    transition_x: i32,
    
    // Biến mất
    is_disappeared: bool,
    disappear_timer: u32,
    disappear_duration: u32,
}

impl SpriteController {
    pub fn new() -> Self {
        info!("Initializing SpriteController");
        
        // Get screen dimensions
        let screen_width = 1920; // Default value, will update later
        
        // Load sprites
        let mut animation_frames = HashMap::new();
        
        // In a real implementation, you would load sprites from a directory
        // Here we just create placeholders
        let idle_right = vec![image::DynamicImage::new_rgba8(100, 100)];
        animation_frames.insert("idle_right".to_string(), idle_right);
        
        let idle_left = vec![image::DynamicImage::new_rgba8(100, 100)];
        animation_frames.insert("idle_left".to_string(), idle_left);
        
        let sitting = vec![image::DynamicImage::new_rgba8(100, 100)];
        animation_frames.insert("sitting".to_string(), sitting);
        
        // Add movement animations
        let move_right = vec![image::DynamicImage::new_rgba8(100, 100)];
        animation_frames.insert("move_right".to_string(), move_right);
        
        let move_left = vec![image::DynamicImage::new_rgba8(100, 100)];
        animation_frames.insert("move_left".to_string(), move_left);
        
        // Initialize commenter
        let commenter = Commenter::new();
        
        let mut controller = Self {
            animation_frames,
            current_animation: "idle_right".to_string(),
            frame_index: 0,
            max_frame_index: 0,
            
            pos: Position { x: 20 },
            direction: Direction::Right,
            animation_state: AnimationState::Idle,
            
            last_update: Instant::now(),
            idle_delay: 0,
            max_idle_delay: 1000,
            
            commenter,
            chat_response: None,
            chat_visible: false,
            chat_duration: 0,
            chat_max_duration: 500,
            
            screen_width,
            
            is_transitioning: false,
            transition_complete: 0,
            transition_x: 0,
            
            is_disappeared: false,
            disappear_timer: 0,
            disappear_duration: 0,
        };
        
        // Generate an initial comment when the application starts
        // We can't call generate_comment directly here because it's not thread-safe
        // So we'll set a flag to generate a comment on the first update
        let mut rng = rand::thread_rng();
        if rng.gen_bool(0.8) { // 80% chance to make an initial comment
            info!("Setting up initial comment");
            let delay = rng.gen_range(50..150);
            controller.idle_delay = controller.max_idle_delay - delay;
        }
        
        controller
    }
    
    // Update animation
    pub fn handle_animation(&mut self) -> Result<()> {
        // Process different sprite states
        match self.animation_state {
            AnimationState::Idle => {
                self.handle_idle()?;
            }
            AnimationState::Transitioning => {
                self.handle_transitioning()?;
            }
        }
        
        // Update chat if visible
        if self.chat_visible {
            self.chat_duration += 1;
            if self.chat_duration >= self.chat_max_duration {
                info!("Chat display timeout, hiding chat");
                self.chat_visible = false;
                self.chat_duration = 0;
                
                // After a chat ends, reduce the time until the next comment
                let mut rng = rand::thread_rng();
                let boost = rng.gen_range(200..500);
                self.idle_delay = self.max_idle_delay - boost;
            }
        }
        
        Ok(())
    }
    
    fn handle_idle(&mut self) -> Result<()> {
        self.idle_delay += 1;
        
        // Make the cat more likely to comment by reducing the delay threshold
        if self.idle_delay >= self.max_idle_delay / 2 && !self.chat_visible {
            // Check if we should generate a comment - increased probability
            let now = Instant::now();
            let elapsed = now.duration_since(self.last_update);
            
            // Generate comment more frequently - every 20 seconds instead of 60
            if elapsed > Duration::from_secs(20) {
                self.last_update = now;
                let mut rng = rand::thread_rng();
                if rng.gen_bool(0.5) { // 50% chance instead of 25%
                    info!("Triggering comment generation due to time elapsed");
                    return self.generate_comment();
                }
            }
        }
        
        if self.idle_delay >= self.max_idle_delay && !self.chat_visible {
            // Move directly instead of changing to Moving state
            self.idle_delay = 0;
            
            // Move based on current direction
            match self.direction {
                Direction::Right => {
                    self.pos.x += 5;
                    // Check if screen transition is needed
                    if self.pos.x > self.screen_width - 70 {
                        self.start_transition(Direction::Right)?;
                    }
                }
                Direction::Left => {
                    self.pos.x -= 5;
                    // Check if screen transition is needed
                    if self.pos.x < -30 {
                        self.start_transition(Direction::Left)?;
                    }
                }
            }
            
            // Small chance to comment after each movement
            let mut rng = rand::thread_rng();
            if rng.gen_bool(0.1) { // 10% chance to comment after movement
                info!("Triggering comment generation after movement");
                self.generate_comment()?;
            }
        }
        
        Ok(())
    }
    
    fn handle_transitioning(&mut self) -> Result<()> {
        // Handle screen transition
        if self.is_disappeared {
            self.disappear_timer += 1;
            if self.disappear_timer >= self.disappear_duration {
                self.is_disappeared = false;
                info!("Cat reappeared");
                
                // Reset position
                match self.direction {
                    Direction::Left => {
                        self.pos.x = self.screen_width - 50;
                        self.direction = Direction::Right;
                    }
                    Direction::Right => {
                        self.pos.x = 0;
                        self.direction = Direction::Left;
                    }
                }
                
                self.is_transitioning = false;
                self.animation_state = AnimationState::Idle;
            }
            return Ok(());
        }
        
        self.transition_complete += 2;
        
        match self.direction {
            Direction::Right => {
                // Move from right to left
                if self.transition_complete <= 50 {
                    let ratio = self.transition_complete as f32 / 50.0;
                    self.transition_x = (self.screen_width - 30) + (ratio * 100.0) as i32;
                } else {
                    let ratio = (self.transition_complete - 50) as f32 / 50.0;
                    self.transition_x = -70 + (ratio * 100.0) as i32;
                }
            }
            Direction::Left => {
                // Move from left to right
                if self.transition_complete <= 50 {
                    let ratio = self.transition_complete as f32 / 50.0;
                    self.transition_x = -70 + (ratio * -30.0) as i32;
                } else {
                    let ratio = (self.transition_complete - 50) as f32 / 50.0;
                    self.transition_x = (self.screen_width + 30) - (ratio * 100.0) as i32;
                }
            }
        }
        
        self.pos.x = self.transition_x;
        
        if self.transition_complete >= 100 {
            self.is_transitioning = false;
            self.direction = self.direction.opposite();
            self.animation_state = AnimationState::Idle;
            self.set_animation(&format!("idle_{}", self.direction.to_string()))?;
        }
        
        Ok(())
    }
    
    fn start_transition(&mut self, from_direction: Direction) -> Result<()> {
        self.is_transitioning = true;
        self.direction = from_direction;
        self.transition_complete = 0;
        self.animation_state = AnimationState::Transitioning;
        
        // Determine if the cat should randomly disappear
        let mut rng = rand::thread_rng();
        if rng.gen_bool(0.3) { // 30% chance
            self.is_disappeared = true;
            self.disappear_timer = 0;
            self.disappear_duration = rng.gen_range(100..300);
            info!("Cat disappeared, will reappear after {} frames", self.disappear_duration);
        }
        
        Ok(())
    }
    
    fn set_animation(&mut self, name: &str) -> Result<()> {
        if !self.animation_frames.contains_key(name) {
            return Err(anyhow!("Animation '{}' not found", name));
        }
        
        self.current_animation = name.to_string();
        self.frame_index = 0;
        self.max_frame_index = self.animation_frames[name].len() - 1;
        
        Ok(())
    }
    
    fn generate_comment(&mut self) -> Result<()> {
        info!("Generating comment");
        
        // Set chat display state
        self.animation_state = AnimationState::Idle;
        self.set_animation("sitting")?;
        
        // Display "Thinking..." in chat
        self.chat_response = Some("Thinking...".to_string());
        self.chat_visible = true;
        
        // Call commenter to get a comment
        match self.commenter.generate_comment() {
            Ok(comment) => {
                info!("Comment generated: {}", comment);
                self.chat_response = Some(comment);
                self.chat_duration = 0;
                
                // TTS will be performed here
                if let Err(e) = self.commenter.speak_comment(&self.chat_response.clone().unwrap_or_default()) {
                    error!("TTS failed: {}", e);
                    // Continue even if TTS fails
                }
            }
            Err(e) => {
                error!("Failed to generate comment: {}", e);
                // Use sample responses
                let sample_responses = [
                    "Meow! What are you doing? That looks interesting!",
                    "Hmm, humans are so strange with the things they look at on screens.",
                    "Hey, I see you're working hard. But don't forget to feed me!",
                ];
                
                let mut rng = rand::thread_rng();
                self.chat_response = Some(sample_responses.choose(&mut rng).unwrap().to_string());
            }
        }
        
        // Make the chat visible for longer time
        self.chat_max_duration = 800; // Increased from 500
        
        Ok(())
    }
    
    // View function cho Iced
    pub fn view(&self) -> Element<Message> {
        // Lấy frame hiện tại từ animation hiện tại
        let current_animation = &self.current_animation;
        if let Some(frames) = self.animation_frames.get(current_animation) {
            let current_frame = &frames[self.frame_index.min(frames.len() - 1)];
            
            // Chuyển đổi DynamicImage thành Handle cho iced
            let image_handle = iced::widget::image::Handle::from_memory(current_frame.to_rgba8().to_vec());
            
            let mut content = Column::new()
                .width(Length::Fill)
                .height(Length::Fill);
            
            // Tính toán vị trí tương đối trong cửa sổ
            // Do mèo chỉ nên hiển thị trong cửa sổ, nên giới hạn pos.x
            let relative_x = self.pos.x.max(0).min(200 - 100); // 200 là width, 100 là width của hình mèo
            
            // Hiển thị sprite mèo với vị trí dựa trên pos.x
            let cat_image = Image::new(image_handle)
                .width(Length::Fixed(100.0))
                .height(Length::Fixed(100.0));
            
            // Tạo container cho hình ảnh mèo với padding trái để tạo vị trí x
            let cat_container = Container::new(cat_image)
                .width(Length::Fill)
                .padding([0, 0, 0, relative_x as u16]);
            
            content = content.push(cat_container);
            
            // Nếu chat hiện tại đang hiển thị, thêm chat bubble
            if self.chat_visible {
                let chat_text = self.chat_response.clone().unwrap_or_default();
                let chat_bubble = Text::new(chat_text)
                    .size(16);
                
                content = content.push(chat_bubble);
            }
            
            return content.into();
        }
        
        // Fallback nếu không có animation
        Column::new()
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
} 