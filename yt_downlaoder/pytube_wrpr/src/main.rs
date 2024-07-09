use pyo3::prelude::*;
use std::fs;

fn main() {
    let pscript = fs::read_to_string(
        "~/Summer_Projects/Yt-Downloader-web-app/yt_downlaoder/pytube_wrpr/src/DlnFetch.py",
    )
    .unwrap();
    Python::with_gil(|py| {
        let download_and_fetch =
            PyModule::from_code_bound(py, &pscript, "DlnFetch.py", "DlnFetch").unwrap();
        download_and_fetch
            .getattr("download_n_fetchmeta")
            .unwrap()
            .call1(("https://www.youtube.com/watch?v=dQw4w9WgXcQ",));
    })
}
