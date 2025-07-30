use std::fs;
use regex::Regex;

fn process_query_inheritance() -> Result<()> {
    let inherit_pattern = Regex::new(r"; inherits (.+)")?;
    
    for entry in fs::read_dir("languages")? {
        let path = entry?.path();
        if !path.join("queries").exists() { continue; }
        
        for query_file in fs::read_dir(path.join("queries"))? {
            let query_path = query_file?.path();
            let content = fs::read_to_string(&query_path)?;
            
            let processed = process_inheritance(&content, &inherit_pattern)?;
            fs::write(&query_path, processed)?;
        }
    }
    Ok(())
}

fn process_inheritance(content: &str, pattern: &Regex) -> Result<String> {
    let mut result = String::new();
    
    for line in content.lines() {
        if let Some(caps) = pattern.captures(line) {
            let inherited_lang = &caps[1];
            let inherited_content = load_inherited_queries(inherited_lang)?;
            result.push_str(&inherited_content);
        } else {
            result.push_str(line);
            result.push('\n');
        }
    }
    
    Ok(result)
}
