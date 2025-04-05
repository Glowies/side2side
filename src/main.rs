use getopts::Options;
use std::{env, path::Path};
use imageproc::image;
use imageproc::drawing;
use imageproc::point::Point;
use ab_glyph::{FontRef, PxScale};

fn print_usage(program: &str, opts: &Options) {
    let brief = format!("Usage: {program} IMAGE_FILE LABEL_TEXT [options]");
    print!("{}", opts.usage(&brief));
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    // declare possible command line arguments
    let mut opts = Options::new();
    opts.optopt(
        "o", 
        "output-dir", 
        "directory to save the annotated image in", 
        "DIR"
    );
    opts.optopt(
        "c", 
        "color", 
        "set the color of the label text (specified in HEX)", 
        "COLOR"
    );

    // attempt to parse the command line arguments
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => {
            print_usage(&program, &opts);
            panic!("{}", f);
        }
    };

    // check if there are two free arguments
    if matches.free.len() != 2 {
        print_usage(&program, &opts);
        return;
    }

    // get the image file and label text from the command line arguments
    let image_path = matches.free[0].clone();
    let label_text = matches.free[1].clone();

    // extract image file name
    let image_file_name = get_image_file_name(&image_path);

    // get optional arguments
    let _color = matches.opt_str("c").unwrap_or_else(|| String::from("#FFFFFF"));
    let output_dir = matches.opt_str("o").unwrap_or_else(|| String::from("./annotated"));

    // load image with imageproc
    let img = image::open(&image_path).expect("Failed to open image");

    // label stats
    let label_size: u32 = (img.height() / 20) as u32;
    let x_margin: u32 = label_size / 5;
    let y_margin: u32 = label_size / 5;
    let y_position: u32 = (img.height() / 20) as u32;

    // initialize scale and font of the label text
    // convert label size to float
    let label_size_float = label_size as f32;
    let scale = PxScale::from(label_size_float);
    let font = FontRef::try_from_slice(
        include_bytes!("../fonts/FiraCodeNerdFont-Regular.ttf") as &[u8]
    ).expect("Failed to load font");

    // draw background rectangle
    let rect = imageproc::rect::Rect::at(0, (y_position - y_margin) as i32)
        .of_size(img.width(), 2 * y_margin + label_size);
    let points = 
    [
        Point::new(rect.left(), rect.top()),
        Point::new(rect.right(), rect.top()),
        Point::new(rect.right(), rect.bottom()),
        Point::new(rect.left(), rect.bottom()),
    ];

    let output_img = drawing::draw_polygon(
        &img,
        &points,
        image::Rgba([241, 162, 130, 255]),
    );

    // write text on image
    let output_img = drawing::draw_text(
        &output_img,
        image::Rgba([255, 255, 255, 255]),
        x_margin as i32,
        y_position as i32,
        scale,
        &font,
        &label_text,
    );

    // save the image with the label text
    save_image(&output_img, &image_file_name, &output_dir);
}

fn get_image_file_name(image_path: &str) -> String {
    let image_file = Path::new(&image_path);
    let image_os_str = match image_file.file_name() {
        Some(name) => name,
        None => {
            panic!("Failed to get file name from path: {}", image_path)
        }
    };
    let image_file_name = match image_os_str.to_str() {
        Some(name) => name,
        None => {
            panic!("Failed to convert file name to string: {}", image_path)
        }
    };

    String::from(image_file_name)
}

fn save_image(
    image: &image::ImageBuffer<image::Rgba<u8>, Vec<u8>>, 
    filename: &str,
    output_dir: &str
) {
    let output_path = Path::new(output_dir);
    if !output_path.exists() {
        std::fs::create_dir_all(output_path).expect("Failed to create output directory");
    }
    let output_file = output_path.join(filename);
    image.save(output_file).expect("Failed to save image");
}
