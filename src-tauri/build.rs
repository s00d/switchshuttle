fn main() {
    let mut attributes = tauri_build::Attributes::new();

    // Windows: tauri-build вшивает манифест (Common Controls v6) только в exe приложения.
    // Тестовый бинарник без него падает при запуске с STATUS_ENTRYPOINT_NOT_FOUND,
    // поэтому вшиваем тот же манифест линкером во все бинарники, включая тесты.
    if std::env::var("CARGO_CFG_TARGET_ENV").as_deref() == Ok("msvc") {
        let manifest = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("windows-app-manifest.xml");
        println!("cargo:rerun-if-changed={}", manifest.display());
        println!("cargo:rustc-link-arg=/MANIFEST:EMBED");
        println!("cargo:rustc-link-arg=/MANIFESTINPUT:{}", manifest.display());
        attributes = attributes
            .windows_attributes(tauri_build::WindowsAttributes::new_without_app_manifest());
    }

    tauri_build::try_build(attributes).expect("failed to run tauri-build");
}
