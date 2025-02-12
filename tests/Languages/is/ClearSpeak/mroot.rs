use crate::common::*;

#[test]
fn msqrt_simple() {
    let expr = "<math>
                    <msqrt> <mi>x</mi> </msqrt>
                </math>";
    test("is", "ClearSpeak", expr, "ferningsrótin af x,");
}

#[test]
fn msqrt_simple_end_root() {
    let expr = "<math>
                    <msqrt> <mi>x</mi> </msqrt>
                </math>";
    test_ClearSpeak("is", "ClearSpeak_Roots", "RootEnd", expr, "ferningsrótin af x, rót endar;");
}

#[test]
fn msqrt_simple_positive() {
    let expr = "<math>
                    <msqrt> <mi>x</mi> </msqrt>
                </math>";
    test_ClearSpeak("is", "ClearSpeak_Roots", "PosNegSqRoot", expr, "jákvæð ferningsrótin af x,");
}

#[test]
fn msqrt_simple_pos_end_root() {
    let expr = "<math>
                    <msqrt> <mi>x</mi> </msqrt>
                </math>";
    test_ClearSpeak("is", "ClearSpeak_Roots", "PosNegSqRootEnd", expr, "jákvæð ferningsrótin af x, rót endar;");
}

#[test]
fn msqrt_simple_pos_end_with_neg_root() {
    let expr = "<math>
                    <mo>-</mo> <msqrt> <mi>x</mi> </msqrt>
                    <mo>-</mo> <mroot> <mi>x</mi> <mn>3</mn></mroot>
                </math>";
    test_ClearSpeak("is", "ClearSpeak_Roots", "PosNegSqRootEnd", expr, 
    "neikvæð ferningsrótin af x, rót endar; mínus, jákvæð teningsrótin af x, rót endar;");
}

#[test]
fn mroot_simple_pos_end_with_neg_root() {
    let expr = "<math>
                    <mo>-</mo> <mroot> <mi>x</mi> <mn>3</mn></mroot>
                    <mo>-</mo> <msqrt> <mi>x</mi> </msqrt>

                </math>";
    test_ClearSpeak("is", "ClearSpeak_Roots", "PosNegSqRoot", expr, 
    "neikvæð teningsrótin af x; mínus jákvæð ferningsrótin af x,");
}

#[test]
fn neg_without_root() {
    let expr = "<math>
                    <mo>-</mo> <mi>x</mi> <mo>-</mo> <mi>y</mi>
                </math>";
    test("is", "ClearSpeak", expr, "neikvætt x mínus y");
}

#[test]
fn msqrt() {
    let expr = "<math>
                    <msqrt>
                        <mrow> <mi>x</mi> <mo>+</mo> <mi>y</mi> </mrow>
                    </msqrt>
                </math>";
    test("is", "ClearSpeak", expr, "ferningsrótin af x plús y;");
}

#[test]
fn mroot_as_square_root() {
    let expr = "<math>
                    <mroot> <mi>x</mi> <mn>2</mn> </mroot>
                </math>";
    test("is", "ClearSpeak", expr, "ferningsrótin af x,");
}

#[test]
fn cube_root() {
    let expr = "<math>
                    <mroot> <mi>x</mi> <mn>3</mn> </mroot>
                </math>";
    test("is", "ClearSpeak", expr, "teningsrótin af x,");
}

#[test]
fn ordinal_root() {
    let expr = "<math>
                    <mroot> <mi>x</mi> <mn>9</mn> </mroot>
                </math>";
    test("is", "ClearSpeak", expr, "níunda rótin af x,");
}

#[test]
fn simple_mi_root() {
    let expr = "<math>
                    <mroot> <mi>x</mi> <mi>n</mi> </mroot>
                </math>";
    test("is", "ClearSpeak", expr, "n -ta rótin af x,");
}

#[test]
fn mroot_simple_pos_end_root() {
    let expr = "<math>
                <mroot> <mi>x</mi> <mi>t</mi> </mroot>
                </math>";
    test_ClearSpeak("is", "ClearSpeak_Roots", "PosNegSqRootEnd", expr, "jákvæð t -ta rótin af x, rót endar;");
}

#[test]
fn mroot_simple_end_root() {
    let expr = "<math>
                    <mroot> <mrow> <mi>x</mi> <mo>+</mo> <mi>y</mi> </mrow> 
                    <mn>21</mn></mroot>
                </math>";
    test_ClearSpeak("is", "ClearSpeak_Roots", "RootEnd", expr, "tuttugasta og fyrsta rótin af x plús y, rót endar;");
}

#[test]
fn simple_fraction_power() {
    let expr = "<math>
                    <mroot>
                        <mi>x</mi> 
                        <mfrac><mn>1</mn><mn>3</mn></mfrac>
                    </mroot>
                </math>";
    test("is", "ClearSpeak", expr, "1 þriðji rótin af x,");
}
