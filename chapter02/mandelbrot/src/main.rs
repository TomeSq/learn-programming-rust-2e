use image::ColorType;
use image::png::PNGEncoder;
use num::Complex;
use std::env;
use std::fs::File;
use std::str::FromStr;

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
    let args: Vec<String> = env::args().collect();

    if args.len() != 5 {
        eprintln!("Usage: {} FILE PIXEL UPPERLEFT LOWERRIGHT", args[0]);
        eprintln!("Example: {} mandel.png 100x750 -1.20,0.35 -1,0.20", args[0]);
        std::process::exit(1);
    }

    let bounds = parse_pair(&args[2], 'x').expect("error parsing image dimensions");
    let upper_left = parse_complex(&args[3]).expect("error parsing upper left point");
    let lower_right = parse_complex(&args[4]).expect("error parsing lower right point");

    //長さbounds.0 * bounds.1でベクタを作り、0で初期化する
    let mut pixels = vec![0; bounds.0 * bounds.1];

    let thrads = 8;
    let rows_per_band = bounds.1 / thrads + 1;

    {
        let bands: Vec<&mut [u8]> = pixels.chunks_mut(rows_per_band * bounds.0).collect();
        crossbeam::scope(|spawner| {
            for (i, band) in bands.into_iter().enumerate() {
                let top = rows_per_band * i;
                let hight = band.len() / bounds.0;
                let band_bounds = (bounds.0, hight);
                let band_upper_left = pixel_to_point(bounds, (0, top), upper_left, lower_right);
                let band_lower_right =
                    pixel_to_point(bounds, (bounds.0, top + hight), upper_left, lower_right);

                spawner.spawn(move |_| {
                    render(band, band_bounds, band_upper_left, band_lower_right);
                });
            }
        })
        .unwrap();
    }

    //    render(&mut pixels, bounds, upper_left, lower_right);

    write_image(&args[1], &pixels, bounds).expect("error writing PNG file");
}

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
    parse_pair(s, ',').map(|(re, im)| Complex { re, im })
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

/// マンデルブロ集合の画像をバイト配列にレンダリングする
///
/// # 引数
/// * `pixels` - レンダリング結果を格納するバイト配列。各バイトは1ピクセルを表す
/// * `bounds` - 画像の寸法 (幅, 高さ) を表すタプル
/// * `upper_left` - 表示する複素平面上の左上隅の座標
/// * `lower_right` - 表示する複素平面上の右下隅の座標
///
/// # 注意
/// * `pixels`の長さは`bounds.0 * bounds.1`と同じである必要がある
/// * 各ピクセル値は0～255の範囲で、0は集合に属する点（黒）、255に近いほど
///   集合から早く発散する点（白に近い）を表す
fn render(
    pixels: &mut [u8],
    bounds: (usize, usize),
    upper_left: Complex<f64>,
    lower_right: Complex<f64>,
) {
    assert!(pixels.len() == bounds.0 * bounds.1);

    // 画像の各行を走査
    for now in 0..bounds.1 {
        // 各行の各列を走査
        for column in 0..bounds.0 {
            // ピクセル座標を複素平面上の点に変換
            let point = pixel_to_point(bounds, (column, now), upper_left, lower_right);

            // 対応する複素数がマンデルブロ集合に属するかを判定し、適切な色を設定
            // 集合に属する点は黒(0)、属さない点は発散の速さに応じたグレースケール値
            pixels[now * bounds.0 + column] = match escape_time(point, 255) {
                None => 0,
                Some(count) => 255 - count as u8,
            };
        }
    }
}

fn write_image(
    filename: &str,
    pixels: &[u8],
    bounds: (usize, usize),
) -> Result<(), std::io::Error> {
    let output = File::create(filename)?;

    let encoder = PNGEncoder::new(output);
    encoder.encode(pixels, bounds.0 as u32, bounds.1 as u32, ColorType::Gray(8))?;

    Ok(())
}
