use clipboard_win::{monitor, formats, get_clipboard, set_clipboard};
use regex::Regex;

fn parse_phone_number(input: &str) -> Option<String> {
    let already_parsed_reg = Regex::new(r"\b\d{10}\b").unwrap();

    if already_parsed_reg.is_match(input) {
        return None;
    }

    let reg = Regex::new(r"\b(?:\(?\d{3}\)?\s?\d{3}[\-\s]?\d{4}|\d{10})\b").unwrap();
    let captures = reg.captures(input)?;
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

fn main() {
    let mut mon = monitor::Monitor::new().unwrap();
    
    loop {
        let _ = mon.recv();

        let res: String = get_clipboard(formats::Unicode).expect("Could not read clipboard");

        dbg!(&res);

        let phone = parse_phone_number(&res);

        if let Some(parsed_num) = phone {
            let _ = set_clipboard(formats::Unicode, parsed_num);
        }

    }
}
