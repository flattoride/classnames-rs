use classnames_rs::{choose, classnames, maybe, pretty_classname, when};

#[test]
fn test_basic_strings() {
    assert_eq!(classnames!("foo"), "foo");
    assert_eq!(classnames!("foo", "bar"), "foo bar");
    assert_eq!(classnames!("foo", "bar", "baz"), "foo bar baz");
}

#[test]
fn test_conditional() {
    let is_active = true;
    let is_disabled = false;

    assert_eq!(
        classnames!(
            "btn",
            when!(is_active, "active"),
            when!(is_disabled, "disabled")
        ),
        "btn active"
    );
}

#[test]
fn test_option_types() {
    let some_class: Option<String> = Some("optional".to_string());
    let none_class: Option<String> = None;
    let true_class: String = "true".to_string();

    assert_eq!(
        classnames!("base", maybe!(some_class), maybe!(none_class), true_class),
        "base optional true"
    );
}

#[test]
fn test_tuple_conditions() {
    let is_primary = true;
    let is_small = false;

    assert_eq!(
        classnames!("btn", (is_primary, "primary"), (is_small, "small")),
        "btn primary"
    );
}

#[allow(unused_braces)]
#[test]
fn test_complex_combinations() {
    let is_active = true;
    let size = Some("large");
    let theme: Option<String> = None;

    assert_eq!(
        classnames!(
            "btn",
            when!(is_active, classnames!("active", "highlighted")),
            maybe!(size),
            maybe!(theme),
            if is_active { "on" } else { "off" }
        ),
        "btn active highlighted large on"
    );
}

// test choose macro
#[test]
fn test_choose() {
    let is_primary = true;
    let is_small = false;

    assert_eq!(
        classnames!(
            "btn",
            choose!(is_primary, "primary", "secondary"),
            choose!(is_small, "small", "large")
        ),
        "btn primary large"
    );
}

#[test]
fn test_empty_and_whitespace() {
    assert_eq!(classnames!("", "foo", "", "bar", ""), "foo bar");
    assert_eq!(classnames!(""), "");
    assert_eq!(classnames!("", ""), "");
}

#[test]
fn test_three_tuple_condition() {
    let is_active = true;

    assert_eq!(
        classnames!(
            "btn",
            (is_active, "active", "inactive"),
            (false, "on", "off")
        ),
        "btn active off"
    );
}

#[test]
fn test_choose_macro() {
    let is_primary = true;

    assert_eq!(
        classnames!(
            "btn",
            choose!(is_primary, "primary", "secondary"),
            choose!(false, "dark", "light")
        ),
        "btn primary light"
    );
}

#[test]
fn test_str() {
    const BASE_CLASS: &str = "class1 class2";
    let simple_class = "class3";
    classnames!(simple_class, "class4");
    assert_eq!(
        classnames!(BASE_CLASS, simple_class, "class5"),
        "class1 class2 class3 class5"
    );
}

#[test]
fn test_str_constant() {
    const SIMPLE_CLASS: &str = "btn primary";
    const MULTILINE_CLASS: &str = "
        text-sm
        leading-6
    ";

    assert_eq!(classnames!(SIMPLE_CLASS), "btn primary");

    assert_eq!(
        classnames!(SIMPLE_CLASS, MULTILINE_CLASS, "extra"),
        "btn primary text-sm leading-6 extra"
    );
}

#[test]
fn test_mixed_types() {
    const CONST_CLASS: &str = "const-class";
    let simple_class = "simple";
    let option_class: Option<&str> = Some("optional");
    let none_class: Option<&str> = None;

    assert_eq!(
        classnames!(
            CONST_CLASS,
            simple_class,
            maybe!(option_class),
            maybe!(none_class)
        ),
        "const-class simple optional"
    );
}

#[test]
fn test_whitespace_normalization() {
    assert_eq!(
        classnames!("foo   bar", "baz\n  qux", "  hello   world  "),
        "foo bar baz qux hello world"
    );
}

#[test]
fn test_pretty_classname() {
    let messy_class = "btn   primary\n  large";
    assert_eq!(pretty_classname!(messy_class), "btn primary large");

    const MESSY_CONST: &str = "text-sm\n    leading-6\n\t  p-4";
    assert_eq!(pretty_classname!(MESSY_CONST), "text-sm leading-6 p-4");
}

// ==================== README Examples Tests ====================

#[test]
fn test_readme_basic_usage() {
    let result = classnames!("btn", "btn-primary");
    assert_eq!(result, "btn btn-primary");
}

#[test]
fn test_readme_conditional_classes() {
    let is_active = true;
    let is_small = false;

    let result = classnames!("btn", (is_active, "active"), (is_small, "btn-sm"));
    assert_eq!(result, "btn active");
}

#[test]
fn test_readme_option_types() {
    let optional_class: Option<&str> = Some("highlight");
    let none_class: Option<&str> = None;

    let result = classnames!("base", maybe!(optional_class), maybe!(none_class));
    assert_eq!(result, "base highlight");
}

#[allow(unused_braces)]
#[test]
fn test_readme_ternary_expressions() {
    let count = 5;
    let result = classnames!("counter", if count > 3 { "high" } else { "low" });
    assert_eq!(result, "counter high");
}

#[test]
fn test_readme_choose_macro() {
    let is_dark = true;
    let result = classnames!("theme", choose!(is_dark, "dark", "light"));
    assert_eq!(result, "theme dark");
}

#[test]
fn test_readme_when_macro() {
    let is_loading = true;
    let result = classnames!("btn", when!(is_loading, "loading"));
    assert_eq!(result, "btn loading");

    // Test when condition is false
    let is_disabled = false;
    let result = classnames!("btn", when!(is_disabled, "disabled"));
    assert_eq!(result, "btn");
}

#[test]
fn test_readme_maybe_macro() {
    let optional: Option<&str> = Some("special");
    let result = classnames!("base", maybe!(optional));
    assert_eq!(result, "base special");

    // Test with None
    let no_class: Option<&str> = None;
    let result = classnames!("base", maybe!(no_class));
    assert_eq!(result, "base");
}

#[test]
fn test_readme_complex_example() {
    let is_primary = true;
    let size: Option<&str> = Some("large");
    let is_disabled = false;
    let count = 3;

    let result = classnames!(
        "btn",
        choose!(is_primary, "btn-primary", "btn-secondary"),
        maybe!(size),
        when!(is_disabled, "disabled"),
        (count > 0, "has-items", "empty")
    );
    assert_eq!(result, "btn btn-primary large has-items");
}

#[allow(dead_code)]
#[test]
fn test_readme_real_world_example() {
    #[derive(Debug)]
    struct ButtonProps {
        variant: ButtonVariant,
        size: Option<ButtonSize>,
        disabled: bool,
        loading: bool,
    }

    #[derive(Debug)]
    enum ButtonVariant {
        Primary,
        Secondary,
        Danger,
    }

    #[derive(Debug)]
    enum ButtonSize {
        Small,
        Medium,
        Large,
    }

    impl ButtonProps {
        fn class_names(&self) -> String {
            let variant_class = match self.variant {
                ButtonVariant::Primary => "btn-primary",
                ButtonVariant::Secondary => "btn-secondary",
                ButtonVariant::Danger => "btn-danger",
            };

            let size_class = self.size.as_ref().map(|s| match s {
                ButtonSize::Small => "btn-sm",
                ButtonSize::Medium => "btn-md",
                ButtonSize::Large => "btn-lg",
            });

            classnames!(
                "btn",
                variant_class,
                maybe!(size_class),
                when!(self.disabled, "disabled"),
                when!(self.loading, "loading")
            )
        }
    }

    let button = ButtonProps {
        variant: ButtonVariant::Primary,
        size: Some(ButtonSize::Large),
        disabled: false,
        loading: true,
    };

    assert_eq!(button.class_names(), "btn btn-primary btn-lg loading");

    // Test different combinations
    let secondary_button = ButtonProps {
        variant: ButtonVariant::Secondary,
        size: None,
        disabled: true,
        loading: false,
    };

    assert_eq!(secondary_button.class_names(), "btn btn-secondary disabled");
}

#[test]
fn test_readme_quick_start_examples() {
    // Basic usage
    let classes = classnames!("btn", "btn-primary");
    assert_eq!(classes, "btn btn-primary");

    // Conditional classes
    let is_active = true;
    let classes = classnames!("btn", (is_active, "active"));
    assert_eq!(classes, "btn active");
}

#[allow(unused_braces)]
#[test]
fn test_readme_supported_expression_types() {
    // String literals
    let result = classnames!("btn");
    assert_eq!(result, "btn");

    // Conditional tuples
    let is_active = true;
    let result = classnames!((is_active, "active"));
    assert_eq!(result, "active");

    // Ternary tuples
    let is_dark = true;
    let result = classnames!((is_dark, "dark", "light"));
    assert_eq!(result, "dark");

    // Option types
    let highlight: Option<&str> = Some("highlight");
    let result = classnames!(maybe!(highlight));
    assert_eq!(result, "highlight");

    // If expressions
    let loading = true;
    let result = classnames!(if loading { "spinner" } else { "" });
    assert_eq!(result, "spinner");
}

#[test]
fn test_choose_macro_standalone() {
    let is_active = true;
    let class = choose!(is_active, "active", "inactive");
    assert_eq!(class, "active");

    let is_primary = false;
    let class = choose!(is_primary, "btn-primary", "btn-secondary");
    assert_eq!(class, "btn-secondary");
}

#[test]
fn test_edge_cases() {
    // Empty strings
    assert_eq!(classnames!("", "btn", ""), "btn");

    // Whitespace handling
    assert_eq!(classnames!("  btn  ", "  primary  "), "btn primary");

    // Multiple conditions
    let a = true;
    let b = false;
    let c = true;
    assert_eq!(
        classnames!(when!(a, "a"), when!(b, "b"), when!(c, "c")),
        "a c"
    );

    // Nested choose
    let theme = "dark";
    let size = "large";
    assert_eq!(
        classnames!(
            choose!(theme == "dark", "theme-dark", "theme-light"),
            choose!(size == "large", "size-lg", "size-sm")
        ),
        "theme-dark size-lg"
    );
}
