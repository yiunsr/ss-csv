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

        let (csv_type, _) = csv_parser.next();
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

        let (csv_type, _) = csv_parser.next();
        assert!(matches!(csv_type, FieldResult::End));
    }

    #[test]
    fn test_0001_02_singleline() {
        println!("==== 01 ====");
        let haystack = b"a1\tb11\r";
        let mut csv_parser = CSVBuilder::new().from_buffer(haystack);

        let (csv_type, col) = csv_parser.next();
        assert!(matches!(csv_type, FieldResult::Field));
        assert_eq!(col, "a1");

        let (csv_type, col) = csv_parser.next();
        assert!(matches!(csv_type, FieldResult::FieldEnd));
        assert_eq!(col,"b11");

        let (csv_type, _) = csv_parser.next();
        assert!(matches!(csv_type, FieldResult::End));

        println!("==== 02 ====");
        let haystack = b"a1|b11\r\n";
        let mut csv_parser = CSVBuilder::new().from_buffer(haystack);

        let (csv_type, col) = csv_parser.next();
        assert!(matches!(csv_type, FieldResult::Field));
        assert_eq!(col, "a1");

        let (csv_type, col) = csv_parser.next();
        assert!(matches!(csv_type, FieldResult::FieldEnd));
        assert_eq!(col,"b11");

        let (csv_type, _) = csv_parser.next();
        assert!(matches!(csv_type, FieldResult::End));
    }

    #[test]
    fn test_0001_03_singleline() {
        println!("==== 01 ====");
        let haystack = b"a1,b11,c111\n";
        let mut csv_parser = CSVBuilder::new().from_buffer(haystack);

        let (csv_type, col) = csv_parser.next();
        assert!(matches!(csv_type, FieldResult::Field));
        assert_eq!(col, "a1");

        let (csv_type, col) = csv_parser.next();
        assert!(matches!(csv_type, FieldResult::Field));
        assert_eq!(col,"b11");

        let (csv_type, col) = csv_parser.next();
        assert!(matches!(csv_type, FieldResult::FieldEnd));
        assert_eq!(col,"c111");

        let (csv_type, _) = csv_parser.next();
        assert!(matches!(csv_type, FieldResult::End));

        println!("==== 02 ====");
        let haystack = b"a,1|b11\r\n";
        let mut csv_parser = CSVBuilder::new().col_sep(b'|').from_buffer(haystack);

        let (csv_type, col) = csv_parser.next();
        assert!(matches!(csv_type, FieldResult::Field));
        assert_eq!(col, "a,1");

        let (csv_type, col) = csv_parser.next();
        assert!(matches!(csv_type, FieldResult::FieldEnd));
        assert_eq!(col,"b11");

        let (csv_type, _) = csv_parser.next();
        assert!(matches!(csv_type, FieldResult::End));

        println!("==== 03 ====");
        let haystack = b"a1,\"b1,1\",c111\r\n";
        let mut csv_parser = CSVBuilder::new().col_sep(b',').from_buffer(haystack);

        csv_parser.skip(1);

        let (csv_type, col) = csv_parser.next();
        assert!(matches!(csv_type, FieldResult::Field));
        assert_eq!(col,"b1,1");

        let (csv_type, col) = csv_parser.next();
        assert!(matches!(csv_type, FieldResult::FieldEnd));
        assert_eq!(col,"c111");

        let (csv_type, _) = csv_parser.next();
        assert!(matches!(csv_type, FieldResult::End));
    }

    #[test]
    fn test_0001_04_singleline() {
        println!("==== 01 ====");
        let haystack = b"a1,,c111\n";
        let mut csv_parser = CSVBuilder::new().from_buffer(haystack);

        let (csv_type, col) = csv_parser.next();
        assert!(matches!(csv_type, FieldResult::Field));
        assert_eq!(col, "a1");

        let (csv_type, col) = csv_parser.next();
        assert!(matches!(csv_type, FieldResult::Field));
        assert_eq!(col,"");

        let (csv_type, col) = csv_parser.next();
        assert!(matches!(csv_type, FieldResult::FieldEnd));
        assert_eq!(col,"c111");

        let (csv_type, _) = csv_parser.next();
        assert!(matches!(csv_type, FieldResult::End));
    }

    #[test]
    fn test_0001_05_singleline() {
        println!("==== 01 ====");
        let haystack = b"\"a1\",\"\",\"\",\"d111\"\n";
        let mut csv_parser = CSVBuilder::new().from_buffer(haystack);

        let (csv_type, col) = csv_parser.next();
        assert!(matches!(csv_type, FieldResult::Field));
        assert_eq!(col, "a1");

        let (csv_type, col) = csv_parser.next();
        assert!(matches!(csv_type, FieldResult::Field));
        assert_eq!(col,"");

        let (csv_type, col) = csv_parser.next();
        assert!(matches!(csv_type, FieldResult::Field));
        assert_eq!(col,"");

        let (csv_type, col) = csv_parser.next();
        assert!(matches!(csv_type, FieldResult::FieldEnd));
        assert_eq!(col,"d111");

        let (csv_type, _) = csv_parser.next();
        assert!(matches!(csv_type, FieldResult::End));
    }

    #[test]
    fn test_0002_01_dualline() {
        println!("==== 01 ====");
        let haystack = b"a1\nb2";
        let mut csv_parser = CSVBuilder::new().from_buffer(haystack);

        let (csv_type, col) = csv_parser.next();
        assert!(matches!(csv_type, FieldResult::FieldEnd));
        assert_eq!(col, "a1");

        let (csv_type, col) = csv_parser.next();
        assert!(matches!(csv_type, FieldResult::FieldEnd));
        assert_eq!(col,"b2");

        let (csv_type, _) = csv_parser.next();
        assert!(matches!(csv_type, FieldResult::End));

        println!("==== 02 ====");
        let haystack = b"a1\nb2\n";
        let mut csv_parser = CSVBuilder::new().from_buffer(haystack);

        let (csv_type, col) = csv_parser.next();
        assert!(matches!(csv_type, FieldResult::FieldEnd));
        assert_eq!(col, "a1");

        let (csv_type, col) = csv_parser.next();
        assert!(matches!(csv_type, FieldResult::FieldEnd));
        assert_eq!(col,"b2");

        let (csv_type, _) = csv_parser.next();
        assert!(matches!(csv_type, FieldResult::End));

        println!("==== 03 ====");
        let haystack = b"a1\rb2\r";
        let mut csv_parser = CSVBuilder::new().from_buffer(haystack);

        let (csv_type, col) = csv_parser.next();
        assert!(matches!(csv_type, FieldResult::FieldEnd));
        assert_eq!(col, "a1");

        let (csv_type, col) = csv_parser.next();
        assert!(matches!(csv_type, FieldResult::FieldEnd));
        assert_eq!(col,"b2");

        let (csv_type, _) = csv_parser.next();
        assert!(matches!(csv_type, FieldResult::End));

        println!("==== 04 ====");
        let haystack = b"a1\r\nb2\r\n";
        let mut csv_parser = CSVBuilder::new().from_buffer(haystack);

        let (csv_type, col) = csv_parser.next();
        assert!(matches!(csv_type, FieldResult::FieldEnd));
        assert_eq!(col, "a1");

        let (csv_type, col) = csv_parser.next();
        assert!(matches!(csv_type, FieldResult::FieldEnd));
        assert_eq!(col,"b2");

        let (csv_type, _) = csv_parser.next();
        assert!(matches!(csv_type, FieldResult::End));

        println!("==== 05 ====");
        let haystack = b"\"a1\n111\"\nb222";
        let mut csv_parser = CSVBuilder::new().from_buffer(haystack);

        let (csv_type, col) = csv_parser.next();
        assert!(matches!(csv_type, FieldResult::FieldEnd));
        assert_eq!(col, "a1\n111");

        let (csv_type, col) = csv_parser.next();
        assert!(matches!(csv_type, FieldResult::FieldEnd));
        assert_eq!(col,"b222");

        let (csv_type, _) = csv_parser.next();
        assert!(matches!(csv_type, FieldResult::End));
    }

    #[test]
    fn test_0002_02_dualline() {
        println!("==== 01 ====");
        let haystack = b"a1,b11\na2,b22";
        let mut csv_parser = CSVBuilder::new().from_buffer(haystack);

        let (csv_type, col) = csv_parser.next();
        assert!(matches!(csv_type, FieldResult::Field));
        assert_eq!(col, "a1");

        let (csv_type, col) = csv_parser.next();
        assert!(matches!(csv_type, FieldResult::FieldEnd));
        assert_eq!(col,"b11");

        let (csv_type, col) = csv_parser.next();
        assert!(matches!(csv_type, FieldResult::Field));
        assert_eq!(col, "a2");

        let (csv_type, col) = csv_parser.next();
        assert!(matches!(csv_type, FieldResult::FieldEnd));
        assert_eq!(col,"b22");

        let (csv_type, _) = csv_parser.next();
        assert!(matches!(csv_type, FieldResult::End));

        println!("==== 02 ====");
        let haystack = b"a1,b11\na2,b22\n";
        let mut csv_parser = CSVBuilder::new().from_buffer(haystack);

        let (csv_type, col) = csv_parser.next();
        assert!(matches!(csv_type, FieldResult::Field));
        assert_eq!(col, "a1");

        let (csv_type, col) = csv_parser.next();
        assert!(matches!(csv_type, FieldResult::FieldEnd));
        assert_eq!(col,"b11");

        let (csv_type, col) = csv_parser.next();
        assert!(matches!(csv_type, FieldResult::Field));
        assert_eq!(col, "a2");

        let (csv_type, col) = csv_parser.next();
        assert!(matches!(csv_type, FieldResult::FieldEnd));
        assert_eq!(col,"b22");

        let (csv_type, _) = csv_parser.next();
        assert!(matches!(csv_type, FieldResult::End));
    }

    #[test]
    fn test_0003_01_parse_csv() {
        let haystack = HAYSTACK_GDP_CSV;
        let mut csv_parser = CSVBuilder::new().from_buffer(haystack);

        let (csv_type, col) = csv_parser.next();
        assert!(matches!(csv_type, FieldResult::Field));
        assert_eq!(col, "Country Name");

        let (csv_type, col) = csv_parser.next();
        assert!(matches!(csv_type, FieldResult::Field));
        assert_eq!(col,"Country Code");

        csv_parser.skip(63);

        let (csv_type, col) = csv_parser.next();
        assert!(matches!(csv_type, FieldResult::FieldEnd));
        assert_eq!(col, "");

        let (csv_type, col) = csv_parser.next();
        assert!(matches!(csv_type, FieldResult::Field));
        assert_eq!(col, "Aruba");

        csv_parser.skip(65);
        csv_parser.skip(66);
        let (csv_type, col) = csv_parser.next();
        assert!(matches!(csv_type, FieldResult::Field));
        assert_eq!(col, "Afghanistan");
        csv_parser.skip(65); // line 4 end
        csv_parser.skip(66*263);
        let (csv_type, _) = csv_parser.next();
        assert!(matches!(csv_type, FieldResult::End));
    }
}