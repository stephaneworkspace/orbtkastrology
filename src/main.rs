#![feature(vec_into_raw_parts)]
extern crate image;
extern crate nsvg;
use orbtk::prelude::*;
use std::env;
use std::path::Path;

fn convert_using_into_raw_parts(v: Vec<u8>) -> Vec<u32> {
    let (ptr, len, cap) = v.into_raw_parts();
    unsafe { Vec::from_raw_parts(ptr as *mut u32, len, cap) }
}

fn main() {
    // use this only if you want to run it as web application.
    // orbtk::initialize();

    const INPUT: &str =
        "/Users/stephanebressani/Code/Rust/orbtkastrology/res/Aries.svg";
    const OUTPUT: &str =
        "/Users/stephanebressani/Code/Rust/orbtkastrology/res/Aries.png";
    /*
        let mut opt = resvg::Options::default();
        opt.usvg.path = Some(INPUT.clone().into());

        let rtree = usvg::Tree::from_file(&INPUT, &opt.usvg).unwrap();
        let backend = resvg::default_backend();
        let mut img = backend.render_to_image(&rtree, &opt).unwrap();
        img.save_png(std::path::Path::new(&OUTPUT));
    */

    let path = Path::new(INPUT);
    let svg = nsvg::parse_file(path, nsvg::Units::Pixel, 96.0).unwrap();
    let svgr = nsvg::parse_file(path, nsvg::Units::Pixel, 96.0).unwrap();

    let image = svg.rasterize(2.0).unwrap();
    let (w_r, h_r, raw_rgba) = svgr.rasterize_to_raw_rgba(2.0).unwrap();
    // let save_path = env::current_dir().unwrap().join("test.png");

    let raw_final: Vec<u32> = convert_using_into_raw_parts(raw_rgba);

    let save_path = OUTPUT;
    let (width, height) = image.clone().dimensions();

    // Write the image to disk as a PNG
    image::save_buffer(
        save_path.clone(),
        &image.into_raw(),
        width,
        height,
        image::ColorType::RGBA(8),
    )
    .expect("Failed to save png.");

    /*

    let save_path = env::current_dir().unwrap().join("example_output.png");
    let (width, height) = image.dimensions();

    // Write the image to disk as a PNG
    image::save_buffer(
      save_path,
      &image.into_raw(),
      width,
      height,
      image::ColorType::RGBA(8),
    ).expect("Failed to save png.");
    */

    let img = Image::from_data(w_r, h_r, raw_final);

    Application::new()
        .window(|ctx| {
            Window::create()
                .title("OrbTk - image example")
                .position((100.0, 100.0))
                .size(800.0, 420.0)
                //.child(ImageWidget::create().image(OUTPUT).build(ctx))
                .child(ImageWidget::create().from(raw_final).build(ctx))
                .build(ctx)
        })
        .run();
}
