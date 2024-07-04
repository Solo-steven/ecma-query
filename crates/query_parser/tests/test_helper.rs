use query_parser::ast::RootNode;
use query_parser::{parse, tokenize, TokenWithSpan};
use serde_json::{from_str, to_string_pretty};
use std::env;
use std::fs::{read_to_string, File};
use std::io::Write;

fn get_esquery_fixture_dir_absolute_path() -> String {
    String::from(
        env::current_dir()
            .unwrap()
            .join("../../fixtures/")
            .as_os_str()
            .to_str()
            .unwrap(),
    )
}
fn get_json_fixture_dir_absolute_path() -> String {
    String::from(
        env::current_dir()
            .unwrap()
            .join("tests/fixtures/")
            .as_os_str()
            .to_str()
            .unwrap(),
    )
}
fn get_esquery_file_absolute_path(file_path: &str) -> String {
    let mut path = get_esquery_fixture_dir_absolute_path();
    path.push_str(file_path);
    path.push_str(".esquery");
    path
}
fn get_json_file_absolute_path(file_path: &str) -> String {
    let mut path = get_json_fixture_dir_absolute_path();
    path.push_str(file_path);
    path.push_str(".json");
    path
}

fn read_file_to_string(path: &str, err_msg: &str ) -> String {
    if let Ok(file) = read_to_string(path) {
        file
    }else {
        panic!("{}", err_msg)
    }
}

fn write_string_to_file(path: &str, result: &str) {
    let mut file = File::create(path).unwrap();
    write!(file, "{}", result).unwrap()
}

pub fn test_by_file_name(name: & str, kind: & str) {
    let esquery_file_path = get_esquery_file_absolute_path(name);
    let json_path = match kind {
        "lexer" => get_json_file_absolute_path(format!("tokenize/{}", name).as_str()),
        "parser" => get_json_file_absolute_path(format!("parse/{}", name).as_str()),
        _ => unreachable!()
    };
    let is_update = env::var("UPDATE").is_ok();
    let esquery = read_file_to_string(&esquery_file_path, format!("Can not read esquery code - {}", esquery_file_path).as_str());
    
    match kind {
        "lexer" => {
            let tokens = tokenize(esquery.as_str());
            if is_update {
                write_string_to_file(&json_path, to_string_pretty(&tokens).unwrap().as_str());
            } else {
                let expect_tokens_string = read_file_to_string(&json_path, format!("Can not read json file - {}", json_path).as_str());
                let expect_ast: Vec<TokenWithSpan> = from_str(&expect_tokens_string).unwrap();
                assert_eq!(expect_ast, tokens);
            }
        }
        "parser" => {
            let ast = parse(esquery.as_str()).unwrap();
            if is_update {
                write_string_to_file(&json_path, to_string_pretty(&ast).unwrap().as_str());
            } else {
                let expect_ast_string = read_file_to_string(&json_path, format!("Can not read json file - {}", json_path).as_str());
                let expect_ast: RootNode = from_str(&expect_ast_string).unwrap();
                assert_eq!(expect_ast, ast);
            }
        }
        _ => unreachable!()
    };
}

/// Generate test case for parser
#[macro_export]
macro_rules! test_parser_cases {
    ( $(
        ($func_name: ident, $test_case: expr)
    ),* ) => {
        $(
            #[test]
            fn $func_name() {
                test_by_file_name($test_case, "parser")
            }
        )*
    };
}

/// Generate test case for parser
#[macro_export]
macro_rules! test_lexer_cases {
    ( $(
        ($func_name: ident, $test_case: expr)
    ),* ) => {
        $(
            #[test]
            fn $func_name() {
                test_by_file_name($test_case, "lexer")
            }
        )*
    };
}
