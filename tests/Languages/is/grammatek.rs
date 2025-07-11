/// Tests cases created during development of MathCAT-is Grammatek version
use crate::common::*;


#[test]
fn times() {
    let expr = "<math> <mn>2</mn> <mo>&#x22C5;</mo><mn>0</mn></math>";
    test("is", "SimpleSpeak", expr, "2 sinnum 0");
    test("is", "ClearSpeak", expr, "2 sinnum 0");
}