use std::env;
use std::sync::mpsc;

use image::{self, GenericImageView, ImageBuffer, Pixel};
use image::io::Reader;
use threads::ThreadPool;
use std::time::Instant;
use util::{self, time_eval};


fn parallel_img() {
    let pool = ThreadPool::new(10);

    let img = Reader::open("./image_flip/earth.png")
        .unwrap()
        .decode()
        .unwrap();

    let (width, height) = img.dimensions();

    let mut out = ImageBuffer::new(width, height);

    let pixels: Vec<(u32, u32, image::Rgba<u8>)> = img.pixels().into_iter().collect();    
    
    let chunk_size = pixels.len() / pool.size();

    let (tx, rx) = mpsc::channel();

    time_eval!("Processing image...", {
        for chunk in pixels.chunks(chunk_size) {
            let chunk = chunk.to_vec();
            let send_chan = tx.clone();
            pool.execute(move || {
                let mut new_pixels = Vec::new();
                for (x, y, pixel) in chunk {
                    let grayscale = pixel.to_luma_alpha();
                    let new_pixel = grayscale.to_rgba();

                    new_pixels.push((x, y, new_pixel));
                }
                send_chan.send(new_pixels).unwrap();
            });
        }

        for _ in 0..pool.size() {
            for chunk in rx.recv() {
                for (x, y, pixel) in chunk {
                    out.put_pixel(x, y, pixel);
                }
            }
        }
    });
    
    out.save_with_format("./image_flip/gray_par.png", image::ImageFormat::Png).unwrap();
}

fn seq_img() {
    let img = Reader::open("./image_flip/earth.png")
        .unwrap()
        .decode()
        .unwrap();

    print!("Processing image... ");

    let now = Instant::now();
    let new_image = img.to_luma_alpha8();

    let elapsed = now.elapsed();

    print!(" Done!, Elapsed: {:.2?}\n", elapsed);

    new_image.save_with_format("./image_flip/gray_seq.png", image::ImageFormat::Png).unwrap();
}

fn main() {
    let mut args = env::args();

    if let Some(arg) = args.nth(1) {

        match arg.as_str() {
            "-p" => {
                parallel_img();
            },
            "-s" => {
                seq_img();
            },
            unkown => {
                println!("Unkown argument: {unkown}");
            }
        }
    }
}
