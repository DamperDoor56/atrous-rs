pub mod recompose; // Declara el módulo `recompose` que corresponde a `recompose.rs`
pub mod transform; // Declara el módulo `transform` que corresponde a `transform.rs`

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
