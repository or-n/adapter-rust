use std::process::Stdio;

pub fn save(
    input_file: &str,
    output_file: &str,
    frame_count: u32,
    ranges: [range::MinSize<u32>; 2],
) -> std::io::Result<std::process::ExitStatus> {
    let output_format =
        format!("{}_%0{}d.jpg", output_file, frame_count.ilog10() + 1);
    let crop_filter = format!(
        "crop={}:{}:{}:{}",
        ranges[0].size, ranges[1].size, ranges[0].min, ranges[1].min
    );
    std::process::Command::new("ffmpeg")
        .args(&["-i", input_file])
        .args(&["-frames:v", &frame_count.to_string()])
        .args(&["-q:v", "2"])
        .args(&["-vf", &crop_filter])
        .args(&[&output_format])
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
}
