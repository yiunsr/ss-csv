#[cfg(test)]
mod tests {
    use ss_csv::ss_csv::{CoreBuilder, FieldResult};
    use super::*;

    static HAYSTACK_GDP_CSV: &'static [u8] = include_bytes!("../data/test/gdp_org.csv");
    static HAYSTACK_CSV_01: &'static [u8] = include_bytes!("../data/test/csv_01.csv");

    #[test]
    fn test_0001_01_singleline() {
        println!("==== 01 ====");
        let haystack = "a1,b11";
        let mut csv_parser = CoreBuilder::new().from_buffer(haystack.as_bytes());

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
    fn test_0001_02_singleline() {
        let haystack = "a1, b11\n";
        let mut csv_parser = CoreBuilder::new().from_buffer(haystack.as_bytes());

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
    fn test_0001_03_singleline() {
        println!("==== 01 ====");
        let haystack = b"a1\tb11\r";
        let mut csv_parser = CoreBuilder::new().from_buffer(haystack);

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
    fn test_0001_04_singleline() {
        let haystack = b"a1|b11\r\n";
        let mut csv_parser = CoreBuilder::new().from_buffer(haystack);

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
    fn test_0001_05_singleline() {
        let haystack = b"a1,b11,c111\n";
        let mut csv_parser = CoreBuilder::new().from_buffer(haystack);

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
    }

    #[test]
    fn test_0001_06_singleline() {
        let haystack = b"a,1|b11\r\n";
        let mut csv_parser = CoreBuilder::new().col_sep(b'|').from_buffer(haystack);

        let (csv_type, col) = csv_parser.next();
        assert!(matches!(csv_type, FieldResult::Field));
        assert_eq!(col, "a,1");

        let (csv_type, col) = csv_parser.next();
        assert!(matches!(csv_type, FieldResult::FieldEnd));
        assert_eq!(col,"b11");

        let (csv_type, _) = csv_parser.next();
        assert!(matches!(csv_type, FieldResult::End));
    }

    #[test]
    fn test_0001_07_singleline() {
        let haystack = b"a1,\"b1,1\",c111\r\n";
        let mut csv_parser = CoreBuilder::new().col_sep(b',').from_buffer(haystack);

        csv_parser.skip(1);

        let (csv_type, col) = csv_parser.next();
        assert!(matches!(csv_type, FieldResult::Field));
        println!("{}", col);
        assert_eq!(col,"b1,1");

        let (csv_type, col) = csv_parser.next();
        assert!(matches!(csv_type, FieldResult::FieldEnd));
        assert_eq!(col,"c111");

        let (csv_type, _) = csv_parser.next();
        assert!(matches!(csv_type, FieldResult::End));
    }

    #[test]
    fn test_0001_08_singleline() {
        let haystack = b"a1,,c111\n";
        let mut csv_parser = CoreBuilder::new().from_buffer(haystack);

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
    fn test_0001_09_singleline() {
        // "a1", "", "", "d111"
        let haystack = br#""a1","","","d111""#;
        let mut csv_parser = CoreBuilder::new().from_buffer(haystack);

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
        println!("col : {}", col);
        assert_eq!(col,"d111");

        let (csv_type, _) = csv_parser.next();
        assert!(matches!(csv_type, FieldResult::End));
    }

    #[test]
    fn test_0001_10_singleline() {
        // ""a1,b"1,c11"""  =>  "a
        let haystack = br#"""a1,b"1,c1"""#;
        let mut csv_parser = CoreBuilder::new().from_buffer(haystack);
        
        let (csv_type, col) = csv_parser.next();
        assert!(matches!(csv_type, FieldResult::Field));
        println!("{}", col);
        assert_eq!(col, "\"a1,b\"1");

        let (csv_type, col) = csv_parser.next();
        assert!(matches!(csv_type, FieldResult::FieldEndWithQQ));
        println!("{}", col);
        assert_eq!(col,"c1\"\"");

        let (csv_type, _) = csv_parser.next();
        println!("{}", csv_type);
        assert!(matches!(csv_type, FieldResult::End));
        println!("End");
    }

    #[test]
    fn test_0001_11_singleline() {
        // ""a1,b"1,c11"""
        let haystack = br#"a1,b"b1"1,c""c1""1,d1""#;
        let mut csv_parser = CoreBuilder::new().from_buffer(haystack);
        
        let (csv_type, col) = csv_parser.next();
        assert!(matches!(csv_type, FieldResult::Field));
        assert_eq!(col, "a1");

        let (csv_type, col) = csv_parser.next();
        assert!(matches!(csv_type, FieldResult::Field));
        println!("{}", col);
        assert_eq!(col, "b\"b1\"1");

        let (csv_type, col) = csv_parser.next();
        assert!(matches!(csv_type, FieldResult::FieldWithQQ));
        println!("{}", col);
        assert_eq!(col, "c\"\"c1\"\"1");

        let (csv_type, col) = csv_parser.next();
        assert!(matches!(csv_type, FieldResult::FieldEnd));
        println!("{}", col);
        assert_eq!(col,"d1");

        let (csv_type, _) = csv_parser.next();
        assert!(matches!(csv_type, FieldResult::End));
    }

    #[test]
    fn test_0001_12_singleline() {
        // """a1""",""b1"","""c1""",""""d1""""
        let haystack = br#""""a1""",""b1"",c1"#;
        let mut csv_parser = CoreBuilder::new().from_buffer(haystack);
        
        let (mut csv_type, mut col) = csv_parser.next();
        assert!(matches!(csv_type, FieldResult::FieldWithQQ));
        assert_eq!(col, "\"\"a1\"\"");

        while matches!(csv_type, FieldResult::Field) || matches!(csv_type, FieldResult::FieldWithQQ){
            (csv_type, col) = csv_parser.next();
            // no rule. just check no panic
            println!("{}", csv_type);
            println!("{}", col);
        }
        (csv_type, col) = csv_parser.next();
        println!("{}", csv_type);
        println!("{}", col);
    }

    #[test]
    fn test_0001_13_singleline() {
        // """a1""","b""b1""1","""c1""","""""d1""""", """e1"""
        let haystack = br#""""a1""","b""b1""1","""c1""","""""d1""""","""e1""""#;
        let mut csv_parser = CoreBuilder::new().from_buffer(haystack);
        
        let (csv_type, col) = csv_parser.next();
        assert!(matches!(csv_type, FieldResult::FieldWithQQ));
        assert_eq!(col, "\"\"a1\"\"");

        let (csv_type, col) = csv_parser.next();
        assert!(matches!(csv_type, FieldResult::FieldWithQQ));
        assert_eq!(col, "b\"\"b1\"\"1");

        let (csv_type, col) = csv_parser.next();
        assert!(matches!(csv_type, FieldResult::FieldWithQQ));
        assert_eq!(col, "\"\"c1\"\"");

        let (csv_type, col) = csv_parser.next();
        println!("{}", col);
        assert!(matches!(csv_type, FieldResult::FieldWithQQ));
        assert_eq!(col, "\"\"\"\"d1\"\"\"\"");

        let (csv_type, col) = csv_parser.next();
        println!("{}", col);
        assert!(matches!(csv_type, FieldResult::FieldEndWithQQ));
        assert_eq!(col, "\"\"e1\"\"");

        let (csv_type, _) = csv_parser.next();
        assert!(matches!(csv_type, FieldResult::End));
    }

    #[test]
    fn test_0002_01_dualline() {
        println!("==== 01 ====");
        let haystack = b"a1\nb2";
        let mut csv_parser = CoreBuilder::new().from_buffer(haystack);

        let (csv_type, col) = csv_parser.next();
        assert!(matches!(csv_type, FieldResult::FieldEnd));
        assert_eq!(col, "a1");

        let (csv_type, col) = csv_parser.next();
        assert!(matches!(csv_type, FieldResult::FieldEnd));
        assert_eq!(col,"b2");

        let (csv_type, _) = csv_parser.next();
        assert!(matches!(csv_type, FieldResult::End));
    }

    #[test]
    fn test_0002_02_dualline() {
        let haystack = b"a1\nb2\n";
        let mut csv_parser = CoreBuilder::new().from_buffer(haystack);

        let (csv_type, col) = csv_parser.next();
        assert!(matches!(csv_type, FieldResult::FieldEnd));
        assert_eq!(col, "a1");

        let (csv_type, col) = csv_parser.next();
        assert!(matches!(csv_type, FieldResult::FieldEnd));
        assert_eq!(col,"b2");

        let (csv_type, _) = csv_parser.next();
        assert!(matches!(csv_type, FieldResult::End));
    }

    #[test]
    fn test_0002_03_dualline() {
        let haystack = b"a1\rb2\r";
        let mut csv_parser = CoreBuilder::new().from_buffer(haystack);

        let (csv_type, col) = csv_parser.next();
        assert!(matches!(csv_type, FieldResult::FieldEnd));
        assert_eq!(col, "a1");

        let (csv_type, col) = csv_parser.next();
        assert!(matches!(csv_type, FieldResult::FieldEnd));
        assert_eq!(col,"b2");

        let (csv_type, _) = csv_parser.next();
        assert!(matches!(csv_type, FieldResult::End));
    }

    #[test]
    fn test_0002_04_dualline() {
        let haystack = b"a1\r\nb2\r\n";
        let mut csv_parser = CoreBuilder::new().from_buffer(haystack);

        let (csv_type, col) = csv_parser.next();
        assert!(matches!(csv_type, FieldResult::FieldEnd));
        assert_eq!(col, "a1");

        let (csv_type, col) = csv_parser.next();
        assert!(matches!(csv_type, FieldResult::FieldEnd));
        assert_eq!(col,"b2");

        let (csv_type, _) = csv_parser.next();
        assert!(matches!(csv_type, FieldResult::End));
    }

    #[test]
    fn test_0002_05_dualline() {
        let haystack = b"\"a1\n111\"\nb222";
        let mut csv_parser = CoreBuilder::new().from_buffer(haystack);

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
    fn test_0002_06_dualline() {
        let haystack = b"a1,b11\na2,b22";
        let mut csv_parser = CoreBuilder::new().from_buffer(haystack);

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
    fn test_0002_07_dualline() {
        let haystack = b"a1,b11\na2,b22\n";
        let mut csv_parser = CoreBuilder::new().from_buffer(haystack);

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
    fn test_0003_01_multiline() {
        println!("==== 01 ====");
        let haystack = br#"a1,b11,c111
a2,"b22"
"a33",b""b3""3,c""c3""3
a4,b4,,,"#;
        let mut csv_parser = CoreBuilder::new().from_buffer(haystack);
        let (csv_type, col) = csv_parser.next();
        assert!(matches!(csv_type, FieldResult::Field));
        assert_eq!(col, "a1");

        let (csv_type, col) = csv_parser.next();
        assert!(matches!(csv_type, FieldResult::Field));
        assert_eq!(col, "b11");

        let (csv_type, col) = csv_parser.next();
        assert!(matches!(csv_type, FieldResult::FieldEnd));
        assert_eq!(col, "c111");

        let (csv_type, col) = csv_parser.next();
        assert!(matches!(csv_type, FieldResult::Field));
        assert_eq!(col, "a2");

        let (csv_type, col) = csv_parser.next();
        assert!(matches!(csv_type, FieldResult::FieldEnd));
        assert_eq!(col, "b22");

        let (csv_type, col) = csv_parser.next();
        assert!(matches!(csv_type, FieldResult::Field));
        assert_eq!(col, "a33");

        let (csv_type, col) = csv_parser.next();
        assert!(matches!(csv_type, FieldResult::FieldWithQQ));
        assert_eq!(col, "b\"\"b3\"\"3");

        let (csv_type, col) = csv_parser.next();
        assert!(matches!(csv_type, FieldResult::FieldEndWithQQ));
        assert_eq!(col, "c\"\"c3\"\"3");
    }

    #[test]
    fn test_0004_01_parse_csv() {
        let haystack = HAYSTACK_GDP_CSV;
        let mut csv_parser = CoreBuilder::new().from_buffer(haystack);

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

    #[test]
    fn test_0003_02_parse_csv() {
        let haystack = HAYSTACK_CSV_01;
        let mut csv_parser = CoreBuilder::new().from_buffer(haystack);

        let mut count_fields = 0;
        let mut count_records = 0;
        loop{
            let (csv_type, _) = csv_parser.next();    
            match csv_type{
                FieldResult::Field => {
                    count_fields += 1;
                },
                FieldResult::FieldEnd =>{
                    count_fields += 1;
                    count_records += 1;
                },
                _ =>{
                    break;
                }
            }
        }
        assert_eq!(count_records, 5);
        assert_eq!(count_fields, 10);
    }
}