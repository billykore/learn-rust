use actix_web::{web, HttpResponse, Responder};
use image::codecs::png::PngEncoder;
use image::{ExtendedColorType, ImageEncoder};
use num::Complex;
use serde::Deserialize;
use std::env;
use std::fs::File;
use std::str::FromStr;
use regex::Regex;
use text_colorizer::*;

/// # Chapter 2. A Tour of Rust
/// Programming Rust
///
/// Fast, Safe System Development
///
/// Jim Blandy, Jason Orendorff & Leonora F.S. Tindall

/// # Rust Functions
///
/// gcd is a function that computes the greatest common divisor of two integers,
/// using Euclid’s algorithm.
pub fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    n
}


/// # Writing and Running Unit Tests
#[test]
fn test_gcd() {
    assert_eq!(gcd(14, 15), 1);
    assert_eq!(gcd(
        2 * 3 * 5 * 11 * 17,
        3 * 7 * 11 * 13 * 19),
               3 * 11);
}

/// # Handling Command-Line Arguments
pub fn gcd_cli() {
    let mut numbers = Vec::new();
    for arg in env::args().skip(1) {
        numbers.push(u64::from_str(&arg)
            .expect("error parsing argument"));
    }

    if numbers.len() == 0 {
        eprintln!("Usage: gcd NUMBER ...");
        std::process::exit(1);
    }

    let mut d = numbers[0];
    for m in &numbers[1..] {
        d = gcd(d, *m);
    }

    println!("The greatest common divisor of {:?} is {}", numbers, d);
}

/// # Serving Pages to the Web
pub async fn get_index() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(
            r#"
                <title>GCD Calculator</title>
                <form action="/gcd" method="post">
                <input type="text" name="n"/>
                <input type="text" name="m"/>
                <button type="submit">Compute GCD</button>
                </form>
            "#,
        )
}

#[derive(Deserialize)]
pub struct GcdParameters {
    n: u64,
    m: u64,
}

pub async fn post_gcd(form: web::Form<GcdParameters>) -> impl Responder {
    if form.n == 0 || form.m == 0 {
        return HttpResponse::BadRequest()
            .content_type("text/html")
            .body("Computing the GCD with zero is boring.");
    }

    let response =
        format!("The greatest common divisor of the numbers {} and {} is <b>{}</b>\n",
                form.n, form.m, gcd(form.n, form.m));

    HttpResponse::Ok()
        .content_type("text/html")
        .body(response)
}

/// # Concurrency
///
/// ## Mandelbrot Set
///
/// Try to determine if `c` is in the Mandelbrot set, using at most `limit`
/// iterations to decide.
///
/// If `c` is not a member, return `Some(i)`, where `i` is the number of
/// iterations it took for `c` to leave the circle of radius 2 centered on the
/// origin. If `c` seems to be a member (more precisely, if we reached the
/// iteration limit without being able to prove that `c` is not a member),
/// return `None`.
fn escape_time(c: Complex<f64>, limit: usize) -> Option<usize> {
    let mut z = Complex { re: 0.0, im: 0.0 };
    for i in 0..limit {
        if z.norm_sqr() > 4.0 {
            return Some(i);
        }
        z = z * z + c;
    }
    None
}

/// ## Parsing Pair Command-Line Arguments
///
/// Parse the string `s` as a coordinate pair, like `"400x600"` or `"1.0,0.5"`.
///
/// Specifically, `s` should have the form `<left>` `<sep>` `<right>`, where `<sep>` is
/// the character given by the `separator` argument, and `<left>` and `<right>` are
/// both strings that can be parsed by `T::from_str`. `separator` must be an
/// ASCII character.
///
/// If `s` has the proper form, return `Some<(x, y)>`. If it doesn't parse
/// correctly, return `None`.
pub fn parse_pair<T: FromStr>(s: &str, separator: char) -> Option<(T, T)> {
    match s.find(separator) {
        None => None,
        Some(index) => {
            match (T::from_str(&s[..index]), T::from_str(&s[index + 1..])) {
                (Ok(l), Ok(r)) => Some((l, r)),
                _ => None
            }
        }
    }
}

#[test]
fn test_parse_pair() {
    assert_eq!(parse_pair::<i32>("", ','), None);
    assert_eq!(parse_pair::<i32>("10,", ','), None);
    assert_eq!(parse_pair::<i32>(",10", ','), None);
    assert_eq!(parse_pair::<i32>("10,20", ','), Some((10, 20)));
    assert_eq!(parse_pair::<i32>("10,20xy", ','), None);
    assert_eq!(parse_pair::<f64>("0.5x", 'x'), None);
    assert_eq!(parse_pair::<f64>("0.5x1.5", 'x'), Some((0.5, 1.5)));
}

/// Parse a pair of floating-point numbers separated by a comma
/// as a complex number.
pub fn parse_complex(s: &str) -> Option<Complex<f64>> {
    match parse_pair(s, ',') {
        Some((re, im)) => Some(Complex { re, im }),
        None => None
    }
}
#[test]
fn test_parse_complex() {
    assert_eq!(parse_complex("1.25,-0.0625"), Some(Complex { re: 1.25, im: -0.0625 }));
    assert_eq!(parse_complex(",-0.0625"), None);
}

/// ## Mapping from Pixels to Complex Numbers
///
/// Given the row and column of a pixel in the output image, return the
/// corresponding point on the complex plane.
///
/// `bounds` is a pair giving the width and height of the image in pixels.
/// `pixel` is a (column, row) pair indicating a particular pixel in that image.
/// The `upper_left` and `lower_right` parameters are points on the complex
/// plane designating the area our image covers.
pub fn pixel_to_point(bounds: (usize, usize), pixel: (usize, usize), upper_left: Complex<f64>,
                      lower_right: Complex<f64>) -> Complex<f64> {
    let (width, height) = (lower_right.re - upper_left.re, upper_left.im - lower_right.im);

    Complex {
        re: upper_left.re + pixel.0 as f64 * width / bounds.0 as f64,
        // Why subtraction here? pixel.1 increases as we go down,
        // but the imaginary component increases as we go up.
        im: upper_left.im - pixel.1 as f64 * height / bounds.1 as f64,
    }
}

#[test]
fn test_pixel_to_point() {
    assert_eq!(pixel_to_point(
        (100, 200), (25, 175), Complex { re: -1.0, im: 1.0 },
        Complex { re: 1.0, im: -1.0 }),
               Complex { re: -0.5, im: -0.75 });
}

/// ## Plotting the Set
///
/// Render a rectangle of the Mandelbrot set into a buffer of pixels.
///
/// The `bounds` argument gives the width and height of the buffer `pixels`,
/// which holds one grayscale pixel per byte. The `upper_left` and `lower_right`
/// arguments specify points on the complex plane corresponding
/// to the `upper-left` and `lower-right` corners of the pixel buffer.
pub fn render(pixels: &mut [u8], bounds: (usize, usize),
              upper_left: Complex<f64>, lower_right: Complex<f64>) {
    assert_eq!(pixels.len(), bounds.0 * bounds.1);

    for row in 0..bounds.1 {
        for column in 0..bounds.0 {
            let point = pixel_to_point(
                bounds,
                (column, row),
                upper_left,
                lower_right);

            pixels[row * bounds.0 + column] = match escape_time(point, 255) {
                None => 0,
                Some(count) => 255 - count as u8
            };
        }
    }
}

#[test]
fn test_render() {
    let bounds = parse_pair("100x100", 'x')
        .expect("error parsing image dimensions");

    let upper_left = parse_complex("-1.20,0.35")
        .expect("error parsing upper left corner point");

    let lower_right = parse_complex("-1,0.20")
        .expect("error parsing lower right corner point");

    let mut pixels = vec![0; bounds.0 * bounds.1];

    render(&mut pixels, bounds, upper_left, lower_right);
}

/// Write the buffer `pixels`, whose dimensions are given by `bounds`, to the
/// file named `filename`.
pub fn write_image(filename: &str, pixels: &[u8],
                   bounds: (usize, usize)) -> Result<(), std::io::Error> {
    let output = File::create(filename)?;
    let encoder = PngEncoder::new(output);

    encoder.write_image(&pixels, bounds.0 as u32, bounds.1 as u32, ExtendedColorType::L8)
        .expect("failed to write to file");
    Ok(())
}

#[test]
fn test_simple_image() {
    let gray = File::create("gray.png")
        .expect("failed to create file");

    let encoder = PngEncoder::new(gray);

    let pixels = vec![154; 100 * 100 * 3];

    encoder.write_image(&pixels, 100, 100, ExtendedColorType::Rgb8)
        .expect("failed to write");
}

#[test]
fn test_write_image() {
    let bounds = parse_pair("1000x750", 'x')
        .expect("error parsing image dimensions");

    let upper_left = parse_complex("-1.20,0.35")
        .expect("error parsing upper left corner point");

    let lower_right = parse_complex("-1,0.20")
        .expect("error parsing lower right corner point");

    let mut pixels = vec![0; bounds.0 * bounds.1];

    render(&mut pixels, bounds, upper_left, lower_right);

    write_image("test_write_image.png", &pixels, bounds)
        .expect("error writing PNG file");
}

#[test]
fn test_write_image_concurrently() {
    let bounds = parse_pair("1000x750", 'x')
        .expect("error parsing image dimensions");

    let upper_left = parse_complex("-1.20,0.35")
        .expect("error parsing upper left corner point");

    let lower_right = parse_complex("-1,0.20")
        .expect("error parsing lower right corner point");

    let mut pixels = vec![0; bounds.0 * bounds.1];

    let threads = 8;
    let rows_per_band = bounds.1 / threads + 1;

    {
        let bands: Vec<&mut [u8]> = pixels.chunks_mut(rows_per_band * bounds.0)
            .collect();

        crossbeam::scope(|spawner| {
            for (i, band) in bands.into_iter().enumerate() {
                let top = rows_per_band * i;
                let height = band.len() / bounds.0;
                let band_bounds = (bounds.0, height);
                let band_upper_left = pixel_to_point(
                    bounds, (0, top), upper_left, lower_right);
                let band_lower_right = pixel_to_point(
                    bounds, (bounds.0, top + height), upper_left, lower_right);
                spawner.spawn(move |_| {
                    render(band, band_bounds, band_upper_left,
                           band_lower_right);
                });
            }
        }).unwrap();
    }

    write_image("test_write_image_concurrently.png", &pixels, bounds)
        .expect("error writing PNG file");
}

/// # Filesystems and Command-Line Tools
///
/// `quickreplace`: search-and-replace tool
#[derive(Debug)]
pub struct Arguments {
    pub target: String,
    pub replacement: String,
    pub filename: String,
    pub output: String,
}

pub fn print_usage() {
    eprintln!("{} - change occurrences of one string into another", "quickreplace".green());
    eprintln!("Usage: quickreplace <target> <replacement> <INPUT> <OUTPUT>");
}

pub fn parse_args() -> Arguments {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.len() < 4 {
        print_usage();
        eprintln!("{} wrong number of arguments: expected 4, got {}.",
                  "Error:".red().bold(), args.len());
        std::process::exit(1);
    }

    Arguments {
        target: args[0].clone(),
        replacement: args[1].clone(),
        filename: args[2].clone(),
        output: args[3].clone(),
    }
}

pub fn replace(target: &str, replacement: &str, text: &str) -> Result<String, regex::Error> {
    let regex = Regex::new(target)?;
    Ok(regex.replace_all(text, replacement).to_string())
}

#[test]
fn test_replace() {
    match replace("I am Oyen", "Groot", "Oyen") {
        Ok(v) => assert_eq!(v, "I am Groot"),
        Err(e) => panic!("{}", e),
    }
}
