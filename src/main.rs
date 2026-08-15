use qmetaobject::*;
use std::env;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::process::{Command, Stdio};
use std::thread;

struct Translations {
    title: &'static str,
    select_prompt: &'static str,
    btn_select: &'static str,
    btn_install: &'static str,
    selected_pkg: &'static str,
    log_title: &'static str,
    log_loaded: &'static str,
    log_starting: &'static str,
    log_success: &'static str,
    log_error: &'static str,
    dialog_title: &'static str,
    pack_instld: &'static str,
    process_start_err: &'static str,
}

fn get_translations(lang: &str) -> Translations {
    match lang {
        "tr" => Translations {
            title: "Luppo Paket Kurucusu",
            select_prompt: "Lütfen kurulacak bir <b>.luppo</b> dosyası seçin.",
            btn_select: "Paket Seç...",
            btn_install: "Paketi Kur",
            selected_pkg: "Seçilen Paket:",
            log_title: "Kurulum Günlüğü:",
            log_loaded: "Paket yüklendi:",
            log_starting: "\nKurulum yetkisi isteniyor...",
            log_success: "\nPaket kurulumu başarıyla tamamlandı!",
            log_error: "\nKurulum başarısız oldu (Çıkış Kodu: {}).",
            dialog_title: "Luppo Paketi Seç",
            pack_instld: "Yüklenen Paket: {}",
            process_start_err: "\nSüreç başlatılamadı: {}\n",
        },
        _ => Translations {
            title: "Luppo Package Installer",
            select_prompt: "Please select a <b>.luppo</b> package to install.",
            btn_select: "Select Package...",
            btn_install: "Install Package",
            selected_pkg: "Selected Package:",
            log_title: "Installation Log:",
            log_loaded: "Package loaded:",
            log_starting: "\nRequesting administrator privileges...",
            log_success: "\nPackage installation completed successfully!",
            log_error: "\nInstallation failed (Exit code: {}).",
            dialog_title: "Select Luppo Package",
            pack_instld: "Installed Package: {}",
            process_start_err: "\nThe process could not be started: {}\n",
        },
    }
}

fn detect_language() -> String {
    if let Some(locale) = sys_locale::get_locale() {
        if locale.to_lowercase().starts_with("tr") {
            return "tr".to_string();
        }
    }
    "en".to_string()
}

#[derive(QObject, Default)]
struct LuppoPackageInstaller {
    base: qt_base_class!(trait QObject),

    status_text: qt_property!(QString; NOTIFY status_text_changed),
    log_text: qt_property!(QString; NOTIFY log_text_changed),
    is_busy: qt_property!(bool; NOTIFY is_busy_changed),
    can_install: qt_property!(bool; NOTIFY can_install_changed),
    selected_path: qt_property!(QString),

    status_text_changed: qt_signal!(),
    log_text_changed: qt_signal!(),
    is_busy_changed: qt_signal!(),
    can_install_changed: qt_signal!(),

    select_file: qt_method!(fn(&mut self, file_url: QString)),
    start_install: qt_method!(fn(&mut self)),
}

impl LuppoPackageInstaller {
    fn load_file(&mut self, path_str: String) {
        let clean_path = path_str.trim_start_matches("file://").to_string();
        let path = Path::new(&clean_path);

        if path.extension().map_or(false, |ext| ext == "luppo") {
            let lang = detect_language();
            let tr = get_translations(&lang);
            let file_name = path.file_name().unwrap_or_default().to_string_lossy();

            self.selected_path = clean_path.clone().into();
            self.status_text = format!(
                "{} <b>{}</b><br><small>{}</small>",
                tr.selected_pkg, file_name, clean_path
            )
            .into();
            self.status_text_changed();

            self.can_install = true;
            self.can_install_changed();

            let log_entry = format!("{} {}\n", tr.log_loaded, file_name);
            self.append_log(log_entry);
        }
    }

    fn append_log(&mut self, text: String) {
        let mut current = self.log_text.to_string();
        current.push_str(&text);
        self.log_text = current.into();
        self.log_text_changed();
    }

    fn select_file(&mut self, file_url: QString) {
        self.load_file(file_url.to_string());
    }

    fn start_install(&mut self) {
        let target = self.selected_path.to_string();
        if target.is_empty() {
            return;
        }

        let lang = detect_language();
        let tr = get_translations(&lang);

        self.is_busy = true;
        self.is_busy_changed();
        self.can_install = false;
        self.can_install_changed();

        self.append_log(tr.log_starting.to_string() + "\n");

        let qpointer = QPointer::from(&*self);
        let qptr = qpointer.clone();

        let append_log_cb = queued_callback(move |line: String| {
            if let Some(inst) = qptr.as_pinned() {
                inst.borrow_mut().append_log(line);
            }
        });

        let qptr = qpointer.clone();
        let tr_log_success = tr.log_success.to_string();
        let tr_log_error = tr.log_error.to_string();
        let finish_cb = queued_callback(move |exit_code: i32| {
            if let Some(inst) = qptr.as_pinned() {
                let mut guard = inst.borrow_mut();
                guard.is_busy = false;
                guard.is_busy_changed();

                if exit_code == 0 {
                    guard.append_log(tr_log_success.clone() + "\n");
                } else {
                    guard.append_log(
                        tr_log_error.replace("{}", &exit_code.to_string()) + "\n",
                    );
                    guard.can_install = true;
                    guard.can_install_changed();
                }
            }
        });

        let qptr = qpointer.clone();
        let error_cb = queued_callback(move |err_msg: String| {
            if let Some(inst) = qptr.as_pinned() {
                let mut guard = inst.borrow_mut();
                guard.is_busy = false;
                guard.is_busy_changed();
                guard.can_install = true;
                guard.can_install_changed();
                guard.append_log(err_msg);
            }
        });

        thread::spawn(move || {
            let child = Command::new("pkexec")
                .args(&["luppo", "it", "--yes-all", &target])
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn();

            match child {
                Ok(mut proc) => {
                    if let Some(stdout) = proc.stdout.take() {
                        let reader = BufReader::new(stdout);
                        for line in reader.lines() {
                            if let Ok(l) = line {
                                append_log_cb(format!("{}\n", l));
                            }
                        }
                    }

                    let status = proc.wait();
                    let exit_code = status.map(|s| s.code().unwrap_or(-1)).unwrap_or(-1);
                    finish_cb(exit_code);
                }
                Err(e) => {
                    let err_msg = format!("{} {}", tr.process_start_err, e);
                    error_cb(err_msg);
                }
            }
        });
    }
}

fn main() {
    qml_register_type::<LuppoPackageInstaller>(
        c"LuppoPackageInstaller",
        1,
        0,
        c"LuppoPackageInstaller",
    );

    let args: Vec<String> = env::args().collect();
    let initial_file = if args.len() > 1 {
        args[1].clone()
    } else {
        String::new()
    };

    let mut engine = QmlEngine::new();

    let lang = detect_language();
    let tr = get_translations(&lang);

    engine.set_property("tr_title".into(), QString::from(tr.title).into());
    engine.set_property("tr_dialog_title".into(), QString::from(tr.dialog_title).into());
    engine.set_property("tr_select_prompt".into(), QString::from(tr.select_prompt).into());
    engine.set_property("tr_btn_select".into(), QString::from(tr.btn_select).into());
    engine.set_property("tr_btn_install".into(), QString::from(tr.btn_install).into());
    engine.set_property("tr_log_title".into(), QString::from(tr.log_title).into());
    
    // &initial_file yerine as_str() kullanılarak tip hatası çözüldü
    engine.set_property("cli_file_path".into(), QString::from(initial_file.as_str()).into());

    let qml_code = r##"
    import QtQuick
    import QtQuick.Controls
    import QtQuick.Layouts
    import QtQuick.Dialogs
    import LuppoPackageInstaller 1.0

    ApplicationWindow {
        id: window
        title: tr_title
        visible: true
        width: 600
        height: 450

        LuppoPackageInstaller {
            id: installer
        }

        Component.onCompleted: {
            if (cli_file_path !== "") {
                installer.select_file(cli_file_path)
            }
        }

        FileDialog {
            id: fileDialog
            title: tr_dialog_title
            nameFilters: ["Luppo Packages (*.luppo)"]
            onAccepted: {
                installer.select_file(fileDialog.selectedFile)
            }
        }

        ColumnLayout {
            anchors.fill: parent
            anchors.margins: 20
            spacing: 15

            Text {
                Layout.fillWidth: true
                text: installer.status_text !== "" ? installer.status_text : tr_select_prompt
                color: "#ffffff"
                textFormat: Text.RichText
                wrapMode: Text.Wrap
                font.pixelSize: 13
            }

            RowLayout {
                spacing: 10
                Button {
                    text: tr_btn_select
                    enabled: !installer.is_busy
                    onClicked: fileDialog.open()
                }

                Button {
                    text: tr_btn_install
                    enabled: installer.can_install && !installer.is_busy
                    onClicked: installer.start_install()
                }
            }

            ProgressBar {
                Layout.fillWidth: true
                indeterminate: true
                visible: installer.is_busy
            }

            Text {
                text: tr_log_title
                color: "#ffffff"
                font.pixelSize: 13
            }

            ScrollView {
                Layout.fillWidth: true
                Layout.fillHeight: true

                TextArea {
                    readOnly: true
                    text: installer.log_text
                    color: "#00ff00"
                    font.family: "monospace"
                    font.pixelSize: 12
                    background: Rectangle {
                        color: "#1e1e1e"
                        border.color: "#444444"
                        radius: 6
                    }
                }
            }
        }
    }
    "##;

    engine.load_data(qml_code.into());

    if !initial_file.is_empty() {
        println!("{} {}", tr.pack_instld, initial_file);
    }

    engine.exec();
}