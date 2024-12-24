pub fn generate_nametag_text(name: String) -> Result<String, String> {
    if name.len() > 0 {
        //Some(format!("Hi! My name is {}", name))
        Ok(format!("Hi! My name is {}", name))
    } else {
        // The error message should be: "`name` was empty; it must be nonempty."
        //None
        Err("`name` was empty; it must be nonempty.".to_string())
    }
}
