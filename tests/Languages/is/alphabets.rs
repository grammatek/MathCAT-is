/// Tests for rules shared between various speech styles:
/// *  this has tests focused on the various alphabets
use crate::common::*;


#[test]
fn special_alphabet_chars() {
  let expr = "<math> <mi>ℌ</mi><mo>,</mo><mi>ℭ</mi></math>";
  test("en", "SimpleSpeak", expr, "fraktúr stórt h komma fraktúr stórt c");
  let expr = "<math> <mi>ℍ</mi><mo>,</mo><mi>ℿ</mi></math>";
  test("en", "SimpleSpeak", expr, "tvístrikað stórt h komma tvístrikað stórt pí");
  let expr = "<math> <mi>ℐ</mi><mo>,</mo><mi>ℳ</mi></math>";
  test("en", "SimpleSpeak", expr, "skrifletur stórt i komma skrifletur stórt m");
}

#[test]
fn greek() {
    let expr = "<math> <mi>Α</mi><mo>,</mo><mi>Ω</mi></math>";
    test("en", "SimpleSpeak", expr, "stórt alfa komma stórt ómega");
    let expr = "<math> <mi>α</mi><mo>,</mo><mi>ω</mi></math>";
    test("en", "SimpleSpeak", expr, "alfa komma ómega");
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("en", "SimpleSpeak", expr, "tvístrikað stórt delta komma tvístrikað stórt ufsilon");
    let expr = "<math> <mi>α</mi><mo>,</mo><mi>ω</mi></math>";
    test("en", "SimpleSpeak", expr, "alfa komma ómega");
}

#[test]
fn cap_cyrillic() {
    let expr = "<math> <mi>А</mi><mo>,</mo><mi>Я</mi></math>";
    test("en", "SimpleSpeak", expr, "stórt a komma stórt ja");
}

#[test]
fn parenthesized() {
    let expr = "<math> <mi>⒜</mi><mo>,</mo><mi>⒵</mi></math>";
    test("en", "SimpleSpeak", expr, "a í sviga komma z í sviga");
}

#[test]
fn circled() {
    let expr = "<math> <mi>Ⓐ</mi><mo>,</mo><mi>Ⓩ</mi></math>";
    test("en", "SimpleSpeak", expr, "stórt a í hring komma stór z í hring");
    let expr = "<math> <mi>ⓐ</mi><mo>,</mo><mi>ⓩ</mi></math>";
    test("en", "SimpleSpeak", expr, "a í hring komma z í hring");
}

#[test]
fn fraktur() {
    let expr = "<math> <mi>𝔄</mi><mo>,</mo><mi>𝔜</mi></math>";
    test("en", "SimpleSpeak", expr, "fraktúr stórt a komma fraktúr stórt y");
    let expr = "<math> <mi>𝔞</mi><mo>,</mo><mi>𝔷</mi></math>";
    test("en", "SimpleSpeak", expr, "fraktúr a komma fraktúr z");
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("en", "SimpleSpeak", expr, "fraktúr stórt a komma fraktúr stórt y");
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("en", "SimpleSpeak", expr, "fraktúr a komma fraktúr z");
}

#[test]
fn bold_fraktur() {
    let expr = "<math> <mi>𝕬</mi><mo>,</mo><mi>𝖅</mi></math>";
    test("en", "SimpleSpeak", expr, "fraktúr feitletrað stórt a, komma fraktúr feitletruð stór z");
    let expr = "<math> <mi>𝖆</mi><mo>,</mo><mi>𝖟</mi></math>";
    test("en", "SimpleSpeak", expr, "fraktúr feitletrað a komma fraktúr feitletruð z");
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("en", "SimpleSpeak", expr, "fraktúr feitletrað stórt a komma fraktúr feitletruð stór z");
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("en", "SimpleSpeak", expr, "fraktúr feitletrað a komma fraktúr feitletruð z");
}

#[test]
fn double_struck() {
    let expr = "<math> <mi>𝔸</mi><mo>,</mo><mi>𝕐</mi></math>";
    test("en", "SimpleSpeak", expr, "tívstrikað stórt a komma tvístrikað stórt y");
    let expr = "<math> <mi>𝕒</mi><mo>,</mo><mi>𝕫</mi></math>";
    test("en", "SimpleSpeak", expr, "tvístrikað a komma tvístrikuð z");
    let expr = "<math> <mi>𝟘</mi><mo>,</mo><mi>𝟡</mi></math>";
    test("en", "SimpleSpeak", expr, "tvístrikað 0 komma tvístrikað 9");
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("en", "SimpleSpeak", expr, "tvístrikað stórt a komma tvístrikað stórt y");
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("en", "SimpleSpeak", expr, "tvístrikað a komma tvístrikuð z");
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("en", "SimpleSpeak", expr, "tvístrikað 0 komma tvístrikað 9");
}

#[test]
fn script() {
    let expr = "<math> <mi>𝒜</mi><mo>,</mo><mi>𝒵</mi></math>";
    test("en", "SimpleSpeak", expr, "skrifletur stórt a komma skrifletur stór z");
    let expr = "<math> <mi>𝒶</mi><mo>,</mo><mi>𝓏</mi></math>";
    test("en", "SimpleSpeak", expr, "skrifletur a komma skrifletur z");
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("en", "SimpleSpeak", expr, "skrifletur stórt a komma skrifletur stór z");
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("en", "SimpleSpeak", expr, "skrifletur a komma skrifletur z");
}

#[test]
fn bold_script() {
    let expr = "<math> <mi>𝓐</mi><mo>,</mo><mi>𝓩</mi></math>";
    test("en", "SimpleSpeak", expr, "skrifletur feitletrað stórt a komma skrifletur feitletruð stór z");
    let expr = "<math> <mi>𝓪</mi><mo>,</mo><mi>𝔃</mi></math>";
    test("en", "SimpleSpeak", expr, "skrifletur feitletrað a komma skrifletur feitletruð z");
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("en", "SimpleSpeak", expr, "skrifletur feitletrað stórt a komma skrifletur feitletruð stór z");
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("en", "SimpleSpeak", expr, "skrifletur feitletrað a komma skrifletur feitletruð z");
}

#[test]
fn bold() {
    let expr = "<math> <mi>𝐀</mi><mo>,</mo><mi>𝐙</mi></math>";
    test("en", "SimpleSpeak", expr, "feitletrað stórt a komma feitletruð stór z");
    let expr = "<math> <mi>𝐚</mi><mo>,</mo><mi>𝐳</mi></math>";
    test("en", "SimpleSpeak", expr, "feitletrað a komma feitletruð z");
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("en", "SimpleSpeak", expr, "feitletrað stórt a komma feitletruð stór z");
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("en", "SimpleSpeak", expr, "feitletrað a komma feitletruð z");
}

#[test]
fn italic() {
    let expr = "<math> <mi>𝐴</mi><mo>,</mo><mi>𝑍</mi></math>";
    test("en", "SimpleSpeak", expr, "stórt a komma stór z");
    let expr = "<math> <mi>𝑎</mi><mo>,</mo><mi>𝑧</mi></math>";
    test("en", "SimpleSpeak", expr, "a komma z");
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("en", "SimpleSpeak", expr, "stórt a komma stór z");
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("en", "SimpleSpeak", expr, "a komma z");
}

#[test]
fn sans_serif() {
  let expr = "<math> <mi>𝖠</mi><mo>,</mo><mi>𝖹</mi></math>";
  test("en", "SimpleSpeak", expr, "stórt a komma stór z");
  let expr = "<math> <mi>𝖺</mi><mo>,</mo><mi>𝗓</mi></math>";
  test("en", "SimpleSpeak", expr, "a komma z");
  // MathType private space versions
  let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
  test("en", "SimpleSpeak", expr, "stórt a komma stór z");
  let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
  test("en", "SimpleSpeak", expr, "a komma z");
}

#[test]
fn sans_serif_bold() {
    let expr = "<math> <mi>𝗔</mi><mo>,</mo><mi>𝗭</mi></math>";
    test("en", "SimpleSpeak", expr, "feitletrað stórt a komma feitletruð stór z");
    let expr = "<math> <mi>𝗮</mi><mo>,</mo><mi>𝘇</mi></math>";
    test("en", "SimpleSpeak", expr, "feitletrað a komma feitletruð z");
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("en", "SimpleSpeak", expr, "feitletrað stórt a komma feitletruð stór z");
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("en", "SimpleSpeak", expr, "feitletrað a komma feitletruð z");
}

#[test]
fn sans_serif_italic() {
    let expr = "<math> <mi>𝘈</mi><mo>,</mo><mi>𝘡</mi></math>";
    test("en", "SimpleSpeak", expr, "stórt a komma stór z");
    let expr = "<math> <mi>𝘢</mi><mo>,</mo><mi>𝘻</mi></math>";
    test("en", "SimpleSpeak", expr, "a komma z");
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("en", "SimpleSpeak", expr, "stórt a komma stór z");
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("en", "SimpleSpeak", expr, "a komma z");
}

#[test]
fn sans_serif_bold_italic() {
    let expr = "<math> <mi>𝘼</mi><mo>,</mo><mi>𝙕</mi></math>";
    test("en", "SimpleSpeak", expr, "feitletrað stórt a komma feitletruð stór z");
    let expr = "<math> <mi>𝙖</mi><mo>,</mo><mi>𝙯</mi></math>";
    test("en", "SimpleSpeak", expr, "feitletrað a komma feitletruð z");
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("en", "SimpleSpeak", expr, "feitletrað stórt a komma feitletruð stór z");
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("en", "SimpleSpeak", expr, "feitletrað a komma feitletruð z");
}

#[test]
fn monospace() {
    let expr = "<math> <mi>𝙰</mi><mo>,</mo><mi>𝚉</mi></math>";
    test("en", "SimpleSpeak", expr, "stórt a komma stór z");
    let expr = "<math> <mi>𝚊</mi><mo>,</mo><mi>𝚣</mi></math>";
    test("en", "SimpleSpeak", expr, "a komma z");
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("en", "SimpleSpeak", expr, "stórt a komma stór z");
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("en", "SimpleSpeak", expr, "a komma z");
}


#[test]
fn bold_greek() {
    let expr = "<math> <mi>𝚨</mi><mo>,</mo><mi>𝛀</mi></math>";
    test("en", "SimpleSpeak", expr, "feitletrað stórt alfa komma feitletrað stórt ómega");
    let expr = "<math> <mi>𝛂</mi><mo>,</mo><mi>𝛚</mi></math>";
    test("en", "SimpleSpeak", expr, "feitletrað alfa komma feitletrað ómega");
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("en", "SimpleSpeak", expr, "feitletrað stórt alfa komma feitletrað stórt ómega");
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("en", "SimpleSpeak", expr, "feitletrað alpha komma feitletrað omega");
}

#[test]
fn bold_greek_others() {
    let expr = "<math> <mi>𝛛</mi><mo>,</mo><mi>𝛡</mi></math>";
    test("en", "SimpleSpeak", expr, "feitletrað partial derivative, komma feitletrað pi");
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("en", "SimpleSpeak", expr, "feitletrað partial derivative, komma feitletrað pi");
}


#[test]
fn italic_greek() {
    let expr = "<math> <mi>𝛢</mi><mo>,</mo><mi>𝛺</mi></math>";
    test("en", "SimpleSpeak", expr, "stórt alfa komma stórt ómega");
    let expr = "<math> <mi>𝛼</mi><mo>,</mo><mi>𝜔</mi></math>";
    test("en", "SimpleSpeak", expr, "alpha komma omega");
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("en", "SimpleSpeak", expr, "stórt alfa komma stórt ómega");
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("en", "SimpleSpeak", expr, "alpha komma omega");
}

#[test]
fn italic_greek_others() {
    let expr = "<math> <mi>𝜕</mi><mo>,</mo><mi>𝜛</mi></math>";
    test("en", "SimpleSpeak", expr, "partial derivative komma pi");
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("en", "SimpleSpeak", expr, "partial derivative komma pi");
}

#[test]
fn bold_italic_greek() {
    let expr = "<math> <mi>𝜜</mi><mo>,</mo><mi>𝜴</mi></math>";
    test("en", "SimpleSpeak", expr, "feitletrað stórt alfa komma feitletrað stórt ómega");
    let expr = "<math> <mi>𝜶</mi><mo>,</mo><mi>𝝎</mi></math>";
    test("en", "SimpleSpeak", expr, "feitletrað alpha komma feitletrað omega");
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("en", "SimpleSpeak", expr, "feitletrað stórt alfa komma feitletrað stórt ómega");
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("en", "SimpleSpeak", expr, "feitletrað alpha komma feitletrað omega");
}

#[test]
fn bold_italic_greek_others() {
    let expr = "<math> <mi>𝝏</mi><mo>,</mo><mi>𝝕</mi></math>";
    test("en", "SimpleSpeak", expr, "feitletrað partial derivative, komma feitletrað pi");
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("en", "SimpleSpeak", expr, "feitletrað partial derivative, komma feitletrað pi");
}

#[test]
fn sans_serif_bold_greek() {
    let expr = "<math> <mi>𝝖</mi><mo>,</mo><mi>𝝮</mi></math>";
    test("en", "SimpleSpeak", expr, "feitletrað stórt alfa komma feitletrað stórt ómega");
    let expr = "<math> <mi>𝝰</mi><mo>,</mo><mi>𝞈</mi></math>";
    test("en", "SimpleSpeak", expr, "feitletrað alpha komma feitletrað omega");
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("en", "SimpleSpeak", expr, "feitletrað stórt alfa komma feitletrað stórt ómega");
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("en", "SimpleSpeak", expr, "feitletrað alpha komma feitletrað omega");
}

#[test]
fn sans_serif_bold_greek_others() {
    let expr = "<math> <mi>𝞉</mi><mo>,</mo><mi>𝞏</mi></math>";
    test("en", "SimpleSpeak", expr, "feitletrað partial derivative, komma feitletrað pi");
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("en", "SimpleSpeak", expr, "feitletrað partial derivative, komma feitletrað pi");
}

#[test]
fn sans_serif_bold_italic_greek() {
    let expr = "<math> <mi>𝞐</mi><mo>,</mo><mi>𝞨</mi></math>";
    test("en", "SimpleSpeak", expr, "feitletrað stórt alfa komma feitletrað stórt ómega");
    let expr = "<math> <mi>𝞪</mi><mo>,</mo><mi>𝟂</mi></math>";
    test("en", "SimpleSpeak", expr, "feitletrað alfa komma feitletrað ómega");
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("en", "SimpleSpeak", expr, "feitletrað stórt alfa komma feitletrað stórt ómega");
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("en", "SimpleSpeak", expr, "feitletrað alfa komma feitletrað ómega");
}

#[test]
fn sans_serif_bold_italic_greek_others() {
    let expr = "<math> <mi>𝟃</mi><mo>,</mo><mi>𝟉</mi></math>";
    test("en", "SimpleSpeak", expr, "feitletruð hlutafleiða komma feitletrað pí");
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("en", "SimpleSpeak", expr, "feitletruð hlutafleiða komma feitletrað pí");
}

#[test]
fn pua_regular() {
  let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
  test("en", "SimpleSpeak", expr, "stórt a komma stór z");
}

#[test]
fn turned() {
    let expr = "<math> <mi>Ⅎ</mi><mo>,</mo><mi>⅄</mi></math>";
    test("en", "SimpleSpeak", expr, "öfugt stórt f komma öfugur blokkstafur stórt y");
  }

#[test]
fn enclosed_numbers() {
  let expr = "<math> <mi>①</mi><mo>,</mo><mi>⑨</mi></math>";
  test("en", "SimpleSpeak", expr, "hringaður einn komma hringað 9");
  let expr = "<math> <mi>⑴</mi><mo>,</mo><mi>⑼</mi></math>";
  test("en", "SimpleSpeak", expr, "einn í sviga komma 9 í sviga");
  let expr = "<math> <mi>⒈</mi><mo>,</mo><mi>⒐</mi></math>";
  test("en", "SimpleSpeak", expr, "einn með punkti komma 9 með punkti");
  let expr = "<math> <mi>⓵</mi><mo>,</mo><mi>⓽</mi></math>";
  test("en", "SimpleSpeak", expr, "tvöfalt hringaður einn komma tvöfalt hringað 9");
}
