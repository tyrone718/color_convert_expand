use clap::Parser;
use image::GenericImageView;
use std::i32;

fn color_distance(pixel1: &image::Rgba<u8>, pixel2: &image::Rgba<u8>) -> f64 {
    let tmp = ((pixel1[0] as i32 - pixel2[0] as i32).pow(3).abs()
        + (pixel1[1] as i32 - pixel2[1] as i32).pow(3).abs()
        + (pixel1[2] as i32 - pixel2[2] as i32).pow(3).abs()) as f64;

    (tmp as f64).powf(1_f64 / 3_f64)
}

fn hex_to_rgb_vec(hex_str: &str) -> image::Rgba<u8> {
    let r = u8::from_str_radix(&hex_str[0..2], 16).unwrap();
    let g = u8::from_str_radix(&hex_str[2..4], 16).unwrap();
    let b = u8::from_str_radix(&hex_str[4..6], 16).unwrap();
    image::Rgba::<u8>([r, g, b, 255])
}

/// Image color changer and image expander program
#[derive(Parser, Debug)]
#[clap(version, about, long_about=None)]
struct Args {
    /// there are two options for mode, "convert_colors" and "expand"
    #[clap(short, long)]
    mode: String,
    #[clap(short, long)]
    in_file: String,
    #[clap(short, long)]
    out_file: String,
}

fn call_conversions(in_args: &Args) {
    if in_args.mode == "convert_colors" {
        let reddit_colors: Vec<image::Rgba<u8>> = vec![
            hex_to_rgb_vec("6d001a"),
            hex_to_rgb_vec("be0039"),
            hex_to_rgb_vec("ff4500"),
            hex_to_rgb_vec("ffa800"),
            hex_to_rgb_vec("ffd635"),
            hex_to_rgb_vec("fff8b8"),
            hex_to_rgb_vec("00a368"),
            hex_to_rgb_vec("00cc78"),
            hex_to_rgb_vec("7eed56"),
            hex_to_rgb_vec("00756f"),
            hex_to_rgb_vec("009eaa"),
            hex_to_rgb_vec("00ccc0"),
            hex_to_rgb_vec("2450a4"),
            hex_to_rgb_vec("3690ea"),
            hex_to_rgb_vec("51e9f4"),
            hex_to_rgb_vec("493ac1"),
            hex_to_rgb_vec("6a5cff"),
            hex_to_rgb_vec("94b3ff"),
            hex_to_rgb_vec("811e9f"),
            hex_to_rgb_vec("b44ac0"),
            hex_to_rgb_vec("e4abff"),
            hex_to_rgb_vec("de107f"),
            hex_to_rgb_vec("ff3881"),
            hex_to_rgb_vec("ff99aa"),
            hex_to_rgb_vec("6d482f"),
            hex_to_rgb_vec("9c6926"),
            hex_to_rgb_vec("ffb470"),
            hex_to_rgb_vec("000000"),
            hex_to_rgb_vec("515252"),
            hex_to_rgb_vec("898d90"),
            hex_to_rgb_vec("d4d7d9"),
            hex_to_rgb_vec("ffffff"),
        ];
        change_colors(&in_args, &reddit_colors);
    } else if in_args.mode == "expand" {
        expand_image(&in_args);
    } else {
        println!("INVALID_MODE");
    }
}

fn expand_image(in_args: &Args) {
    println!("reading: {}", in_args.in_file);
    let in_img = image::open(format!("{}", in_args.in_file)).unwrap();

    println!("dimensions: {:?}", in_img.dimensions());
    println!("color {:?}", in_img.color());

    let in_max_y = in_img.dimensions().1;

    let out_max_x = in_img.dimensions().0 * 3;
    let out_max_y = in_img.dimensions().1 * 3;

    let mut out_img = image::ImageBuffer::<image::Rgba<u8>, Vec<u8>>::new(out_max_x, out_max_y);

    let mut in_x = 0;
    let mut in_y = 0;
    for out_x in 0..out_max_x {
        for out_y in 0..out_max_y {
            if ((out_x != 0) && (out_x != out_max_x - 1) && ((out_x - 1) % 3 == 0))
                && ((out_y != 0) && (out_y != out_max_y - 1) && ((out_y - 1) % 3 == 0))
            {
                let target_pixel = out_img.get_pixel_mut(out_x, out_y);
                *target_pixel = in_img.get_pixel(in_x, in_y);

                in_y += 1;
                if in_y >= in_max_y {
                    in_y -= in_max_y;
                    in_x += 1;
                }
            } else {
                let new_pixel = out_img.get_pixel_mut(out_x, out_y);
                *new_pixel = image::Rgba([0, 0, 0, 0]);
            }
        }
    }

    println!("saving:{}", in_args.in_file);

    out_img.save(format!("{}", in_args.out_file)).unwrap();
}

fn change_colors(in_args: &Args, reddit_colors: &Vec<image::Rgba<u8>>) {
    println!("reading: {}", in_args.in_file);
    let in_img = image::open(format!("{}", in_args.in_file)).unwrap();

    println!("dimensions: {:?}", in_img.dimensions());
    println!("dimensions 0: {:?}", in_img.dimensions().0);
    println!("dimensions 1: {:?}", in_img.dimensions().1);
    println!("color {:?}", in_img.color());

    println!("starting to process");
    let mut in_img = in_img.to_rgba8();
    for in_img_pixel in in_img.pixels_mut() {
        if in_img_pixel[3] != 0 {
            let mut min_distance = color_distance(in_img_pixel, &reddit_colors[0]);
            let mut closest_index = 0;
            for (i, color) in reddit_colors.iter().enumerate() {
                let distance = color_distance(in_img_pixel, &color);
                if distance < min_distance {
                    min_distance = distance;
                    closest_index = i;
                }
            }

            let selected_color = &reddit_colors[closest_index];
            *in_img_pixel = image::Rgba([
                selected_color[0] as u8,
                selected_color[1] as u8,
                selected_color[2] as u8,
                in_img_pixel[3],
            ]);
        }
    }
    println!("saving:{}", in_args.out_file);

    in_img.save(format!("{}", in_args.out_file)).unwrap();
}

fn main() {
    let in_args = Args::parse();
    call_conversions(&in_args);
}
