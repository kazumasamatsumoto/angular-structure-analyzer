pub mod string_utils {
    #[allow(dead_code)]
    pub fn kebab_case(s: &str) -> String {
        let mut result = String::new();
        let mut prev_char_is_lowercase = false;
        
        for (i, c) in s.char_indices() {
            if c.is_uppercase() {
                if i > 0 && prev_char_is_lowercase {
                    result.push('-');
                }
                result.push(c.to_ascii_lowercase());
                prev_char_is_lowercase = false;
            } else {
                result.push(c);
                prev_char_is_lowercase = c.is_lowercase();
            }
        }
        
        result
    }
    
    #[allow(dead_code)]
    pub fn pascal_case(s: &str) -> String {
        let mut result = String::new();
        let mut capitalize_next = true;
        
        for c in s.chars() {
            if c == '-' || c == '_' {
                capitalize_next = true;
            } else if capitalize_next {
                result.push(c.to_ascii_uppercase());
                capitalize_next = false;
            } else {
                result.push(c);
            }
        }
        
        result
    }
    
    #[allow(dead_code)]
    pub fn camel_case(s: &str) -> String {
        let pascal = pascal_case(s);
        if pascal.is_empty() {
            return String::new();
        }
        
        let mut result = String::new();
        let first_char = pascal.chars().next().unwrap();
        result.push(first_char.to_ascii_lowercase());
        result.push_str(&pascal[first_char.len_utf8()..]);
        
        result
    }
}

pub mod fs_utils {
    use anyhow::{Context, Result};
    use std::fs;
    use std::path::Path;
    
    #[allow(dead_code)]
    pub fn read_file(path: impl AsRef<Path>) -> Result<String> {
        fs::read_to_string(path.as_ref())
            .context(format!("Failed to read file '{}'", path.as_ref().display()))
    }
    
    #[allow(dead_code)]
    pub fn write_file(path: impl AsRef<Path>, content: &str) -> Result<()> {
        fs::write(path.as_ref(), content)
            .context(format!("Failed to write file '{}'", path.as_ref().display()))
    }
    
    #[allow(dead_code)]
    pub fn ensure_dir_exists(path: impl AsRef<Path>) -> Result<()> {
        if !path.as_ref().exists() {
            fs::create_dir_all(path.as_ref())
                .context(format!("Failed to create directory '{}'", path.as_ref().display()))?;
        }
        Ok(())
    }
    
    #[allow(dead_code)]
    pub fn relative_path(path: impl AsRef<Path>, base: impl AsRef<Path>) -> String {
        let path = path.as_ref();
        let base = base.as_ref();
        
        match path.strip_prefix(base) {
            Ok(rel_path) => rel_path.display().to_string(),
            Err(_) => path.display().to_string(),
        }
    }
}
