use clap::{Parser, ValueEnum};

use rsmpeg::avcodec::{AVCodec, AVCodecContext};
use rsmpeg::avutil::{self, ra, AVFrame};
use rsmpeg::error::RsmpegError;
use rsmpeg::ffi::{AV_PIX_FMT_YUV420P, AV_PIX_FMT_UYVY422};
use rsmpeg::ffi::AV_CODEC_FLAG_LOW_DELAY;

use std::time::{Duration, Instant};

/// Rockchip MPP Benchmark
#[derive(Debug, Parser)]
#[command(version, about)]
struct Args {
    /// Benchmark frame width
    #[arg(long, short = 'w')]
    width: u16,
    /// Benchmark frame height
    #[arg(long, short = 'h')]
    height: u16,
    /// Video codec
    #[arg(long, short = 'c')]
    codec: Codec,
    /// Pixel format
    #[arg(long, value_enum, default_value_t = PixelFormat::Yuv420p)]
    pixel_format: PixelFormat,
    /// Number of frames to process
    #[arg(long, default_value_t = 1000)]
    num_frames: u32
}

#[derive(Clone, Debug, ValueEnum)]
enum Codec {
    #[value(alias("mjpeg_enc"))]
    MjpegEnc,
    #[value(alias("h264_enc"))]
    H264Enc,
    #[value(alias("hevc_enc"))]
    HevcEnc,
    // TODO
    // #[value(alias("mjpeg_dec"))]
    // MjpegDec,
    // #[value(alias("h264_dec"))]
    // H264Dec,
    // #[value(alias("hevc_dec"))]
    // HevcDec,
}

#[derive(Clone, Debug, ValueEnum)]
enum PixelFormat {
    #[value(alias("uyvy422"))]
    Uyvy422,
    #[value(alias("yuv420p"))]
    Yuv420p,
}

fn main() {
    let args = Args::parse();

    println!("FFMpeg version: {}", avutil::version_info().to_string_lossy());

    println!("Available codecs:");
    for av_codec_ref in AVCodec::iterate() {
        println!("- {}, {}, {}", av_codec_ref.name().to_string_lossy(), av_codec_ref.long_name().to_string_lossy(), av_codec_ref.id);
    }

    let width = args.width as usize;
    let height = args.height as usize;

    let codec = match args.codec {
        Codec::MjpegEnc => {
            AVCodec::find_encoder_by_name(c"mjpeg_rkmpp")
        }
        Codec::H264Enc => {
            AVCodec::find_encoder_by_name(c"h264_rkmpp")
        }
        Codec::HevcEnc => {
            AVCodec::find_encoder_by_name(c"hevc_rkmpp")
        }
        // Codec::MjpegDec => {
        //     AVCodec::find_decoder_by_name(c"mjpeg_rkmpp")
        // }
        // Codec::H264Dec => {
        //     AVCodec::find_decoder_by_name(c"h264_rkmpp")
        // }
        // Codec::HevcDec => {
        //     AVCodec::find_decoder_by_name(c"hevc_rkmpp")
        // }
    };
    let pixel_format = match args.pixel_format {
        PixelFormat::Yuv420p => AV_PIX_FMT_YUV420P,
        PixelFormat::Uyvy422 => AV_PIX_FMT_UYVY422,
    };
    let mut codec_ctx = AVCodecContext::new(&codec.expect("codec not found"));
    codec_ctx.set_pix_fmt(pixel_format);
    codec_ctx.set_width(width as i32);
    codec_ctx.set_height(height as i32);
    codec_ctx.set_flags(AV_CODEC_FLAG_LOW_DELAY as i32);
    codec_ctx.set_time_base(ra(1, 25));

    codec_ctx.open(None).expect("codec context open");

    let mut frame = AVFrame::new();
    frame.set_format(pixel_format);
    frame.set_width(width as i32);
    frame.set_height(height as i32);
    frame.alloc_buffer().expect("alloc frame buffer");

    // let linesize_count = frame.data.iter().map(|plane| !plane.is_null()).count();
    // println!("Linesize count: {linesize_count}");

    let start_at = Instant::now();
    let mut gen_frame_total_time = Duration::ZERO;

    let mut total_size = 0;
    for i in 0..args.num_frames as usize {
        frame.make_writable().expect("make frame writable");

        let gen_frame_start_at = Instant::now();
        match args.pixel_format {
            PixelFormat::Yuv420p => generate_yuv420p_frame(&mut frame, i),
            PixelFormat::Uyvy422 => generate_uyvy422_frame(&mut frame, i),
        };
        gen_frame_total_time += gen_frame_start_at.elapsed();

        frame.set_pts(i as i64);

        codec_ctx.send_frame(Some(&frame)).expect("send frame");
        loop {
            let packet = match codec_ctx.receive_packet() {
                Ok(packet) => packet,
                Err(RsmpegError::EncoderDrainError) | Err(RsmpegError::EncoderFlushedError) => break,
                Err(e) => panic!("{e}"),
            };
            let data = unsafe { std::slice::from_raw_parts(packet.data, packet.size as usize) };
            total_size += data.len();
        }
    }
    codec_ctx.send_frame(None).expect("send frame");
    loop {
        let packet = match codec_ctx.receive_packet() {
            Ok(packet) => packet,
            Err(RsmpegError::EncoderDrainError) | Err(RsmpegError::EncoderFlushedError) => break,
            Err(e) => panic!("{e}"),
        };
        let data = unsafe { std::slice::from_raw_parts(packet.data, packet.size as usize) };
        total_size += data.len();
    }
    let encode_total_time = start_at.elapsed() - gen_frame_total_time;
    println!("{} frames processed for {:?}", args.num_frames, start_at.elapsed());
    println!("{} frames encoded/decodec for {:?}", args.num_frames, encode_total_time);
    println!("1 frame for {:?}", encode_total_time / args.num_frames);
    println!("Total encoded size: {total_size}");
}

#[inline(always)]
fn generate_yuv420p_frame(frame: &mut AVFrame, i: usize) {
    // assert!(
    //     frame.linesize.len() == 3,
    //     "YUV420P pixel format must have 3 planes but had {}", frame.linesize.len()
    // );

    let width = frame.width as usize;
    let height = frame.height as usize;
    let linesize = frame.linesize;
    let linesize_y = linesize[0] as usize;
    let linesize_cb = linesize[1] as usize;
    let linesize_cr = linesize[2] as usize;
    let data = frame.data;
    let y_data = unsafe { std::slice::from_raw_parts_mut(data[0], height * linesize_y) };
    let cb_data = unsafe { std::slice::from_raw_parts_mut(data[1], height / 2 * linesize_cb) };
    let cr_data = unsafe { std::slice::from_raw_parts_mut(data[2], height / 2 * linesize_cr) };

    for y in 0..height {
        for x in 0..width {
            y_data[y * linesize_y + x] = (x + y + i * 3) as u8;
        }
    }

    for y in 0..height / 2 {
        for x in 0..width / 2 {
            cb_data[y * linesize_cb + x] = (128 + y + i * 2) as u8;
            cr_data[y * linesize_cr + x] = (64 + x + i * 5) as u8;
        }
    }
}

#[inline(always)]
fn generate_uyvy422_frame(frame: &mut AVFrame, i: usize) {
    // assert!(
    //     frame.linesize.len() == 1,
    //     "uyvy422 pixel format must have 1 planes but had {}", frame.linesize.len()
    // );

    let width = frame.width as usize;
    let height = frame.height as usize;
    let linesize = frame.linesize[0] as usize;
    let data = unsafe { std::slice::from_raw_parts_mut(frame.data[0], height * linesize) };

    for y in 0..height {
        for x in 0..width {
            if x % 2 == 0 {
                data[y * linesize + x] = (128 + y + i * 2) as u8;
                data[y * linesize + x + 2] = (64 + x + i * 5) as u8;
            }
            data[y * linesize + x + 1] = (x + y + i * 3) as u8;
        }
    }
}
