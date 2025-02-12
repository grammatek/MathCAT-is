use crate::common::*;

#[test]
fn multiplication() {
    let expr = "<math>
                    <mn>2</mn><mo>×</mo><mn>3</mn>
                </math>";
    test("is", "ClearSpeak", expr, "2 sinnum 3");
}

#[test]
fn multiplication_by() {
    let expr = "<math>
                    <mn>2</mn><mo>×</mo><mn>3</mn>
                </math>";
    test_ClearSpeak("is", "ClearSpeak_MultSymbolX", "By", expr, "2 sinnum 3");
}

#[test]
fn multiplication_cross() {
    let expr = "<math>
                    <mi>u</mi><mo>×</mo><mi>v</mi>
                </math>";
    test_ClearSpeak("is", "ClearSpeak_MultSymbolX", "Cross", expr, "u kross v");
}

#[test]
fn ellipses_auto_start() {
    let expr = "<math>
            <mi>…</mi><mo>,</mo>
            <mo>-</mo><mn>2</mn><mo>,</mo><mo>-</mo><mn>1</mn><mo>,</mo><mn>0</mn>
        </math>";
    test("is", "ClearSpeak", expr, "punktur punktur punktur komma mínus 2 komma mínus 1 komma 0");
}

#[test]
fn ellipses_auto_end() {
    let expr = "<math>
            <mn>1</mn>
            <mo>,</mo>
            <mn>2</mn>
            <mo>,</mo>
            <mn>3</mn>
            <mo>,</mo>
            <mi>…</mi>
        </math>";
    test_ClearSpeak("is", "ClearSpeak_Ellipses", "Auto", expr, "1 komma 2 komma 3 komma punktur punktur punktur");
}

#[test]
fn ellipses_auto_middle() {
    let expr = "<math>
            <mrow>
                <mn>1</mn>
                <mo>,</mo>
                <mn>2</mn>
                <mo>,</mo>
                <mn>3</mn>
                <mo>,</mo>
                <mi>…</mi>
                <mo>,</mo>
                <mn>20</mn>
            </mrow>
        </math>";
    test_ClearSpeak("is", "ClearSpeak_Ellipses", "Auto", expr,
            "1 komma 2 komma 3 komma punktur punktur punktur, komma 20");
}

#[test]
fn ellipses_auto_both() {
    let expr = "<math>
            <mi>…</mi><mo>,</mo>
            <mo>-</mo><mn>2</mn><mo>,</mo><mo>-</mo><mn>1</mn><mo>,</mo><mn>0</mn><mo>,</mo><mn>1</mn><mo>,</mo><mn>2</mn>
            <mo>,</mo><mi>…</mi>
       </math>";
    test_ClearSpeak("is", "ClearSpeak_Ellipses", "Auto", expr,
            "punktur punktur punktur komma mínus 2 komma mínus 1 komma 0 komma 1 komma 2 komma punktur punktur punktur");
}

#[test]
fn ellipses_and_so_on_start() {
    let expr = "<math>
            <mi>…</mi><mo>,</mo>
            <mo>-</mo><mn>2</mn><mo>,</mo><mo>-</mo><mn>1</mn><mo>,</mo><mn>0</mn>
        </math>";
        test_ClearSpeak("is", "ClearSpeak_Ellipses", "AndSoOn", expr, "punktur punktur punktur komma mínus 2 komma mínus 1 komma 0");
}

#[test]
fn ellipses_and_so_on_end() {
    let expr = "<math>
            <mn>1</mn>
            <mo>,</mo>
            <mn>2</mn>
            <mo>,</mo>
            <mn>3</mn>
            <mo>,</mo>
            <mi>…</mi>
        </math>";
    test_ClearSpeak("is", "ClearSpeak_Ellipses", "AndSoOn", expr, "1 komma 2 komma 3 og svo framvegis");
}

#[test]
fn ellipses_and_so_on_middle() {
    let expr = "<math>
            <mrow>
                <mn>1</mn>
                <mo>,</mo>
                <mn>2</mn>
                <mo>,</mo>
                <mn>3</mn>
                <mo>,</mo>
                <mi>…</mi>
                <mo>,</mo>
                <mn>20</mn>
            </mrow>
        </math>";
    test_ClearSpeak("is", "ClearSpeak_Ellipses", "AndSoOn", expr,
            "1 komma 2 komma 3 og svo framvegis upp að, 20");
}

#[test]
fn ellipses_and_so_on_both() {
    let expr = "<math>
            <mi>…</mi><mo>,</mo>
            <mo>-</mo><mn>2</mn><mo>,</mo><mo>-</mo><mn>1</mn><mo>,</mo><mn>0</mn><mo>,</mo><mn>1</mn><mo>,</mo><mn>2</mn>
            <mo>,</mo><mi>…</mi>
       </math>";
    test_ClearSpeak("is", "ClearSpeak_Ellipses", "AndSoOn", expr,
            "punktur punktur punktur komma mínus 2 komma mínus 1 komma 0 komma 1 komma 2 komma punktur punktur punktur");
}

#[test]
fn vertical_line_auto() {
    let expr = "<math>
        <mn>3</mn><mo>|</mo><mn>6</mn>
    </math>";
    test_ClearSpeak("is", "ClearSpeak_VerticalLine", "Auto", expr,
            "3 skiptir 6");
}

#[test]
fn vertical_line_divides() {
    let expr = "<math>
        <mn>3</mn><mo>|</mo><mn>6</mn>
    </math>";
    test_ClearSpeak("is", "ClearSpeak_VerticalLine", "Divides", expr,
            "3 skiptir 6");
}

    #[test]
    fn vertical_line_given() {
        let expr = "<math>
            <mn>3</mn><mo>|</mo><mn>6</mn>
        </math>";
        test_ClearSpeak("is", "ClearSpeak_VerticalLine", "Given", expr,
                "3 gefið að 6");
    }

    #[test]
    fn vertical_line_probability_given() {
        let expr = "<math>
                <mi>P</mi>
                <mrow>
                    <mo>(</mo>
                    <mrow>
                        <mi>A</mi>
                        <mo>|</mo>
                        <mi>B</mi>
                    </mrow>
                    <mo>)</mo>
                </mrow>
            </math>";
        test_ClearSpeak_prefs("is", vec![("ClearSpeak_VerticalLine", "Given"), ("ClearSpeak_ImpliedTimes", "None")]
                        , expr, "stórt p; svigi opnast, stórt a gefið að stórt b; svigi lokast");
    }

#[test]
fn vertical_line_set() {
    let expr = "<math>
        <mo>{</mo>
        <mrow>
            <mi>x</mi>
            <mo>|</mo>
            <mi>x</mi>
            <mo>&gt;</mo>
            <mn>0</mn>
        </mrow>
        <mo>}</mo>    
    </math>";
    test_ClearSpeak("is", "ClearSpeak_VerticalLine", "Auto", expr,
            "mengi allra x þannig að x er stærra en 0");
}


#[test]
fn vertical_line_set_such_that() {
    let expr = "<math>
        <mo>{</mo>
        <mrow>
            <mi>x</mi>
            <mo>|</mo>
            <mi>x</mi>
            <mo>&gt;</mo>
            <mn>0</mn>
        </mrow>
        <mo>}</mo>    
    </math>";
    test_ClearSpeak("is", "ClearSpeak_VerticalLine", "SuchThat", expr,
            "mengi allra x þannig að x er stærra en 0");
}

#[test]
fn vertical_line_set_given() {
    let expr = "<math>
        <mo>{</mo>
        <mrow>
            <mi>x</mi>
            <mo>|</mo>
            <mi>x</mi>
            <mo>&gt;</mo>
            <mn>0</mn>
        </mrow>
        <mo>}</mo>    
    </math>";
    // the rules for set will override all the options -- ClearSpeak spec should be clarified
    test_ClearSpeak("is", "ClearSpeak_VerticalLine", "Given", expr,
            "mengi allra x þannig að x er stærra en 0");
}

#[test]
fn vertical_line_set_and_abs() {
    let expr = "<math>
            <mo>{</mo>
            <mrow>
                <mi>x</mi>
                <mo>&#x007C;</mo>
                <mrow>
                    <mo>|</mo>
                    <mi>x</mi>
                    <mo>|</mo>
                </mrow>
                <mo>&gt;</mo>
                <mn>2</mn>
            </mrow>
            <mo>}</mo>
        </math>";
    test_ClearSpeak("is", "ClearSpeak_VerticalLine", "Auto", expr,
        "mengi allra x þannig að algildi x, er stærra en 2");
}

#[test]
fn vertical_line_evaluated_at() {
    let expr = "<math>
            <mi>f</mi>
            <mrow>
                <mo>(</mo>
                <mi>x</mi>
                <mo>)</mo>
            </mrow>
            <msub>
                <mo>&#x007C;</mo>
                <mrow>
                    <mi>x</mi>
                    <mo>=</mo>
                    <mn>5</mn>
                </mrow>
            </msub>
        </math>";
    test_ClearSpeak("is", "ClearSpeak_VerticalLine", "Auto", expr,
        "f af x gildi fundið við, x er jafnt og 5");
}

#[test]
fn vertical_line_evaluated_at_both() {
    let expr = "<math>
            <msup>
            <mi>x</mi>
            <mn>2</mn>
            </msup>
            <mo>+</mo>
            <mi>x</mi>
            <msubsup>
                <mstyle mathsize='140%' displaystyle='true'> <mo>&#x007C;</mo> </mstyle>
                <mn>0</mn>
                <mn>1</mn>
            </msubsup>
        </math>";
    test_ClearSpeak("is", "ClearSpeak_VerticalLine", "Auto", expr,
        "x í öðru plús x, gildi fundið við 1 að frádreginni sömu stæðu þar sem gildi var fundið við 0");
}
#[test]
fn vertical_line_evaluated_at_divides() {
    let expr = "<math>
            <mi>f</mi>
            <mrow>
                <mo>(</mo>
                <mi>x</mi>
                <mo>)</mo>
            </mrow>
            <msub>
                <mo>&#x007C;</mo>
                <mrow>
                    <mi>x</mi>
                    <mo>=</mo>
                    <mn>5</mn>
                </mrow>
            </msub>
        </math>";
    test_ClearSpeak("is", "ClearSpeak_VerticalLine", "Divides", expr,
        "f af x gildi fundið við, x er jafnt og 5");
}

#[test]
fn vertical_line_evaluated_at_both_given() {
    let expr = "<math>
            <msup>
            <mi>x</mi>
            <mn>2</mn>
            </msup>
            <mo>+</mo>
            <mi>x</mi>
            <msubsup>
                <mstyle mathsize='140%' displaystyle='true'> <mo>&#x007C;</mo> </mstyle>
                <mn>0</mn>
                <mn>1</mn>
            </msubsup>
        </math>";
    test_ClearSpeak("is", "ClearSpeak_VerticalLine", "Given", expr,
        "x í öðru plús x, gildi fundið við 1 að frádreginni sömu stæðu þar sem gildi var fundið við 0");
}