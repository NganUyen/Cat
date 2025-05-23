use anyhow::{Result, anyhow};
use log::{info, error};
use rand::seq::SliceRandom;
use screenshots::Screen;
use tts::Tts;
use reqwest;
use std::time::Duration;
use serde_json::{json, Value};
use base64::{Engine as _, engine::general_purpose};
use image;
use image::codecs::png::PngEncoder;
use image::ImageEncoder;

#[derive(Clone)]
pub struct Commenter {
    api_key: String,
    latest_response: Option<String>,
    sample_responses: Vec<String>,
    tts: Option<Tts>,
}

impl Commenter {
    pub fn new() -> Self {
        // Initialize Text-to-Speech
        let tts = match Tts::default() {
            Ok(mut tts) => {
                // Liệt kê các voice có sẵn
                if let Ok(voices) = tts.voices() {
                    for voice in &voices {
                        println!("Voice: {} ({})", voice.name(), voice.language());
                    }
                    // Chọn voice tiếng Anh đầu tiên tìm thấy
                    if let Some(voice) = voices.iter().find(|v| v.language().starts_with("en")) {
                        if let Err(e) = tts.set_voice(voice) {
                            error!("Failed to set English voice: {}", e);
                        }
                    }
                }
                Some(tts)
            }
            Err(e) => {
                error!("Failed to initialize TTS: {}", e);
                None
            }
        };
        
        // Sample responses in English
        let sample_responses = vec![
            "Meow! What are you doing? That looks interesting!".to_string(),
            "Hmm, humans are so strange with the things they look at on screens.".to_string(),
            "Hey, I see you're working hard. But don't forget to feed me!".to_string(),
            "Oh, that's interesting! But not as interesting as a ball of yarn.".to_string(),
            "I see you're using your computer. I'd like to walk on your keyboard too!".to_string(),
        ];
        
        Self {
            // Update with your actual API key
            api_key: "AIzaSyDQBl4RaXZdIyAyTcdazBp2WZ0RCUK0sMY".to_string(),
            latest_response: None,
            sample_responses,
            tts,
        }
    }
    
    fn take_screenshot(&self) -> Result<Vec<u8>> {
        info!("Taking screenshot");
        
        let screens = Screen::all()?;
        if screens.is_empty() {
            return Err(anyhow!("No screens found"));
        }
        
        let screen = &screens[0];
        let image = screen.capture()?;
        
        // Convert to PNG format
        let mut png_data = Vec::new();
        let encoder = PngEncoder::new(std::io::Cursor::new(&mut png_data));
        encoder.write_image(
            image.as_raw(),
            image.width() as u32,
            image.height() as u32,
            image::ColorType::Rgba8
        )?;
        
        Ok(png_data)
    }
    
    // Encode the screenshot as base64
    fn encode_screenshot_base64(&self, screenshot: &[u8]) -> String {
        general_purpose::STANDARD.encode(screenshot)
    }
    
    pub fn generate_comment(&mut self) -> Result<String> {
        info!("Generating comment");
        
        let prompt = "Context: I'm sharing a screenshot of my Windows desktop. Role: You are Whiskers, a mischievous, over-dramatic house cat who's secretly judging humans. Task: Look at the screenshot and riff—make a witty, cat-centric one-liner or mini-rant about what you \"see me doing.\" Tone: Snarky, playful, a little entitled (remember, cats think they own everything). Extra flavor: Throw in at least one kitty idiom (\"nap corner,\" \"laser-pointer envy,\" \"treat negotiator,\" etc.) and sprinkle in a mild existential cat crisis (\"when will the humans learn...\"). Make sure the comment is in English and 2 or 3 sentences long.";
        
        // Use Gemini API or sample comments
        match self.api_request(prompt) {
            Ok(response) => {
                self.latest_response = Some(response.clone());
                info!("Comment generated: {}", response);
                Ok(response)
            }
            Err(e) => {
                error!("Error generating comment: {}", e);
                let mut rng = rand::thread_rng();
                let fallback = self.sample_responses.choose(&mut rng)
                    .unwrap_or(&"Meow!".to_string())
                    .clone();
                    
                self.latest_response = Some(fallback.clone());
                info!("Using fallback comment: {}", fallback);
                Ok(fallback)
            }
        }
    }
    
    fn api_request(&self, prompt: &str) -> Result<String> {
        // Take a screenshot
        let screenshot = self.take_screenshot()?;
        info!("Captured screenshot size: {} bytes", screenshot.len());
        
        // Encode screenshot as base64
        let screenshot_base64 = self.encode_screenshot_base64(&screenshot);
        info!("Encoded screenshot as base64");
        
        // Create a blocking client with timeout
        let client = reqwest::blocking::Client::builder()
            .timeout(Duration::from_secs(15))
            .build()?;
        
        // Prepare the API request URL
        let url = format!(
            "https://generativelanguage.googleapis.com/v1beta/models/gemini-1.5-flash:generateContent?key={}",
            self.api_key
        );
        
        // Prepare the request body with the image
        let request_body = json!({
            "contents": [
                {
                    "role": "user",
                    "parts": [
                        {
                            "text": prompt
                        },
                        {
                            "inline_data": {
                                "mime_type": "image/png",
                                "data": screenshot_base64
                            }
                        }
                    ]
                }
            ],
            "generationConfig": {
                "temperature": 0.7,
                "maxOutputTokens": 100,
            }
        });
        
        info!("Sending API request to Gemini Vision API");
        
        // Make the actual API call
        let response = client.post(url)
            .json(&request_body)
            .send();
            
        match response {
            Ok(res) => {
                // Store status code before consuming the response
                let status = res.status();
                
                if status.is_success() {
                    // Parse the response
                    let json: Value = res.json()?;
                    
                    // Extract the generated text
                    if let Some(candidates) = json.get("candidates") {
                        if let Some(first_candidate) = candidates.as_array().and_then(|arr| arr.first()) {
                            if let Some(content) = first_candidate.get("content") {
                                if let Some(parts) = content.get("parts") {
                                    if let Some(first_part) = parts.as_array().and_then(|arr| arr.first()) {
                                        if let Some(text) = first_part.get("text") {
                                            if let Some(text_str) = text.as_str() {
                                                info!("Successfully received response from Gemini Vision API");
                                                return Ok(text_str.to_string());
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    
                    // Debug output to see the actual JSON response
                    let json_str = serde_json::to_string_pretty(&json)?;
                    info!("Received JSON: {}", json_str);
                    
                    error!("Failed to parse API response");
                    Err(anyhow!("Failed to parse API response"))
                } else {
                    // Debug error response - fixed the move issue
                    let error_body = res.text()?;
                    error!("API request failed with status: {}, body: {}", status, error_body);
                    Err(anyhow!("API request failed: {}", status))
                }
            }
            Err(e) => {
                error!("API request error: {}", e);
                Err(anyhow!("API request error: {}", e))
            }
        }
    }
    
    pub fn speak_comment(&self, text: &str) -> Result<()> {
        info!("Speaking comment: {}", text);
        
        if let Some(tts) = &self.tts {
            let mut tts_clone = tts.clone();
            tts_clone.speak(text, false)?;
            Ok(())
        } else {
            Err(anyhow!("TTS not initialized"))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_commenter() {
        let mut commenter = Commenter::new();
        let comment = commenter.generate_comment().unwrap();
        println!("Generated comment: {}", comment);
    }
} 