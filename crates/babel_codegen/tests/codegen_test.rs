use std::env;
use std::fs::{read_to_string, File};
use std::io::Write;
use query_parser::parse;
use babel_codegen::generate_babel_visitor;

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
fn get_js_file_absolute_path(file_path: &str) -> String {
    let mut path = get_json_fixture_dir_absolute_path();
    path.push_str(file_path);
    path.push_str(".js");
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

pub fn test_by_file_name(name: & str) {
    let esquery_file_path = get_esquery_file_absolute_path(name);
    let js_file_path = get_js_file_absolute_path(name);
    let is_update = env::var("UPDATE").is_ok();
    let esquery = read_file_to_string(&esquery_file_path, format!("Can not read esquery code - {}", esquery_file_path).as_str());
    let ast = parse(&esquery).unwrap();
    if is_update {
        let js_code = generate_babel_visitor(&ast);
        write_string_to_file(&js_file_path, &js_code);
    }else {
        let js_code = generate_babel_visitor(&ast);
        let expect_js_code = read_file_to_string(&js_file_path, format!("Can not read js code - {}",js_file_path ).as_str());
        assert_eq!(js_code, expect_js_code);
    }
}

/// Generate test case for babel codegen
macro_rules! test_babel_codegen_cases {
    ( $(
        ($func_name: ident, $test_case: expr)
    ),* ) => {
        $(
            #[test]
            fn $func_name() {
                test_by_file_name($test_case)
            }
        )*
    };
}

test_babel_codegen_cases!(
    (jsx_open_element, "jsx_open_element"),
    (jsx_open_element_with_name, "jsx_open_element_with_name")
);
