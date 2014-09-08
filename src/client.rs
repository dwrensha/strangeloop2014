#![allow(dead_code)]

extern crate capnp;
extern crate strangeloop_include_generated;
pub use strangeloop_include_generated::image_capnp;

mod things {
//    use capnp::{MessageBuilder};
    use image_capnp::{image, analysis_result, detected_object};

    pub fn print_result(result : analysis_result::Reader) {
        for object in result.get_objects().iter() {
            let aabb = object.get_bounding_box();
            println!("object at ({},{}) - ({},{})",
                     aabb.get_min_x(), aabb.get_min_y(),
                     aabb.get_max_x(), aabb.get_max_y());

            match object.which() {
                Some(detected_object::Person(p)) => {
                    println!("  a person of height {}", p.get_height());
                }
                Some(detected_object::Cat(c)) => {
                    println!("  a cat with {} colors",
                             c.get_fur_colors().size());
                }
                None => println!("  unknown object")
            }
        }
    }
}

pub fn main() {
    println!("hi");
}
