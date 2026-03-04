use std::path::Path;
use symphonia::core::codecs::CODEC_TYPE_NULL;
use symphonia::core::formats::FormatOptions;
use symphonia::core::io::MediaSourceStream;
use symphonia::core::meta::MetadataOptions;
use symphonia::core::probe::Hint;

#[derive(Debug, Clone, PartialEq)]
pub struct MediaInfo {
    pub sample_rate: Option<u32>, // Hz
    pub bit_rate: Option<u32>,    // kbps
    pub channels: Option<u8>,
    pub duration_ms: Option<u64>, // milliseconds
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
        .format(
            &hint,
            mss,
            &FormatOptions::default(),
            &MetadataOptions::default(),
        )
        .ok()?;

    let format = probed.format;

    let track = format
        .tracks()
        .iter()
        .find(|t| t.codec_params.codec != CODEC_TYPE_NULL)?;

    let params = &track.codec_params;

    let duration_ms = params
        .n_frames
        .zip(params.sample_rate)
        .map(|(frames, rate)| (frames * 1000) / rate as u64);

    let channels = params.channels.map(|c| c.count() as u8);

    let bit_rate: Option<u32> = params.bits_per_sample.map(|b| b);

    Some(MediaInfo {
        sample_rate: params.sample_rate,
        bit_rate,
        channels,
        duration_ms,
        codec: Some(format!("{:?}", params.codec)),
    })
}