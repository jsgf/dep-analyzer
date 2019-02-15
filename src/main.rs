#![warn(clippy::all)]
#![feature(arbitrary_self_types)]

use std::fs;

use clap::clap_app;

mod kratedb;
mod rls_data;
mod context;

fn main() -> Result<(), std::io::Error> {
    let matches = clap_app! {sa_parser =>
        (@arg FILES: +takes_value +required ... "Path to json save-analysis files")
    }
    .get_matches();

    let files: Vec<_> = matches.values_of("FILES").into_iter().flatten().collect();

    let mut db = kratedb::KrateDb::new();

    for file in files {
        let json = fs::read_to_string(file)?;

        let sa: rls_data::Analysis = serde_json::from_str(&json)?;

        println!("{:#?}", sa);

        db.ingest_krate(sa);
    }

    Ok(())
}
