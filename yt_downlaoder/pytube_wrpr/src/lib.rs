use pyo3::prelude::*;
use std::env;
use std::vec::Vec;

pub fn download_n_fetch(url: &str) -> Result<Vec<String>, Error> {
    let dir = env::var("CARGO_MANIFEST_DIR").expect("No such directory");

    let pscript = r#"
from pytubefix import YouTube
#Fetches meta data of video and downloads video
def download_n_fetchmeta (url,dir):
    query = []
    yt = YouTube(url)
    query.append(yt.thumbnail_url)
    query.append(yt.streams.first().default_filename)
    yt.streams.first().download(output_path=dir)
    return query
    "#;
    let dwnld_folder_path = format!("{dir}/downloads/");
    let result: Result<Vec<String>, PyErr> = Python::with_gil(|py| {
        let download_and_fetch =
            PyModule::from_code_bound(py, &pscript, "DlnFetch.py", "DlnFetch").unwrap();
        download_and_fetch
            .getattr("download_n_fetchmeta")
            .expect("Fatal error, could not find function")
            .call1((url, dwnld_folder_path))?
            .extract()
    });

    result.map_err(|_err| {
        Error::PytubeErr(
            "Url is invalid/private or youtube changed their api again. Try runinng python script."
                .to_string(),
        )
    })
}

#[derive(Debug)]
pub enum Error {
    MissingScript(String),
    PytubeErr(String),
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    #[test]
    fn test_dwnld() {
        let mut query = download_n_fetch("https://www.youtube.com/watch?v=dQw4w9WgXcQ").unwrap();

        let file_name = query.remove(1);
        println!("{file_name}");
        fs::read(format!("./downloads/{file_name}")).expect("No such file");

        assert_eq!(
            query,
            vec!["https://i.ytimg.com/vi/dQw4w9WgXcQ/sddefault.jpg",]
        );
    }
}
