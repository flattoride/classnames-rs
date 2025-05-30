/// Conditional selection helper macro for simplifying conditional class name logic.
///
/// Accepts three parameters:
/// - condition: A conditional expression
/// - true_value: Class name returned when condition is true
/// - false_value: Class name returned when condition is false
///
/// # Examples
///
/// ```rust
/// use classnames_rs::{classnames, choose};
///
/// let is_active = true;
/// let result = classnames!(
///     "btn",
///     choose!(is_active, "active", "inactive")
/// );
/// assert_eq!(result, "btn active");
///
/// // Can be combined with classnames! macro
/// let is_dark = false;
/// let size = "large";
/// let result = classnames!(
///     "theme",
///     choose!(is_dark, "dark", "light"),
///     size
/// );
/// assert_eq!(result, "theme light large");
/// ```
use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input,
    punctuated::Punctuated,
    Expr, ExprBlock, ExprIf, ExprTuple, Token,
};

struct ClassNamesInput {
    exprs: Vec<Expr>,
}

impl Parse for ClassNamesInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let exprs = Punctuated::<Expr, Token![,]>::parse_terminated(input)?;
        Ok(ClassNamesInput {
            exprs: exprs.into_iter().collect(),
        })
    }
}

/// A procedural macro for dynamically building CSS class names.
///
/// # Features
/// - Support for string literals
/// - Support for conditional class names
/// - Support for Option types (Use `maybe!` macro)
/// - Support for ternary expressions
/// - Support for block expressions
/// - Automatic whitespace normalization
///
/// # Examples
///
/// ### Basic usage:
/// ```rust
/// use classnames_rs::classnames;
///
/// let result = classnames!("btn", "btn-primary");
/// assert_eq!(result, "btn btn-primary");
/// ```
///
/// ### Conditional class names:
/// ```rust
/// use classnames_rs::classnames;
///
/// let is_active = true;
/// let result = classnames!(
///     "btn",
///     (is_active, "active")
/// );
/// assert_eq!(result, "btn active");
/// ```
///
/// ### Option types:
/// ```rust
/// use classnames_rs::{classnames, maybe};
///
/// let optional_class: Option<&str> = Some("highlight");
/// let result = classnames!("base", maybe!(optional_class));
/// assert_eq!(result, "base highlight");
/// ```
///
/// ### Ternary expressions:
/// ```rust
/// use classnames_rs::classnames;
///
/// let is_dark = true;
/// let result = classnames!(
///     "theme",
///     if is_dark { "dark" } else { "light" }
/// );
/// assert_eq!(result, "theme dark");
/// ```
///
/// ### Triple tuple conditions:
/// ```rust
/// use classnames_rs::classnames;
///
/// let count = 5;
/// let result = classnames!(
///     "list",
///     (count > 0, "has-items", "empty")
/// );
/// assert_eq!(result, "list has-items");
/// ```
#[proc_macro]
pub fn classnames(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ClassNamesInput);
    let mut tokens = Vec::new();

    for expr in input.exprs {
        tokens.push(parse_expr(expr));
    }

    quote! {
        {
            let mut classes = Vec::new();
            #(#tokens)*
            classes.into_iter()
                .filter(|s| !s.is_empty())
                .collect::<Vec<_>>()
                .join(" ")
        }
    }
    .into()
}

/// Inline function for normalizing class name strings
#[inline]
#[allow(dead_code)]
fn normalize_classname(input: &str) -> String {
    input
        .split_whitespace()
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join(" ")
}

fn parse_expr(expr: Expr) -> proc_macro2::TokenStream {
    // Detailed debug output for development
    // eprintln!("DEBUG - Full Expression: {:#?}", expr);

    match expr {
        // Regular Path (constants or variable references)
        Expr::Path(path) => {
            // eprintln!("DEBUG - Matched Regular Path: {:#?}", path);
            quote! {
                {
                    let class_str = #path;
                    let normalized = class_str.split_whitespace()
                        .filter(|s| !s.is_empty())
                        .collect::<Vec<_>>()
                        .join(" ");
                    classes.push(normalized);
                }
            }
        }
        Expr::Reference(expr_ref) => {
            // eprintln!("DEBUG - Matched Reference: {:#?}", expr_ref);
            quote! {
                {
                    let class_str = #expr_ref;
                    classes.push(class_str.to_string());
                }
            }
        }
        // String literals: "text"
        Expr::Lit(syn::ExprLit {
            lit: syn::Lit::Str(s),
            ..
        }) => {
            let value = s.value();
            quote! {
                classes.push(
                    (#value).split_whitespace()
                        .filter(|s| !s.is_empty())
                        .collect::<Vec<_>>()
                        .join(" ")
                );
            }
        }
        // Tuple conditions: (cond, "class")
        Expr::Tuple(ExprTuple { elems, .. }) if elems.len() == 2 => {
            let cond = &elems[0];
            let class = &elems[1];
            quote! {
                if #cond {
                    let class = #class.to_string();
                    if !class.is_empty() { classes.push(class); }
                }
            }
        }
        // Ternary expressions: cond ? a : b
        Expr::If(ExprIf {
            cond,
            then_branch,
            else_branch,
            ..
        }) => {
            if let Some((_, else_expr)) = else_branch {
                quote! {
                    {
                        let value = if #cond {
                            #then_branch
                        } else {
                            #else_expr
                        };
                        let class = value.to_string();
                        if !class.is_empty() {
                            classes.push(class);
                        }
                    }
                }
            } else {
                // Handle cases without else branch
                quote! {
                    if #cond {
                        let class = #then_branch.to_string();
                        if !class.is_empty() {
                            classes.push(class);
                        }
                    }
                }
            }
        }
        // Block expressions: if x { ... }
        Expr::Block(ExprBlock { block, .. }) => {
            quote! {
                {
                    let result = #block;
                    if let Some(class) = result {
                        let class = class.to_string();
                        if !class.is_empty() { classes.push(class); }
                    }
                }
            }
        }
        // Triple tuple conditions: (cond, true_value, false_value)
        Expr::Tuple(ExprTuple { elems, .. }) if elems.len() == 3 => {
            let cond = &elems[0];
            let true_val = &elems[1];
            let false_val = &elems[2];
            quote! {
                {
                    let class = if #cond { #true_val } else { #false_val };
                    let class = class.to_string();
                    if !class.is_empty() { classes.push(class); }
                }
            }
        }
        // Other expressions (variables, function calls, etc.)
        _ => {
            // eprintln!("DEBUG - Matched Other: {:#?}", expr);
            quote! {
                {
                    let class = #expr.to_string();
                    if !class.is_empty() { classes.push(class); }
                }
            }
        }
    }
}

/// Conditional class name selection macro for dynamically choosing different class names based on conditions
///
/// # Description
/// - Accepts a conditional expression and two class name values
/// - Returns the corresponding class name based on whether the condition is true or false
/// - Automatically handles excess whitespace in class names
/// - Can be combined with other class name macros
///
/// # Parameters
/// - `condition`: Any expression that evaluates to a boolean value
/// - `true_value`: Class name returned when condition is true
/// - `false_value`: Class name returned when condition is false
///
/// # Examples
///
/// ### Basic usage:
/// ```rust
/// use classnames_rs::choose;
///
/// let is_active = true;
/// let class = choose!(is_active, "active", "inactive");
/// assert_eq!(class, "active");
/// ```
///
/// ### Combined with classnames!:
/// ```rust
/// use classnames_rs::{classnames, choose};
///
/// let is_primary = true;
/// let result = classnames!(
///     "btn",
///     choose!(is_primary, "btn-primary", "btn-secondary")
/// );
/// assert_eq!(result, "btn btn-primary");
/// ```
///
/// ### Complex condition evaluation:
/// ```rust
/// use classnames_rs::{classnames, choose};
///
/// let score = 85;
/// let result = classnames!(
///     "grade",
///     choose!(score >= 80, "excellent", "normal")
/// );
/// assert_eq!(result, "grade excellent");
/// ```
///
/// ### Nested usage:
/// ```rust
/// use classnames_rs::{classnames, choose};
///
/// let is_dark = true;
/// let is_active = false;
/// let result = classnames!(
///     "theme",
///     choose!(is_dark, "dark", "light"),
///     choose!(is_active, "active", "inactive")
/// );
/// assert_eq!(result, "theme dark inactive");
/// ```
#[proc_macro]
pub fn choose(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ClassNamesInput);
    let exprs: Vec<_> = input.exprs.into_iter().collect();

    if exprs.len() != 3 {
        return syn::Error::new(
            proc_macro2::Span::call_site(),
            "choose! macro requires exactly three arguments: condition, true_value, false_value",
        )
        .to_compile_error()
        .into();
    }

    let cond = &exprs[0];
    let true_val = &exprs[1];
    let false_val = &exprs[2];

    // Wrap the result in a string expression
    quote! {
        ({
            let result = if #cond {
                let raw = #true_val.to_string();
                raw.split_whitespace()
                    .filter(|s| !s.is_empty())
                    .collect::<Vec<_>>()
                    .join(" ")
            } else {
                let raw = #false_val.to_string();
                raw.split_whitespace()
                    .filter(|s| !s.is_empty())
                    .collect::<Vec<_>>()
                    .join(" ")
            };
            result
        })
    }
    .into()
}

/// Helper macro for handling optional types
///
/// # Examples
/// ```rust
/// use classnames_rs::{classnames, maybe};
///
/// let optional_class: Option<&str> = Some("highlight");
/// let result = classnames!(
///     "base",
///     maybe!(optional_class)
/// );
/// assert_eq!(result, "base highlight");
///
/// let no_class: Option<&str> = None;
/// let result = classnames!(
///     "base",
///     maybe!(no_class)
/// );
/// assert_eq!(result, "base");
/// ```
#[proc_macro]
pub fn maybe(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ClassNamesInput);
    let exprs: Vec<_> = input.exprs.into_iter().collect();

    if exprs.len() != 1 {
        return syn::Error::new(
            proc_macro2::Span::call_site(),
            "maybe! macro requires exactly one argument",
        )
        .to_compile_error()
        .into();
    }

    let value = &exprs[0];
    quote! {
        ({
            match #value {
                Some(value) => {
                    let raw = value.to_string();
                    raw.split_whitespace()
                        .filter(|s| !s.is_empty())
                        .collect::<Vec<_>>()
                        .join(" ")
                },
                None => String::new()
            }
        })
    }
    .into()
}

/// Conditional helper macro for cleaner syntax
///
/// # Examples
/// ```rust
/// use classnames_rs::{classnames, when};
///
/// let is_active = true;
/// let result = classnames!(
///     "btn",
///     when!(is_active, "active")  // More concise syntax
/// );
/// assert_eq!(result, "btn active");
///
/// let is_disabled = false;
/// let result = classnames!(
///     "btn",
///     when!(is_disabled, "disabled")
/// );
/// assert_eq!(result, "btn");
/// ```
#[proc_macro]
pub fn when(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ClassNamesInput);
    let exprs: Vec<_> = input.exprs.into_iter().collect();

    if exprs.len() != 2 {
        return syn::Error::new(
            proc_macro2::Span::call_site(),
            "when! macro requires exactly two arguments: condition and value",
        )
        .to_compile_error()
        .into();
    }

    let cond = &exprs[0];
    let value = &exprs[1];

    quote! {
        ({
            if #cond {
                let raw = #value.to_string();
                raw.split_whitespace()
                    .filter(|s| !s.is_empty())
                    .collect::<Vec<_>>()
                    .join(" ")
            } else {
                String::new()
            }
        })
    }
    .into()
}

/// Public macro for formatting class names and normalizing whitespace
///
/// # Examples
/// ```rust
/// use classnames_rs::pretty_classname;
///
/// let messy = "class1   class2\n\t  class3";
/// assert_eq!(pretty_classname!(messy), "class1 class2 class3");
///
/// let with_tabs = "\tprimary\t\tsecondary\t";
/// assert_eq!(pretty_classname!(with_tabs), "primary secondary");
/// ```
#[proc_macro]
pub fn pretty_classname(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ClassNamesInput);
    let expr = &input.exprs[0];

    quote! {
        {
            let raw = #expr.to_string();
            raw.split_whitespace()
                .filter(|s| !s.is_empty())
                .collect::<Vec<_>>()
                .join(" ")
        }
    }
    .into()
}
