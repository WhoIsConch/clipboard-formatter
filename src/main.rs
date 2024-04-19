use clipboard_win::{monitor, formats, get_clipboard, set_clipboard, ErrorCode};
use regex::Regex;

fn parse_phone_number(input: &str) -> Option<String> {
    let already_formatted_reg = Regex::new(r"\b\d{10}\b").unwrap();

    if already_formatted_reg.is_match(&input) {
        return None;
    }

    let phone_reg = Regex::new(r"\b(?:\(?\d{3}\)?\s?\d{3}[\-\s]?\d{4}|\d{10})\b").unwrap();
    let captures = phone_reg.captures(&input)?;
    let phone_number = captures
        .get(0)
        .map(|m| m.as_str())
        .unwrap_or_default()
        .chars()
        .filter(|c| c.is_digit(10))
        .collect::<String>();

    if phone_number.len() == 10 {
        Some(phone_number)
    } else {
        None
    }
}

fn parse_mac_address(input: &str) -> Option<String> {
    let already_formatted_reg = Regex::new(r"^([0-9A-Fa-f]{4}\.)([0-9A-Fa-f]{4}\.)([0-9A-Fa-f]{4})$").unwrap();

    if already_formatted_reg.is_match(&input) {
        return None;
    }

    let mac_reg = Regex::new(r"^([0-9A-Fa-f]{2}[:-]){5}([0-9A-Fa-f]{2})$").unwrap();
    let captures = mac_reg.captures(&input)?;
    let mac_addr = captures
        .get(0)
        .map(|m| m.as_str())
        .unwrap_or_default()
        .chars()
        .filter(|c| c.is_ascii_hexdigit())
        .enumerate()
        .fold(String::new(), |mut acc, (idx, c)| {
            acc.push(c);
            if idx % 4 == 3 && idx != 11 {
                acc.push('.');
            }
            acc
        });

    if mac_addr.len() == 14 {
        Some(mac_addr)
    } else {
        None
    }

}

fn main() {
    let mut mon = monitor::Monitor::new().unwrap();
    
    loop {
        let _ = mon.recv();

        let res: Result<String, ErrorCode> = get_clipboard(formats::Unicode);

        let copied = match res {
            Err(error) => {
                eprintln!("Failed to get clipboard data: {error}.");
                continue;
            },
            Ok(result) => {
                result
            }
        };

        let copied = copied.trim();

        let phone = parse_phone_number(&copied);

        if let Some(parsed_num) = phone {
            let _ = set_clipboard(formats::Unicode, &parsed_num);
            println!("Copied {parsed_num} to clipboard.");
            continue;
        } 

        let mac = parse_mac_address(&copied);

        if let Some(parsed_mac) = mac {
            let _ = set_clipboard(formats::Unicode, &parsed_mac);
            println!("Copied {parsed_mac} to clipboard.");
            continue;
        }

    }
}
