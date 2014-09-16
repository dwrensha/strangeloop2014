#![allow(dead_code)]

extern crate capnp;
extern crate strangeloop_include_generated;

pub use strangeloop_include_generated::image_capnp;

mod things {
    use capnp::{MessageBuilder, MessageReader};
    use image_capnp::{color, image, analysis_result, object_detector};


    pub fn average_pixel(image : image::Reader,
                         average_pixel : color::Builder) {
        let (mut red_total,
             mut green_total,
             mut blue_total) = (0u64, 0u64, 0u64);

        for pixel in image.get_pixels().iter() {
            red_total   += pixel.get_red() as u64;
            green_total += pixel.get_green() as u64;
            blue_total  += pixel.get_blue() as u64;
        }

        let size = image.get_pixels().size() as u64;
        average_pixel.set_red((red_total / size) as u8);
        average_pixel.set_green((green_total / size) as u8);
        average_pixel.set_blue((blue_total / size) as u8);
    }


    pub fn pixel_at(image : image::Reader,
                    x : u16,
                    y : u16) -> color::Reader {

        image.get_pixels()
             .get(x as uint +
                  y as uint * image.get_width() as uint)

    }

    fn pixel_at_wrong<'a>(image : image::Reader<'a>,
                          x : u16,
                          y : u16) -> color::Reader<'a> {

        //::capnp::MallocMessageBuilder::new_default()
        //    .init_root::<color::Builder>().as_reader()
        fail!()
    }

    fn find_match<'a, 'b>(image : image::Reader<'a>,
                          color : color::Reader<'b>)
                          -> color::Reader<'a> {
        fail!();
//        return color;
    }


    fn analyze_image(image : image::Reader, result : analysis_result::Builder) {
        /* magic analysis algorithm */
    }

    pub fn serve() -> ::std::io::IoResult<()> {
        let message_reader =
            try!(::capnp::serialize::new_reader(
                &mut ::std::io::stdin(),
                ::capnp::ReaderOptions::new()));

        let mut message_builder =
            ::capnp::MallocMessageBuilder::new_default();

        analyze_image(message_reader.get_root(),
                      message_builder.init_root());

        ::capnp::serialize::write_message(&mut ::std::io::stdout(),
                                          &message_builder)
    }

    pub fn serve_wrong() -> ::std::io::IoResult<()> {
        let message_reader =
            try!(::capnp::serialize::new_reader(
                &mut ::std::io::stdin(),
                ::capnp::ReaderOptions::new()));

        let mut message_builder =
            ::capnp::MallocMessageBuilder::new_default();

        //let result =
        //    message_builder.init_root::<analysis_result::Builder>();

        analyze_image(message_reader.get_root(),
                      message_builder.init_root());

        //result.init_objects(0);

        Ok(())
        //::capnp::serialize::write_message(&mut ::std::io::stdout(),
        //                                  &message_builder)
    }


    struct ObjectDetectorImpl;

    impl object_detector::Server for ObjectDetectorImpl {
        fn analyze(&mut self, mut context : object_detector::AnalyzeContext) {
            let (params, results) = context.get();
            analyze_image(params, results);
            context.done();
        }
    }

}

pub fn main() {
    println!("hi");


}
