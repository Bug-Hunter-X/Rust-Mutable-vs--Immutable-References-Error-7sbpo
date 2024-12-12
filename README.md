# Rust Mutable vs. Immutable References
This example showcases a common error in Rust when dealing with mutable and immutable references.  Rust's borrow checker prevents data races by strictly controlling how mutable and immutable references can coexist.

The `bug.rs` file contains code that attempts to modify a value through both a mutable and an immutable reference simultaneously, resulting in a compile-time error. The `bugSolution.rs` demonstrates how to correct this issue by using proper reference management.