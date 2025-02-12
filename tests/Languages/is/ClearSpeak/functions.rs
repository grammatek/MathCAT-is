/// Tests for:
/// *  functions including trig functions, logs, and functions to powers
/// *  implied times/functional call and explicit times/function call
/// *  parens
/// These are all intertwined, so they are in one file
use crate::common::*;

#[test]
fn trig_names() {
    let expr = "<math><mrow>
    <mi>sin</mi><mi>x</mi><mo>+</mo>
    <mi>cos</mi><mi>y</mi><mo>+</mo>
    <mi>tan</mi><mi>z</mi><mo>+</mo>
    <mi>sec</mi><mi>&#x03B1;</mi><mo>+</mo>
    <mi>csc</mi><mi>&#x03D5;</mi><mo>+</mo>
    <mi>cot</mi><mi>&#x03C6;</mi>
    </mrow></math>";
    test("is", "ClearSpeak", expr, "sínus af x plús kósínus af y, plús tangens af z plús sekans af alfa, plús kósekans af fí, plús kótangens af fí");
}

#[test]
fn hyperbolic_trig_names() {
    let expr = "<math><mrow>
    <mi>sinh</mi><mi>x</mi><mo>+</mo>
    <mi>cosh</mi><mi>y</mi><mo>+</mo>
    <mi>tanh</mi><mi>z</mi><mo>+</mo>
    <mi>sech</mi><mi>&#x03B1;</mi><mo>+</mo>
    <mi>csch</mi><mi>&#x03D5;</mi><mo>+</mo>
    <mi>coth</mi><mi>&#x03C6;</mi>
    </mrow></math>";
    test("is", "ClearSpeak", expr, "breiðbogasínus af x, plús \
                                breiðbogakósínus af y, plús \
                                breiðbogatangens af z, plús \
                                breiðbogasekans af alfa, plús \
                                breiðbogakósekans af fí, plús, \
                                breiðbogakótangens af fí");
}


#[test]
fn inverse_trig() {
    let expr = "<math><msup><mi>sin</mi><mrow><mo>-</mo><mn>1</mn></mrow></msup><mi>x</mi></math>";
    test("is", "ClearSpeak", expr, "andhverfan sínus af x");
}

#[test]
fn inverse_trig_trig_inverse() {
    let expr = "<math><msup><mi>tan</mi><mrow><mo>-</mo><mn>1</mn></mrow></msup><mi>x</mi></math>";
    test_ClearSpeak("is", "ClearSpeak_Trig", "TrigInverse",expr,
        "tangens andhverfan af x");
}

#[test]
fn inverse_trig_arc() {
    let expr = "<math><msup><mi>cosh</mi><mrow><mo>-</mo><mn>1</mn></mrow></msup><mi>x</mi></math>";
    test_ClearSpeak("is", "ClearSpeak_Trig", "ArcTrig",expr,
        "bogi breiðbogakósínus af x");
}

#[test]
fn trig_squared() {
    let expr = "<math><msup><mi>sin</mi><mn>2</mn></msup><mi>x</mi></math>";
    test("is", "ClearSpeak", expr, "sínus í öðru af x");
}

#[test]
fn trig_cubed() {
    let expr = "<math><msup><mi>tan</mi><mn>3</mn></msup><mi>x</mi></math>";
    test("is", "ClearSpeak", expr, "tangens í þriðja af x");
}

#[test]
fn trig_fourth() {
    let expr = "<math><msup><mi>sec</mi><mn>4</mn></msup><mi>x</mi></math>";
    test("is", "ClearSpeak", expr, "fjórða veldi af, sekans af x");
}


#[test]
fn trig_power_other() {
    let expr = "<math><msup><mi>sinh</mi><mrow>><mi>n</mi><mo>-</mo><mn>1</mn></mrow></msup><mi>x</mi></math>";
    test("is", "ClearSpeak", expr, "n mínus 1 veldi af, breiðbogasínus af x");
}

#[test]
fn simple_log() {
    let expr = "<math> <mrow>  <mi>log</mi><mi>x</mi></mrow> </math>";
    test("is", "ClearSpeak", expr, "logrinn x");
}

#[test]
fn normal_log() {
    let expr = "<math><mrow><mi>log</mi><mrow><mo>(</mo><mrow><mi>x</mi><mo>+</mo><mi>y</mi></mrow><mo>)</mo></mrow></mrow></math>";
    test("is", "ClearSpeak", expr, "logrinn af, svigi opnast x plús y, svigi lokast");
}

#[test]
fn simple_log_with_base() {
    let expr = "<math> <mrow>  <msub><mi>log</mi><mi>b</mi></msub><mi>x</mi></mrow> </math>";
    test("is", "ClearSpeak", expr, "logri með grunntölu b af x");
}

#[test]
fn normal_log_with_base() {
    let expr = "<math><mrow><msub><mi>log</mi><mi>b</mi></msub><mrow><mo>(</mo><mrow><mi>x</mi><mo>+</mo><mi>y</mi></mrow><mo>)</mo></mrow></mrow></math>";
    test("is", "ClearSpeak", expr, "logri með grunntölu b af, svigi opnast x plús y, svigi lokast");
}

#[test]
fn simple_ln() {
    let expr = "<math> <mrow>  <mi>ln</mi><mi>x</mi></mrow> </math>";
    test("is", "ClearSpeak", expr, "l n x");
}

#[test]
fn normal_ln() {
    let expr = "<math><mrow><mi>ln</mi><mrow><mo>(</mo><mrow><mi>x</mi><mo>+</mo><mi>y</mi></mrow><mo>)</mo></mrow></mrow></math>";
    test("is", "ClearSpeak", expr, "l n af, svigi opnast x plús y, svigi lokast");
}

    
#[test]
fn simple_natural_log() {
    let expr = "<math> <mrow>  <mi>ln</mi><mi>x</mi></mrow> </math>";
    test_ClearSpeak("is", "ClearSpeak_Log", "LnAsNaturalLog",expr,
        "náttúrulegi logrinn x");
}

    
#[test]
fn natural_log() {
    let expr = "<math><mi>ln</mi><mo>(</mo><mi>x</mi><mo>+</mo><mi>y</mi><mo>)</mo></math>";
    test_ClearSpeak("is", "ClearSpeak_Log", "LnAsNaturalLog",expr,
        "náttúrulegi logrinn af, svigi opnast x plús y, svigi lokast");
}


#[test]
fn explicit_function_call_with_parens() {
    let expr = "<math><mrow><mi>t</mi><mo>&#x2061;</mo><mrow><mo>(</mo><mi>x</mi><mo>)</mo></mrow></mrow></math>";
    test("is", "ClearSpeak", expr, "t af x");
}


#[test]
fn explicit_times_with_parens() {
    let expr = "<math><mrow><mi>t</mi><mo>&#x2062;</mo><mrow><mo>(</mo><mi>x</mi><mo>)</mo></mrow></mrow></math>";
    test("is", "ClearSpeak", expr, "t sinnum x");
}

#[test]
fn explicit_function_call() {
    let expr = "<math><mrow><mi>t</mi><mo>&#x2061;</mo><mrow><mi>x</mi></mrow></mrow></math>";
    test("is", "ClearSpeak", expr, "t af x");
}

#[test]
fn explicit_times() {
    let expr = "<math><mrow><mi>t</mi><mo>&#x2062;</mo><mrow><mi>x</mi></mrow></mrow></math>";
    test("is", "ClearSpeak", expr, "t x");
}


#[test]
fn test_functions_none_pref() {
    let expr = "<math>
    <mi>log</mi><mo>&#x2061;</mo><mrow><mo>(</mo><mrow><mi>x</mi><mo>+</mo><mi>y</mi></mrow><mo>)</mo></mrow>
    <mo>+</mo>
    <mi>f</mi><mo>&#x2061;</mo><mrow><mo>(</mo><mrow><mi>x</mi><mo>+</mo><mi>y</mi></mrow><mo>)</mo></mrow>
    </math>";
    test_ClearSpeak("is", "ClearSpeak_Functions", "None",expr,
        "logrinn af, svigi opnast x plús y, svigi lokast; plús, f sinnum, svigi opnast x plús y, svigi lokast");
}

#[test]
fn test_functions_none_pref_multiple_args() {
    let expr = "<math>
        <mi>B</mi> <mrow><mo>(</mo> <mrow> <mn>2</mn><mo>,</mo><mn>6</mn></mrow> <mo>)</mo></mrow>
    </math>";
    test_ClearSpeak("is", "ClearSpeak_Functions", "None",expr,
        "stórt b sinnum, svigi opnast 2 komma 6, svigi lokast");
}


/*
    * Tests for times
    */
#[test]
fn no_times_binomial() {
    let expr = "<math><mrow><mi>x</mi> <mo>&#x2062;</mo> <mi>y</mi></mrow></math>";
    test("is", "ClearSpeak", expr, "x y");
}

#[test]
fn times_following_paren() {
    let expr = "<math><mrow>
        <mn>2</mn>
        <mrow>  <mo>(</mo> <mn>3</mn>  <mo>)</mo> </mrow>
        </mrow></math>";
    test("is", "ClearSpeak", expr, "2 sinnum 3");
}

#[test]
fn times_preceding_paren() {
    let expr = "<math><mrow>
        <mrow>  <mo>(</mo> <mn>2</mn>  <mo>)</mo> </mrow>
        <mn>3</mn>
        </mrow></math>";
    test("is", "ClearSpeak", expr, "2 sinnum 3");
}

#[test]
fn no_times_sqrt() {
    let expr = "<math><mrow>
        <msqrt> <mi>a</mi>  </msqrt>
        <msqrt> <mi>b</mi>  </msqrt>
        <mo>=</mo>
        <msqrt> <mrow>  <mi>a</mi><mi>b</mi></mrow> </msqrt>
        </mrow></math>";
    test("is", "ClearSpeak", expr, "ferningsrótin af a; ferningsrótin af b; er jafnt og, ferningsrótin af a b,");
}

#[test]
fn more_implied_times() {
    let expr = "<math><mrow>
    <mrow>
    <msup>
        <mrow>
        <mrow><mo>(</mo>
        <mrow> <mn>2</mn><mi>x</mi></mrow>
        <mo>)</mo></mrow></mrow>
        <mn>2</mn>
    </msup>
    </mrow>
    </mrow></math>";
    test_ClearSpeak("is", "ClearSpeak_ImpliedTimes", "MoreImpliedTimes",expr,
        "svigi opnast 2 sinnum x, svigi lokast í öðru");
}

#[test]
fn explicit_times_more_implied_times() {
    let expr = "<math><mrow><mi>t</mi><mo>&#x2062;</mo><mrow><mi>x</mi></mrow></mrow></math>";
    test_ClearSpeak("is", "ClearSpeak_ImpliedTimes", "MoreImpliedTimes",expr, "t sinnum x");
}

#[test]
fn explicit_times_none_simple_right() {
    let expr = "<math><mn>2</mn><mo>[</mo><mn>3</mn> <mo>]</mo></math>";
    test_ClearSpeak("is", "ClearSpeak_ImpliedTimes", "None",
        expr, "2, hornklofi opnast 3 hornklofi lokast");
}

#[test]
fn explicit_times_none_simple_left() {
    let expr = "<math><mo>(</mo><mn>2</mn><mo>&#x2212;</mo><mn>1</mn><mo>)</mo><mi>x</mi></math>";
    test_ClearSpeak("is", "ClearSpeak_ImpliedTimes", "None",
        expr, "svigi opnast 2 mínus 1, svigi lokast; x");
}

#[test]
fn explicit_times_none_superscript() {
    let expr = "<math> 
    <mi>f</mi><mo>(</mo><mi>x</mi><mo>)</mo><mo>=</mo><msup>
<mi>x</mi>
<mn>2</mn>
</msup>
<mrow><mo>(</mo>
<mrow>
<mi>x</mi><mo>+</mo><mn>1</mn></mrow>
<mo>)</mo></mrow>
    </math>";
    test_ClearSpeak_prefs("is", 
        vec![("ClearSpeak_ImpliedTimes", "None"), ("ClearSpeak_Functions", "None")],
        expr, "f, svigi opnast x svigi lokast; er jafnt og; x í öðru, svigi opnast x plús 1, svigi lokast");
}

/*
    * Tests for parens
    */
    #[test]
    fn no_parens_number() {
        let expr = "<math><mrow>
        <mrow><mo>(</mo>
        <mn>25</mn>
        <mo>)</mo></mrow>
        <mi>x</mi>
        </mrow></math>";
        test("is", "ClearSpeak", expr, "25 sinnum x");
    }

    #[test]
    fn no_parens_monomial() {
        let expr = "<math><mrow>
        <mi>b</mi>
        <mrow><mo>(</mo>
        <mrow><mi>x</mi><mi>y</mi></mrow>
        <mo>)</mo></mrow>
        </mrow></math>";
        test("is", "ClearSpeak", expr, "b x y");
    }

    #[test]
    fn no_parens_negative_number() {
        let expr = "<math><mrow>
        <mn>2</mn><mo>+</mo>
        <mrow><mo>(</mo>
        <mrow><mo>&#x2212;</mo><mn>2</mn></mrow>
        <mo>)</mo></mrow>
        </mrow></math>";
        test("is", "ClearSpeak", expr, "2 plús mínus 2");
    }


    #[test]
    fn no_parens_negative_number_with_var() {
        let expr = "<math><mrow>
        <mrow><mo>(</mo>
        <mrow><mo>&#x2212;</mo><mn>2</mn></mrow><mi>x</mi>
        <mo>)</mo>
        </mrow>
        <mo>+</mo><mn>1</mn>
        </mrow></math>";
        test("is", "ClearSpeak", expr, "mínus 2 x, plús 1");
    }

    #[test]
    fn parens_superscript() {
        let expr = "<math><mrow>
        <mrow>
        <msup>
        <mrow>
            <mrow><mo>(</mo>
            <mrow> <mn>2</mn><mi>x</mi></mrow>
            <mo>)</mo></mrow></mrow>
        <mn>2</mn>
        </msup>
        </mrow>
    </mrow></math>";
        test("is", "ClearSpeak", expr, "svigi opnast 2 x svigi lokast í öðru");
    }

    #[test]
    fn no_parens_fraction() {
        let expr = "<math><mrow>
        <mn>2</mn>
        <mo>+</mo>
        <mrow>
            <mrow><mo>(</mo>
            <mfrac> <mn>1</mn><mn>2</mn></mfrac>
            <mo>)</mo></mrow></mrow>
    </mrow></math>";
        test("is", "ClearSpeak", expr, "2 plús 1 hálfur");
    }


    // Tests for the ten types of intervals in ClearSpeak
    #[test]
    fn parens_interval_open_open() {
        let expr = "<math> 
        <mrow><mo>(</mo>
        <mrow> <mi>c</mi><mo>,</mo><mi>d</mi></mrow>
        <mo>)</mo></mrow>
    </math>";
    test_ClearSpeak("is", "ClearSpeak_Paren", "Interval",expr,
    "opna bilið frá c til d");
}

#[test]
    fn parens_interval_closed_open() {
        let expr = "<math> 
        <mrow><mo>[</mo>
        <mrow> <mi>c</mi><mo>,</mo><mi>d</mi></mrow>
        <mo>)</mo></mrow>
    </math>";
    test_ClearSpeak("is", "ClearSpeak_Paren", "Interval ",expr,
    "hálfopna bilið frá og með c til d");
}


#[test]
fn parens_interval_open_closed() {
    let expr = "<math> 
    <mrow><mo>(</mo>
        <mrow> <mi>c</mi><mo>,</mo><mi>d</mi></mrow>
    <mo>]</mo></mrow>
    </math>";
    test_ClearSpeak("is", "ClearSpeak_Paren", "Interval ",expr,
    "hálfopna bilið frá c til og með d");
}


#[test]
fn parens_interval_closed_closed() {
    let expr = "<math> 
    <mrow><mo>[</mo>
    <mrow> <mi>c</mi><mo>,</mo><mi>d</mi></mrow>
    <mo>]</mo></mrow>
</math>";
test_ClearSpeak("is", "ClearSpeak_Paren", "Interval ",expr,
"lokaða bilið frá c til d");
}

    #[test]
    fn parens_interval_neg_infinity_open_open() {
        let expr = "<math> 
        <mrow><mo>(</mo>
        <mrow><mo>-</mo> <mi>∞</mi><mo>,</mo><mi>d</mi></mrow>
        <mo>)</mo></mrow>
    </math>";
    test_ClearSpeak("is", "ClearSpeak_Paren", "Interval ",expr,
    "opna bilið frá mínus óendanlegt til d");
}

    #[test]
    fn parens_interval_neg_infinity_closed_open() {
        let expr = "<math> 
        <mrow><mo>(</mo>
        <mrow> <mo>-</mo> <mi>∞</mi><mo>,</mo><mi>d</mi></mrow>
        <mo>]</mo></mrow>
    </math>";
    test_ClearSpeak("is", "ClearSpeak_Paren", "Interval ",expr,
    "hálfopna bilið frá mínus óendanlegt til og með d");
}


#[test]
fn parens_interval_open_open_infinity() {
    let expr = "<math> 
    <mrow><mo>(</mo>
        <mrow> <mi>c</mi><mo>,</mo><mi>∞</mi></mrow>
    <mo>)</mo></mrow>
    </math>";
    test_ClearSpeak("is", "ClearSpeak_Paren", "Interval ",expr,
    "opna bilið frá c til óendanlegt");
}


#[test]
fn parens_interval_closed_open_infinity() {
    let expr = "<math> 
        <mrow><mo>[</mo>
        <mrow> <mi>c</mi><mo>,</mo><mi>∞</mi></mrow>
        <mo>)</mo></mrow>
    </math>";
    test_ClearSpeak("is", "ClearSpeak_Paren", "Interval ",expr,
"hálfopna bilið frá og með c til óendanlegt");
}

#[test]
fn parens_interval_neg_infinity_to_infinity() {
    let expr = "<math> 
    <mrow><mo>(</mo>
        <mrow><mo>-</mo> <mi>∞</mi><mo>,</mo><mi>∞</mi></mrow>
        <mo>)</mo></mrow>
    </math>";
    test_ClearSpeak("is", "ClearSpeak_Paren", "Interval ",expr,
    "opna bilið frá mínus óendanlegt til óendanlegt,");
}

#[test]
fn parens_interval_neg_infinity_to_pos_infinity() {
    let expr = "<math> 
    <mrow><mo>(</mo>
        <mrow><mo>-</mo> <mi>∞</mi><mo>,</mo><mo>+</mo><mi>∞</mi></mrow>
    <mo>)</mo></mrow>
    </math>";
    test_ClearSpeak("is", "ClearSpeak_Paren", "Interval ",expr,
    "opna bilið frá mínus óendanlegt til óendanlegt,");
}
