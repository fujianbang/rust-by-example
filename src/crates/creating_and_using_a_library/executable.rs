fn main() {
    rary::public_function();

    rary::indirect_access();
}

// rustc executable.rs --extern rary=library.rlib --edition=2021 && ./executable
