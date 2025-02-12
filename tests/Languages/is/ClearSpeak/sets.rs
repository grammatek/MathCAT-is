use crate::common::*;

#[test]
fn complex() {
    let expr = "<math>
                    <mi>ℂ</mi>
                </math>";
    test("is", "ClearSpeak", expr, "tvinntölurnar");
}

#[test]
fn natural() {
    let expr = "<math>
                    <mi>ℕ</mi>
                </math>";
    test("is", "ClearSpeak", expr, "náttúrulegu tölurnar");
}

#[test]
fn rationals() {
    let expr = "<math>
                    <mi>ℚ</mi>
                </math>";
    test("is", "ClearSpeak", expr, "ræðu tölurnar");
}

#[test]
fn reals() {
    let expr = "<math>
                    <mi>ℝ</mi>
                </math>";
    test("is", "ClearSpeak", expr, "rauntölurnar");
}

#[test]
fn integers() {
    let expr = "<math>
                    <mi>ℤ</mi>
                </math>";
    test("is", "ClearSpeak", expr, "heilu tölurnar");
}



#[test]
fn msup_complex() {
    let expr = "<math>
                <msup>
                    <mi>ℂ</mi>
                    <mn>2</mn>
                </msup>
                </math>";
    test("is", "ClearSpeak", expr, "C 2");
}

#[test]
fn msup_natural() {
    let expr = "<math>
                <msup>
                    <mi>ℕ</mi>
                    <mn>2</mn>
                </msup>
            </math>";
    test("is", "ClearSpeak", expr, "N 2");
}

#[test]
fn msup_rationals() {
    let expr = "<math>
                <msup>
                    <mi>ℚ</mi>
                    <mn>2</mn>
                </msup>
            </math>";
    test("is", "ClearSpeak", expr, "Q 2");
}

#[test]
fn msup_reals() {
    let expr = "<math>
                <msup>
                    <mi>ℝ</mi>
                    <mn>3</mn>
                </msup>
            </math>";
    test("is", "ClearSpeak", expr, "R 3");
}

#[test]
fn msup_integers() {
    let expr = "<math>
                <msup>
                    <mi>ℤ</mi>
                    <mn>4</mn>
                </msup>
            </math>";
    test("is", "ClearSpeak", expr, "Z 4");
}

#[test]
fn msup_positive_integers() {
    let expr = "<math>
                <msup>
                    <mi>ℤ</mi>
                    <mo>+</mo>
                </msup>
            </math>";
    test("is", "ClearSpeak", expr, "jákvæðar heilar tölur");
}

#[test]
fn msup_negative_integers() {
    let expr = "<math>
                <msup>
                    <mi>ℤ</mi>
                    <mo>-</mo>
                </msup>
            </math>";
    test("is", "ClearSpeak", expr, "neikvæðar heilar tölur");
}

#[test]
fn msup_positive_rationals() {
    let expr = "<math>
                <msup>
                    <mi>ℚ</mi>
                    <mo>+</mo>
                </msup>
            </math>";
    test("is", "ClearSpeak", expr, "jákvæðar ræðar tölur");
}

#[test]
fn msup_negative_rationals() {
    let expr = "<math>
                <msup>
                    <mi>ℚ</mi>
                    <mo>-</mo>
                </msup>
            </math>";
    test("is", "ClearSpeak", expr, "neikvæðar ræðar tölur");
}

#[test]
fn empty_set() {
    let expr = "<math>
                <mo>{</mo> <mo>}</mo>
            </math>";
    test("is", "ClearSpeak", expr, "tómamengið");
}

#[test]
fn single_element_set() {
    let expr = "<math>
                <mo>{</mo> <mn>12</mn><mo>}</mo>
            </math>";
    test("is", "ClearSpeak", expr, "mengið 12");
}

#[test]
fn multiple_element_set() {
    let expr = "<math>
                <mo>{</mo> <mn>5</mn> <mo>,</mo> <mn>10</mn>  <mo>,</mo> <mn>15</mn> <mo>}</mo>
            </math>";
    test("is", "ClearSpeak", expr, "mengið 5 komma 10 komma 15");
}

#[test]
fn set_with_colon() {
    let expr = "<math>
                    <mo>{</mo> <mrow><mi>x</mi><mo>:</mo><mi>x</mi><mo>&#x003E;</mo><mn>2</mn></mrow> <mo>}</mo>
            </math>";
    test("is", "ClearSpeak", expr, "mengi allra x þannig að x er stærra en 2");
}

#[test]
fn set_with_bar() {
    let expr = "<math>
                    <mo>{</mo> <mrow><mi>x</mi><mo>|</mo><mi>x</mi><mo>&#x003E;</mo><mn>2</mn></mrow> <mo>}</mo>
            </math>";
    test("is", "ClearSpeak", expr, "mengi allra x þannig að x er stærra en 2");
}

#[test]
fn element_alone() {
    let expr = "<math>
            <mn>3</mn><mo>+</mo><mn>2</mn><mi>i</mi><mo>∉</mo><mi>ℝ</mi>
        </math>";
    test("is", "ClearSpeak", expr, "3 plús 2 i, er ekki meðlimur í, rauntölunum");
}

#[test]
fn element_under_sum() {
    let expr = "<math>
            <munder>
                <mo>∑</mo>
                <mrow> <mi>i</mi> <mo>∈</mo> <mi>ℤ</mi> </mrow>
            </munder>
            <mfrac>
                <mn>1</mn>
                <mrow> <msup>  <mi>i</mi> <mn>2</mn> </msup> </mrow>
            </mfrac>
        </math>";
    test("is", "ClearSpeak", expr,
                    "summan yfir i er meðlimur í heilu tölunum af; brot með teljarann 1; og nefnarann i í öðru;");
}

#[test]
fn complicated_set_with_colon() {
    let expr = "<math>
            <mo>{</mo>
            <mi>x</mi>
            <mo>∈</mo>
            <mi>ℤ</mi>
            <mo>:</mo>
            <mn>2</mn>
            <mo>&#x003C;</mo>
            <mi>x</mi>
            <mo>&#x003C;</mo>
            <mn>7</mn>
            <mo>}</mo>
        </math>";
    test("is", "ClearSpeak", expr, "the set of all x in the integers such that 2 is less than x is less than 7");
    test("is", "ClearSpeak", expr, "mengi allra x í heilu tölunum þannig að 2 er minna en x er minna en 7");
}

#[test]
fn complicated_set_with_mtext() {
    // as of 8/5/21, parsing of "|" is problematic in the example, so <mrows> are needed for this test
    let expr = "<math>
        <mo>{</mo>
        <mrow> <mi>x</mi><mo>∈</mo><mi>ℕ</mi></mrow>
        <mo>|</mo>
        <mrow><mi>x</mi> <mtext>er slétt tala</mtext> </mrow>
        <mo>}</mo>
        </math>";
    test("is", "ClearSpeak", expr, 
            "mengi allra x í náttúrulegu tölunum þannig að x er slétt tala");
}


#[test]
fn set_with_bar_member() {
    let expr = "<math>
            <mo>{</mo>
            <mi>x</mi>
            <mo>∈</mo>
            <mi>ℤ</mi>
            <mo>:</mo>
            <mi>x</mi>
            <mo>&#x003E;</mo>
            <mn>5</mn>
            <mo>}</mo>
            </math>";
    test_ClearSpeak("is", "ClearSpeak_SetMemberSymbol", "Member",
                expr, "mengi allra x meðlimir í heilu tölunum þannig að x er stærra en 5");
}

#[test]
fn element_alone_member() {
    let expr = "<math>
            <mn>3</mn><mo>+</mo><mn>2</mn><mi>i</mi><mo>∉</mo><mi>ℝ</mi>
        </math>";
    test_ClearSpeak("is", "ClearSpeak_SetMemberSymbol", "Member",
                expr, "3 plús 2 i, er ekki meðlimur í, rauntölunum");
}

#[test]
fn element_under_sum_member() {
    let expr = "<math>
            <munder>
                <mo>∑</mo>
                <mrow> <mi>i</mi> <mo>∈</mo> <mi>ℤ</mi> </mrow>
            </munder>
            <mfrac>
                <mn>1</mn>
                <mrow> <msup>  <mi>i</mi> <mn>2</mn> </msup> </mrow>
            </mfrac>
        </math>";
    test_ClearSpeak("is", "ClearSpeak_SetMemberSymbol", "Member",
                expr, "summan yfir i er meðlimur í heilu tölunum af; brot með teljarann 1; og nefnarann i í öðru;");
}


#[test]
fn set_with_bar_element() {
    let expr = "<math>
            <mo>{</mo>
            <mi>x</mi>
            <mo>∈</mo>
            <mi>ℤ</mi>
            <mo>:</mo>
            <mi>x</mi>
            <mo>&#x003E;</mo>
            <mn>5</mn>
            <mo>}</mo>
            </math>";
    test_ClearSpeak("is", "ClearSpeak_SetMemberSymbol", "Element",
                expr, "mengi allra x staka í heilu tölunum þannig að x er stærra en 5");
}

#[test]
fn element_alone_element() {
    let expr = "<math>
            <mn>3</mn><mo>+</mo><mn>2</mn><mi>i</mi><mo>∉</mo><mi>ℝ</mi>
        </math>";
    test_ClearSpeak("is", "ClearSpeak_SetMemberSymbol", "Element",
                expr, "3 plús 2 i, er ekki stak í rauntölunum");
}

#[test]
fn element_under_sum_element() {
    let expr = "<math>
            <munder>
                <mo>∑</mo>
                <mrow> <mi>i</mi> <mo>∈</mo> <mi>ℤ</mi> </mrow>
            </munder>
            <mfrac>
                <mn>1</mn>
                <mrow> <msup>  <mi>i</mi> <mn>2</mn> </msup> </mrow>
            </mfrac>
        </math>";
    test_ClearSpeak("is", "ClearSpeak_SetMemberSymbol", "Element",
                expr, "summan yfir i er stak í heilu tölunum af; brot með teljarann 1; og nefnarann i í öðru;");
}

#[test]
fn set_with_bar_in() {
    let expr = "<math>
            <mo>{</mo>
            <mi>x</mi>
            <mo>∈</mo>
            <mi>ℤ</mi>
            <mo>:</mo>
            <mi>x</mi>
            <mo>&#x003E;</mo>
            <mn>5</mn>
            <mo>}</mo>
            </math>";
    test_ClearSpeak("is", "ClearSpeak_SetMemberSymbol", "In",
                expr, "mengi allra x í heilu tölunum þannig að x er stærra en 5");
}

#[test]
fn element_alone_in() {
    let expr = "<math>
            <mn>3</mn><mo>+</mo><mn>2</mn><mi>i</mi><mo>∉</mo><mi>ℝ</mi>
        </math>";
    test_ClearSpeak("is", "ClearSpeak_SetMemberSymbol", "In",
                expr, "3 plús 2 i, er ekki í rauntölunum");
}

#[test]
fn element_under_sum_in() {
    let expr = "<math>
            <munder>
                <mo>∑</mo>
                <mrow> <mi>i</mi> <mo>∈</mo> <mi>ℤ</mi> </mrow>
            </munder>
            <mfrac>
                <mn>1</mn>
                <mrow> <msup>  <mi>i</mi> <mn>2</mn> </msup> </mrow>
            </mfrac>
        </math>";
    test_ClearSpeak("is", "ClearSpeak_SetMemberSymbol", "In",
                expr, "summan yfir i er í heilu tölurnar af; brot með teljarann 1; og nefnarann i í öðru;");
}

#[test]
fn set_with_bar_belongs() {
    let expr = "<math>
            <mo>{</mo>
            <mi>x</mi>
            <mo>∈</mo>
            <mi>ℤ</mi>
            <mo>:</mo>
            <mi>x</mi>
            <mo>&#x003E;</mo>
            <mn>5</mn>
            <mo>}</mo>
            </math>";
    test_ClearSpeak("is", "ClearSpeak_SetMemberSymbol", "Belongs",
                expr, "mengi allra x sem tilheyra heilu tölunum þannig að x er stærra en 5");
}

#[test]
fn element_alone_belongs() {
    let expr = "<math>
            <mn>3</mn><mo>+</mo><mn>2</mn><mi>i</mi><mo>∉</mo><mi>ℝ</mi>
        </math>";
    test_ClearSpeak("is", "ClearSpeak_SetMemberSymbol", "Belongs",
                expr, "3 plús 2 i, tilheyrir ekki rauntölunum");
}

#[test]
fn element_under_sum_belongs() {
    let expr = "<math>
            <munder>
                <mo>∑</mo>
                <mrow> <mi>i</mi> <mo>∈</mo> <mi>ℤ</mi> </mrow>
            </munder>
            <mfrac>
                <mn>1</mn>
                <mrow> <msup>  <mi>i</mi> <mn>2</mn> </msup> </mrow>
            </mfrac>
        </math>";
    test_ClearSpeak("is", "ClearSpeak_SetMemberSymbol", "Belongs",
                expr, "summan yfir i tilheyrir heilu tölunum af; brot með teljarann 1; og nefnarann i í öðru;");
}


#[test]
fn set_member_woall() {
    let expr = "<math>
            <mo>{</mo>
            <mi>x</mi>
            <mo>∈</mo>
            <mi>ℤ</mi>
            <mo>:</mo>
            <mi>x</mi>
            <mo>&#x003E;</mo>
            <mn>5</mn>
            <mo>}</mo>
            </math>";
            test_ClearSpeak_prefs("is", vec![("ClearSpeak_SetMemberSymbol", "Member"), ("ClearSpeak_Sets", "woAll")],
                expr, "mengi x meðlimur í heilu tölunum þannig að x er stærra en 5");
}

#[test]
fn multiple_element_set_woall() {
    let expr = "<math>
                <mo>{</mo> <mn>5</mn> <mo>,</mo> <mn>10</mn>  <mo>,</mo> <mn>15</mn> <mo>}</mo>
            </math>";
    test_ClearSpeak("is", "ClearSpeak_Sets", "woAll", expr, "mengið 5 komma 10 komma 15");
}

#[test]
fn multiple_element_set_silent_bracket() {
    let expr = "<math>
                <mo>{</mo> <mn>5</mn> <mo>,</mo> <mn>10</mn>  <mo>,</mo> <mn>15</mn> <mo>}</mo>
            </math>";
            test_ClearSpeak("is", "ClearSpeak_Sets", "SilentBracket", expr, "5 komma 10 komma 15");
        }

#[test]
fn silent_bracket() {
    let expr = "<math>
                <mo>{</mo><mrow><mi>x</mi><mo>|</mo><mi>x</mi><mo>&#x003E;</mo><mn>2</mn></mrow><mo>}</mo>
            </math>";
            test_ClearSpeak("is", "ClearSpeak_Sets", "SilentBracket", expr,
                    "mengi allra x þannig að x er stærra en 2");
        }

