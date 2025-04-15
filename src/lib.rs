use parsing::{parse, CharParser, Rule};
use pest::Parser;

pub mod parsing;
mod ir;
mod execution;
mod builtin;

#[test]
fn test_package() {
    let input = "
struct Point{
    int x;
}
Point p() {
    string x = \"Hello\";
    return Point(x);
}
Point p2 = p();
    ";
    let tokens = parsing::parse(input);
    if tokens.is_err(){
        println!("{}", tokens.unwrap_err());
        return;
    }
    let tokens = tokens.unwrap();
    let ir = ir::IR::from_tokens(tokens);
    let mut program = execution::Program::new();
    let result = program.run(&ir);
    if result.is_err(){
        println!("{}", result.unwrap_err());
        return;
    }
    println!("{:?}", program.get_variable("p2".to_string()).unwrap().value.get_value());
}

/// Checks if the given code is syntactically correct
/// 
/// # Arguments
/// 
/// * `code` - A string slice containing source code to validate
/// 
/// # Returns
/// 
/// * `Option<String>` - None if parsing succeeds, Some containing error message if parsing fails
/// 
/// # Examples
/// 
/// ```
/// use charlang::check;
/// let valid_code = "int main() { return 0; }";
/// assert_eq!(check(valid_code), None);
/// 
/// let invalid_code = "int main() { return 0";  // Missing closing brace
/// assert!(check(invalid_code).is_some());
/// ```
/// 
pub fn check(code: &str) -> Option<String> {
    let res = CharParser::parse(Rule::program, code);
    if res.is_ok(){return None;}
    return Some(res.err().unwrap().to_string())
}

pub fn run(code: &str) -> Result<(), String> {
    let res = parse(code);
    if res.is_err(){return Err(res.unwrap_err().to_string());}
    let tokens = res.unwrap();
    let ir = ir::IR::from_tokens(tokens);
    let mut program = execution::Program::new();
    program.include_std_library(builtin::get_std_lib(), builtin::get_std_functions());
    let result = program.run(&ir);
    if result.is_err(){return Err(result.unwrap_err().to_string());}
    return Ok(());
}