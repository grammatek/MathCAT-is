/// Tests for geometry listed in intent
///   ABC as mtext and as separated letters
use crate::common::*;

#[test]
fn arc() {
  let expr = "<math>  <mover><mrow><mi>B</mi><mi>C</mi></mrow><mo>⌒</mo></mover> </math>";
  test("is", "SimpleSpeak", expr, "bogi stórt b stórt c");  // en. 'arc' = 1. 'örk', 'hringbogi', 'bogi' 2. 'bogi', 'ferilbogi'
}

#[test]
fn ray() {
  let expr = "<math> <mover><mrow><mi>X</mi><mi>Y</mi></mrow><mo>&#xAF;</mo></mover> </math>";
  test("is", "SimpleSpeak", expr, "strik stórt x stórt y"); // en. 'line segment' = 'línustrik', 'strik'
}

#[test]
fn arc_mtext() {
  let expr = "<math> <mover><mtext>BC</mtext><mo>⌒</mo></mover> </math>";
  test("is", "SimpleSpeak", expr, "bogi stórt b stórt c");
}

#[test]
fn ray_mtext() {
  let expr = "<math> <mover><mtext>XY</mtext><mo>→</mo></mover> </math>";
  test("is", "SimpleSpeak", expr, "hálflína stórt x stórt y"); // en. 'ray'
}
