pub fn create_output(origin_output: Option<String>) -> String {
    let output: String;
    if let Some(result) = origin_output {
        output = result;
    } else {
        output = String::from("output.jpeg");
    }
    output
}
