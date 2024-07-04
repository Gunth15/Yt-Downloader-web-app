fn main() {
    let url = "https://www.youtube.com/watch?v=dQw4w9WgXcQ";
    let _path_to_video = rustube::blocking::download_best_quality(url).unwrap();
}
