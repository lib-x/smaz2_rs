#[cfg(test)]
mod tests {
    use crate::smaz2::{compress, decompress};

    #[test]
    fn test_compress() {
        let s = "hello world";
        let compressed = compress(s).unwrap();
      assert_eq!(compressed, vec![132, 177, 111, 32, 6, 53])
    }

    #[test]
    fn test_decompress() {
       let s:Vec<u8> = vec![132, 177, 111, 32, 6, 53];
       let origin = decompress(&s).unwrap();
        assert_eq!(origin, "hello world")
    }
}