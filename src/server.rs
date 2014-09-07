extern crate capnp;
extern crate strangeloop_include_generated;

pub use strangeloop_include_generated::image_capnp;

mod things {
    use capnp::{MessageBuilder};
    use image_capnp::{image, pixel, analysis};

    fn average_pixel(image : image::Reader, average_pixel : pixel::Builder) {
        let mut red_total   : u64 = 0;
        let mut green_total : u64 = 0;
        let mut blue_total  : u64 = 0;

        for pixel in image.get_pixels().iter() {
            red_total   += pixel.get_red() as u64;
            green_total += pixel.get_green() as u64;
            blue_total  += pixel.get_blue() as u64;
        }

        let size = image.get_pixels().size() as u64;
        average_pixel.set_red(  (red_total   / size) as u8);
        average_pixel.set_green((green_total / size) as u8);
        average_pixel.set_blue( (blue_total  / size) as u8);
    }


    fn pixel_at(image : image::Reader, x : u16, y : u16) -> pixel::Reader {

        // this doesn't work:
        //return ::capnp::MallocMessageBuilder::new_default().init_root::<pixel::Builder>().as_reader();

        image.get_pixels().get((x as uint + y as uint * image.get_width() as uint))
    }


    fn do_analysis(image : image::Reader, result : analysis::Builder) {
        
    }

}

pub fn main() {
    println!("hi");
}
