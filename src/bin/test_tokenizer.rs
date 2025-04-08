use std::env;
use std::path::Path;
use tokenizers::tokenizer::Tokenizer;

fn main() {
    // Get the model name from command line arguments
    let args: Vec<String> = env::args().collect();
    let model_name = if args.len() > 1 {
        &args[1]
    } else {
        "bert-base-uncased"
    };

    println!("Testing with model: {}", model_name);

    // First check if it's a file path
    if Path::new(model_name).exists() {
        println!("Loading tokenizer from file: {}", model_name);
        
        match Tokenizer::from_file(model_name) {
            Ok(tokenizer) => {
                println!("Successfully loaded tokenizer from file");
                test_tokenizer(&tokenizer);
            }
            Err(e) => {
                eprintln!("Error loading from file: {:?}", e);
            }
        }
    } else {
        println!("Trying to load tokenizer from HuggingFace Hub: {}", model_name);
        
        // Otherwise try to load it as a pretrained model from HuggingFace Hub
        match Tokenizer::from_pretrained(model_name, None) {
            Ok(tokenizer) => {
                println!("Successfully loaded from HuggingFace Hub");
                test_tokenizer(&tokenizer);
            },
            Err(e) => {
                eprintln!("Failed to load from pretrained: {:?}", e);
                
                // Try to load from cache directly (works with previously downloaded models)
                let cache_path = format!("/root/.cache/huggingface/hub/models--{}/snapshots", 
                                         model_name.replace('/', "--"));
                println!("Trying to find in cache: {}", cache_path);
                
                if Path::new(&cache_path).exists() {
                    println!("Cache directory exists, looking for tokenizer.json");
                    
                    // List contents of the directory
                    if let Ok(entries) = std::fs::read_dir(&cache_path) {
                        for entry in entries {
                            if let Ok(entry) = entry {
                                println!("Found directory: {:?}", entry.path());
                                let tokenizer_path = entry.path().join("tokenizer.json");
                                if tokenizer_path.exists() {
                                    println!("Found tokenizer.json at {:?}", tokenizer_path);
                                    match Tokenizer::from_file(&tokenizer_path) {
                                        Ok(tokenizer) => {
                                            println!("Successfully loaded from cache");
                                            test_tokenizer(&tokenizer);
                                            return;
                                        },
                                        Err(e) => {
                                            eprintln!("Error loading from cache file: {:?}", e);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

fn test_tokenizer(tokenizer: &Tokenizer) {
    let text = "Hello, world! This is a test.";
    match tokenizer.encode(text, false) {
        Ok(encoding) => {
            println!("Tokens: {:?}", encoding.get_tokens());
            println!("IDs: {:?}", encoding.get_ids());
        },
        Err(e) => {
            eprintln!("Error encoding text: {:?}", e);
        }
    }
} 