pub mod ss_csv;
use std::io::Cursor;

#[doc(hidden)]
pub fn test_ss_csv() {

    println!("======== Start lib.rs  ========");
    let haystack = r#"a1,b11,c111
a2,b22
"a33",bb33
a4,b4,,,"#;
    let mut csv_parser = ss_csv::core_reader::CoreBuilder::new().from_buffer(haystack.as_bytes());
    let (csv_type, col) = csv_parser.next();
    assert!(matches!(csv_type, ss_csv::FieldResult::Field));
    assert_eq!(col, "a1");

    let (csv_type, col) = csv_parser.next();
    assert!(matches!(csv_type, ss_csv::FieldResult::Field));
    assert_eq!(col, "b11");

    let (csv_type, col) = csv_parser.next();
    assert!(matches!(csv_type, ss_csv::FieldResult::FieldEnd));
    assert_eq!(col, "c111");

    let (csv_type, col) = csv_parser.next();
    assert!(matches!(csv_type, ss_csv::FieldResult::Field));
    assert_eq!(col, "a2");

    let (csv_type, col) = csv_parser.next();
    assert!(matches!(csv_type, ss_csv::FieldResult::FieldEnd));
    assert_eq!(col, "b22");

    let (csv_type, col) = csv_parser.next();
    assert!(matches!(csv_type, ss_csv::FieldResult::FieldEnd));
    assert_eq!(col, "c222");

    let (csv_type, col) = csv_parser.next();
    assert!(matches!(csv_type, ss_csv::FieldResult::Field));
    assert_eq!(col, "a3");

    let (csv_type, col) = csv_parser.next();
    assert!(matches!(csv_type, ss_csv::FieldResult::FieldEnd));
    assert_eq!(col, "b33");

    

    println!("======== End lib.rs ========");
}
