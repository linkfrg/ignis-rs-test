use std::fs;
use std::path::PathBuf;
use std::sync::LazyLock;

pub static CACHE_DIR: LazyLock<PathBuf> = LazyLock::new(|| {
    let dir = glib::user_cache_dir().join("ignis_notifications");
    fs::create_dir_all(&dir).unwrap();
    dir
});

pub static FILE_PATH: LazyLock<PathBuf> =
    LazyLock::new(|| (&*CACHE_DIR).join("notifications.json"));

pub static IMAGE_DIR: LazyLock<PathBuf> = LazyLock::new(|| {
    let dir = (&*CACHE_DIR).join("images");
    fs::create_dir_all(&dir).unwrap();
    dir
});
