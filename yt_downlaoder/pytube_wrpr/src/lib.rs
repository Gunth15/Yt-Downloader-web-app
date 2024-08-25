use pyo3::prelude::*;
use std::fmt;
use std::vec::Vec;

pub fn download_n_fetch(url: &str, dir: &str) -> Result<Vec<String>, Error> {
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
    let result: Result<Vec<String>, PyErr> = Python::with_gil(|py| {
        let download_and_fetch =
            PyModule::from_code_bound(py, &pscript, "DlnFetch.py", "DlnFetch").unwrap();
        download_and_fetch
            .getattr("download_n_fetchmeta")
            .expect("Fatal error, could not find function")
            .call1((url, dir))?
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
impl Error {
    fn err_response(&self) -> String {
        match self {
            Error::MissingScript(msg) => msg.to_string(),
            Error::PytubeErr(msg) => msg.to_string(),
        }
    }
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", self.err_response())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::fs;

    #[test]
    //TODO: test does not work rn: need to change the input dir
    fn test_dwnld() {
        let dir = env::var("CARGO_MANIFEST_DIR").expect("No such directory");

        let mut query =
            download_n_fetch("https://www.youtube.com/watch?v=dQw4w9WgXcQ", &dir).unwrap();

        let file_name = query.remove(1);
        println!("{file_name}");
        fs::read(format!("./downloads/{file_name}")).expect("No such file");

        assert_eq!(
            query,
            vec!["https://i.ytimg.com/vi/dQw4w9WgXcQ/sddefault.jpg",]
        );
    }
}
