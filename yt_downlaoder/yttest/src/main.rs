fn main() {
    let url = "https://youtu.be/nv2wQvn6Wxc";
    let _path_to_video = rustube::blocking::download_best_quality(url);
}
