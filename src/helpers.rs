use std::io;

pub fn parse_curl_command(command: &str) -> Result<(String, Vec<String>), Box<dyn std::error::Error>> {
    let mut parts = command.split_whitespace();
    let mut url = String::new();
    let mut headers = Vec::new();

    while let Some(part) = parts.next() {
        match part {
            "-H" | "--header" => {
                if let Some(header) = parts.next() {
                    headers.push(header.to_string());
                }
            }
            _ if part.starts_with("http://") || part.starts_with("https://") => {
                url = part.to_string();
            }
            _ => {}
        }
    }

    if url.is_empty() {
        Err(Box::new(io::Error::new(io::ErrorKind::InvalidInput, "Invalid curl command")))
    } else {
        Ok((url, headers))
    }
}
