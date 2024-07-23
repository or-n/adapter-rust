#[derive(Debug)]
pub enum Error {
    Command(std::io::Error),
    Utf8(std::str::Utf8Error),
    Count(std::num::ParseIntError),
}

pub fn extract(input_file: &str) -> Result<u32, Error> {
    let output = std::process::Command::new("ffprobe")
        .args(&["-v", "error"])
        .args(&["-select_streams", "v:0"])
        .args(&["-show_entries", "stream=nb_frames"])
        .args(&["-of", "default=noprint_wrappers=1:nokey=1"])
        .args(&[input_file])
        .output()
        .map_err(Error::Command)?;
    let text = std::str::from_utf8(&output.stdout)
        .map_err(Error::Utf8)?
        .trim();
    Ok(text.parse().map_err(Error::Count)?)
}
