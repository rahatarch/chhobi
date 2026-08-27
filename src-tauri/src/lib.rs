use std::path::Path;
use tauri::Emitter;

#[tauri::command]
async fn transform_images(app_handle: tauri::AppHandle, input: String, output: String) -> Result<String, String> {
    let input_path = Path::new(&input);

    let (paths, unsupported) = chhobi::pipeline::walk_images(input_path);

    if paths.is_empty() {
        return Err("No images found in the selected folder.".to_string());
    }

    let total = paths.len();

    let result = tokio::task::spawn_blocking(move || {
        let mut results: Vec<chhobi::pipeline::FileResult> = Vec::with_capacity(total);

        for (i, path) in paths.iter().enumerate() {
            let result = chhobi::pipeline::process_file(path);
            results.push(result);
            let _ = app_handle.emit("progress", serde_json::json!({
                "current": i + 1,
                "total": total,
            }));
        }

        let summary = chhobi::pipeline::aggregate_results(&results);

        if summary.processed == 0 {
            let reasons: String = summary.skip_reasons.join("; ");
            return Err(format!("No images processed: {}", reasons));
        }

        let deduped = chhobi::pipeline::dedup_results(&results);
        chhobi::archive::create_zip(&output, &deduped);

        let msg = format!(
            "Done. Processed {} images ({} skipped, {} unsupported). Output: {}",
            summary.processed,
            summary.skipped,
            unsupported.len(),
            output
        );
        Ok(msg)
    })
    .await
    .map_err(|e| format!("Task join error: {}", e))?;

    result
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![transform_images])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}