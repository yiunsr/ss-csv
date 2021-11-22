#[cfg(test)]
mod tests {
    use ss_csv::ss_csv::{CSV, CSVBuilder, FieldResult};
    use super::*;

    static HAYSTACK_GDP_CSV: &'static [u8] = include_bytes!("../data/test/gdp_org.csv");

    #[test]
    fn test_0001_01_singleline() {
        println!("==== 01 ====");
        let haystack = "a1,b11";
        let mut csv_parser = CSVBuilder::new().from_buffer(haystack.as_bytes());

        let (csv_type, col) = csv_parser.next();
        assert!(matches!(csv_type, FieldResult::Field));
        assert_eq!(col, "a1");

        let (csv_type, col) = csv_parser.next();
        assert!(matches!(csv_type, FieldResult::FieldEnd));
        assert_eq!(col,"b11");

        let (csv_type, col) = csv_parser.next();
        assert!(matches!(csv_type, FieldResult::End));

        println!("==== 02 ====");
        let haystack = "a1, b11\n";
        let mut csv_parser = CSVBuilder::new().from_buffer(haystack.as_bytes());

        let (csv_type, col) = csv_parser.next();
        assert!(matches!(csv_type, FieldResult::Field));
        assert_eq!(col, "a1");

        let (csv_type, col) = csv_parser.next();
        assert!(matches!(csv_type, FieldResult::FieldEnd));
        assert_eq!(col," b11");

        let (csv_type, col) = csv_parser.next();
        assert!(matches!(csv_type, FieldResult::End));
    }

    // #[test]
    // fn test_0001_02_checkshort() {
    //     let haystack = b"a1\tb11\r";
    //     let mut haystack = haystack.to_vec();
    //     let mut csv_parser = CSV::new(&mut haystack);

    //     let (csv_type, col) = csv_parser.next();
    //     assert!(matches!(csv_type, FieldResult::Field));
    //     assert_eq!(col, "a1");

    //     let (csv_type, col) = csv_parser.next();
    //     assert!(matches!(csv_type, FieldResult::FieldEnd));
    //     assert_eq!(col,"b11");

    //     let (csv_type, col) = csv_parser.next();
    //     assert!(matches!(csv_type, FieldResult::End));

    //     let haystack = b"a1|tb11\r\n";
    //     let mut haystack = haystack.to_vec();
    //     let mut csv_parser = CSV::new(&mut haystack);

    //     let (csv_type, col) = csv_parser.next();
    //     assert!(matches!(csv_type, FieldResult::Field));
    //     assert_eq!(col, "a1");

    //     let (csv_type, col) = csv_parser.next();
    //     assert!(matches!(csv_type, FieldResult::FieldEnd));
    //     assert_eq!(col,"b11");

    //     let (csv_type, col) = csv_parser.next();
    //     assert!(matches!(csv_type, FieldResult::End));
    // }
}