use num::Complex;

/// "limit"を繰り返しの回数の上限として、"C"がマンデルブロ集合に属するかを判定する。
/// "c"がマンデルブロ集合に含まれなかったら、'Some(i)'を返す。
/// "i"は"c"が原点を中心とする半径2の円から出るまでにかかった繰り返し回数となる
/// "c"がマンデルブロ集合に含まれる場合は(正確に言うと、繰り返し回数の上限に達しても"C"がマンデルブロ集合に含まれないことを示せなかった場合)
/// "None"を返す
fn escape_time(c: Complex<f64>, limit: usize) -> Option<usize> {
    let mut z = Complex { re: 0.0, im: 0.0 };
    for i in 0..limit {
        if z.norm_sqr() > 4.0 {
            return Some(i);
        }
        z = z * z + c;
    }
    None
}

fn main() {
    println!("Hello, world!");
}

fn complex_square_add_loop(c: Complex<f64>) {
    let mut z = Complex { re: 0.0, im: 0.0 };
    loop {
        z = z * z + c;
    }
}

use std::str::FromStr;

/// 文字列"s"は座標系のペア`"400×600"`、`"1.0,0.5"`など
/// "s"は<left><seq><right>の形式である必要がある
/// <seq>は"separator"引数で与えられる文字で、
/// <left>と<right>は双方とも"T::from_str"でパースできる文字列
/// "separator"はASCIII文字でなければならない
/// "s"が適切な形でなければ、"Some<(x,y)>"を返す
/// パースできなければ"None"を返す
fn parse_pair<T: FromStr>(s: &str, separator: char) -> Option<(T, T)> {
    match s.find(separator) {
        None => None,
        //separatorの前後で文字列を数値化する
        Some(index) => match (T::from_str(&s[..index]), T::from_str(&s[index + 1..])) {
            (Ok(l), Ok(r)) => Some((l, r)),
            _ => None,
        },
    }
}

#[test]
fn test_parse_pair() {
    assert_eq!(parse_pair::<i32>("", ','), None);
    assert_eq!(parse_pair::<i32>("10,", ','), None);
    assert_eq!(parse_pair::<i32>(",10", ','), None);
    assert_eq!(parse_pair::<i32>("10,20", ','), Some((10, 20)));
    assert_eq!(parse_pair::<i32>("10,20xy", ','), None);
    assert_eq!(parse_pair::<f64>("0.5x", 'x'), None);
    assert_eq!(parse_pair::<f64>("0.5x1.5", 'x'), Some((0.5, 1.5)));
}

fn parse_complex(s: &str) -> Option<Complex<f64>> {
    match parse_pair(s, ',') {
        Some((re, im)) => Some(Complex { re, im }),
        None => None,
    }
}

#[test]
fn test_parse_complex() {
    assert_eq!(
        parse_complex("1.25,-0.0625"),
        Some(Complex {
            re: 1.25,
            im: -0.0625
        })
    );
    assert_eq!(
        parse_complex("1.25,-0.0625"),
        Some(Complex {
            re: 1.25,
            im: -0.0625
        })
    );
}

/// 画素の座標をマンデルブロ集合の複素平面上の点に変換する
///
/// # 引数
/// * `bounds` - 出力画像の全体のピクセル寸法 (幅, 高さ) のタプル
/// * `pixel` - 変換する画像上のピクセル座標 (x, y) のタプル
/// * `upper_left` - 複素平面上の左上隅に対応する点
/// * `lower_right` - 複素平面上の右下隅に対応する点
///
/// # 戻り値
/// 指定されたピクセルに対応する複素平面上の座標
///
/// # 注意
/// 画像座標系では左上が原点(0,0)で、y座標は下に向かって増加します。
/// 一方、複素平面では虚部は上に向かって増加するため、y座標の変換時には
/// 反転する必要があります。
fn pixel_to_point(
    bounds: (usize, usize),
    pixel: (usize, usize),
    upper_left: Complex<f64>,
    lower_right: Complex<f64>,
) -> Complex<f64> {
    let (width, height) = (
        lower_right.re - upper_left.re,
        upper_left.im - lower_right.im,
    );
    Complex {
        re: upper_left.re + pixel.0 as f64 * width / bounds.0 as f64,
        //ここが引き算となっているのなぜか?
        // 上に動くとpixel.1は増えるが、虚部は小さくなるから
        im: upper_left.im - pixel.1 as f64 * height / bounds.1 as f64,
    }
}

#[test]
fn test_pixel_to_point() {
    assert_eq!(
        pixel_to_point(
            (100, 200),
            (25, 175),
            Complex { re: -1.0, im: 1.0 },
            Complex { re: 1.0, im: -1.0 }
        ),
        Complex {
            re: -0.5,
            im: -0.75
        }
    );
}
