# UMO

UMO (US Market Open) is a Rust library that provides functionality to list all open time ranges of the US stock market and check the current or next open range by a specific timestamp.

## Features

- List all open time ranges for the US stock market until 2025.
- Check the current or next open range by a specific timestamp.

## Usage

Add the following to your `Cargo.toml`:

```toml
[dependencies]
umo = { version = "0.1.3", features = ["2023", "2024", "2025] }
```

Or `cargo add umo -F full`

## Example

```rust
use umo::{get_open_ranges, pick_open_range};

fn main() {
    let open_ranges = get_open_ranges();
    let timestamp = 1735273592000; // Example timestamp in milliseconds
    if let Some((start, end)) = pick_open_range(timestamp, &open_ranges) {
        println!("Open range: {} - {}", start, end);
    } else {
        println!("No open range found for the given timestamp.");
    }
}
```

## License

This project is licensed under the MIT License.

Feel free to modify it as needed!