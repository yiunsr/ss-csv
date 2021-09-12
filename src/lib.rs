pub mod ss_csv;
use crate::ss_csv::csv_reader::FieldResult;

#[doc(hidden)]
pub fn test_ss_csv() {
    println!("======== Start test_ss_csv ========");
    let haystack = b"a1,b11,c111\n\
        a2,b22,c222\n\
        a3,b33,c333\n\
        a4,b4,,,";
    let mut haystack = haystack.to_vec();
    let mut csv_parser = ss_csv::csv_reader::CSV::new(&mut haystack);
    let (csv_type, col) = csv_parser.next();
    assert!(matches!(csv_type, FieldResult::Field));
    // assert_eq!(col, 3);

    // let (csv_type, col) = csv_parser.next();
    // assert_eq!(csv_type, FieldResult::Field);
    // assert_eq!(col, 7);

    // let (csv_type, col) = csv_parser.next();
    // assert_eq!(csv_type, FieldResult::Field);
    // assert_eq!(col, 12);
}
