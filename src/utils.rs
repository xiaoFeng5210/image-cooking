pub fn create_output(origin_output: Option<String>) -> String {
    let output: String;
    if let Some(result) = origin_output {
        output = result;
    } else {
        output = String::from("output.jpeg");
    }
    output
}

// 计算大小，返回KB MB GB
pub fn calculate_size(size: u64) -> String {
    match size {
        s if s < 1024 => format!("{s}B"),
        s if s < 1024 * 1024 => format!("{:.2}KB", s as f64 / 1024.0),
        s if s < 1024 * 1024 * 1024 => format!("{:.2}MB", s as f64 / (1024.0 * 1024.0)),
        s if s < 1024 * 1024 * 1024 * 1024 => {
            format!("{:.2}GB", s as f64 / (1024.0 * 1024.0 * 1024.0))
        }
        _ => format!("{size}B"),
    }
}
