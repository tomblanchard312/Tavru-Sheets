use crate::SheetApp;
use enki_speech_ui::{Action, VoiceCommand};
use serde_json::json;

fn launch_daemon(open: bool) {
    if !open {
        return;
    }
    use std::sync::atomic::{AtomicBool, Ordering};
    static STARTED: AtomicBool = AtomicBool::new(false);
    if STARTED.swap(true, Ordering::Relaxed) {
        return;
    }
    let _ = std::thread::Builder::new().name("sharedai-launch".into()).spawn(|| {
        use nanshe_enki as enki;
        let Ok(endpoint) = enki::endpoint("") else { return };
        let _ = enki::ensure_running(&endpoint, None, &[]);
    });
}
pub(crate) fn show(app: &mut SheetApp, ctx: &egui::Context) {
    launch_daemon(app.speech.open);
    let scope =
        app.session.active().map(|d| format!("{}:{}:{:?}:{:?}", d.uid, d.revision, d.wb.active_sheet, d.selection.active)).unwrap_or_default();
    let Some(action) = app.speech.show(ctx, &scope, "Insert as cell text") else { return };
    match action {
        Action::Insert(text) => app.run_or_alert("cell.set", json!({"input":format!("'{text}")})),
        Action::Command(command) => {
            let id = match command {
                VoiceCommand::OpenAssistant => "view.scribe",
                VoiceCommand::Next => "sheet.next",
                VoiceCommand::Previous => "sheet.previous",
                VoiceCommand::ZoomIn | VoiceCommand::ZoomOut => {
                    let zoom = app.session.active().and_then(|d| d.wb.active()).map(|s| s.zoom).unwrap_or(100);
                    let percent = if command == VoiceCommand::ZoomIn { zoom.saturating_add(10) } else { zoom.saturating_sub(10) };
                    app.run_or_alert("view.zoom", json!({"percent":percent.clamp(10,400)}));
                    return;
                }
                VoiceCommand::Undo => "edit.undo",
                VoiceCommand::Redo => "edit.redo",
                VoiceCommand::Stop => {
                    app.speech.open = false;
                    return;
                }
            };
            app.run_or_alert(id, json!({}));
        }
    }
}
