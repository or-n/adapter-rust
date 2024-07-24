use std::process::Stdio;

pub fn save(
    input_file: &str,
    output_file: &str,
    fps: &str,
    frame_count: u32,
    clip: Option<&super::clip::Clip>,
) -> std::io::Result<std::process::ExitStatus> {
    let input_format =
        format!("{}_%0{}d.jpg", input_file, frame_count.ilog10() + 1);
    let mut command = std::process::Command::new("ffmpeg");
    command
        .args(&["-y"])
        .args(&["-framerate", fps])
        .args(&["-i", &input_format])
        .args(&["-c:v", "libx264"])
        .args(&["-pix_fmt", "yuv420p"]);
    if let Some(clip) = clip {
        command
            .args(&["-start_number", &clip.start.to_string()])
            .args(&["-frames:v", &clip.length.to_string()]);
    }
    command
        .args(&[&output_file])
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
}
