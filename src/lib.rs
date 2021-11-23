extern crate ffmpeg_next as ffmpeg;
use ffmpeg::format::{input, Pixel};
use ffmpeg::media::Type;
use ffmpeg::software::scaling::{context::Context, flag::Flags};
use ffmpeg::util::frame::video::Video;
use std::error::Error;
// use std::fs;

pub struct Config {
  pub filename: String,
  pub threshold: f64,
}

impl Config {
  pub fn new(args: &[String]) -> Result<Config, &str> {
    if args.len() < 2 {
      return Err("not enough arguments");
    }

    let filename = args[1].clone();
    let threshold = args[2].parse::<f64>().unwrap();

    Ok(Config {
      filename,
      threshold,
    })
  }
}

// fn main() -> Result<(), ffmpeg::Error> {
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
  ffmpeg::init().unwrap();
  let input_file = config.filename;

  let mut context = input(&input_file).unwrap();
  let frame_count_total = context
    .streams()
    .best(ffmpeg::media::Type::Video)
    .unwrap()
    .frames();
  let duration = context.duration() as f64 / f64::from(ffmpeg::ffi::AV_TIME_BASE);

  let input = context
    .streams()
    .best(Type::Video)
    .ok_or(ffmpeg::Error::StreamNotFound)?;
  let video_stream_index = input.index();

  let mut decoder = input.codec().decoder().video()?;

  let mut scaler = Context::get(
    decoder.format(),
    decoder.width(),
    decoder.height(),
    Pixel::RGB24,
    32,
    1,
    Flags::BILINEAR,
  )?;

  let mut frame_index = 0;
  let mut prev: i32 = 0;
  println!("diff\t time\t frame");
  let mut receive_and_process_decoded_frames =
    |decoder: &mut ffmpeg::decoder::Video| -> Result<(), ffmpeg::Error> {
      let mut decoded = Video::empty();
      while decoder.receive_frame(&mut decoded).is_ok() {
        let mut rgb_frame = Video::empty();
        scaler.run(&decoded, &mut rgb_frame)?;
        let mut value: i32 = 0;
        let max_value: f64 = rgb_frame.data(0).len() as f64 * 255 as f64;
        for b in rgb_frame.data(0).iter() {
          value = value + *b as i32;
        }
        if frame_index == 0 || frame_index + 1 == frame_count_total {
          println!(
            "----\t{:6.2}\t{:5}/{}",
            frame_index as f64 / frame_count_total as f64 * duration,
            frame_index + 1,
            frame_count_total,
          );
        } else if i32::abs(prev - value) as f64 / max_value as f64 > config.threshold {
          println!(
            "{:.3}\t{:6.2}\t{:5}/{}",
            i32::abs(prev - value) as f64 / max_value as f64,
            frame_index as f64 / frame_count_total as f64 * duration,
            frame_index + 1,
            frame_count_total,
          );
          // save_file(&rgb_frame, frame_index).unwrap();
        } else {
          // println!(
          //   "{}/{}\t{:.2}\t{:.3}",
          //   frame_index + 1,
          //   frame_count_total,
          //   frame_index as f64 / frame_count_total as f64 * duration,
          //   i32::abs(prev - value) as f64 / max_value as f64
          // );
        }
        prev = value;
        frame_index += 1;
      }
      Ok(())
    };

  for (stream, packet) in context.packets() {
    if stream.index() == video_stream_index {
      decoder.send_packet(&packet)?;
      receive_and_process_decoded_frames(&mut decoder)?;
    }
  }
  decoder.send_eof()?;
  receive_and_process_decoded_frames(&mut decoder)?;

  Ok(())
}

// fn save_file(frame: &Video, index: usize) -> std::result::Result<(), std::io::Error> {
//   let mut file = File::create(format!("frame/{}.ppm", index))?;
//   file.write_all(format!("P6\n{} {}\n255\n", frame.width(), frame.height()).as_bytes())?;
//   file.write_all(frame.data(0))?;
//   Ok(())
// }
