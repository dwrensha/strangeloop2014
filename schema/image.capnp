@0xf458422d115a3938;

struct Pixel {
  red @0 : UInt8;
  green @1 : UInt8;
  blue @2 : UInt8;
}

struct Image {
  width @0 : UInt16;
  height @1 : UInt16;
  pixels @2 : List(Pixel);
  # the width * height pixels in row-major order
}


struct Analysis {
  objects @0 : List(DetectedObject);
}

struct DetectedObject {
  union {
    person @0 : Person;
    cat    @1 : Cat;
    dog    @2 : Dog;
  }
}

struct Person {

}

struct Cat {

}

struct Dog {

}