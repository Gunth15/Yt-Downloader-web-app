use pyo3::prelude::*;
use std::env;
use std::fs;

fn main() {
    let dir = env::var("CARGO_MANIFEST_DIR").expect("No such directory");

    let pscript = fs::read_to_string(format!("{dir}/src/DlnFetch.py")).unwrap();
    let dwnld_folder_path = format!("{dir}/download/");
    let mut query: std::vec::Vec<String> = Python::with_gil(|py| {
        let download_and_fetch =
            PyModule::from_code_bound(py, &pscript, "DlnFetch.py", "DlnFetch").unwrap();
        download_and_fetch
            .getattr("download_n_fetchmeta")
            .unwrap()
            .call1(("http://youtube.com/watch?v=2lAe1cqCOXo", dwnld_folder_path))
            .expect("Fatal error")
            .extract()
            .unwrap()
    });
    println!("{},{}", &query.pop().unwrap(), &query.pop().unwrap());
}
