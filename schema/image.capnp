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
  # width * height pixels in row-major order
}


struct AnalysisResult {
  objects @0 : List(DetectedObject);
}

struct Coordinates {
  x @0 : UInt16;
  y @1 : UInt16;
}

struct DetectedObject {
  position @0 : Coordinates;
  union {
    person @1 : Person;
    cat    @2 : Cat;
    dog    @3 : Dog;
  }
}

struct Person {

}

struct Cat {

}

struct Dog {

}