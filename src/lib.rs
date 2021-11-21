pub mod ss_csv;
use std::io::Cursor;

#[doc(hidden)]
pub fn test_ss_csv() {
    println!("======== Start lib.rs  ========");
    let haystack = "a1,b11,c111\n\
        a2,b22,c222\n\
        a3,b33,c333\n\
        a4,b4,,,";
    let mut csv_parser = ss_csv::csv_reader::CSVBuilder::new().from_read(haystack.as_bytes());
    let (csv_type, col) = csv_parser.next();
    assert!(matches!(csv_type, ss_csv::FieldResult::Field));

    // assert_eq!(col, 3);

    // let (csv_type, col) = csv_parser.next();
    // assert_eq!(csv_type, FieldResult::Field);
    // assert_eq!(col, 7);

    // let (csv_type, col) = csv_parser.next();
    // assert_eq!(csv_type, FieldResult::Field);
    // assert_eq!(col, 12);

    println!("======== End lib.rs ========");
}
