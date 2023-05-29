use image::codecs::png::PngEncoder;
use image::ColorType;
use image::ImageEncoder;
use num::Complex;
use std::env;
use std::fs::File;
use std::iter::successors;
use std::str::FromStr;
use std::time::Instant;
use text_colorizer::*;

/// Parse a pair of numbers separated by a character as a pair.
fn parse_pair<T: FromStr>(s: &str, separator: char) -> Option<(T, T)> {
    match s.find(separator) {
        None => None,
        Some(index) => match (T::from_str(&s[..index]), T::from_str(&s[index + 1..])) {
            (Ok(l), Ok(r)) => Some((l, r)),
            _ => None,
        },
    }
}

#[test]
fn test_parse_pair() {
    assert_eq!(parse_pair::<i32>("", ','), None);
    assert_eq!(parse_pair::<i32>("10,", ','), None);
    assert_eq!(parse_pair::<i32>("10,20", ','), Some((10, 20)));
    assert_eq!(parse_pair::<i32>("10x20", 'x'), Some((10, 20)));
}

/// Parse a string of number separated by a , as a Complex<f64> number.
fn parse_complex(s: &str) -> Option<Complex<f64>> {
    match parse_pair(s, ',') {
        None => None,
        Some((re, im)) => Some(Complex { re, im }),
    }
}

#[test]
fn test_parse_complex() {
    assert_eq!(
        parse_complex("1.25,-0.0625"),
        Some(Complex {
            re: 1.25,
            im: -0.0625
        })
    );
    assert_eq!(parse_complex(",-0.0625"), None);
    assert_eq!(parse_complex("-0.0625,"), None);
}

fn pixel_to_point(
    bounds: (usize, usize),
    pixel: (usize, usize),
    upper_left: Complex<f64>,
    lower_right: Complex<f64>,
) -> Complex<f64> {
    let (width, height) = (
        lower_right.re - upper_left.re,
        upper_left.im - lower_right.im,
    );
    Complex {
        re: upper_left.re + pixel.0 as f64 * width / bounds.0 as f64,
        im: upper_left.im - pixel.1 as f64 * height / bounds.1 as f64,
    }
}

#[test]
fn test_pixel_to_point() {
    assert_eq!(
        pixel_to_point(
            (100, 200),
            (25, 175),
            Complex { re: -1.0, im: 1.0 },
            Complex { re: 1.0, im: -1.0 }
        ),
        Complex {
            re: -0.5,
            im: -0.75
        }
    );
}

fn escape_time(c: Complex<f64>, limit: u32) -> Option<u32> {
    let mut z = Complex::<f64> { re: 0.0, im: 0.0 };
    for i in 0..limit {
        z = z * z + c;
        if z.norm_sqr() > 4.0 {
            return Some(i);
        }
    }
    None
}

#[test]
fn test_escape_time() {
    {
        let c = Complex::<f64> { re: 2., im: 0.0 };
        let start = Instant::now();
        let i = escape_time(c, 10);
        println!("Escape Time elapsed: {:?}", start.elapsed().as_nanos());
        assert_eq!(i, Some(1));
    }
    {
        let c = Complex::<f64> {
            re: 0.00011,
            im: 0.0,
        };
        let start = Instant::now();
        let i = escape_time(c, 10);
        println!("Escape Time elapsed: {:?}", start.elapsed().as_nanos());
        assert_eq!(i, None);
    }
}

// Using iterators (base on chapter 15)
// Slower than for loop by 30% (but same spped as filter)
fn escape_time_iter_other(c: Complex<f64>, limit: u32) -> Option<u32> {
    let zero = Complex::<f64> { re: 0.0, im: 0.0 };
    successors(Some(zero), |z| Some(z * z + c))
        .take(limit as usize)
        .position(|z| z.norm_sqr() > 4.0)
        .map(|i| (i - 1) as u32)
}

#[test]
fn test_escape_time_iter_other() {
    {
        let c = Complex::<f64> { re: 2., im: 0.0 };
        assert_eq!(escape_time_iter_other(c, 10), Some(1));
    }
    {
        let c = Complex::<f64> {
            re: 0.00011,
            im: 0.0,
        };
        assert_eq!(escape_time_iter_other(c, 10), None);
    }
}

// Using iterators (base on chapter 15)
// Slower than for loop by 30% (but same speed as position iterator)
fn escape_time_iter(c: Complex<f64>, limit: u32) -> Option<u32> {
    let zero = Complex::<f64> { re: 0.0, im: 0.0 };
    successors(Some(zero), |z| Some(z * z + c))
        .take(limit as usize)
        .enumerate()
        .find(|(_, z)| z.norm_sqr() > 4.0)
        .map(|(i, _z)| (i - 1) as u32)
}

#[test]
fn test_escape_time_iter() {
    {
        let c = Complex::<f64> { re: 2., im: 0.0 };
        assert_eq!(escape_time_iter(c, 10), Some(1));
    }
    {
        let c = Complex::<f64> {
            re: 0.00011,
            im: 0.0,
        };
        assert_eq!(escape_time_iter(c, 10), None);
    }
}

fn render(
    pixels: &mut [u8],
    bounds: (usize, usize),
    upper_left: Complex<f64>,
    lower_right: Complex<f64>,
) {
    let start = Instant::now();
    for row in 0..bounds.1 {
        for column in 0..bounds.0 {
            let point = pixel_to_point(bounds, (column, row), upper_left, lower_right);
            pixels[row * bounds.0 + column] = match escape_time(point, 255) {
                // pixels[row * bounds.0 + column] = match escape_time_iter(point, 255) {
                // pixels[row * bounds.0 + column] = match escape_time_iter_other(point, 255) {
                None => 0,
                Some(count) => 255 - count as u8,
            };
        }
    }
    println!("Render elapsed: {:?}ms", start.elapsed().as_millis());
}

fn write_image(
    filename: &str,
    pixels: &[u8],
    bounds: (usize, usize),
) -> Result<(), std::io::Error> {
    let output = File::create(filename)?;
    let encoder = PngEncoder::new(output);
    encoder
        .write_image(&pixels, bounds.0 as u32, bounds.1 as u32, ColorType::L8)
        .expect("Failed to write PNG image");
    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 5 {
        eprintln!(
            "{}: mandelbrot FILE PIXELS UPPERLEFT LOWERRIGHT",
            "Usage".red().bold()
        );
        eprintln!(
            "{}: mandel.png 1000x750 -1.20,0.35 -1,0.20",
            "Example".red().bold()
        );
        println!("Checkout chapter 19 for concurrency using rayon");
        std::process::exit(1);
    }
    let bounds = parse_pair(&args[2], 'x').expect("error parsing image dimensions");
    let upper_left = parse_complex(&args[3]).expect("error parsing upper left corner point");
    let lower_right = parse_complex(&args[4]).expect("error parsing lower right corner point");
    let mut pixels = vec![0; bounds.0 * bounds.1];

    let threads = 12;
    if threads <= 1 {
        // Single thread
        render(&mut pixels, bounds, upper_left, lower_right);
    } else {
        let (columns, rows) = bounds;
        let rows_per_band = (rows / threads) + 1;
        println!(
            "Using {} threads with {} rows per band",
            threads, rows_per_band
        );
        // Split pixels into chunks of rows_per_band (iterator with the last one may not have the
        // full rows_per_band * columns
        let bands: Vec<&mut [u8]> = pixels.chunks_mut(rows_per_band * columns).collect();
        // Spawn threads (See chapter 19 for drastic improvements)
        crossbeam::scope(|spawner| {
            for (i, band) in bands.into_iter().enumerate() {
                //into_iter to have exclusive
                //ownership
                let top = rows_per_band * i;
                let height = band.len() / columns; // will correct for the last band that may have
                                                   // fewer rows than the others
                let band_bounds = (columns, height);
                let band_upper_left = pixel_to_point(bounds, (0, top), upper_left, lower_right);
                let band_lower_right =
                    pixel_to_point(bounds, (columns, top + height), upper_left, lower_right);
                spawner.spawn(move |_| {
                    // move takes ownerships of variables
                    render(band, band_bounds, band_upper_left, band_lower_right);
                });
            }
        })
        .unwrap();
    }

    write_image(&args[1], &pixels, bounds).expect("error writing PNG file");
}
