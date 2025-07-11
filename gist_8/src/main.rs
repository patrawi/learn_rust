fn parse_positive_integer(value: &str) -> Result<i32, String> {
    value
        .trim()
        .parse::<i32>()
        .map_err(|_| "INvalid integer format".to_string())
        .and_then(|num| {
            if num > 0 {
                Ok(num)
            } else {
                Err("Number must be positive".to_string())
            }
        })
}
fn main() {
    let result_1 = parse_positive_integer("123").unwrap();
    println!("Parsed {} (unwrap): {}", "123", result_1);
    match parse_positive_integer("abc") {
        Ok(num) => println!("Parsed 'abc' (match) {}", num),
        Err(err) => println!("Error: {}", err),
    };
    let result_3 = parse_positive_integer("0").unwrap_or(1);
    println!(
        "Parsing '0' (unwrap_or): Resulting number: {} (defaulted)",
        result_3
    );
    print!("Parsing '-5' (expect):");
    parse_positive_integer("-5").expect("Expected a positive number but encountered an error.");
}
