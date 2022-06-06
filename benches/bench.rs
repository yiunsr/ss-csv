#[macro_use]
extern crate bencher;

use ss_csv::ss_csv::CoreBuilder;
use ss_csv::ss_csv::FieldResult;
use bencher::Bencher;
use csv_core::{Reader, ReadFieldResult};


static HAYSTACK_GDP_CSV: &'static [u8] = include_bytes!("../data/test/gdp_org.csv");

fn parse_gdp_csv(bench: &mut Bencher) {
    bench.iter(|| {
        let mut csv_parser = CoreBuilder::new().from_buffer(HAYSTACK_GDP_CSV);
        let mut csv_type = FieldResult::Field;
        let mut count_fields = 0;
        let mut count_records = 0;
        loop{
            let (csv_type_, col) = csv_parser.next();    
            match csv_type{
                FieldResult::Field => {
                    count_fields += 1;
                },
                FieldResult::FieldWithQQ => {
                    count_fields += 1;
                    let col_string = col.to_string();
                    let _ = col_string.replace("\"\"", "\"");
                },
                FieldResult::FieldEnd  =>{
                    count_fields += 1;
                    count_records += 1;
                },
                FieldResult::FieldEndWithQQ => {
                    count_fields += 1;
                    count_records += 1;
                    let col_string = col.to_string();
                    let _ = col_string.replace("\"\"", "\"");
                },
                _ =>{
                    break;
                }
            }
            csv_type = csv_type_;
        }
        println!("row : {}, col : {}", count_records, count_fields);
    });
}


benchmark_group!(benches, parse_gdp_csv);
benchmark_main!(benches);