use crate::common::*;

#[test]
fn matrix_1x1() {
    let expr = "
    <math xmlns='http://www.w3.org/1998/Math/MathML'>
      <mrow>
      <mrow><mo>(</mo>
        <mtable><mtr><mtd>
        <mn>3</mn>
      </mtd> </mtr></mtable>
        <mo>)</mo></mrow></mrow>
    </math>
                                ";
    test("is", "ClearSpeak",  expr, "einu sinni einn fylki með stak 3;");
    test("is", "SimpleSpeak", expr, "einu sinni einn fylki með stak 3;");
}

#[test]
fn determinant_1x1() {
    let expr = "
    <math xmlns='http://www.w3.org/1998/Math/MathML'>
      <mrow>
      <mrow><mo>|</mo>
        <mtable><mtr><mtd>
        <mn>3</mn>
      </mtd> </mtr></mtable>
        <mo>|</mo></mrow></mrow>
    </math>
                                ";
    test("is", "ClearSpeak",  expr, "einu sinni einn ákveða með stak 3;");
    test("is", "SimpleSpeak", expr, "einu sinni einn ákveða með stak 3;");
}


#[test]
fn matrix_1x2() {
    let expr = "
    <math xmlns='http://www.w3.org/1998/Math/MathML'>
      <mrow>
      <mrow><mo>(</mo>
        <mtable>
          <mtr>
          <mtd>
            <mn>3</mn>
          </mtd>
          <mtd>
            <mn>5</mn>
          </mtd>
          </mtr>
        </mtable>
      <mo>)</mo></mrow></mrow>
    </math>
                                ";
    test("is", "ClearSpeak",  expr, "einu sinni 2 línu fylki; 3, 5;");
    test("is", "SimpleSpeak", expr, "einu sinni 2 línu fylki; 3, 5;");
}


#[test]
fn matrix_1x3() {
    let expr = "
    <math xmlns='http://www.w3.org/1998/Math/MathML'>
      <mrow>
      <mrow><mo>(</mo>
        <mtable>
          <mtr>
          <mtd>
            <mrow><mo>-</mo><mi>x</mi></mrow>
          </mtd>
          <mtd>
            <mn>5</mn>
          </mtd>
          <mtd>
            <mn>12</mn>
          </mtd>
          </mtr>
        </mtable>
      <mo>)</mo></mrow></mrow>
    </math>
                                ";
    test("is", "ClearSpeak", expr, "einu sinni 3 línu fylki; mínus x, 5, 12;");
    test("is", "SimpleSpeak", expr, "einu sinni 3 línu fylki; mínus x, 5, 12;");
}

#[test]
fn matrix_2x1_not_simple() {
    let expr = "
    <math xmlns='http://www.w3.org/1998/Math/MathML'>
      <mrow>
      <mrow><mo>(</mo>
        <mtable>
          <mtr>
          <mtd>
            <mrow>
            <mi>x</mi><mo>+</mo><mn>1</mn>
            </mrow>
          </mtd>
          </mtr>
          <mtr>
          <mtd>
            <mrow>
            <mi>x</mi><mo>-</mo><mn>1</mn></mrow>
          </mtd>
          </mtr>
        </mtable>
      <mo>)</mo></mrow></mrow>
    </math>
                                ";
    test("is", "ClearSpeak", expr, "2 sinnum einn dálkur fylki; lína 1; x plús 1; lína 2; x mínus 1;");
    test("is", "SimpleSpeak", expr, "2 sinnum einn dálkur fylki; lína 1; x plús 1; lína 2; x mínus 1;");
}
#[test]
fn matrix_3x1_not_simple() {
    let expr = "
    <math xmlns='http://www.w3.org/1998/Math/MathML'>
      <mrow>
      <mrow><mo>(</mo>
        <mtable>
          <mtr>
          <mtd>
            <mrow>
            <mi>x</mi>
            </mrow>
          </mtd>
          </mtr>
          <mtr>
          <mtd>
            <mrow>
            <mi>a</mi>
            </mrow>
          </mtd>
          </mtr>
          <mtr>
          <mtd>
            <mfrac>
              <mi>x</mi>
              <mrow>
                <mi>x</mi><mo>+</mo><mn>1</mn>
              </mrow>
            </mfrac>
          </mtd>
          </mtr>
        </mtable>
      <mo>)</mo></mrow></mrow>
    </math>";
    test("is", "SimpleSpeak", expr, "3 sinnum einn dálkur fylki; \
            lína 1; x; \
            lína 2; a; \
            lína 3; brot með teljarann x; og nefnarann x plús 1;");
    test("is", "ClearSpeak",  expr, "3 sinnum einn dálkur fylki; \
            lína 1; x; \
            lína 2; a; \
            lína 3; brot með teljarann x; og nefnarann x plús 1;");
}

#[test]
fn determinant_2x2() {
    let expr = "<math>
      <mrow>
      <mrow><mo>|</mo>
        <mtable>
          <mtr>
          <mtd>
            <mn>2</mn>
          </mtd>
          <mtd>
            <mn>1</mn>
          </mtd>
          </mtr>
          <mtr>
          <mtd>
            <mn>7</mn>
          </mtd>
          <mtd>
            <mn>5</mn>
          </mtd>
          </mtr>
          
        </mtable>
      <mo>|</mo></mrow></mrow>
                        </math>";
    test("is", "ClearSpeak",  expr, "2 sinnum 2 ákveða; lína 1; 2, 1; lína 2; 7, 5;");
    test("is", "SimpleSpeak", expr, "2 sinnum 2 ákveða; lína 1; 2, 1; lína 2; 7, 5;");
}

#[test]
fn matrix_2x3() {
    let expr = "
    <math display='block' xmlns='http://www.w3.org/1998/Math/MathML'>
      <mrow>
      <mrow><mo>[</mo>
        <mtable>
          <mtr>
          <mtd>
            <mn>3</mn>
          </mtd>
          <mtd>
            <mn>1</mn>
          </mtd>
          <mtd>
            <mn>4</mn>
          </mtd>
          </mtr>
          <mtr>
          <mtd>
            <mn>0</mn>
          </mtd>
          <mtd>
            <mn>2</mn>
          </mtd>
          <mtd>
            <mn>6</mn>
          </mtd>
          </mtr>
        </mtable>
      <mo>]</mo></mrow></mrow>
    </math>
                                ";
    test("is", "ClearSpeak",  expr, "2 sinnum 3 fylki; lína 1; 3, 1, 4; lína 2; 0, 2, 6;");
    test("is", "SimpleSpeak", expr, "2 sinnum 3 fylki; lína 1; 3, 1, 4; lína 2; 0, 2, 6;");
}

#[test]
fn matrix_2x3_labeled() {
    let expr = "
    <math display='block' xmlns='http://www.w3.org/1998/Math/MathML'>
      <mrow>
      <mrow><mo>[</mo>
        <mtable>
          <mlabeledtr>
          <mtd>
            <mtext>(3.1)</mtext>
          </mtd>
          <mtd>
            <mn>3</mn>
          </mtd>
          <mtd>
            <mn>1</mn>
          </mtd>
          <mtd>
            <mn>4</mn>
          </mtd>
          </mlabeledtr>
          <mtr>
          <mtd>
            <mn>0</mn>
          </mtd>
          <mtd>
            <mn>2</mn>
          </mtd>
          <mtd>
            <mn>6</mn>
          </mtd>
          </mtr>
        </mtable>
      <mo>]</mo></mrow></mrow>
    </math>
                                ";
    test("is", "ClearSpeak",  expr,
        "2 sinnum 3 fylki; lína 1 merkt (3 punktur 1); dálkur 2; 3, dálkur 3; 1, dálkur 4; 4; \
                                   lína 2; dálkur 1; 0, dálkur 2; 2, dálkur 3; 6;");
    test("is", "SimpleSpeak", expr,
       "2 sinnum 3 fylki; lína 1 merkt (3 punktur 1); dálkur 2; 3, dálkur 3; 1, dálkur 4; 4; \
                                   lína 2; dálkur 1; 0, dálkur 2; 2, dálkur 3; 6;");
}

#[test]
fn matrix_3x1() {
    let expr = "
    <math xmlns='http://www.w3.org/1998/Math/MathML'>
      <mrow>
      <mrow><mo>[</mo>
        <mtable>
        <mtr>
          <mtd>
          <mn>1</mn>
          </mtd>
        </mtr>
        <mtr>
          <mtd>
          <mn>2</mn>
          </mtd>
        </mtr>
        <mtr>
          <mtd>
          <mn>3</mn>
          </mtd>
        </mtr>           
        </mtable> <mo>]</mo></mrow></mrow>
    </math>
                                ";
    test("is", "ClearSpeak",  expr, "3 sinnum einn dálkur fylki; 1; 2; 3;");
    test("is", "SimpleSpeak", expr, "3 sinnum einn dálkur fylki; 1; 2; 3;");
}

#[test]
fn matrix_4x1() {
    let expr = "
    <math xmlns='http://www.w3.org/1998/Math/MathML'>
      <mrow>
      <mrow><mo>(</mo>
        <mtable>
          <mtr>
          <mtd>
            <mn>3</mn>
          </mtd>
          </mtr>
          <mtr>
          <mtd>
            <mn>6</mn>
          </mtd>
          </mtr>
          <mtr>
          <mtd>
            <mn>1</mn>
          </mtd>
          </mtr>
          <mtr>
          <mtd>
            <mn>2</mn>
          </mtd>
          </mtr>            
        </mtable>
      <mo>)</mo></mrow></mrow>
    </math>
                                ";
    test("is", "ClearSpeak",  expr, "4 sinnum einn dálkur fylki; lína 1; 3; lína 2; 6; lína 3; 1; lína 4; 2;");
    test("is", "SimpleSpeak", expr, "4 sinnum einn dálkur fylki; lína 1; 3; lína 2; 6; lína 3; 1; lína 4; 2;");
}

#[test]
fn matrix_4x1_labeled() {
    let expr = "
    <math xmlns='http://www.w3.org/1998/Math/MathML'>
      <mrow>
      <mrow><mo>(</mo>
        <mtable>
          <mtr>
          <mtd>
            <mn>3</mn>
          </mtd>
          </mtr>
          <mtr>
          <mtd>
            <mn>6</mn>
          </mtd>
          </mtr>
          <mtr>
          <mtd>
            <mn>1</mn>
          </mtd>
          </mtr>
          <mlabeledtr>
          <mtd>
            <mtext>(3.1)</mtext>
          </mtd>
          <mtd>
            <mn>2</mn>
          </mtd>
          </mlabeledtr>            
        </mtable>
      <mo>)</mo></mrow></mrow>
    </math>
                                ";
    test("is", "ClearSpeak",  expr,
        "4 sinnum einn dálkur fylki; lína 1; 3; lína 2; 6; lína 3; 1; lína 4 merkt (3 punktur 1); 2;");
    test("is", "SimpleSpeak", expr,
        "4 sinnum einn dálkur fylki; lína 1; 3; lína 2; 6; lína 3; 1; lína 4 merkt (3 punktur 1); 2;");
}

#[test]
fn matrix_1x4() {
    let expr = "
    <math xmlns='http://www.w3.org/1998/Math/MathML'>
      <mrow>
      <mrow><mo>(</mo>
        <mtable>
          <mtr>
          <mtd>
            <mn>3</mn>
          </mtd>
          <mtd>
            <mn>6</mn>
          </mtd>
          <mtd>
            <mn>1</mn>
          </mtd>
          <mtd>
            <mn>2</mn>
          </mtd>
          </mtr>
        </mtable>
      <mo>)</mo></mrow></mrow>
    </math>
                                ";
    test("is", "ClearSpeak",  expr, "einu sinni 4 línu fylki; dálkur 1; 3, dálkur 2; 6, dálkur 3; 1, dálkur 4; 2;");
    test("is", "SimpleSpeak", expr, "einu sinni 4 línu fylki; dálkur 1; 3, dálkur 2; 6, dálkur 3; 1, dálkur 4; 2;");
}

#[test]
fn matrix_4x4() {
    let expr = "
    <math xmlns='http://www.w3.org/1998/Math/MathML'>
      <mrow>
      <mrow><mo>(</mo>
        <mtable>
          <mtr>
          <mtd>
            <mn>0</mn>
          </mtd>
          <mtd>
            <mn>3</mn>
          </mtd>
          <mtd>
            <mn>4</mn>
          </mtd>
          <mtd>
            <mn>3</mn>
          </mtd>
          </mtr>
          <mtr>
          <mtd>
            <mn>2</mn>
          </mtd>
          <mtd>
            <mn>1</mn>
          </mtd>
          <mtd>
            <mn>0</mn>
          </mtd>
          <mtd>
            <mn>9</mn>
          </mtd>
          </mtr>
          <mtr>
          <mtd>
            <mn>3</mn>
          </mtd>
          <mtd>
            <mn>0</mn>
          </mtd>
          <mtd>
            <mn>2</mn>
          </mtd>
          <mtd>
            <mn>1</mn>
          </mtd>
          </mtr>
          <mtr>
          <mtd>
            <mn>6</mn>
          </mtd>
          <mtd>
            <mn>2</mn>
          </mtd>
          <mtd>
            <mn>9</mn>
          </mtd>
          <mtd>
            <mn>0</mn>
          </mtd>
          </mtr>
        </mtable>
      <mo>)</mo></mrow></mrow>
    </math>
                                ";
    test("is", "ClearSpeak",  expr, "4 sinnum 4 fylki; \
          lína 1; dálkur 1; 0, dálkur 2; 3, dálkur 3; 4, dálkur 4; 3; \
          lína 2; dálkur 1; 2, dálkur 2; 1, dálkur 3; 0, dálkur 4; 9; \
          lína 3; dálkur 1; 3, dálkur 2; 0, dálkur 3; 2, dálkur 4; 1; \
          lína 4; dálkur 1; 6, dálkur 2; 2, dálkur 3; 9, dálkur 4; 0;");
    test("is", "SimpleSpeak", expr, "4 sinnum 4 fylki; \
          lína 1; dálkur 1; 0, dálkur 2; 3, dálkur 3; 4, dálkur 4; 3; \
          lína 2; dálkur 1; 2, dálkur 2; 1, dálkur 3; 0, dálkur 4; 9; \
          lína 3; dálkur 1; 3, dálkur 2; 0, dálkur 3; 2, dálkur 4; 1; \
          lína 4; dálkur 1; 6, dálkur 2; 2, dálkur 3; 9, dálkur 4; 0;");}

#[test]
fn matrix_4x2() {
    let expr = "
    <math xmlns='http://www.w3.org/1998/Math/MathML'>
    <mrow>
      <mrow><mo>(</mo>
        <mtable>
        <mtr>
          <mtd>
          <mn>1</mn>
          </mtd>
          <mtd>
          <mn>3</mn>
          </mtd>
        </mtr>
        <mtr>
          <mtd>
          <mn>4</mn>
          </mtd>
          <mtd>
          <mn>2</mn>
          </mtd>
        </mtr>
        <mtr>
          <mtd>
          <mn>2</mn>
          </mtd>
          <mtd>
          <mn>1</mn>
          </mtd>
        </mtr>
        <mtr>
          <mtd>
          <mn>0</mn>
          </mtd>
          <mtd>
          <mn>5</mn>
          </mtd>
        </mtr>
        
        </mtable>
      <mo>)</mo></mrow></mrow>
    </math>
      ";
    test("is", "ClearSpeak",  expr, "4 sinnum 2 fylki; \
              lína 1; dálkur 1; 1, dálkur 2; 3; \
              lína 2; dálkur 1; 4, dálkur 2; 2; \
              lína 3; dálkur 1; 2, dálkur 2; 1; \
              lína 4; dálkur 1; 0, dálkur 2; 5;\
    ");
    test("is", "SimpleSpeak", expr, "4 sinnum 2 fylki; \
              lína 1; dálkur 1; 1, dálkur 2; 3; \
              lína 2; dálkur 1; 4, dálkur 2; 2; \
              lína 3; dálkur 1; 2, dálkur 2; 1; \
              lína 4; dálkur 1; 0, dálkur 2; 5;\
    ");}

// put absolute value test here since it is related to determinate and is small for its own file
#[test]
fn simple_absolute_value() {
  let expr = "<math>
    <mrow><mrow><mo>|</mo> <mi>x</mi> <mo>|</mo></mrow></mrow>
  </math>";
  test("is", "SimpleSpeak", expr, "algildi x,");
  test("is", "ClearSpeak",  expr, "algildi x,");
  test_prefs("is", "ClearSpeak", vec![("Verbosity", "Terse"), ("ClearSpeak_AbsoluteValue", "Auto")], expr, "algildi x,");
  test_prefs("is", "ClearSpeak", vec![("Verbosity", "Verbose"), ("ClearSpeak_AbsoluteValue", "AbsEnd")],
             expr, "algildi x, algildi endar,");
}
  
#[test]
fn absolute_value_plus_1() {
let expr = "<math>
    <mrow><mrow><mo>|</mo>
      <mrow><mi>x</mi><mo>+</mo><mn>1</mn> </mrow>
    <mo>|</mo></mrow></mrow>
  </math>";
  test("is", "ClearSpeak", expr, "algildi x plús 1,");
  test_prefs("is", "ClearSpeak", vec![("Verbosity", "Terse"), ("ClearSpeak_AbsoluteValue", "AbsEnd")],
             expr, "algildi x plús 1, algildi endar,");
}

#[test]
fn simple_cardinality_value() {
  let expr = "<math>
    <mrow><mrow><mo>|</mo> <mi>S</mi> <mo>|</mo></mrow></mrow>
  </math>";
  test_prefs("is", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_AbsoluteValue", "Cardinality")], expr,
             "fjöldatala stórt s,");
}
  
// Test preferences
#[test]
fn simple_matrix_speak_col_num() {
let expr = "<math display='block' xmlns='http://www.w3.org/1998/Math/MathML'>
  <mrow>
    <mrow><mo>(</mo>
    <mrow>
      <mtable>
      <mtr>
        <mtd> <mn>2</mn> </mtd>
        <mtd><mn>1</mn> </mtd>
      </mtr>
      <mtr>
        <mtd><mn>7</mn> </mtd>
        <mtd><mn>5</mn> </mtd>
      </mtr>
      </mtable></mrow>
    <mo>)</mo></mrow></mrow>
  </math>";
  test_ClearSpeak("is", "ClearSpeak_Matrix", "SpeakColNum",
        expr, "2 sinnum 2 fylki; lína 1; dálkur 1; 2, dálkur 2; 1; lína 2; dálkur 1; 7, dálkur 2; 5;");
}

#[test]
fn col_matrix_3x1_speak_col_num() {
let expr = "<math display='block' xmlns='http://www.w3.org/1998/Math/MathML'>
  <mrow>
    <mrow><mo>(</mo>
    <mrow>
      <mtable>
      <mtr>
        <mtd><mn>1</mn> </mtd>
      </mtr>
      <mtr>
        <mtd><mn>2</mn> </mtd>
      </mtr>
      <mtr>
        <mtd><mn>3</mn> </mtd>
      </mtr>
      </mtable></mrow>
    <mo>)</mo></mrow></mrow>
  </math>";
test_ClearSpeak("is", "ClearSpeak_Matrix", "SpeakColNum",
        expr, "3 sinnum einn dálkur fylki; lína 1; 1; lína 2; 2; lína 3; 3;");
}

#[test]
fn row_matrix_1x2_speak_col_num() {
let expr = "<math display='block' xmlns='http://www.w3.org/1998/Math/MathML'>
  <mrow>
    <mrow><mo>[</mo>
    <mrow>
      <mtable>
      <mtr>
        <mtd><mn>1</mn> </mtd> <mtd><mn>2</mn> </mtd>
      </mtr>
      </mtable></mrow>
    <mo>]</mo></mrow></mrow>
  </math>";
test_ClearSpeak("is", "ClearSpeak_Matrix", "SpeakColNum",
        expr, "einu sinni 2 línu fylki; dálkur 1; 1, dálkur 2; 2;");
}

#[test]
fn matrix_2x2_speak_col_num() {
let expr = "<math><mrow><mrow><mo>(</mo><mrow>
    <mtable>
    <mtr>
        <mtd><mrow><msub><mi>b</mi><mrow><mn>1</mn><mn>1</mn></mrow></msub></mrow></mtd>
        <mtd><mrow><msub><mi>b</mi><mrow><mn>1</mn><mn>2</mn></mrow></msub></mrow></mtd>
    </mtr>
    <mtr>
        <mtd><mrow><msub><mi>b</mi><mrow><mn>2</mn><mn>1</mn></mrow></msub></mrow></mtd>
        <mtd><mrow><msub><mi>b</mi><mrow><mn>2</mn><mn>2</mn></mrow></msub></mrow></mtd>
    </mtr>
    </mtable>
    </mrow><mo>)</mo></mrow></mrow></math>";
test_ClearSpeak("is", "ClearSpeak_Matrix", "SpeakColNum",
        expr, "2 sinnum 2 fylki; lína 1; dálkur 1; b lágvísir 1 1, dálkur 2; b lágvísir 1 2; \
                                                lína 2; dálkur 1; b lágvísir 2 1, dálkur 2; b lágvísir 2 2;");
}


#[test]
fn simple_matrix_silent_col_num() {
let expr = "<math display='block' xmlns='http://www.w3.org/1998/Math/MathML'>
  <mrow>
    <mrow><mo>(</mo>
    <mrow>
      <mtable>
      <mtr>
        <mtd> <mn>2</mn> </mtd>
        <mtd><mn>1</mn> </mtd>
      </mtr>
      <mtr>
        <mtd><mn>7</mn> </mtd>
        <mtd><mn>5</mn> </mtd>
      </mtr>
      </mtable></mrow>
    <mo>)</mo></mrow></mrow>
  </math>";
  test_ClearSpeak("is", "ClearSpeak_Matrix", "SilentColNum",
        expr, "2 sinnum 2 fylki; lína 1; 2, 1; lína 2; 7, 5;");
}

#[test]
fn col_matrix_3x1_silent_col_num() {
let expr = "<math display='block' xmlns='http://www.w3.org/1998/Math/MathML'>
  <mrow>
    <mrow><mo>(</mo>
    <mrow>
      <mtable>
      <mtr>
        <mtd><mn>1</mn> </mtd>
      </mtr>
      <mtr>
        <mtd><mn>2</mn> </mtd>
      </mtr>
      <mtr>
        <mtd><mn>3</mn> </mtd>
      </mtr>
      </mtable></mrow>
    <mo>)</mo></mrow></mrow>
  </math>";
test_ClearSpeak("is", "ClearSpeak_Matrix", "SilentColNum",
        expr, "3 sinnum einn dálkur fylki; 1; 2; 3;");
}

#[test]
fn row_matrix_1x2_silent_col_num() {
let expr = "<math display='block' xmlns='http://www.w3.org/1998/Math/MathML'>
  <mrow>
    <mrow><mo>[</mo>
    <mrow>
      <mtable>
      <mtr>
        <mtd><mn>1</mn> </mtd> <mtd><mn>2</mn> </mtd>
      </mtr>
      </mtable></mrow>
    <mo>]</mo></mrow></mrow>
  </math>";
test_ClearSpeak("is", "ClearSpeak_Matrix", "SilentColNum",
        expr, "einu sinni 2 línu fylki; 1, 2;");
}

#[test]
fn matrix_2x2_silent_col_num() {
let expr = "<math><mrow><mrow><mo>(</mo><mrow>
    <mtable>
    <mtr>
        <mtd><mrow><msub><mi>b</mi><mrow><mn>1</mn><mn>1</mn></mrow></msub></mrow></mtd>
        <mtd><mrow><msub><mi>b</mi><mrow><mn>1</mn><mn>2</mn></mrow></msub></mrow></mtd>
    </mtr>
    <mtr>
        <mtd><mrow><msub><mi>b</mi><mrow><mn>2</mn><mn>1</mn></mrow></msub></mrow></mtd>
        <mtd><mrow><msub><mi>b</mi><mrow><mn>2</mn><mn>2</mn></mrow></msub></mrow></mtd>
    </mtr>
    </mtable>
    </mrow><mo>)</mo></mrow></mrow></math>";
test_ClearSpeak("is", "ClearSpeak_Matrix", "SilentColNum",
        expr, "2 sinnum 2 fylki; lína 1; b lágvísir 1 1, b lágvísir 1 2; \
                                                lína 2; b lágvísir 2 1, b lágvísir 2 2;");
}


#[test]
fn simple_matrix_end_matrix() {
let expr = "<math display='block' xmlns='http://www.w3.org/1998/Math/MathML'>
  <mrow>
    <mrow><mo>(</mo>
    <mrow>
      <mtable>
      <mtr>
        <mtd> <mn>2</mn> </mtd>
        <mtd><mn>1</mn> </mtd>
      </mtr>
      <mtr>
        <mtd><mn>7</mn> </mtd>
        <mtd><mn>5</mn> </mtd>
      </mtr>
      </mtable></mrow>
    <mo>)</mo></mrow></mrow>
  </math>";
  test_ClearSpeak("is", "ClearSpeak_Matrix", "EndMatrix",
        expr, "2 sinnum 2 fylki; lína 1; 2, 1; lína 2; 7, 5; fylki endar");
}

#[test]
fn col_matrix_3x1_end_matrix() {
let expr = "<math display='block' xmlns='http://www.w3.org/1998/Math/MathML'>
  <mrow>
    <mrow><mo>(</mo>
    <mrow>
      <mtable>
      <mtr>
        <mtd><mn>1</mn> </mtd>
      </mtr>
      <mtr>
        <mtd><mn>2</mn> </mtd>
      </mtr>
      <mtr>
        <mtd><mn>3</mn> </mtd>
      </mtr>
      </mtable></mrow>
    <mo>)</mo></mrow></mrow>
  </math>";
test_ClearSpeak("is", "ClearSpeak_Matrix", "EndMatrix",
        expr, "3 sinnum einn dálkur fylki; 1; 2; 3; fylki endar");
}

#[test]
fn row_matrix_1x2_end_matrix() {
let expr = "<math display='block' xmlns='http://www.w3.org/1998/Math/MathML'>
  <mrow>
    <mrow><mo>[</mo>
    <mrow>
      <mtable>
      <mtr>
        <mtd><mn>1</mn> </mtd> <mtd><mn>2</mn> </mtd>
      </mtr>
      </mtable></mrow>
    <mo>]</mo></mrow></mrow>
  </math>";
test_ClearSpeak("is", "ClearSpeak_Matrix", "EndMatrix",
        expr, "einu sinni 2 línu fylki; 1, 2; fylki endar");
}

#[test]
fn matrix_2x2_end_matrix() {
let expr = "<math><mrow><mrow><mo>(</mo><mrow>
    <mtable>
    <mtr>
        <mtd><mrow><msub><mi>b</mi><mrow><mn>1</mn><mn>1</mn></mrow></msub></mrow></mtd>
        <mtd><mrow><msub><mi>b</mi><mrow><mn>1</mn><mn>2</mn></mrow></msub></mrow></mtd>
    </mtr>
    <mtr>
        <mtd><mrow><msub><mi>b</mi><mrow><mn>2</mn><mn>1</mn></mrow></msub></mrow></mtd>
        <mtd><mrow><msub><mi>b</mi><mrow><mn>2</mn><mn>2</mn></mrow></msub></mrow></mtd>
    </mtr>
    </mtable>
    </mrow><mo>)</mo></mrow></mrow></math>";
test_ClearSpeak("is", "ClearSpeak_Matrix", "EndMatrix",
        expr, "2 sinnum 2 fylki; lína 1; dálkur 1; b lágvísir 1 1, dálkur 2; b lágvísir 1 2; \
                                                lína 2; dálkur 1; b lágvísir 2 1, dálkur 2; b lágvísir 2 2; fylki endar");
}


#[test]
fn simple_matrix_vector() {
let expr = "<math display='block' xmlns='http://www.w3.org/1998/Math/MathML'>
  <mrow>
    <mrow><mo>(</mo>
    <mrow>
      <mtable>
      <mtr>
        <mtd> <mn>2</mn> </mtd>
        <mtd><mn>1</mn> </mtd>
      </mtr>
      <mtr>
        <mtd><mn>7</mn> </mtd>
        <mtd><mn>5</mn> </mtd>
      </mtr>
      </mtable></mrow>
    <mo>)</mo></mrow></mrow>
  </math>";
  test_ClearSpeak("is", "ClearSpeak_Matrix", "Vector",
        expr, "2 sinnum 2 fylki; lína 1; 2, 1; lína 2; 7, 5;");
}

#[test]
fn col_matrix_3x1_vector() {
let expr = "<math display='block' xmlns='http://www.w3.org/1998/Math/MathML'>
  <mrow>
    <mrow><mo>(</mo>
    <mrow>
      <mtable>
      <mtr>
        <mtd><mn>1</mn> </mtd>
      </mtr>
      <mtr>
        <mtd><mn>2</mn> </mtd>
      </mtr>
      <mtr>
        <mtd><mn>3</mn> </mtd>
      </mtr>
      </mtable></mrow>
    <mo>)</mo></mrow></mrow>
  </math>";
test_ClearSpeak("is", "ClearSpeak_Matrix", "Vector",
        expr, "3 sinnum einn dálkur vigur; 1; 2; 3;");
}

#[test]
fn row_matrix_1x2_vector() {
let expr = "<math display='block' xmlns='http://www.w3.org/1998/Math/MathML'>
  <mrow>
    <mrow><mo>[</mo>
    <mrow>
      <mtable>
      <mtr>
        <mtd><mn>1</mn> </mtd> <mtd><mn>2</mn> </mtd>
      </mtr>
      </mtable></mrow>
    <mo>]</mo></mrow></mrow>
  </math>";
test_ClearSpeak("is", "ClearSpeak_Matrix", "Vector",
        expr, "einu sinni 2 línu vigur; 1, 2;");
}

#[test]
fn matrix_2x2_vector() {
let expr = "<math><mrow><mrow><mo>(</mo><mrow>
    <mtable>
    <mtr>
        <mtd><mrow><msub><mi>b</mi><mrow><mn>1</mn><mn>1</mn></mrow></msub></mrow></mtd>
        <mtd><mrow><msub><mi>b</mi><mrow><mn>1</mn><mn>2</mn></mrow></msub></mrow></mtd>
    </mtr>
    <mtr>
        <mtd><mrow><msub><mi>b</mi><mrow><mn>2</mn><mn>1</mn></mrow></msub></mrow></mtd>
        <mtd><mrow><msub><mi>b</mi><mrow><mn>2</mn><mn>2</mn></mrow></msub></mrow></mtd>
    </mtr>
    </mtable>
    </mrow><mo>)</mo></mrow></mrow></math>";
test_ClearSpeak("is", "ClearSpeak_Matrix", "Vector",
        expr, "2 sinnum 2 fylki; lína 1; dálkur 1; b lágvísir 1 1, dálkur 2; b lágvísir 1 2; \
                                                lína 2; dálkur 1; b lágvísir 2 1, dálkur 2; b lágvísir 2 2;");
}


#[test]
fn simple_matrix_end_vector() {
let expr = "<math display='block' xmlns='http://www.w3.org/1998/Math/MathML'>
  <mrow>
    <mrow><mo>(</mo>
    <mrow>
      <mtable>
      <mtr>
        <mtd> <mn>2</mn> </mtd>
        <mtd><mn>1</mn> </mtd>
      </mtr>
      <mtr>
        <mtd><mn>7</mn> </mtd>
        <mtd><mn>5</mn> </mtd>
      </mtr>
      </mtable></mrow>
    <mo>)</mo></mrow></mrow>
  </math>";
  test_ClearSpeak("is", "ClearSpeak_Matrix", "EndVector",
        expr, "2 sinnum 2 fylki; lína 1; 2, 1; lína 2; 7, 5; fylki endar");
}

#[test]
fn col_matrix_3x1_end_vector() {
let expr = "<math display='block' xmlns='http://www.w3.org/1998/Math/MathML'>
  <mrow>
    <mrow><mo>(</mo>
    <mrow>
      <mtable>
      <mtr>
        <mtd><mn>1</mn> </mtd>
      </mtr>
      <mtr>
        <mtd><mn>2</mn> </mtd>
      </mtr>
      <mtr>
        <mtd><mn>3</mn> </mtd>
      </mtr>
      </mtable></mrow>
    <mo>)</mo></mrow></mrow>
  </math>";
test_ClearSpeak("is", "ClearSpeak_Matrix", "EndVector",
        expr, "3 sinnum einn dálkur vigur; 1; 2; 3; vigur endar");
}

#[test]
fn row_matrix_1x2_end_vector() {
let expr = "<math display='block' xmlns='http://www.w3.org/1998/Math/MathML'>
  <mrow>
    <mrow><mo>[</mo>
    <mrow>
      <mtable>
      <mtr>
        <mtd><mn>1</mn> </mtd> <mtd><mn>2</mn> </mtd>
      </mtr>
      </mtable></mrow>
    <mo>]</mo></mrow></mrow>
  </math>";
test_ClearSpeak("is", "ClearSpeak_Matrix", "EndVector",
        expr, "einu sinni 2 línu vigur; 1, 2; vigur endar");
}

#[test]
fn matrix_2x2_end_vector() {
let expr = "<math><mrow><mrow><mo>(</mo><mrow>
    <mtable>
    <mtr>
        <mtd><mrow><msub><mi>b</mi><mrow><mn>1</mn><mn>1</mn></mrow></msub></mrow></mtd>
        <mtd><mrow><msub><mi>b</mi><mrow><mn>1</mn><mn>2</mn></mrow></msub></mrow></mtd>
    </mtr>
    <mtr>
        <mtd><mrow><msub><mi>b</mi><mrow><mn>2</mn><mn>1</mn></mrow></msub></mrow></mtd>
        <mtd><mrow><msub><mi>b</mi><mrow><mn>2</mn><mn>2</mn></mrow></msub></mrow></mtd>
    </mtr>
    </mtable>
    </mrow><mo>)</mo></mrow></mrow></math>";
test_ClearSpeak("is", "ClearSpeak_Matrix", "EndVector",
        expr, "2 sinnum 2 fylki; lína 1; dálkur 1; b lágvísir 1 1, dálkur 2; b lágvísir 1 2; \
                                                lína 2; dálkur 1; b lágvísir 2 1, dálkur 2; b lágvísir 2 2; fylki endar");
}



#[test]
fn matrix_binomial() {
  let expr = "<math>
      <mo>(</mo><mrow>
        <mtable><mtr><mtd><mn>3</mn></mtd></mtr><mtr><mtd><mn>2</mn></mtd></mtr></mtable>
      </mrow><mo>)</mo>
    </math>";
  test_ClearSpeak("is", "ClearSpeak_Matrix", "Combinatorics", expr, "3 velja 2");
}
