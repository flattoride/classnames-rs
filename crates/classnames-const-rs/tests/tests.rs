use classnames_const_rs::{classnames_concat, trim_format};

const BASE_STYLE: &str = "btn";
const SIZE_LG: &str = "lg";
const THEME_PRIMARY: &str = "primary";

#[test]
fn test_const_classnames() {
    const BUTTON: &str = classnames_concat!(BASE_STYLE, SIZE_LG);
    assert_eq!(BUTTON, "btn lg");

    const FULL_STYLE: &str = classnames_concat!(BASE_STYLE, SIZE_LG, THEME_PRIMARY);
    assert_eq!(FULL_STYLE, "btn lg primary");
}

#[test]
fn test_const_multiline() {
    const COMPLEX_STYLE: &str = classnames_concat!(
        "flex
         items-center",
        "p-4
         m-2",
        "text-center"
    );
    assert_eq!(COMPLEX_STYLE, "flex items-center p-4 m-2 text-center");
}

#[test]
fn test_compile_time_evaluation() {
    const BASE: &str = "btn";
    const SIZE: &str = "lg";

    const STYLE: &str = classnames_concat!(BASE, SIZE);

    // Verify compile-time calculation result
    assert_eq!(STYLE, "btn lg");

    // Ensure type is `&'static str`
    const _: &'static str = STYLE;
}

#[test]
fn test_const_compilation() {
    const STYLE: &str = classnames_concat!(
        "static-class",
        BASE_STYLE, // Compiler will directly use constant value
        SIZE_LG,
    );

    assert_eq!(STYLE, "static-class btn lg");

    // Verify type
    const _: &'static str = STYLE;
}

// README Example Tests

#[test]
fn test_readme_basic_usage() {
    const BUTTON_CLASS: &str = classnames_concat!("btn", "btn-primary");
    assert_eq!(BUTTON_CLASS, "btn btn-primary");
}

#[test]
fn test_readme_messy_whitespace() {
    const COMPLEX_CLASS: &str = classnames_concat!("header  ", "  main  ", "footer");
    assert_eq!(COMPLEX_CLASS, "header main footer");
}

#[test]
fn test_readme_empty_strings() {
    const SPARSE_CLASS: &str = classnames_concat!("", "active", "", "highlight");
    assert_eq!(SPARSE_CLASS, "active highlight");
}

#[test]
fn test_readme_messy_classes() {
    const MESSY_CLASSES: &str = classnames_concat!("  header ", " main  ", "footer  ");
    assert_eq!(MESSY_CLASSES, "header main footer");
}

#[test]
fn test_readme_complex_example() {
    const BASE_CLASSES: &str = "container mx-auto";
    const RESPONSIVE_CLASSES: &str = "md:w-1/2 lg:w-1/3";
    const STATE_CLASSES: &str = "hover:bg-blue-500 focus:outline-none";

    const COMPONENT_CLASSES: &str = classnames_concat!(
        BASE_CLASSES,
        RESPONSIVE_CLASSES,
        STATE_CLASSES,
        "p-4 rounded-lg"
    );

    assert_eq!(
        COMPONENT_CLASSES,
        "container mx-auto md:w-1/2 lg:w-1/3 hover:bg-blue-500 focus:outline-none p-4 rounded-lg"
    );
}

#[test]
fn test_readme_classnames_concat_example() {
    const CLASSES: &str = classnames_concat!("btn", "btn-large", "btn-primary");
    assert_eq!(CLASSES, "btn btn-large btn-primary");

    const MESSY_CLASSES: &str = classnames_concat!("  header ", " main  ", "footer  ");
    assert_eq!(MESSY_CLASSES, "header main footer");
}

#[test]
fn test_readme_trim_format_example() {
    const NORMALIZED: &str = trim_format!("  hello    world  ");
    assert_eq!(NORMALIZED, "hello world");
}

#[test]
fn test_trim_format_with_various_whitespace() {
    const CLASSES: &str = trim_format!("  btn   primary \n active \tdark");
    assert_eq!(CLASSES, "btn primary active dark");

    // Verify type
    const _: &'static str = CLASSES;
}

#[test]
fn test_edge_cases() {
    // Test with only spaces
    const ONLY_SPACES: &str = trim_format!("   ");
    assert_eq!(ONLY_SPACES, "");

    // Test with empty string
    const EMPTY: &str = trim_format!("");
    assert_eq!(EMPTY, "");

    // Test single word
    const SINGLE: &str = trim_format!("  word  ");
    assert_eq!(SINGLE, "word");

    // Test mixed whitespace characters
    const MIXED_WS: &str = trim_format!("\t\n  hello  \r\n  world  \t");
    assert_eq!(MIXED_WS, "hello world");
}

#[test]
fn test_classnames_concat_edge_cases() {
    // Test with empty strings
    const EMPTY_STRINGS: &str = classnames_concat!("", "", "");
    assert_eq!(EMPTY_STRINGS, "");

    // Test mixed empty and non-empty
    const MIXED: &str = classnames_concat!("", "test", "", "class", "");
    assert_eq!(MIXED, "test class");

    // Test single argument
    const SINGLE: &str = classnames_concat!("single");
    assert_eq!(SINGLE, "single");
}
