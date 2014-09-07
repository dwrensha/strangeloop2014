@0xf458422d115a3938;

struct Color {
  red   @0 : UInt8;
  green @1 : UInt8;
  blue  @2 : UInt8;
}

struct Image {
  width  @0 : UInt16;
  height @1 : UInt16;
  pixels @2 : List(Color);
  # width * height pixels in row-major order
}


struct AnalysisResult {
  objects @0 : List(DetectedObject);
}

struct AxisAlignedBoundingBox {
  minX @0 : UInt16;
  maxX @1 : UInt16;
  minY @2 : UInt16;
  maxY @3 : UInt16;
}

struct DetectedObject {
  boundingBox @0 : AxisAlignedBoundingBox;
  union {
    person @1 : Person;
    cat    @2 : Cat;
  }
}

struct Person {
  height @0 : Float32; # in meters
}

struct Cat {
  furColors @0 : List(Color);
}



interface ObjectDetector {
  doAnalysis @0 Image -> AnalysisResult;
}
