use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use tokenizers::tokenizer::Tokenizer;
use tokenizers::FromPretrainedParameters;
use std::ptr;
use std::path::Path;
use std::fs::{OpenOptions};
use std::io::Write as IoWrite;

// Helper function to create a debug log and return it
fn log_debug(message: &str) -> String {
    // Append to debug.log file for better visibility
    if let Ok(mut file) = OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open("/tmp/tokenizer_debug.log") 
    {
        let _ = writeln!(file, "{}", message);
    }
    eprintln!("{}", message);
    message.to_string()
}

// Helper function to load a tokenizer from a model name or file path
fn load_tokenizer(model_name: &str, token: Option<&str>) -> Result<(Tokenizer, Vec<String>), (tokenizers::Error, Vec<String>)> {
    let mut debug_logs = Vec::new();
    
    debug_logs.push(log_debug(&format!("==== load_tokenizer called with: {} ====", model_name)));
    if token.is_some() {
        debug_logs.push(log_debug("Using authentication token for Hugging Face Hub"));
    }
    
    // First check if it's a file path
    if Path::new(model_name).exists() {
        let msg = format!("Loading tokenizer from file: {}", model_name);
        debug_logs.push(log_debug(&msg));
        match Tokenizer::from_file(model_name) {
            Ok(tokenizer) => {
                debug_logs.push(log_debug("Successfully loaded tokenizer from file"));
                Ok((tokenizer, debug_logs))
            },
            Err(e) => {
                let msg = format!("Error loading from file: {:?}", e);
                debug_logs.push(log_debug(&msg));
                Err((e, debug_logs))
            }
        }
    } else {
        let msg = format!("Trying to load tokenizer from HuggingFace Hub: {}", model_name);
        debug_logs.push(log_debug(&msg));
        
        // Otherwise try to load it as a pretrained model from HuggingFace Hub
        // If we have a token, use it for authentication
        let params = match token {
            Some(token) => {
                debug_logs.push(log_debug("Creating parameters with token"));
                Some(FromPretrainedParameters {
                    token: Some(token.to_string()),
                    ..Default::default()
                })
            },
            None => None
        };
        
        match Tokenizer::from_pretrained(model_name, params) {
            Ok(tokenizer) => {
                debug_logs.push(log_debug("Successfully loaded from HuggingFace Hub"));
                Ok((tokenizer, debug_logs))
            },
            Err(e) => {
                let msg = format!("Failed to load from pretrained '{}': {:?}", model_name, e);
                debug_logs.push(log_debug(&msg));
                
                // Try to load from cache directly (works with previously downloaded models)
                let cache_path = format!("/root/.cache/huggingface/hub/models--{}/snapshots", 
                                         model_name.replace('/', "--"));
                let msg = format!("Trying to find in cache: {}", cache_path);
                debug_logs.push(log_debug(&msg));
                
                if Path::new(&cache_path).exists() {
                    debug_logs.push(log_debug("Cache directory exists, looking for tokenizer.json"));
                    
                    // List contents of the directory
                    if let Ok(entries) = std::fs::read_dir(&cache_path) {
                        for entry in entries {
                            if let Ok(entry) = entry {
                                let msg = format!("Found directory: {:?}", entry.path());
                                debug_logs.push(log_debug(&msg));
                                let tokenizer_path = entry.path().join("tokenizer.json");
                                if tokenizer_path.exists() {
                                    let msg = format!("Found tokenizer.json at {:?}", tokenizer_path);
                                    debug_logs.push(log_debug(&msg));
                                    match Tokenizer::from_file(&tokenizer_path) {
                                        Ok(tokenizer) => {
                                            debug_logs.push(log_debug("Successfully loaded from cache"));
                                            return Ok((tokenizer, debug_logs));
                                        },
                                        Err(e) => {
                                            let msg = format!("Error loading from cache file: {:?}", e);
                                            debug_logs.push(log_debug(&msg));
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                
                Err((e, debug_logs))
            }
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn tokenize_text(text: *const c_char, model_name: *const c_char) -> *mut c_char {
    log_debug("tokenize_text called");
    
    // Convert C strings to Rust strings
    let c_text = unsafe {
        if text.is_null() {
            log_debug("Text pointer is null");
            return ptr::null_mut();
        }
        CStr::from_ptr(text)
    };
    
    let c_model = unsafe {
        if model_name.is_null() {
            log_debug("Model name pointer is null");
            return ptr::null_mut();
        }
        CStr::from_ptr(model_name)
    };
    
    let text_str = match c_text.to_str() {
        Ok(s) => s,
        Err(_) => {
            log_debug("Invalid UTF-8 in text");
            return ptr::null_mut();
        }
    };
    
    let model_str = match c_model.to_str() {
        Ok(s) => s,
        Err(_) => {
            log_debug("Invalid UTF-8 in model name");
            return ptr::null_mut();
        }
    };
    
    log_debug(&format!("Processing: text='{}', model='{}'", text_str, model_str));
    
    // Try to get HF_TOKEN environment variable
    let hf_token = std::env::var("HF_TOKEN").ok();
    
    // Load the tokenizer
    let (tokenizer, logs) = match load_tokenizer(model_str, hf_token.as_deref()) {
        Ok((t, logs)) => (t, logs),
        Err((e, logs)) => {
            let error_msg = format!("Failed to load tokenizer '{}': {:?}", model_str, e);
            log_debug(&error_msg);
            for log in logs {
                log_debug(&log);
            }
            return ptr::null_mut();
        }
    };
    
    // Tokenize the text
    let encoding = match tokenizer.encode(text_str, false) {
        Ok(e) => e,
        Err(e) => {
            let error_msg = format!("Failed to encode text: {:?}", e);
            log_debug(&error_msg);
            return ptr::null_mut();
        }
    };
    
    // Convert the result to a JSON string
    let tokens = encoding.get_tokens();
    let ids = encoding.get_ids();
    
    // Create a result structure with token IDs and text
    let result = serde_json::json!({
        "tokens": tokens,
        "ids": ids,
        "debug_logs": logs
    });
    
    let result_string = result.to_string();
    log_debug(&format!("Result: {}", result_string));
    
    // Convert to C string
    match CString::new(result_string) {
        Ok(c_string) => c_string.into_raw(),
        Err(e) => {
            log_debug(&format!("Error creating C string: {:?}", e));
            ptr::null_mut()
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn tokenize_text_with_token(text: *const c_char, model_name: *const c_char, token: *const c_char) -> *mut c_char {
    log_debug("tokenize_text_with_token called");
    
    // Convert C strings to Rust strings
    let c_text = unsafe {
        if text.is_null() {
            log_debug("Text pointer is null");
            return ptr::null_mut();
        }
        CStr::from_ptr(text)
    };
    
    let c_model = unsafe {
        if model_name.is_null() {
            log_debug("Model name pointer is null");
            return ptr::null_mut();
        }
        CStr::from_ptr(model_name)
    };
    
    let c_token = unsafe {
        if token.is_null() {
            log_debug("Token is null, will try without authentication");
            None
        } else {
            match CStr::from_ptr(token).to_str() {
                Ok(s) => Some(s),
                Err(_) => {
                    log_debug("Invalid UTF-8 in token");
                    None
                }
            }
        }
    };
    
    let text_str = match c_text.to_str() {
        Ok(s) => s,
        Err(_) => {
            log_debug("Invalid UTF-8 in text");
            return ptr::null_mut();
        }
    };
    
    let model_str = match c_model.to_str() {
        Ok(s) => s,
        Err(_) => {
            log_debug("Invalid UTF-8 in model name");
            return ptr::null_mut();
        }
    };
    
    log_debug(&format!("Processing with token: text='{}', model='{}'", text_str, model_str));
    
    // Load the tokenizer
    let (tokenizer, logs) = match load_tokenizer(model_str, c_token) {
        Ok((t, logs)) => (t, logs),
        Err((e, logs)) => {
            let error_msg = format!("Failed to load tokenizer '{}': {:?}", model_str, e);
            log_debug(&error_msg);
            for log in logs {
                log_debug(&log);
            }
            return ptr::null_mut();
        }
    };
    
    // Tokenize the text
    let encoding = match tokenizer.encode(text_str, false) {
        Ok(e) => e,
        Err(e) => {
            let error_msg = format!("Failed to encode text: {:?}", e);
            log_debug(&error_msg);
            return ptr::null_mut();
        }
    };
    
    // Convert the result to a JSON string
    let tokens = encoding.get_tokens();
    let ids = encoding.get_ids();
    
    // Create a result structure with token IDs and text
    let result = serde_json::json!({
        "tokens": tokens,
        "ids": ids,
        "debug_logs": logs
    });
    
    let result_string = result.to_string();
    log_debug(&format!("Result: {}", result_string));
    
    // Convert to C string
    match CString::new(result_string) {
        Ok(c_string) => c_string.into_raw(),
        Err(e) => {
            log_debug(&format!("Error creating C string: {:?}", e));
            ptr::null_mut()
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn free_string(s: *mut c_char) {
    if !s.is_null() {
        unsafe {
            let _ = CString::from_raw(s);
        }
    }
}