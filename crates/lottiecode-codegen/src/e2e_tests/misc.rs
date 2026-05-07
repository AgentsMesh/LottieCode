//! e2e_tests::misc

use super::{build, compile, shape_items};

// ============================================================
// dotLottie zip 打包
// ============================================================

#[test]
fn dotlottie_zip_round_trip() {
    use std::io::Read;
    let ir = compile(
        r#"composition "x" {
            width=100 height=100 fps=30 duration=1s
            shape s { rect { size = [40, 40] } fill = #000000 position = [50, 50] }
        }"#,
    );
    let buf = crate::dotlottie::pack(&ir).expect("pack succeeded");
    assert!(buf.len() > 100, "zip 应该非空");

    let mut zip = zip::ZipArchive::new(std::io::Cursor::new(buf)).unwrap();
    let names: Vec<String> = (0..zip.len())
        .map(|i| zip.by_index(i).unwrap().name().to_string())
        .collect();
    assert!(names.iter().any(|n| n == "manifest.json"));
    assert!(names.iter().any(|n| n.starts_with("animations/")));

    let anim_path = names.iter().find(|n| n.starts_with("animations/")).unwrap().clone();
    let mut anim_str = String::new();
    zip.by_name(&anim_path)
        .unwrap()
        .read_to_string(&mut anim_str)
        .unwrap();
    assert!(anim_str.contains("\"v\""));
}
