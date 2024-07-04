mod test_helper;
use test_helper::test_by_file_name;

test_lexer_cases!(
    (jsx_open_element, "jsx_open_element"),
    (jsx_open_element_with_name, "jsx_open_element_with_name")
);
