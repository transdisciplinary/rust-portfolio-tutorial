#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize};
    use serde_json::json;

    #[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
    #[serde(tag = "type", content = "data")]
    pub enum BlockContent {
        Text(String),
        Gallery(Vec<String>), // List of image URLs
        Video(String),        // Embed URL
        Audio(Vec<(String, String)>), // List of (url, title)
        File(Vec<(String, String)>), // List of (url, description)
    }

    fn form_to_block_content(block_type: &str, content: &str) -> BlockContent {
        match block_type {
            "Text" => BlockContent::Text(content.to_string()),
            "Video" => BlockContent::Video(content.to_string()),
            "Gallery" => {
                let items: Vec<String> = serde_json::from_str(content).unwrap_or_default();
                BlockContent::Gallery(items)
            },
            "Audio" => {
                let items: Vec<(String, String)> = serde_json::from_str(content).unwrap_or_default();
                BlockContent::Audio(items)
            },
            "File" => {
                let items: Vec<(String, String)> = serde_json::from_str(content).unwrap_or_default();
                BlockContent::File(items)
            },
            _ => BlockContent::Text(content.to_string()),
        }
    }

    #[test]
    fn test_gallery_deserialization() {
        let content = r#"["https://example.com/image.jpg"]"#;
        let block = form_to_block_content("Gallery", content);
        if let BlockContent::Gallery(items) = block {
            assert_eq!(items.len(), 1);
            assert_eq!(items[0], "https://example.com/image.jpg");
        } else {
            panic!("Expected Gallery");
        }
    }

    #[test]
    fn test_audio_deserialization() {
        // Frontend sends array of arrays: [["url", "title"]]
        let content = r#"[["https://example.com/audio.mp3", "My Song"]]"#;
        let block = form_to_block_content("Audio", content);
        if let BlockContent::Audio(items) = block {
            assert_eq!(items.len(), 1);
            assert_eq!(items[0].0, "https://example.com/audio.mp3");
            assert_eq!(items[0].1, "My Song");
        } else {
            panic!("Expected Audio");
        }
    }

    #[test]
    fn test_empty_content() {
        let content = "";
        let block = form_to_block_content("Gallery", content);
        if let BlockContent::Gallery(items) = block {
            assert_eq!(items.len(), 0);
        } else {
            panic!("Expected Gallery");
        }
    }

    #[test]
    fn test_invalid_json() {
        let content = "invalid json";
        let block = form_to_block_content("Gallery", content);
        if let BlockContent::Gallery(items) = block {
            assert_eq!(items.len(), 0); // Should default to empty
        } else {
            panic!("Expected Gallery");
        }
    }
    
    #[test]
    fn test_text_content() {
        let content = "<p>Hello</p>";
        let block = form_to_block_content("Text", content);
        if let BlockContent::Text(s) = block {
            assert_eq!(s, "<p>Hello</p>");
        } else {
            panic!("Expected Text");
        }
    }
}
