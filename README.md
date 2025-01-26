# Multiple Mutable Borrows in Rust

This example demonstrates a common error in Rust related to mutable borrows. Rust's ownership and borrowing system prevents data races by disallowing multiple mutable references to the same data at the same time.  This repo provides a minimal reproducible example (MRE) and its solution.

**Error:** Attempting to create multiple mutable references to the same variable (`x`) within the same scope. This results in a compiler error.

**Solution:** Refactor the code to avoid multiple mutable borrows. Potential solutions involve using a single mutable reference, cloning the data, or using interior mutability (e.g., `RefCell`, `Mutex`).