use crate::common::*;

#[test]
fn menclose_actuarial() {
    let expr = "<math>
                    <menclose notation='actuarial'>  <mn>3</mn><mo>+</mo><mn>2</mn><mi>i</mi> </menclose>
                </math>";
    test("is", "ClearSpeak", expr, "tryggingamerki, umlykja 3 plús 2 i umlykja endar,");
}

#[test]
fn menclose_box() {
    let expr = "<math>
                    <menclose notation='box circle'>  <mn>3</mn><mo>+</mo><mn>2</mn><mi>i</mi> </menclose>
                </math>";
    test("is", "ClearSpeak", expr, "kassi, hringur, umlykja 3 plús 2 i umlykja endar,");
}

#[test]
fn menclose_left() {
    let expr = "<math>
                    <menclose notation='left'>  <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("is", "ClearSpeak", expr, "lína vinstra megin, umlykja 3 aðrir umlykja endar,");
}

#[test]
fn menclose_right() {
    let expr = "<math>
                    <menclose notation='right'>  <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("is", "ClearSpeak", expr, "lína hægra megin, umlykja 3 aðrir umlykja endar,");
}

#[test]
fn menclose_top_bottom() {
    let expr = "<math>
                    <menclose notation='top bottom'>  <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("is", "ClearSpeak", expr, "lína fyrir ofan, fyrir neðan, umlykja 3 aðrir umlykja endar,");
}

#[test]
fn menclose_updiagonalstrike() {
    let expr = "<math>
                    <menclose notation='updiagonalstrike'>  <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("is", "ClearSpeak", expr, "upp á ská, útstrikun, umlykja 3 aðrir umlykja endar,");
}

#[test]
fn menclose_downdiagonalstrike() {
    let expr = "<math>
                    <menclose notation='downdiagonalstrike'>  <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("is", "ClearSpeak", expr, "niður á ská, útstrikun, umlykja 3 aðrir umlykja endar,");
}

#[test]
fn menclose_cross_out() {
    let expr = "<math>
                    <menclose notation='updiagonalstrike downdiagonalstrike'>  <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("is", "ClearSpeak", expr, "x, útstrikun, umlykja 3 aðrir umlykja endar,");
}

#[test]
fn menclose_vertical_horizontal_strike() {
    let expr = "<math>
                    <menclose notation='verticalstrike horizontalstrike'>  <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("is", "ClearSpeak", expr, "lóðrétt, lárétt, útstrikun, umlykja 3 aðrir umlykja endar,");
}

#[test]
fn menclose_leftarrow() {
    let expr = "<math>
                    <menclose notation='leftarrow'> <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("is", "ClearSpeak", expr, "ör til vinstri, umlykja 3 aðrir umlykja endar,");
}

#[test]
fn menclose_right_up_down_arrow() {
    let expr = "<math>
                    <menclose notation=' rightarrow downarrow  uparrow  '> <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("is", "ClearSpeak", expr, "ör upp, ör niður, ör til hægri, umlykja 3 aðrir umlykja endar,");
}

#[test]
fn menclose_northeastarrow() {
    let expr = "<math>
                    <menclose notation='northeastarrow'> <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("is", "ClearSpeak", expr, "ör í norðaustur, umlykja 3 aðrir umlykja endar,");
}

#[test]
fn menclose_other_single_arrows() {
    let expr = "<math>
                    <menclose notation='northwestarrow southwestarrow southeastarrow'> <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("is", "ClearSpeak", expr, "ör í suðaustur, ör í suðvestur, ör í norðvestur, umlykja 3 aðrir umlykja endar,");
}

#[test]
fn menclose_northwestsoutheastarrow() {
    let expr = "<math>
                    <menclose notation='northwestsoutheastarrow'> <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("is", "ClearSpeak", expr, "tvíátta ör niður á ská, umlykja 3 aðrir umlykja endar,");
}

#[test]
fn menclose_other_double_arrows() {
    let expr = "<math>
                    <menclose notation='updownarrow leftrightarrow northeastsouthwestarrow'>  <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("is", "ClearSpeak", expr, "tvíátta lóðrétt ör, tvíátta lárétt ör, tvíátta ör upp á ská, umlykja 3 aðrir umlykja endar,");
}

#[test]
fn menclose_madrub() {
    let expr = "<math>
                    <menclose notation='madrub'> <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("is", "ClearSpeak", expr, "arabískt aðfeldismerki, umlykja 3 aðrir umlykja endar,");
}

#[test]
fn menclose_phasorangle() {
    let expr = "<math>
                    <menclose notation='phasorangle'> <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("is", "ClearSpeak", expr, "fasorhorn, umlykja 3 aðrir umlykja endar,");
}

#[test]
fn menclose_circle_phasorangle() {
    let expr = "<math>
                    <menclose notation='phasorangle circle'> <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("is", "ClearSpeak", expr, "hringur, fasorhorn, umlykja 3 aðrir umlykja endar,");
}

#[test]
fn menclose_longdiv() {
    let expr = "<math>
                    <menclose notation='longdiv'> <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("is", "ClearSpeak", expr, "löng deiling, umlykja 3 aðrir umlykja endar,");
}

#[test]
fn menclose_longdiv_default() {
    let expr = "<math>
                    <menclose> <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("is", "ClearSpeak", expr, "löng deiling, umlykja 3 aðrir umlykja endar,");
}

#[test]
fn menclose_longdiv_empty_string() {
    let expr = "<math>
                    <menclose notation=''> <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("is", "ClearSpeak", expr, "löng deiling, umlykja 3 aðrir umlykja endar,");
}

#[test]
fn menclose_longdiv_whitespace_string() {
    let expr = "<math>
                    <menclose notation='  '> <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("is", "ClearSpeak", expr, "löng deiling, umlykja 3 aðrir umlykja endar,");
}

#[test]
fn menclose_radical() {
    let expr = "<math>
                    <menclose notation='radical'> <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("is", "ClearSpeak", expr, "rótin, umlykja 3 aðrir umlykja endar,");
}

#[test]
fn simple_speak_menclose_top_bottom() {
    let expr = "<math>
                    <menclose notation='top bottom'>  <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("is", "SimpleSpeak", expr, "lína fyrir ofan, fyrir neðan, umlykja 3 aðrir umlykja endar,");
}
