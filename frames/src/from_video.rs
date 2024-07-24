use std::process::Stdio;

pub fn save(
    input_file: &str,
    output_file: &str,
    frame_count: u32,
) -> std::io::Result<std::process::ExitStatus> {
    let output_format =
        format!("{}_%0{}d.jpg", output_file, frame_count.ilog10() + 1);
    std::process::Command::new("ffmpeg")
        .args(&["-i", input_file])
        .args(&["-frames:v", &frame_count.to_string()])
        .args(&[&output_format])
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
}
