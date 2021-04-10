use image::{io::Reader as ImageReader, ImageBuffer, RgbaImage};
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "easy-stitch")]
struct Options {
    #[structopt(name = "FILE", required = true, parse(from_os_str))]
    file: Vec<PathBuf>,

    #[structopt(short, parse(from_os_str))]
    output: Option<PathBuf>,

    #[structopt(short, long)]
    vertical: bool,

    #[structopt(short, long)]
    force: bool,
}

fn stitch_horizontal<'a>(images: impl Iterator<Item = &'a RgbaImage> + Clone) -> RgbaImage {
    let new_width: u32 = images.clone().map(RgbaImage::width).sum();
    let new_height: u32 = images.clone().map(RgbaImage::height).max().unwrap();

    let mut new_image: RgbaImage = ImageBuffer::new(new_width, new_height);
    let mut current_x = 0;
    for image in images {
        for (x, y, px) in image.enumerate_pixels() {
            new_image.put_pixel(current_x + x, y, *px);
        }
        current_x += image.width();
    }

    new_image
}

fn stitch_vertical<'a>(images: impl Iterator<Item = &'a RgbaImage> + Clone) -> RgbaImage {
    let new_width: u32 = images.clone().map(RgbaImage::width).max().unwrap();
    let new_height: u32 = images.clone().map(RgbaImage::height).sum();

    let mut new_image: RgbaImage = ImageBuffer::new(new_width, new_height);
    let mut current_y = 0;
    for image in images {
        for (x, y, px) in image.enumerate_pixels() {
            new_image.put_pixel(x, current_y + y, *px);
        }
        current_y += image.height();
    }

    new_image
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let options = Options::from_args();

    let output_path = options.output.unwrap_or(PathBuf::from("output.png"));
    if !options.force && output_path.exists() {
        println!(
            "Error: file {:?} already exists, use -f to overwrite",
            output_path
        );
        return Ok(());
    }

    let mut images = Vec::new();
    for file in &options.file {
        let image = ImageReader::open(file)?.decode()?.to_rgba8();
        images.push(image);
    }

    let new_image = if options.vertical {
        stitch_vertical(images.iter())
    } else {
        stitch_horizontal(images.iter())
    };

    new_image.save(output_path)?;
    Ok(())
}
