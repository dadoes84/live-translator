use std::fs;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use tauri::{Emitter, Manager};
use windows::{
    Graphics::Imaging::{BitmapPixelFormat, SoftwareBitmap},
    Media::Ocr::OcrEngine,
    Globalization::Language,
    Storage::Streams::DataWriter,
    core::HSTRING,
};

// ---------- 区域结构体 ----------
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Region {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

// ---------- 配置结构体 ----------
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AppConfig {
    pub scan_frequency: u32,
    pub ocr_language: String,
    pub font_family: String,
    pub font_size: u32,
    pub text_color: String,
    pub bg_color: String,
    pub is_pinned: bool,
    pub api_key: String,
    pub model: String,
    pub prompt: String,
    pub shortcut_toggle: String,
    pub shortcut_area: String,
    pub region: Option<Region>,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            scan_frequency: 3,
            ocr_language: "ja".into(),
            font_family: "Microsoft YaHei".into(),
            font_size: 18,
            text_color: "#ffffff".into(),
            bg_color: "#1a1a2e".into(),
            is_pinned: false,
            api_key: String::new(),
            model: "deepseek-v4-flash".into(),
            prompt: "你是一个专业的本地化翻译器，只将用户输入的文本翻译成简体中文。\n注意：输入文本中的换行可能是同一句话的折行，也可能是段落分隔。请根据语义自行判断，在译文中正确处理段落和换行。不要添加任何解释，只输出译文。".into(),
            shortcut_toggle: "Ctrl+Shift+T".into(),
            shortcut_area: "Ctrl+Shift+A".into(),
            region: None,
        }
    }
}

fn get_config_path(app_handle: &tauri::AppHandle) -> PathBuf {
    let mut path = app_handle.path().app_config_dir().expect("无法获取应用配置目录");
    fs::create_dir_all(&path).ok();
    path.push("config.json");
    path
}

// ---------- 配置读写 ----------
#[tauri::command]
fn load_config(app_handle: tauri::AppHandle) -> Result<AppConfig, String> {
    let path = get_config_path(&app_handle);
    if path.exists() {
        let content = fs::read_to_string(&path).map_err(|e| e.to_string())?;
        serde_json::from_str(&content).map_err(|e| format!("配置文件解析失败: {}", e))
    } else {
        Ok(AppConfig::default())
    }
}

#[tauri::command]
fn save_config(app_handle: tauri::AppHandle, mut config: AppConfig) -> Result<(), String> {
    let path = get_config_path(&app_handle);
    // 保留已有的选区
    if config.region.is_none() && path.exists() {
        if let Ok(content) = fs::read_to_string(&path) {
            if let Ok(existing) = serde_json::from_str::<AppConfig>(&content) {
                config.region = existing.region;
            }
        }
    }
    let json = serde_json::to_string_pretty(&config).map_err(|e| e.to_string())?;
    fs::write(&path, json).map_err(|e| e.to_string())
}

// ---------- 选区控制 ----------
#[tauri::command]
fn confirm_area_selector(
    app: tauri::AppHandle,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
) -> Result<(), String> {
    let region = Region { x, y, width, height };

    let path = get_config_path(&app);
    let mut config: AppConfig = if path.exists() {
        let content = fs::read_to_string(&path).map_err(|e| e.to_string())?;
        serde_json::from_str(&content).unwrap_or_default()
    } else {
        AppConfig::default()
    };
    config.region = Some(region.clone());
    let json = serde_json::to_string_pretty(&config).map_err(|e| e.to_string())?;
    fs::write(&path, json).map_err(|e| e.to_string())?;

    app.emit("area-selected", &region).map_err(|e| e.to_string())?;

    if let Some(window) = app.get_webview_window("area-selector") {
        window.hide().map_err(|e| e.to_string())?;
    }
    if let Some(main) = app.get_webview_window("main") {
        main.show().map_err(|e| e.to_string())?;
        main.set_focus().map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
fn cancel_area_selector(app: tauri::AppHandle) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("area-selector") {
        window.hide().map_err(|e| e.to_string())?;
    }
    if let Some(main) = app.get_webview_window("main") {
        main.show().map_err(|e| e.to_string())?;
        main.set_focus().map_err(|e| e.to_string())?;
    }
    Ok(())
}

// ---------- OCR ----------
#[tauri::command]
fn capture_and_ocr(
    x: i32, y: i32, width: i32, height: i32, language: String,
) -> Result<String, String> {
    // 1. 找到目标显示器
    let monitors = xcap::Monitor::all().map_err(|e| e.to_string())?;
    let monitor = monitors
        .into_iter()
        .find(|m| {
            let mx = m.x().unwrap_or(0);
            let my = m.y().unwrap_or(0);
            let mw = m.width().unwrap_or(0) as i32;
            let mh = m.height().unwrap_or(0) as i32;
            x >= mx && y >= my && x + width <= mx + mw && y + height <= my + mh
        })
        .ok_or("选区不在任何显示器范围内".to_string())?;

    // 2. 截图
    let monitor_x = monitor.x().unwrap_or(0);
    let monitor_y = monitor.y().unwrap_or(0);
    let img = monitor.capture_image().map_err(|e| e.to_string())?;

    // 3. 裁切（相对显示器坐标）
    let rx = (x - monitor_x) as u32;
    let ry = (y - monitor_y) as u32;
    let cropped = image::imageops::crop_imm(&img, rx, ry, width as u32, height as u32);
    let rgba = cropped.to_image().into_raw();

    // 4. RGBA → BGRA
    let len = (width * height) as usize;
    let mut bgra = vec![0u8; len * 4];
    for i in 0..len {
        bgra[i * 4] = rgba[i * 4 + 2];     // B
        bgra[i * 4 + 1] = rgba[i * 4 + 1]; // G
        bgra[i * 4 + 2] = rgba[i * 4];     // R
        bgra[i * 4 + 3] = rgba[i * 4 + 3]; // A
    }

    // 5. 创建 SoftwareBitmap
    let writer = DataWriter::new().map_err(|e| e.to_string())?;
    writer.WriteBytes(&bgra).map_err(|e| e.to_string())?;
    let buffer = writer.DetachBuffer().map_err(|e| e.to_string())?;
    let bitmap = SoftwareBitmap::CreateCopyFromBuffer(
        &buffer,
        BitmapPixelFormat::Bgra8,
        width,
        height,
    ).map_err(|e| format!("创建位图失败: {}", e))?;

    // 6. OCR 识别
    let lang_tag: HSTRING = match language.as_str() {
        "ja" => "ja".into(),
        _ => "en-US".into(),
    };
    let lang = Language::CreateLanguage(&lang_tag)
        .map_err(|e| format!("创建语言失败: {}", e))?;
    let engine = OcrEngine::TryCreateFromLanguage(&lang)
        .map_err(|e| format!("创建OCR引擎失败（请确认已安装该语言OCR包）: {}", e))?;

    let ocr_result = engine
        .RecognizeAsync(&bitmap)
        .map_err(|e| format!("OCR调用失败: {}", e))?
        .get()
        .map_err(|e| format!("OCR识别失败: {}", e))?;

    // 7. 获取所有行文本（换行判断交给 AI）
    let lines = ocr_result.Lines().map_err(|e| e.to_string())?;
    let line_count = lines.Size().map_err(|e| e.to_string())? as usize;
    if line_count == 0 {
        return Ok(String::new());
    }

    let mut texts: Vec<String> = Vec::new();
    for i in 0..line_count as u32 {
        let line = lines.GetAt(i).map_err(|e| e.to_string())?;
        texts.push(line.Text().map_err(|e| e.to_string())?.to_string());
    }

    Ok(texts.join("\n"))
}

// ---------- 翻译 ----------
#[tauri::command]
async fn translate(
    app_handle: tauri::AppHandle,
    text: String,
    source_lang: String,
) -> Result<String, String> {
    let path = get_config_path(&app_handle);
    let config: AppConfig = if path.exists() {
        let content = fs::read_to_string(&path).map_err(|e| e.to_string())?;
        serde_json::from_str(&content).unwrap_or_default()
    } else {
        AppConfig::default()
    };

    if config.api_key.is_empty() {
        return Err("请先在 AI 设置中填写 API Key".to_string());
    }

    let source_name = match source_lang.as_str() {
        "ja" => "日语",
        _ => "英语",
    };

    let system_prompt = if config.prompt.is_empty() {
        format!("你是一个专业的本地化翻译器，只将用户输入的{source_name}文本翻译成简体中文。\n注意：输入文本中的换行可能是同一句话的折行，也可能是段落分隔。请根据语义自行判断，在译文中正确处理段落和换行。不要添加任何解释，只输出译文。")
    } else {
        config.prompt.clone()
    };

    let client = reqwest::Client::new();
    let resp = client
        .post("https://api.deepseek.com/chat/completions")
        .header("Authorization", format!("Bearer {}", config.api_key))
        .header("Content-Type", "application/json")
        .json(&serde_json::json!({
            "model": config.model,
            "messages": [
                {"role": "system", "content": system_prompt},
                {"role": "user", "content": text}
            ],
            "max_tokens": 2048
        }))
        .send()
        .await
        .map_err(|e| format!("翻译请求失败: {}", e))?;

    let body: serde_json::Value = resp.json().await.map_err(|e| e.to_string())?;

    let translation = body["choices"][0]["message"]["content"]
        .as_str()
        .ok_or("解析翻译结果失败")?
        .to_string();

    Ok(translation.trim().to_string())
}

// ---------- API 检测 ----------
#[tauri::command]
async fn test_api_connection(
    api_key: String,
    model: String,
) -> Result<String, String> {
    if api_key.is_empty() {
        return Err("API Key 不能为空".to_string());
    }

    let client = reqwest::Client::new();
    let resp = client
        .post("https://api.deepseek.com/chat/completions")
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .json(&serde_json::json!({
            "model": model,
            "messages": [
                {"role": "system", "content": "你是一个翻译器。"},
                {"role": "user", "content": "hello"}
            ],
            "max_tokens": 32
        }))
        .send()
        .await
        .map_err(|e| format!("请求失败: {}", e))?;

    let status = resp.status();
    let body: serde_json::Value = resp.json().await.map_err(|e| e.to_string())?;

    if status.is_success() {
        Ok("连接成功".to_string())
    } else {
        let err_msg = body["error"]["message"]
            .as_str()
            .unwrap_or("未知错误");
        Err(format!("API 返回错误 ({}): {}", status.as_u16(), err_msg))
    }
}

// ---------- 调试日志 ----------
#[tauri::command]
fn debug_log(message: String) {
    let path = std::env::current_dir()
        .unwrap_or_default()
        .join("debug.log");
    let ts = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default();
    let line = format!("[{}.{:03}] {}\n", ts.as_secs(), ts.subsec_millis(), message);
    std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&path)
        .map(|mut f| {
            use std::io::Write;
            let _ = f.write_all(line.as_bytes());
        })
        .ok();
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let window = app.get_webview_window("main").expect("主窗口未找到");
            let config: AppConfig = {
                let path = get_config_path(&app.handle());
                if path.exists() {
                    let content = fs::read_to_string(&path).unwrap_or_default();
                    serde_json::from_str(&content).unwrap_or_default()
                } else {
                    AppConfig::default()
                }
            };
            let _ = window.set_always_on_top(config.is_pinned);
            Ok(())
        })
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .invoke_handler(tauri::generate_handler![
            load_config,
            save_config,
            confirm_area_selector,
            cancel_area_selector,
            capture_and_ocr,
            translate,
            test_api_connection,
            debug_log,
        ])
        .run(tauri::generate_context!())
        .expect("启动应用失败");
}