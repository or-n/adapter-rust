#[derive(Debug)]
pub struct FPS {
    pub numerator: u32,
    pub denumerator: u32,
}

#[derive(Debug)]
pub enum Error {
    Command(std::io::Error),
    Utf8(std::str::Utf8Error),
    Parts,
    Numerator(std::num::ParseIntError),
    Denumerator(std::num::ParseIntError),
}

pub fn extract(input_file: &str) -> Result<FPS, Error> {
    let output = std::process::Command::new("ffprobe")
        .args(&["-v", "error"])
        .args(&["-select_streams", "v:0"])
        .args(&["-show_entries", "stream=r_frame_rate"])
        .args(&["-of", "default=noprint_wrappers=1:nokey=1"])
        .args(&[input_file])
        .output()
        .map_err(Error::Command)?;
    let text = std::str::from_utf8(&output.stdout)
        .map_err(Error::Utf8)?
        .trim();
    let parts: Vec<&str> = text.split('/').collect();
    if parts.len() == 2 {
        Ok(FPS {
            numerator: parts[0].parse().map_err(Error::Numerator)?,
            denumerator: parts[1].parse().map_err(Error::Denumerator)?,
        })
    } else {
        Err(Error::Parts)
    }
}
