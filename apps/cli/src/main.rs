//! `anotai` command-line interface.

fn main() {
    println!("anotai {}", env!("CARGO_PKG_VERSION"));
}

#[cfg(test)]
mod tests {
    #[test]
    fn binary_builds() {
        assert_eq!(env!("CARGO_PKG_NAME"), "anotai-cli");
    }
}
