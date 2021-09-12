#[cfg(test)]
mod tests {
    use bufchr::{Bufchr, Bufchr2, Bufchr3};
    use super::*;

    static HAYSTACK_ISO_3166: &'static [u8] = include_bytes!("../data/test/ISO-3166-1.csv");

    #[test]
    fn test_0001_01_checkshort() {
        let haystack = b"a1,b11,c111\n\
        a2,b22,c222\n\
        a3,b4,";
        let mut haystack = haystack.to_vec();
        let mut csv_parser = ss_csv::csv_reader::CSV::new(&mut haystack);

        let (csv_type, col) = csv_parser.next();
        assert_eq!(csv_type, FieldResult::Field);
        assert_eq!(col, "a1");

        let (csv_type, col) = csv_parser.next();
        assert_eq!(csv_type, FieldResult::Field);
        assert_eq!(col,"b11");

        let (csv_type, col) = csv_parser.next();
        assert_eq!(csv_type, FieldResult::FieldEnd);
        assert_eq!(col, "c111");
    }
}