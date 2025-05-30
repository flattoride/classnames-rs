# classnames-const-rs

A Rust macro library for compile-time CSS class name concatenation and processing.

[![Crates.io](https://img.shields.io/crates/v/classnames-const-rs.svg)](https://crates.io/crates/classnames-const-rs)
[![Documentation](https://docs.rs/classnames-const-rs/badge.svg)](https://docs.rs/classnames-const-rs)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

## Features

- 🚀 **Zero runtime overhead** - Everything happens at compile time
- 🧹 **Automatic whitespace handling** - Removes extra spaces and normalizes whitespace
- 🔗 **Multiple class concatenation** - Combine any number of class names
- 📝 **Type-safe** - Compile-time string processing with full type safety
- 🎯 **Simple API** - Easy to use macro interface

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
classnames-const-rs = "0.1.0"
```

## Usage

### Basic Usage

```rust
use classnames_const_rs::*;

const BUTTON_CLASS: &str = classnames_concat!("btn", "btn-primary");
assert_eq!(BUTTON_CLASS, "btn btn-primary");
```

### Handling Messy Whitespace

The macro automatically handles extra whitespace:

```rust
use classnames_const_rs::*;

const MESSY_CLASSES: &str = classnames_concat!("  header ", " main  ", "footer  ");
assert_eq!(MESSY_CLASSES, "header main footer");
```

### Empty Strings

Empty strings are handled gracefully:

```rust
use classnames_const_rs::*;

const SPARSE_CLASS: &str = classnames_concat!("", "active", "", "highlight");
assert_eq!(SPARSE_CLASS, "active highlight");
```

### Complex Example

```rust
use classnames_const_rs::*;

const BASE_CLASSES: &str = "container mx-auto";
const RESPONSIVE_CLASSES: &str = "md:w-1/2 lg:w-1/3";
const STATE_CLASSES: &str = "hover:bg-blue-500 focus:outline-none";

const COMPONENT_CLASSES: &str = classnames_concat!(
    BASE_CLASSES,
    RESPONSIVE_CLASSES,
    STATE_CLASSES,
    "p-4 rounded-lg"
);

// Result: "container mx-auto md:w-1/2 lg:w-1/3 hover:bg-blue-500 focus:outline-none p-4 rounded-lg"
```

## API Reference

### `classnames_concat!`

Concatenates multiple class name strings with automatic whitespace handling.

**Syntax:**
```rust
classnames_concat!(class1, class2, ..., classN)
```

**Features:**
- Removes leading and trailing whitespace
- Collapses multiple consecutive spaces into single spaces
- Handles empty strings gracefully
- Works with any number of arguments

### `trim_format!`

Internal macro for whitespace normalization. Generally not needed for direct use.

## Use Cases

This library is perfect for:

- 🎨 **CSS-in-Rust** applications where you need to build class strings
- 🌐 **Web frameworks** that generate HTML with dynamic classes
- 📱 **UI libraries** that combine multiple styling concerns
- 🔧 **Build-time optimizations** where class strings are known at compile time

## Performance

Since all processing happens at compile time, there is **zero runtime overhead**. The resulting strings are embedded directly into your binary as static string literals.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Built on top of the excellent [`constcat`](https://crates.io/crates/constcat) crate for compile-time string concatenation
- Inspired by the JavaScript [`classnames`](https://github.com/JedWatson/classnames) library