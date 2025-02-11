/// Tests for superscripts
///   simple superscripts
///   complex/nested superscripts
use crate::common::*;

#[test]
fn squared() {
    let expr = "<math>
                    <msup> <mi>x</mi> <mn>2</mn> </msup>
                </math>";
    test("is", "SimpleSpeak", expr, "x í öðru");
}

#[test]
fn cubed() {
    let expr = "<math>
                    <msup> <mi>x</mi> <mn>3</mn> </msup>
                </math>";
    test("is", "SimpleSpeak", expr, "x í þriðja");
}

#[test]
    fn ordinal_power() {
        let expr = "<math>
                        <msup> <mi>x</mi> <mn>4</mn> </msup>
                    </math>";
        test("is", "SimpleSpeak", expr, "x í fjórða");
    }

#[test]
fn simple_mi_power() {
    let expr = "<math>
                    <msup> <mi>x</mi> <mi>n</mi> </msup>
                </math>";
  test("is", "SimpleSpeak", expr, "x í n -ta");
}

#[test]
fn zero_power() {
    let expr = "<math>
                    <msup> <mi>x</mi> <mn>0</mn> </msup>
                </math>";
    test("is", "SimpleSpeak", expr, "x í 0-ta");
}


#[test]
fn decimal_power() {
    let expr = "<math>
                    <msup> <mi>x</mi> <mn>2.0</mn> </msup>
                </math>";
    test("is", "SimpleSpeak", expr, "x í 2,0 veldi");
}

#[test]
fn non_simple_power() {
    let expr = "<math>
      <mrow>
      <msup>
        <mn>3</mn>
        <mrow>
        <mi>y</mi><mo>+</mo><mn>2</mn></mrow>
      </msup>
      </mrow>
                </math>";
    test("is", "SimpleSpeak", expr, "3 í y plús 2 veldi,");
}

#[test]
fn negative_power() {
    let expr = "<math>
                    <msup>
                        <mi>x</mi>
                        <mrow> <mo>-</mo> <mn>2</mn> </mrow>
                    </msup>
                </math>";
    test("is", "SimpleSpeak", expr, "x í mínus 2");
}

#[test]
fn simple_fraction_power() {
  let expr = "<math>
                  <msup>
                      <mi>x</mi> 
                      <mfrac><mn>1</mn><mn>3</mn></mfrac>
                  </msup>
              </math>";
  test("is", "SimpleSpeak", expr, "x í 1 þriðja veldi,");
}

#[test]
fn nested_squared_power_with_coef() {
    let expr = "<math>
      <mrow>
      <msup>
        <mn>3</mn>
        <mrow>
        <mn>2</mn>
        <msup>
          <mi>x</mi>
          <mn>2</mn>
        </msup>
        </mrow>
      </msup>
      </mrow>
      </math>";
  test("is", "SimpleSpeak", expr, "3 í 2 x í öðru veldi,");
}

#[test]
fn nested_squared_power_with_neg_coef() {
    let expr = "<math>
    <mrow>
    <msup>
      <mn>3</mn>
      <mrow>
      <mo>-</mo>
      <mn>2</mn>
      <msup>
        <mi>x</mi>
        <mn>2</mn>
      </msup>
      </mrow>
    </msup>
    </mrow>
  </math>";
  test("is", "SimpleSpeak", expr, "3 í mínus 2 x í öðru veldi,");
}


#[test]
fn nested_cubed_power() {
    let expr = "<math>
      <msup>
      <mi>y</mi> 
      <msup>
          <mfrac><mn>4</mn><mn>5</mn></mfrac>
          <mn>3</mn>
      </msup>
    </msup>
  </math>";
  test("is", "SimpleSpeak", expr, "y í 4 fimmtu í þriðja veldi,");
}

#[test]
fn nested_cubed_power_with_neg_base() {
    let expr = "<math>
      <msup>
      <mi>y</mi> 
        <mrow>
            <mo>-</mo>
            <msup>
                <mfrac><mn>4</mn><mn>5</mn></mfrac>
                <mn>3</mn>
            </msup>
        </mrow>
    </msup>
    </math>";
  test("is", "SimpleSpeak", expr, "y í mínus 4 fimmtu í þriðja veldi,");
}

#[test]
fn nested_number_times_squared() {
    let expr = "<math>
        <mrow>
        <msup>
          <mi>e</mi>
          <mrow>
          <mfrac>
            <mn>1</mn>
            <mn>2</mn>
            </mfrac>
            <msup>
            <mi>x</mi>
            <mn>2</mn>
            </msup>
          </mrow>
        </msup>
        </mrow>
        </math>";
  test("is", "SimpleSpeak", expr, "e í 1 hálfur x í öðru veldi,");
}

#[test]
fn nested_negative_number_times_squared() {
  let expr = "<math>
    <mrow>
    <msup>
      <mi>e</mi>
      <mrow>
      <mo>&#x2212;</mo><mfrac>
        <mn>1</mn>
        <mn>2</mn>
      </mfrac>
      <msup>
        <mi>x</mi>
        <mn>2</mn>
      </msup>
      </mrow>
    </msup>
    </mrow>
    </math>";
  test("is", "SimpleSpeak", expr, "e í mínus 1 hálfur x í öðru veldi,");
}

#[test]
fn nested_expr_to_tenth() {
    let expr = "<math>
      <mrow>
      <msup>
        <mn>3</mn>
        <mrow>
        <msup>
          <mn>3</mn>
          <mrow>
          <mn>10</mn></mrow>
        </msup>
        </mrow>
      </msup>
      </mrow>
      </math>";
  test("is", "SimpleSpeak", expr, "3 í 3 í tíunda veldi,");
}

#[test]
fn nested_non_simple_squared_exp() {
    let expr = "<math>
      <mrow>
      <msup>
        <mn>3</mn>
        <mrow>
        <msup>
          <mrow>
          <mrow><mo>(</mo>
            <mrow>
            <mi>x</mi><mo>+</mo><mn>1</mn></mrow>
          <mo>)</mo></mrow></mrow>
          <mn>2</mn>
        </msup>
        </mrow>
      </msup>
      </mrow>
      </math>";
  test("is", "SimpleSpeak", expr, "3 í svigi opnast x plús 1, svigi lokast í öðru veldi,");
}

#[test]
fn nested_simple_power() {
    let expr = "<math>
      <msup>
      <mi>t</mi> 
      <msup>
          <mfrac><mn>4</mn><mn>5</mn></mfrac>
          <mi>n</mi>
      </msup>
    </msup>
  </math>";
  test("is", "SimpleSpeak", expr, "t í 4 fimmtu í n -ta veldi,");
}

#[test]
fn nested_end_exponent_power() {
    let expr = "<math>
      <msup>
      <mi>t</mi> 
      <msup>
          <mfrac><mn>4</mn><mn>5</mn></mfrac>
          <mrow><mi>n</mi><mo>+</mo><mn>1</mn></mrow>
      </msup>
    </msup>
  </math>";
  test("is", "SimpleSpeak", expr, "t í 4 fimmtu í n plús 1 veldi; veldisvísir endar,");
  test_prefs("is", "SimpleSpeak", vec![("Impairment", "LearningDisability")], expr,
  "t í 4 fimmtu í n plús 1 veldi;");
}

#[test]
fn nested_end_exponent_neg_power() {
    let expr = "<math>
      <msup>
      <mi>t</mi> 
      <msup>
          <mfrac><mn>4</mn><mn>5</mn></mfrac>
          <mrow><mo>-</mo><mn>3</mn></mrow>
      </msup>
    </msup>
  </math>";
  test("is", "SimpleSpeak", expr, "t í 4 fimmtu í mínus 3, veldisvísir endar,");
}

#[test]
fn nested_complex_power() {
    let expr = "<math>
      <mrow>
      <msup>
        <mi>e</mi>
        <mrow>
        <mo>&#x2212;</mo><mfrac>
          <mn>1</mn>
          <mn>2</mn>
        </mfrac>
        <msup>
          <mrow>
          <mrow><mo>(</mo>
            <mrow>
            <mfrac>
              <mrow>
              <mi>x</mi><mo>&#x2212;</mo><mi>&#x03BC;</mi></mrow>
              <mi>&#x03C3;</mi>
            </mfrac>
            </mrow>
          <mo>)</mo></mrow></mrow>
          <mn>2</mn>
        </msup>
        </mrow>
      </msup>
      </mrow>
      </math>";
  test("is", "SimpleSpeak", expr, "e í mínus 1 hálfur sinnum; svigi opnast; brotið, x mínus mý, deilt með sigma, brot endar; svigi lokast í öðru veldi,");
}

#[test]
fn default_power() {
    let expr = "<math>
      <msup>
      <mi>t</mi> 
      <mfrac>
          <mrow><mi>b</mi><mo>+</mo><mn>1</mn></mrow>
          <mn>3</mn>
      </mfrac>
    </msup>
  </math>";
  test("is", "SimpleSpeak", expr, "t í brotið, b plús 1, deilt með 3, brot endar; veldi,");
}
