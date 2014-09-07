extern crate capnp;
extern crate strangeloop_include_generated;
pub use strangeloop_include_generated::image_capnp;

mod things {
    use capnp::{MessageBuilder};
    use image_capnp::{image, pixel, analysis_result, detected_object};

    fn use_result(result : analysis_result::Reader) {
        for object in result.get_objects().iter() {

            match object.which() {
                Some(detected_object::Person(p)) => {}
                _ => {}
            }

        }
    }
}

pub fn main() {
    println!("hi");
}
