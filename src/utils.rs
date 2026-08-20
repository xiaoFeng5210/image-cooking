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

#[cfg(test)]
mod calculate_size_tests {
    use super::*;

    #[test]
    fn test_bytes() {
        assert_eq!(calculate_size(0), "0B");
        assert_eq!(calculate_size(1023), "1023B");
    }

    #[test]
    fn test_kb() {
        assert_eq!(calculate_size(1024), "1.00KB");
        assert_eq!(calculate_size(1536), "1.50KB");
    }

    #[test]
    fn test_mb() {
        assert_eq!(calculate_size(1024 * 1024), "1.00MB");
        assert_eq!(calculate_size(1024 * 1024 + 1024 * 512), "1.50MB");
    }

    #[test]
    fn test_gb() {
        assert_eq!(calculate_size(1024 * 1024 * 1024), "1.00GB");
    }
}
