//! OpenRouter AI integration for Nephclick Picshot (AI Assistant prompt edits and AI Lens Blur).

#![forbid(unsafe_code)]
#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::unimplemented, clippy::todo, clippy::unreachable)]

use std::sync::mpsc::{Receiver, channel};

/// Result from an AI request.
#[derive(Debug)]
pub enum AiResult {
    AssistantEdit {
        adjustments: serde_json::Value,
        explanation: String,
    },
    LensBlur {
        amount: f32,
        boost: f32,
        transition: f32,
        shape: String,
        explanation: String,
    },
    Error(String),
}

/// Call OpenRouter Chat Completions API with system and user messages.
#[cfg(not(target_arch = "wasm32"))]
pub fn call_openrouter_sync(
    api_key: &str,
    model: &str,
    system_prompt: &str,
    user_prompt: &str,
) -> Result<String, String> {
    let key = api_key.trim();
    if key.is_empty() {
        return Err("OpenRouter API key is not configured. Please open Edit ▸ AI Settings to add your API key.".into());
    }
    let model_name = if model.trim().is_empty() { "google/gemini-2.5-flash" } else { model.trim() };

    let body = serde_json::json!({
        "model": model_name,
        "messages": [
            { "role": "system", "content": system_prompt },
            { "role": "user", "content": user_prompt }
        ],
        "response_format": { "type": "json_object" }
    });

    let body_bytes = serde_json::to_vec(&body).map_err(|e| format!("Serialization error: {e}"))?;

    let agent: ureq::Agent = ureq::Agent::config_builder()
        .timeout_connect(Some(std::time::Duration::from_secs(15)))
        .timeout_recv_response(Some(std::time::Duration::from_secs(60)))
        .build()
        .into();

    let resp = agent
        .post("https://openrouter.ai/api/v1/chat/completions")
        .header("Authorization", &format!("Bearer {key}"))
        .header("Content-Type", "application/json")
        .header("HTTP-Referer", "https://nephclick.com")
        .header("X-Title", "Nephclick Picshot")
        .send(&body_bytes)
        .map_err(|e| format!("OpenRouter HTTP request failed: {e}"))?;

    let resp_bytes = resp.into_body().read_to_vec().map_err(|e| format!("Failed reading response: {e}"))?;
    let json_resp: serde_json::Value = serde_json::from_slice(&resp_bytes).map_err(|e| format!("Invalid JSON response: {e}"))?;

    if let Some(err) = json_resp.get("error") {
        let msg = err.get("message").and_then(|m| m.as_str()).unwrap_or("Unknown OpenRouter error");
        return Err(format!("OpenRouter error: {msg}"));
    }

    let content = json_resp
        .get("choices")
        .and_then(|c| c.get(0))
        .and_then(|m| m.get("message"))
        .and_then(|c| c.get("content"))
        .and_then(|s| s.as_str())
        .ok_or_else(|| "Empty message content in OpenRouter response".to_string())?;

    Ok(content.to_string())
}

#[cfg(target_arch = "wasm32")]
pub fn call_openrouter_sync(
    _api_key: &str,
    _model: &str,
    _system_prompt: &str,
    _user_prompt: &str,
) -> Result<String, String> {
    Err("OpenRouter API calls are not supported on WebAssembly".into())
}

/// Request AI Assistant to generate photo develop slider edits based on a user prompt.
pub fn request_assistant_edit(
    api_key: String,
    model: String,
    user_prompt: String,
    current_settings: serde_json::Value,
) -> Receiver<AiResult> {
    let (tx, rx) = channel();
    std::thread::spawn(move || {
        let system_prompt = "You are an expert photo editor and colorist for Nephclick Picshot.\n\
        Analyze the user's prompt and current develop settings, then return ONLY a valid JSON object with the develop adjustments to apply.\n\
        Format:\n\
        {\n\
          \"exposure\": <f32 between -5.0 and 5.0>,\n\
          \"contrast\": <f32 between -100.0 and 100.0>,\n\
          \"highlights\": <f32 between -100.0 and 100.0>,\n\
          \"shadows\": <f32 between -100.0 and 100.0>,\n\
          \"whites\": <f32 between -100.0 and 100.0>,\n\
          \"blacks\": <f32 between -100.0 and 100.0>,\n\
          \"temperature\": <f32 between -100.0 and 100.0>,\n\
          \"tint\": <f32 between -100.0 and 100.0>,\n\
          \"vibrance\": <f32 between -100.0 and 100.0>,\n\
          \"saturation\": <f32 between -100.0 and 100.0>,\n\
          \"clarity\": <f32 between -100.0 and 100.0>,\n\
          \"dehaze\": <f32 between -100.0 and 100.0>,\n\
          \"vignette\": <f32 between -100.0 and 100.0>,\n\
          \"explanation\": \"<concise 1-2 sentence description of the creative adjustments>\"\n\
        }\n\
        Only include the numeric keys that should be modified.";

        let user_msg = format!(
            "User Edit Request: \"{}\"\nCurrent Settings:\n{}",
            user_prompt,
            serde_json::to_string_pretty(&current_settings).unwrap_or_default()
        );

        match call_openrouter_sync(&api_key, &model, system_prompt, &user_msg) {
            Ok(content) => {
                match serde_json::from_str::<serde_json::Value>(&content) {
                    Ok(mut parsed) => {
                        let explanation = parsed
                            .get("explanation")
                            .and_then(|v| v.as_str())
                            .unwrap_or("Adjustments applied.")
                            .to_string();
                        if let Some(obj) = parsed.as_object_mut() {
                            obj.remove("explanation");
                        }
                        let _ = tx.send(AiResult::AssistantEdit {
                            adjustments: parsed,
                            explanation,
                        });
                    }
                    Err(e) => {
                        let _ = tx.send(AiResult::Error(format!("Failed to parse AI response: {e}\nResponse: {content}")));
                    }
                }
            }
            Err(e) => {
                let _ = tx.send(AiResult::Error(e));
            }
        }
    });
    rx
}

/// Request AI to analyze the photo and calculate the optimal lens blur parameters.
pub fn request_ai_lens_blur(
    api_key: String,
    model: String,
    meta_info: String,
) -> Receiver<AiResult> {
    let (tx, rx) = channel();
    std::thread::spawn(move || {
        let system_prompt = "You are an expert optical and camera lens analyst for Nephclick Picshot.\n\
        Analyze the photo parameters and determine the ideal, aesthetically pleasing lens blur (bokeh) settings.\n\
        Return ONLY a JSON object with this exact structure:\n\
        {\n\
          \"amount\": <f32 from 20.0 to 90.0, default ~50.0>,\n\
          \"boost\": <f32 bokeh highlight boost from 0.0 to 50.0, default ~15.0>,\n\
          \"transition\": <f32 focal transition from 10.0 to 50.0, default ~25.0>,\n\
          \"shape\": <\"auto\" or \"circle\" or \"oval\">,\n\
          \"explanation\": \"<short reason for these optical settings>\"\n\
        }";

        match call_openrouter_sync(&api_key, &model, system_prompt, &meta_info) {
            Ok(content) => {
                match serde_json::from_str::<serde_json::Value>(&content) {
                    Ok(v) => {
                        let amount = v.get("amount").and_then(|x| x.as_f64()).unwrap_or(50.0) as f32;
                        let boost = v.get("boost").and_then(|x| x.as_f64()).unwrap_or(15.0) as f32;
                        let transition = v.get("transition").and_then(|x| x.as_f64()).unwrap_or(25.0) as f32;
                        let shape = v.get("shape").and_then(|x| x.as_str()).unwrap_or("auto").to_string();
                        let explanation = v.get("explanation").and_then(|x| x.as_str()).unwrap_or("Optimal lens blur applied.").to_string();

                        let _ = tx.send(AiResult::LensBlur {
                            amount,
                            boost,
                            transition,
                            shape,
                            explanation,
                        });
                    }
                    Err(e) => {
                        let _ = tx.send(AiResult::Error(format!("Failed to parse Lens Blur AI response: {e}\nResponse: {content}")));
                    }
                }
            }
            Err(e) => {
                let _ = tx.send(AiResult::Error(e));
            }
        }
    });
    rx
}

/// Convert flat AI adjustment values into structured DevelopSettings partial JSON.
pub fn format_adjustments_to_develop_settings(val: &serde_json::Value) -> serde_json::Value {
    let mut light = serde_json::Map::new();
    let mut wb = serde_json::Map::new();
    let mut color = serde_json::Map::new();
    let mut effects = serde_json::Map::new();
    let mut vignette = serde_json::Map::new();

    if let Some(obj) = val.as_object() {
        for (k, v) in obj {
            match k.as_str() {
                "exposure" => { light.insert("exposure".into(), v.clone()); }
                "contrast" => { light.insert("contrast".into(), v.clone()); }
                "highlights" => { light.insert("highlights".into(), v.clone()); }
                "shadows" => { light.insert("shadows".into(), v.clone()); }
                "whites" => { light.insert("whites".into(), v.clone()); }
                "blacks" => { light.insert("blacks".into(), v.clone()); }
                "temperature" | "temp" => { wb.insert("temp".into(), v.clone()); }
                "tint" => { wb.insert("tint".into(), v.clone()); }
                "vibrance" => { color.insert("vibrance".into(), v.clone()); }
                "saturation" => { color.insert("saturation".into(), v.clone()); }
                "clarity" => { effects.insert("clarity".into(), v.clone()); }
                "dehaze" => { effects.insert("dehaze".into(), v.clone()); }
                "vignette" => { vignette.insert("amount".into(), v.clone()); }
                _ => {}
            }
        }
    }

    let mut out = serde_json::Map::new();
    if !light.is_empty() { out.insert("light".into(), serde_json::Value::Object(light)); }
    if !wb.is_empty() { out.insert("wb".into(), serde_json::Value::Object(wb)); }
    if !color.is_empty() { out.insert("color".into(), serde_json::Value::Object(color)); }
    if !effects.is_empty() { out.insert("effects".into(), serde_json::Value::Object(effects)); }
    if !vignette.is_empty() { out.insert("vignette".into(), serde_json::Value::Object(vignette)); }

    serde_json::Value::Object(out)
}

/// Poll any pending AI results from the background worker.
pub fn poll(app: &mut crate::PicshotApp, ctx: &egui::Context) {
    let Some(rx_arc) = app.ui.ai_receiver.clone() else { return };
    let Ok(rx) = rx_arc.lock() else { return };
    match rx.try_recv() {
        Ok(res) => {
            drop(rx);
            app.ui.ai_in_flight = false;
            match res {
                AiResult::AssistantEdit { adjustments, explanation } => {
                    let partial = format_adjustments_to_develop_settings(&adjustments);
                    let _ = app.run("develop.merge", serde_json::json!({
                        "settings": partial,
                        "label": "AI Assistant Edit"
                    }));
                    app.toast(ctx, format!("AI Edit Applied: {explanation}"));
                    if let Some(crate::state::Dialog::AiAssistant { in_flight, response_text, error, .. }) = &mut app.ui.dialog {
                        *in_flight = false;
                        *response_text = explanation;
                        *error = None;
                    }
                }
                AiResult::LensBlur { amount, boost, transition, shape, explanation } => {
                    let patch = serde_json::json!({
                        "enabled": true,
                        "amount": amount,
                        "boost": boost,
                        "transition": transition,
                        "shape": shape,
                    });
                    let _ = app.run("develop.merge", serde_json::json!({
                        "settings": { "lens_blur": patch },
                        "label": "AI Lens Blur"
                    }));
                    app.ui.ai_lens_blur_enabled = true;
                    app.toast(ctx, format!("AI Lens Blur Applied: {explanation}"));
                }
                AiResult::Error(err) => {
                    app.toast_error(ctx, format!("AI Error: {err}"));
                    if let Some(crate::state::Dialog::AiAssistant { in_flight, error, .. }) = &mut app.ui.dialog {
                        *in_flight = false;
                        *error = Some(err);
                    }
                }
            }
            app.ui.ai_receiver = None;
            ctx.request_repaint();
        }
        Err(std::sync::mpsc::TryRecvError::Empty) => {
            ctx.request_repaint_after(std::time::Duration::from_millis(100));
        }
        Err(std::sync::mpsc::TryRecvError::Disconnected) => {
            app.ui.ai_in_flight = false;
            app.ui.ai_receiver = None;
            ctx.request_repaint();
        }
    }
}

/// Save app UI state directly to disk (`<config>/ui.json`).
pub fn save_ui_to_disk(ui: &crate::state::UiState) -> Result<(), String> {
    let Some(path) = picshot_engine::config::config_dir().map(|d| d.join("ui.json")) else {
        return Ok(());
    };
    if let Some(parent) = path.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    let bytes = serde_json::to_vec_pretty(ui).map_err(|e| e.to_string())?;
    std::fs::write(&path, &bytes).map_err(|e| format!("Failed to save settings to disk: {e}"))?;
    Ok(())
}
