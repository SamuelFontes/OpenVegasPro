use ffmpeg_next as ffmpeg;
use ffmpeg::{format, media, software::scaling, util::frame};
use ffmpeg::codec::decoder::Video as VideoDecoder;
use std::path::Path;

pub struct MediaEngine {
    input: format::context::Input,
    video_stream_index: usize,
    decoder: VideoDecoder,
    scaler: scaling::Context,
    width: u32,
    height: u32,
    time_base: f64,
    last_frame_time: Option<f64>,
}

impl MediaEngine {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self, ffmpeg::Error> {
        ffmpeg::init()?;

        let input = format::input(&path)?;
        
        // Find best video stream
        let stream = input.streams()
            .best(media::Type::Video)
            .ok_or(ffmpeg::Error::StreamNotFound)?;
            
        let video_stream_index = stream.index();
        
        // Setup decoder
        let context = ffmpeg::codec::context::Context::from_parameters(stream.parameters())?;
        let decoder = context.decoder().video()?;
        
        let width = decoder.width();
        let height = decoder.height();
        let time_base = f64::from(stream.time_base());

        // Setup SWScale context to convert everything to RGBA
        let scaler = scaling::Context::get(
            decoder.format(),
            width,
            height,
            format::Pixel::RGBA,
            width,
            height,
            scaling::flag::Flags::BILINEAR,
        )?;

        Ok(Self {
            input,
            video_stream_index,
            decoder,
            scaler,
            width,
            height,
            time_base,
            last_frame_time: None,
        })
    }

    pub fn width(&self) -> u32 { self.width }
    pub fn height(&self) -> u32 { self.height }

    /// Return the media duration in seconds if known.
    pub fn duration_secs(&self) -> Option<f64> {
        let dur = self.input.duration();
        if dur > 0 && dur != ffmpeg::ffi::AV_NOPTS_VALUE {
            // ffmpeg input duration is in AV_TIME_BASE fractional seconds (typically microseconds)
            Some(dur as f64 / ffmpeg::ffi::AV_TIME_BASE as f64)
        } else {
            None
        }
    }

    /// Seeks to a specific timestamp in seconds and decodes the nearest frame.
    pub fn get_frame(&mut self, target_time_secs: f64) -> Option<Vec<u8>> {
        let should_seek = match self.last_frame_time {
            Some(last_time) => {
                // Seek if we jump backwards, or jump forward by more than 0.5s (e.g. user scrubs timeline)
                target_time_secs < last_time || (target_time_secs - last_time) > 0.5
            },
            None => true,
        };

        if should_seek {
            let target_pts = (target_time_secs * ffmpeg::ffi::AV_TIME_BASE as f64) as i64;
            let _ = self.input.seek(target_pts, ..target_pts);
            self.decoder.flush();
        }
        
        let mut decoded_frame = frame::Video::empty();
        let mut rgba_frame = frame::Video::empty();

        // Feed packets
        let mut packet_iter = self.input.packets();
        while let Some((stream, packet)) = packet_iter.next() {
            if stream.index() == self.video_stream_index {
                // Send packet to decoder
                if self.decoder.send_packet(&packet).is_err() {
                    continue;
                }
                
                // Receive frames from decoder
                while self.decoder.receive_frame(&mut decoded_frame).is_ok() {
                    let frame_pts = decoded_frame.pts().unwrap_or(0);
                    let frame_time = frame_pts as f64 * self.time_base;
                    self.last_frame_time = Some(frame_time);

                    // If we just seeked, ignore frames that are too far behind our target time
                    if should_seek && frame_time + 0.1 < target_time_secs {
                        continue;
                    }

                    // Convert the decoded frame into RGBA format
                    if self.scaler.run(&decoded_frame, &mut rgba_frame).is_ok() {
                        let data = rgba_frame.data(0);
                        let linesize = rgba_frame.stride(0);
                        
                        let w4 = self.width as usize * 4;
                        let mut packed_rgba = Vec::with_capacity((self.height as usize) * w4);
                        
                        for y in 0..self.height as usize {
                            let start = y * linesize;
                            packed_rgba.extend_from_slice(&data[start..start + w4]);
                        }
                        return Some(packed_rgba);
                    }
                }
            }
        }
        None
    }
}
