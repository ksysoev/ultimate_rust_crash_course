// FINAL PROJECT
//
// Create an image processing application.  Exactly what it does and how it does
// it is up to you, though I've stubbed a good amount of suggestions for you.
// Look for comments labeled **OPTION** below.
//
// Two image files are included in the project root for your convenience: dyson.png and pens.png
// Feel free to use them or provide (or generate) your own images.
//
// Don't forget to have fun and play around with the code!
//
// Documentation for the image library is here: https://docs.rs/image/0.21.0/image/
//
// NOTE 1: Image processing is very CPU-intensive.  Your program will run *noticeably* faster if you
// run it with the `--release` flag.
//
//     cargo run --release [ARG1 [ARG2]]
//
// For example:
//
//     cargo run --release blur image.png blurred.png
//
// NOTE 2: This is how you parse a number from a string (or crash with a
// message). It works with any integer or float type.
//
//     let positive_number: u32 = some_string.parse().expect("Failed to parse a number");

fn main() {
    // 1. First, you need to implement some basic command-line argument handling
    // so you can make your program do different things.  Here's a little bit
    // to get you started doing manual parsing.
    //
    // Challenge: If you're feeling really ambitious, you could delete this code
    // and use the "clap" library instead: https://docs.rs/clap/2.32.0/clap/
    let mut args: Vec<String> = std::env::args().skip(1).collect();
    if args.is_empty() {
        print_usage_and_exit();
    }
    let subcommand = args.remove(0);
    match subcommand.as_str() {
        // EXAMPLE FOR CONVERSION OPERATIONS
        "blur" => {
            if args.len() != 3 {
                print_usage_and_exit();
            }
            let infile = args.remove(0);
            let outfile = args.remove(0);
            let level = args.remove(0);

            blur(infile, outfile, level);
        }

        // **OPTION**
        // Brighten -- see the brighten() function below
        "brighten" => {
            if args.len() != 3 {
                print_usage_and_exit();
            }
            let infile = args.remove(0);
            let outfile = args.remove(0);
            let level = args.remove(0);

            brighten(infile, outfile, level);
        }

        "crop" => {
            if args.len() != 5 {
                print_usage_and_exit();
            }
            let infile = args.remove(0);
            let outfile = args.remove(0);
            let x = args.remove(0);
            let y = args.remove(0);
            let width = args.remove(0);
            let height = args.remove(0);

            crop(infile, outfile, x, y, width, height);
        }

        "rotate" => {
            if args.len() != 3 {
                print_usage_and_exit();
            }
            let infile = args.remove(0);
            let outfile = args.remove(0);
            let angle = args.remove(0);

            rotate(infile, outfile, angle);
        }

        "invert" => {
            if args.len() != 2 {
                print_usage_and_exit();
            }
            let infile = args.remove(0);
            let outfile = args.remove(0);

            invert(infile, outfile);
        }

        "grayscale" => {
            if args.len() != 2 {
                print_usage_and_exit();
            }
            let infile = args.remove(0);
            let outfile = args.remove(0);

            grayscale(infile, outfile);
        }

        // **OPTION**
        // Crop -- see the crop() function below

        // **OPTION**
        // Rotate -- see the rotate() function below

        // **OPTION**
        // Invert -- see the invert() function below

        // **OPTION**
        // Grayscale -- see the grayscale() function below

        // A VERY DIFFERENT EXAMPLE...a really fun one. :-)
        "fractal" => {
            if args.len() != 1 {
                print_usage_and_exit();
            }
            let outfile = args.remove(0);
            fractal(outfile);
        }

        // **OPTION**
        // Generate -- see the generate() function below -- this should be sort of like "fractal()"!

        // For everything else...
        _ => {
            print_usage_and_exit();
        }
    }
}

fn print_usage_and_exit() {
    println!("USAGE (when in doubt, use a .png extension on your filenames)");
    println!("blur INFILE OUTFILE");
    println!("fractal OUTFILE");
    // **OPTION**
    // Print useful information about what subcommands and arguments you can use
    // println!("...");
    std::process::exit(-1);
}

fn blur(infile: String, outfile: String, level: String) {
    // Here's how you open an existing image file
    let img = image::open(infile).expect("Failed to open INFILE.");
    // **OPTION**
    // Parse the blur amount (an f32) from the command-line and pass it through
    // to this function, instead of hard-coding it to 2.0.
    let value :f32 = level.parse().expect("Failed to parse a number"); 
    let img2 = img.blur(value);
    // Here's how you save an image to a file.
    img2.save(outfile).expect("Failed writing OUTFILE.");
}

fn brighten(infile: String, outfile: String, level: String) {
    // See blur() for an example of how to open / save an image.
    let img = image::open(infile).expect("Failed to open INFILE.");

    // .brighten() takes one argument, an i32.  Positive numbers brighten the
    // image. Negative numbers darken it.  It returns a new image.

    let value :i32 = level.parse().expect("Failed to parse a number"); 
    let img2 = img.brighten(value);

    // Challenge: parse the brightness amount from the command-line and pass it
    // through to this function.
    img2.save(outfile).expect("Failed writing OUTFILE.");
}

fn crop(infile: String, outfile: String, x: String, y: String, width: String, height: String) {
    // See blur() for an example of how to open an image.
    let img = image::open(infile).expect("Failed to open INFILE.");
    // .crop() takes four arguments: x: u32, y: u32, width: u32, height: u32
    // You may hard-code them, if you like.  It returns a new image.

    x_value :u32 = x.parse().expect("Failed to parse a number");
    y_value :u32 = y.parse().expect("Failed to parse a number");
    width_value :u32 = width.parse().expect("Failed to parse a number");
    height_value :u32 = height.parse().expect("Failed to parse a number");
   

    // Challenge: parse the four values from the command-line and pass them
    // through to this function.

    let img2 = img.crop(x_value, y_value, width_value, height_value);

    // See blur() for an example of how to save the image.
    img2.save(outfile).expect("Failed writing OUTFILE.");
}

fn rotate(infile: String, outfile: String, angle: String) {
    // See blur() for an example of how to open an image.

    let img = image::open(infile).expect("Failed to open INFILE.");

    // There are 3 rotate functions to choose from (all clockwise):
    //   .rotate90()
    //   .rotate180()
    //   .rotate270()
    // All three methods return a new image.  Pick one and use it!

    match angle.as_str() {
        "90" => {
            let img2 = img.rotate90();
            img2.save(outfile).expect("Failed writing OUTFILE.");
        }
        "180" => {
            let img2 = img.rotate180();
            img2.save(outfile).expect("Failed writing OUTFILE.");
        }
        "270" => {
            let img2 = img.rotate270();
            img2.save(outfile).expect("Failed writing OUTFILE.");
        }
        _ => {
            print_usage_and_exit();
        }
    }
    // Challenge: parse the rotation amount from the command-line, pass it
    // through to this function to select which method to call.

    // See blur() for an example of how to save the image.
}

fn invert(infile: String, outfile: String) {
    // See blur() for an example of how to open an image.
    let img = image::open(infile).expect("Failed to open INFILE.");

    // .invert() takes no arguments and converts the image in-place, so you
    // will use the same image to save out to a different file.

    let img2 = img.invert();

    img2.save(outfile).expect("Failed writing OUTFILE.");
    // See blur() for an example of how to save the image.
}

fn grayscale(infile: String, outfile: String) {
    // See blur() for an example of how to open an image.
    let img = image::open(infile).expect("Failed to open INFILE.");

    // .grayscale() takes no arguments. It returns a new image.
    let img2 = img.grayscale();

    // See blur() for an example of how to save the image.
    img2.save(outfile).expect("Failed writing OUTFILE.");
}

fn generate(outfile: String) {
    // Create an ImageBuffer -- see fractal() for an example
    let width = 800;
    let height = 800;

    let mut imgbuf = image::ImageBuffer::new(width, height);
    

    // Iterate over the coordinates and pixels of the image -- see fractal() for an example
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        // Use red, green, blue, and alpha to be a pretty gradient background
        let red = (0.3 * x as f32) as u8;
        let green = (0.3 * y as f32) as u8;
        let blue = (0.3 * x as f32) as u8;
        let alpha = (0.3 * y as f32) as u8;

        // Actually set the pixel. red, green, blue, and alpha are u8 values!
        *pixel = image::Rgba([red, green, blue, alpha]);
    }

    // Set the image to some solid color. -- see fractal() for an example

    // Challenge: parse some color data from the command-line, pass it through
    // to this function to use for the solid color.

    // Challenge 2: Generate something more interesting!

    // See blur() for an example of how to save the image
}

// This code was adapted from https://github.com/PistonDevelopers/image
fn fractal(outfile: String) {
    let width = 800;
    let height = 800;

    let mut imgbuf = image::ImageBuffer::new(width, height);

    let scale_x = 3.0 / width as f32;
    let scale_y = 3.0 / height as f32;

    // Iterate over the coordinates and pixels of the image
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        // Use red and blue to be a pretty gradient background
        let red = (0.3 * x as f32) as u8;
        let blue = (0.3 * y as f32) as u8;

        // Use green as the fractal foreground (here is the fractal math part)
        let cx = y as f32 * scale_x - 1.5;
        let cy = x as f32 * scale_y - 1.5;

        let c = num_complex::Complex::new(-0.4, 0.6);
        let mut z = num_complex::Complex::new(cx, cy);

        let mut green = 0;
        while green < 255 && z.norm() <= 2.0 {
            z = z * z + c;
            green += 1;
        }

        // Actually set the pixel. red, green, and blue are u8 values!
        *pixel = image::Rgb([red, green, blue]);
    }

    imgbuf.save(outfile).unwrap();
}

// **SUPER CHALLENGE FOR LATER** - Let's face it, you don't have time for this during class.
//
// Make all of the subcommands stackable!
//
// For example, if you run:
//
//   cargo run infile.png outfile.png blur 2.5 invert rotate 180 brighten 10
//
// ...then your program would:
// - read infile.png
// - apply a blur of 2.5
// - invert the colors
// - rotate the image 180 degrees clockwise
// - brighten the image by 10
// - and write the result to outfile.png
//
// Good luck!
