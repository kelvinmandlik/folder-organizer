pub const FOLDERS: &[(&str, &[&str])] = &[
    ("APKS", &["apk"]),
    ("ARCHIVES", &["iso"]),
    (
        "AUDIOS",
        &[
            "3gp", "aa", "aac", "aax", "act", "aiff", "alac", "amr", "ape",
            "au", "awb", "dct", "dss", "dvf", "flac", "gsm", "iklax", "ivs",
            "m4a", "m4b", "m4p", "mmf", "mp3", "mpc", "msv", "nmf", "nsf",
            "ogg", "oga", "mogg", "opus", "ra", "rm", "raw", "rf64", "sln",
            "tta", "voc", "vox", "wav", "wma", "wv", "8svx", "cda",
        ],
    ),
    ("COMPRESSED", &["zip", "tar", "rar", "7z", "dmg"]),
    ("DOCUMENTS", &["txt", "doc", "docx", "pdf", "xlsx", "rtf"]),
    ("IMAGES", &["jpg", "jpeg", "png", "gif", "bmp", "svg"]),
    (
        "PACKAGES",
        &["jar", "msi", "deb", "rpm", "pkg", "crx", "appimage"],
    ),
    ("PROGRAMS", &["exe"]),
    (
        "VIDEOS",
        &[
            "webm", "mpg", "mp2", "mpeg", "mpe", "mpv", "ogg", "mp4", "m4p",
            "m4v", "avi", "wmv", "mov", "qt", "flv", "swf", "mkv",
        ],
    ),
];

pub const CONSOLE_WIDTH: usize = 70;
pub const PROGRESS_BAR_LENGTH: usize = 20;
pub const FOLDER_DIR_NAME: &str = "FOLDERS";
pub const OTHER_DIR_NAME: &str = "OTHER";
