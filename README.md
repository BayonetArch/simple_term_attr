# Introduction

use simple terminal atrributes like colors,clear_line,clear_screen,show_cursor,hide_cursor,move,etc.this library uses simple terminal escape to do so.

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
use simple_term_attr::{TerminalAttribute, StyleAttributes};

fn main() {
    println!("{}", "Red!".red());
    println!("{}", "underlined text".underline());

    simple_term_attr::clear_line(); //clears current line
    simple_term_attr::clear_screen(); //clear the screen

    simple_term_attr::hide_cursor();  // hide terminal cursor  
    simple_term_attr::show_cursor(); // show terminal cursor
    
    let row = 10;
    let col = 20;
    simple_term_attr::move_cursor(row,col);  // move cursor to specified row and col

}
```
