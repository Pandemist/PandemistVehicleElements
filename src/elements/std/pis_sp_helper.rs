pub fn split_into_lines(group_strings: &str) -> Vec<String> {
    group_strings
        .replace("\t", "\n")
        .split('\n')
        .map(str::to_string)
        .collect()
}

pub fn get_line_as_str(lines: &[String], row: usize) -> &str {
    lines.get(row).map(|s| s.trim()).unwrap_or("0")
}

pub fn get_line_as_int(lines: &[String], row: usize) -> usize {
    get_line_as_str(lines, row).parse::<usize>().unwrap_or(0)
}

pub fn get_line_as_property(lines: &[String], row: usize) -> (String, String) {
    let line = &lines.get(row).map(|s| s.trim()).unwrap_or("0").to_string();

    match line.split_once('=') {
        Some((key, value)) => (key.to_string(), value.to_string()),
        None => (String::new(), String::new()),
    }
}

pub fn get_value_by_name(lines: &[String], key: &str) -> Option<String> {
    lines.iter().find_map(|line| {
        let (line_key, value) = line.split_once('=')?;

        if line_key == key {
            Some(value.to_string())
        } else {
            None
        }
    })
}
