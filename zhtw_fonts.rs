use eframe::egui;
use egui::{
    epaint::text::{FontInsert, FontPriority, InsertFontFamily},
    Context, FontData, FontFamily,
};
use std::path::PathBuf;

fn zh_tw_font_candidates() -> Vec<PathBuf> {
    #[cfg(target_os = "macos")]
    {
        vec![
            PathBuf::from("/System/Library/Fonts/STHeiti Medium.ttc"),
            PathBuf::from("/System/Library/Fonts/STHeiti Light.ttc"),
            PathBuf::from("/System/Library/Fonts/Hiragino Sans GB.ttc"),
            PathBuf::from("/System/Library/Fonts/AppleSDGothicNeo.ttc"),
            PathBuf::from("/System/Library/Fonts/Supplemental/Songti.ttc"),
        ]
    }

    #[cfg(target_os = "windows")]
    {
        vec![
            PathBuf::from("C:/Windows/Fonts/msjh.ttc"), // Microsoft JhengHei
            PathBuf::from("C:/Windows/Fonts/mingliu.ttc"), // MingLiU
        ]
    }

    #[cfg(target_os = "linux")]
    {
        vec![
            PathBuf::from("/usr/share/fonts/noto-cjk/NotoSansCJK-Regular.ttc"),
            PathBuf::from("/usr/share/fonts/noto-cjk/NotoSerifCJK-Regular.ttc"),
        ]
    }

    #[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "linux")))]
    {
        vec![]
    }
}

pub fn add_zh_tw_ui_fonts(ctx: &Context) {
    let candidates = zh_tw_font_candidates();

    for (i, path) in candidates.iter().enumerate() {
        if path.exists() {
            match std::fs::read(path) {
                Ok(bytes) => {
                    let priority = if i == 0 {
                        FontPriority::Highest
                    } else {
                        FontPriority::Lowest
                    };

                    let font_name = path.file_name().unwrap().to_string_lossy();

                    ctx.add_font(FontInsert::new(
                        &font_name,
                        FontData::from_owned(bytes),
                        vec![
                            InsertFontFamily {
                                family: FontFamily::Proportional,
                                priority,
                            },
                            InsertFontFamily {
                                family: FontFamily::Monospace,
                                priority: FontPriority::Lowest,
                            },
                        ],
                    ));

                    eprintln!("✅ 已載入系統字型: {}", font_name);
                    return;
                }
                Err(err) => {
                    eprintln!("⚠️ 無法讀取字型檔 {:?}: {}", path, err);
                }
            }
        } else {
            eprintln!("⚠️ 找不到字型檔案: {:?}", path);
        }
    }

    eprintln!("❌ 無法載入任何繁體中文字型！");
}
