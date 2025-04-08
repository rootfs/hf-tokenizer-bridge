use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use tokenizers::tokenizer::{Tokenizer};
use std::ptr;

// Helper function to load a tokenizer from a model name
fn load_tokenizer(model_name: &str) -> Result<Tokenizer, tokenizers::Error> {
    Tokenizer::from_file(model_name)
}

#[unsafe(no_mangle)]
pub extern "C" fn tokenize_text(text: *const c_char, model_name: *const c_char) -> *mut c_char {
    // Convert C strings to Rust strings
    let c_text = unsafe {
        if text.is_null() {
            return ptr::null_mut();
        }
        CStr::from_ptr(text)
    };
    
    let c_model = unsafe {
        if model_name.is_null() {
            return ptr::null_mut();
        }
        CStr::from_ptr(model_name)
    };
    
    let text_str = match c_text.to_str() {
        Ok(s) => s,
        Err(_) => return ptr::null_mut(),
    };
    
    let model_str = match c_model.to_str() {
        Ok(s) => s,
        Err(_) => return ptr::null_mut(),
    };
    
    // Load the tokenizer
    let tokenizer = match load_tokenizer(model_str) {
        Ok(t) => t,
        Err(_) => return ptr::null_mut(),
    };
    
    // Tokenize the text
    let encoding = match tokenizer.encode(text_str, false) {
        Ok(e) => e,
        Err(_) => return ptr::null_mut(),
    };
    
    // Convert the result to a JSON string
    let tokens = encoding.get_tokens();
    let ids = encoding.get_ids();
    
    // Create a result structure with token IDs and text
    let result = serde_json::json!({
        "tokens": tokens,
        "ids": ids
    });
    
    let result_string = result.to_string();
    
    // Convert to C string
    match CString::new(result_string) {
        Ok(c_string) => c_string.into_raw(),
        Err(_) => ptr::null_mut(),
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