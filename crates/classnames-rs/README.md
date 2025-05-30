# classnames-rs

[![Crates.io](https://img.shields.io/crates/v/classnames-rs.svg)](https://crates.io/crates/classnames-rs)
[![Documentation](https://docs.rs/classnames-rs/badge.svg)](https://docs.rs/classnames-rs)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A Rust macro library for dynamically building CSS class names, inspired by the JavaScript [classnames](https://github.com/JedWatson/classnames) library.

## Features

- ✨ **String literals support** - Basic class name strings
- 🔀 **Conditional rendering** - Dynamic class names based on conditions
- 🎯 **Option type support** - Handle optional class names gracefully
- 🚀 **Ternary expressions** - Inline conditional logic
- 🛠️ **Helper macros** - Additional utilities like `choose!`, `when!`, `maybe!`
- 🧹 **Automatic whitespace normalization** - Clean output formatting

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
classnames-rs = "0.1.0"
```

## Quick Start

```rust
use classnames_rs::classnames;

// Basic usage
let classes = classnames!("btn", "btn-primary");
assert_eq!(classes, "btn btn-primary");

// Conditional classes
let is_active = true;
let classes = classnames!(
    "btn",
    (is_active, "active")
);
assert_eq!(classes, "btn active");
```

## Usage Examples

All examples below are tested and verified. You can run `cargo test` to verify them yourself.

### Basic Usage

```rust
use classnames_rs::classnames;

let result = classnames!("btn", "btn-primary");
assert_eq!(result, "btn btn-primary");
```

### Conditional Class Names

```rust
use classnames_rs::classnames;

let is_active = true;
let is_small = false;

let result = classnames!(
    "btn",
    (is_active, "active"),
    (is_small, "btn-sm")
);
assert_eq!(result, "btn active");
```

### Option Type Support

```rust
use classnames_rs::classnames;

let optional_class: Option<&str> = Some("highlight");
let none_class: Option<&str> = None;

let result = classnames!(
    "base",
    optional_class,
    none_class
);
assert_eq!(result, "base highlight");
```

### Ternary Expressions

```rust
use classnames_rs::classnames;

let count = 5;
let result = classnames!(
    "counter",
    if count > 3 { "high" } else { "low" }
);
assert_eq!(result, "counter high");
```

### Helper Macros

#### `choose!` Macro

```rust
use classnames_rs::{classnames, choose};

let is_dark = true;
let result = classnames!(
    "theme",
    choose!(is_dark, "dark", "light")
);
assert_eq!(result, "theme dark");
```

#### `when!` Macro

```rust
use classnames_rs::{classnames, when};

let is_loading = true;
let result = classnames!(
    "btn",
    when!(is_loading, "loading")
);
assert_eq!(result, "btn loading");
```

#### `maybe!` Macro

```rust
use classnames_rs::{classnames, maybe};

let optional: Option<&str> = Some("special");
let result = classnames!(
    "base",
    maybe!(optional)
);
assert_eq!(result, "base special");
```

### Complex Example

```rust
use classnames_rs::{classnames, choose, when};

let is_primary = true;
let size: Option<&str> = Some("large");
let is_disabled = false;
let count = 3;

let result = classnames!(
    "btn",
    choose!(is_primary, "btn-primary", "btn-secondary"),
    size,
    when!(is_disabled, "disabled"),
    (count > 0, "has-items", "empty")
);
assert_eq!(result, "btn btn-primary large has-items");
```

## Supported Expression Types

| Expression Type | Syntax | Example |
|----------------|--------|---------|
| **String literals** | `"class-name"` | `"btn"` |
| **Conditional tuples** | `(condition, "class")` | `(is_active, "active")` |
| **Ternary tuples** | `(condition, "true-class", "false-class")` | `(is_dark, "dark", "light")` |
| **Option types** | `some_option` | `Some("highlight")` |
| **If expressions** | `if condition { "true" } else { "false" }` | `if loading { "spinner" } else { "" }` |
| **Block expressions** | `{ /* returns Option<T> or &str */ }` | `{ get_dynamic_class() }` |

## API Reference

### Core Macros

- **`classnames!(...)`** - Main macro for building class names
- **`choose!(condition, true_val, false_val)`** - Conditional selection
- **`when!(condition, value)`** - Conditional inclusion
- **`maybe!(option)`** - Handle Option types
- **`pretty_classname!(input)`** - Normalize whitespace

## Real-world Example

```rust
use classnames_rs::{classnames, choose, when};

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
            size_class,
            when!(self.disabled, "disabled"),
            when!(self.loading, "loading")
        )
    }
}

// Usage
let button = ButtonProps {
    variant: ButtonVariant::Primary,
    size: Some(ButtonSize::Large),
    disabled: false,
    loading: true,
};

assert_eq!(button.class_names(), "btn btn-primary btn-lg loading");
```

## Testing Examples

All examples in this README are tested to ensure they work as documented. Run the following to verify:

```bash
# Run all tests
cargo test

# Run only documentation tests
cargo test --doc

# Run specific test module
cargo test readme_examples
```

## Performance

**Important Note**: This library has **runtime overhead** for conditional evaluations. The macro generates code that performs condition checks at runtime.

**For zero runtime overhead**, consider using [classnames-const-rs](https://crates.io/crates/classnames-const-rs) which provides compile-time class name resolution for static use cases.

### Performance Characteristics:
- ✅ **Macro expansion**: Zero overhead (compile-time)
- ⚠️ **Conditional logic**: Runtime evaluation overhead
- ✅ **String operations**: Minimal allocation overhead
- ✅ **Memory usage**: Efficient string building

Choose the right tool for your use case:
- Use `classnames-rs` for dynamic, runtime-dependent class names
- Use `classnames-const-rs` for static, compile-time class names

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Inspired by the JavaScript [classnames](https://github.com/JedWatson/classnames) library
- Built with ❤️ for the Rust community
