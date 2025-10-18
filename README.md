# Introduction

use simple terminal atrributes like colors,clear_line,clear_screen.

# Quick start

```bash
cargo add simple_term_attr
```

OR

```bash
cargo add --git https://github.com/BayonetArch/simple_term_attr.git
```

# Example 

```rust
use simple_term_attr::{AttrDisplay, Attrs};

fn main() {
    println!("{}", "Red!".red());
    println!("{}", "underlined text".underline());

    println!("{}", AttrDisplay::clear_line()); //clears current line
    println!("{}", AttrDisplay::clear_screen()); //clear the screen
}
```
