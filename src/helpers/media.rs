use symphonia::core::codecs::CODEC_TYPE_NULL;
use symphonia::core::formats::FormatOptions;
use symphonia::core::io::MediaSourceStream;
use symphonia::core::meta::MetadataOptions;
use symphonia::core::probe::Hint;
use std::path::Path;

#[derive(Debug, Clone, PartialEq)]
pub struct MediaInfo {
    pub sample_rate: Option<u32>,   // Hz
    pub bit_rate: u32,      // kbps
    pub channels: Option<u8>,
    pub duration_ms: Option<u64>,   // milliseconds
    pub codec: Option<String>,
}

pub fn get_audio_info(path: &Path) -> Option<MediaInfo> {
    let file = std::fs::File::open(path).ok()?;
    let mss = MediaSourceStream::new(Box::new(file), Default::default());

    let mut hint = Hint::new();
    if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
        hint.with_extension(ext);
    }

    let probed = symphonia::default::get_probe()
        .format(&hint, mss, &FormatOptions::default(), &MetadataOptions::default())
        .ok()?;

    let format = probed.format;

    let track = format
        .tracks()
        .iter()
        .find(|t| t.codec_params.codec != CODEC_TYPE_NULL)?;

    let params = &track.codec_params;

    let duration_ms = params.n_frames
        .zip(params.sample_rate)
        .map(|(frames, rate)| (frames * 1000) / rate as u64);

    let channels = params.channels.map(|c| c.count() as u8);

    // bit_rate từ container (kbps)
    let bit_rate = params.bits_per_coded_sample
        .map(|b| b).unwrap_or(params.bits_per_sample.map(|b| b).unwrap_or(0));

    Some(MediaInfo {
        sample_rate: params.sample_rate,
        bit_rate,
        channels,
        duration_ms,
        codec: Some(format!("{:?}", params.codec)),
    })
}
pub fn get_command(input_file: &str,output_type: &str,output_bit_rate: u32) -> String {
    format!("ffmpeg -i {} -ac 1 -c:a lib{} -b:a {}k -vbr on -compression_level 0 -application voip {}", input_file, output_type, output_bit_rate, input_file.replace(".mp3", ".opus"))
}