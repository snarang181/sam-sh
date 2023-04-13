use std::io;
use std::io::Write; 

pub fn flush_buffer() {
    let result = io::stdout().flush().unwrap();
    if result != () {
       eprintln!("Error flushing buffer");
    }
    return result;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_flush_buffer() {
        let result = flush_buffer();
        assert_eq!(result, ());
    }
}