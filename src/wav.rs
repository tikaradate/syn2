use std::env;
use std::fs;
use std::fmt;
use std::io;

#[derive(Debug, Clone)]
pub struct DataFormat {
    format_block: [u8; 4],
    block_size: u32,
    audio_format: u16,
    num_channels: u16,
    sample_rate: u32,
    bytes_per_sec: u32,
    block_align: u16,
    bits_per_sample: u16,
}

#[derive(Debug, Clone)]
pub struct SampledData {
    data_block_id: [u8; 4],
    data_size: u32,
    data: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct Wav {
    format_chunk: DataFormat,
    data_chunk: SampledData,
}

#[derive(Debug)]
pub enum WavError {
    Io(io::Error),
    BadHeader(&'static str),
    MissingChunk(&'static str),
    Truncated,
    Invalid(&'static str),
    UnsupportedFormat {
        audio_format: u16,
        bits_per_sample: u16,
        channels: u16,
    },
}

impl Wav {
    pub fn format(&self) -> &DataFormat {
        &self.format_chunk
    }

    pub fn data_chunk(&self) -> &SampledData {
        &self.data_chunk
    }

    pub fn data_bytes(&self) -> &[u8] {
        &self.data_chunk.data
    }
}

impl SampledData {
    pub fn len_bytes(&self) -> usize {
        self.data.len()
    }
}


impl From<io::Error> for WavError {
    fn from(e: io::Error) -> Self {
        WavError::Io(e)
    }
}

impl fmt::Display for WavError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WavError::Io(e) => write!(f, "I/O error: {e}"),
            WavError::BadHeader(msg) => write!(f, "bad WAV header: {msg}"),
            WavError::MissingChunk(id) => write!(f, "missing required chunk: {id}"),
            WavError::Truncated => write!(f, "file is truncated"),
            WavError::Invalid(msg) => write!(f, "invalid WAV: {msg}"),
            WavError::UnsupportedFormat { audio_format, bits_per_sample, channels } => {
                write!(
                    f,
                    "unsupported WAV format: audio_format={audio_format}, bits_per_sample={bits_per_sample}, channels={channels}"
                )
            }
        }
    }
}

impl std::error::Error for WavError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            WavError::Io(e) => Some(e),
            _ => None,
        }
    }
}

fn u16_le(x: &[u8]) -> Result<u16, WavError> {
    let a: [u8; 2] = x.try_into().map_err(|_| WavError::Truncated)?;
    Ok(u16::from_le_bytes(a))
}

fn u32_le(x: &[u8]) -> Result<u32, WavError> {
    let a: [u8; 4] = x.try_into().map_err(|_| WavError::Truncated)?;
    Ok(u32::from_le_bytes(a))
}

fn read_bytes<'a>(buf: &'a [u8], len: usize, offset: &mut usize) -> Result<&'a [u8], WavError> {
    let start = *offset;
    let end = start.checked_add(len).ok_or(WavError::Truncated)?;
    if end > buf.len() {
        return Err(WavError::Truncated);
    }
    *offset = end;
    Ok(&buf[start..end])
}

fn read_fourcc(buf: &[u8], fp: &mut usize) -> Result<[u8; 4], WavError> {
    read_bytes(buf, 4, fp)?
        .try_into()
        .map_err(|_| WavError::Truncated)
}

fn skip_bytes(buf: &[u8], n: usize, fp: &mut usize) -> Result<(), WavError> {
    let _ = read_bytes(buf, n, fp)?;
    Ok(())
}

fn parse_fmt_payload(buf: &[u8], fp: &mut usize, block_size: u32) -> Result<DataFormat, WavError> {
    if block_size < 16 {
        return Err(WavError::BadHeader("fmt chunk too small"));
    }

    let audio_format    = u16_le(read_bytes(buf, 2, fp)?)?;
    let num_channels    = u16_le(read_bytes(buf, 2, fp)?)?;
    let sample_rate     = u32_le(read_bytes(buf, 4, fp)?)?;
    let bytes_per_sec   = u32_le(read_bytes(buf, 4, fp)?)?;
    let block_align     = u16_le(read_bytes(buf, 2, fp)?)?;
    let bits_per_sample = u16_le(read_bytes(buf, 2, fp)?)?;

    // skip optional extra fmt bytes (if any)
    if block_size > 16 {
        let extra_u32 = block_size - 16;
        let extra = usize::try_from(extra_u32).map_err(|_| WavError::Invalid("fmt chunk too large"))?;
        skip_bytes(buf, extra, fp)?;
    }

    Ok(DataFormat {
        format_block: *b"fmt ",
        block_size,
        audio_format,
        num_channels,
        sample_rate,
        bytes_per_sec,
        block_align,
        bits_per_sample,
    })
}

fn parse_data_payload(buf: &[u8], fp: &mut usize, data_size: u32) -> Result<SampledData, WavError> {
    let n = usize::try_from(data_size).map_err(|_| WavError::Invalid("data chunk too large"))?;
    let data = read_bytes(buf, n, fp)?.to_vec();
    Ok(SampledData {
        data_block_id: *b"data",
        data_size,
        data,
    })
}

pub fn read_raw_wav(file_path: &String) -> Result<Wav, WavError> {
    let bytes = fs::read(file_path)?;
    let mut fp: usize = 0;

    let riff = read_fourcc(&bytes, &mut fp)?;
    if riff != *b"RIFF" {
        return Err(WavError::BadHeader("RIFF missing"));
    }

    let _riff_size = u32_le(read_bytes(&bytes, 4, &mut fp)?)?; // consumed
    let wave = read_fourcc(&bytes, &mut fp)?;
    if wave != *b"WAVE" {
        return Err(WavError::BadHeader("WAVE missing"));
    }

    let mut fmt: Option<DataFormat> = None;
    let mut data: Option<SampledData> = None;

    // scan chunks until we find both fmt and data.
    while bytes.len().saturating_sub(fp) >= 8 {
        let chunk_id = read_fourcc(&bytes, &mut fp)?;
        let chunk_size = u32_le(read_bytes(&bytes, 4, &mut fp)?)?;
        let size_usize = usize::try_from(chunk_size).map_err(|_| WavError::Invalid("chunk too large"))?;
        let payload_start = fp;

        match &chunk_id {
            b"fmt " => {
                if fmt.is_some() {
                    return Err(WavError::Invalid("duplicate fmt chunk"));
                }
                let parsed = parse_fmt_payload(&bytes, &mut fp, chunk_size)?;
                fmt = Some(parsed);
            }
            b"data" => {
                if data.is_some() {
                    return Err(WavError::Invalid("duplicate data chunk"));
                }
                let parsed = parse_data_payload(&bytes, &mut fp, chunk_size)?;
                data = Some(parsed);
            }
            _ => {
                skip_bytes(&bytes, size_usize, &mut fp)?;
            }
        }

        // If the parser didn’t consume exactly chunk_size bytes (shouldn’t happen), fix/validate here.
        let consumed = fp - payload_start;
        if consumed < size_usize {
            skip_bytes(&bytes, size_usize - consumed, &mut fp)?;
        } else if consumed > size_usize {
            return Err(WavError::Invalid("overread chunk payload"));
        }

        // pad byte if payload size is odd.
        if chunk_size % 2 == 1 {
            skip_bytes(&bytes, 1, &mut fp)?;
        }

        if fmt.is_some() && data.is_some() {
            break;
        }
    }

    let fmt = fmt.ok_or(WavError::MissingChunk("fmt "))?;
    let data = data.ok_or(WavError::MissingChunk("data"))?;

    if fmt.audio_format != 1 || fmt.bits_per_sample != 16 {
        return Err(WavError::UnsupportedFormat {
            audio_format: fmt.audio_format,
            bits_per_sample: fmt.bits_per_sample,
            channels: fmt.num_channels,
        });
    }

    Ok(Wav {
        format_chunk: fmt,
        data_chunk: data,
    })
}
