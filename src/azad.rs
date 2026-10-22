//! AZAD QUICK SUPPORT 独自の設定。
//!
//! 接続先やアプリ名など、RustDesk 標準から変えている値はここにまとめる。
//! VPS を移すときは DNS（remote.azad.co.jp）を移せばよく、ここは変えなくてよい。
use hbb_common::config::{self, keys, Config};

/// 接続先（hbbs/hbbr の前段の Caddy）。WSS（443）だけで通信する。
pub const SERVER: &str = "remote.azad.co.jp";
/// 設定フォルダー・IPC の名前。お客様のPCに入っている RustDesk と混ざらないよう、別の名前にする。
pub const APP_NAME: &str = "AZADQuickSupport";
/// 配布する exe の名前。この名前で起動したときに Quick Support として動く。
pub const EXE_NAME: &str = "azad_quick_support.exe";
/// 1 にするとログを書く（調査用）。既定ではログを残さない。
const LOG_ENV: &str = "AZQS_LOG";

/// 自分が AZAD QUICK SUPPORT の exe として起動されたか。
/// ポータブル版の起動用 exe は、展開した本体を同じ名前で起動し、RUSTDESK_APPNAME にもその名前を入れる。
pub fn is_azad_exe() -> bool {
    let is_name = |s: &str| s.to_lowercase().ends_with(EXE_NAME);
    if let Ok(exe) = std::env::current_exe() {
        if is_name(&exe.to_string_lossy()) {
            return true;
        }
    }
    std::env::var(crate::common::PORTABLE_APPNAME_RUNTIME_ENV_KEY)
        .map(|v| is_name(&v))
        .unwrap_or(false)
}

/// 設定を読む前に呼ぶ。設定フォルダーと IPC の名前を AZAD 用にする。
pub fn init() {
    *config::APP_NAME.write().unwrap() = APP_NAME.to_owned();
}

/// Quick Support として動くときの設定。ファイルには保存せず、起動中だけ有効にする。
pub fn apply_quick_support_options() {
    let set = |k: &str, v: &str| Config::set_option_no_store(k.to_owned(), v.to_owned());
    set(keys::OPTION_ALLOW_WEBSOCKET, "Y");
    set("custom-rendezvous-server", SERVER);
    set("api-server", &format!("https://{SERVER}"));
    // 中継先はサーバー（hbbs の -r）が指定したものを使う
    set("relay-server", "");
    // 接続はお客様のクリックで許可する（パスワードでは接続させない）
    set("approve-mode", "click");
}

/// 接続の承諾・切断を、別ウィンドウ（--cm）ではなく ID の画面で行うか。
/// Quick Support のメインプロセスが接続管理の IPC（_cm）を受け持つので、別ウィンドウは起動されない。
pub fn cm_in_main_window() -> bool {
    #[cfg(windows)]
    {
        crate::portable_service::client::is_quick_support()
    }
    #[cfg(not(windows))]
    {
        false
    }
}

/// Quick Support のメインプロセスで、接続管理の IPC（_cm）を受け持つ。
/// サーバーは接続時にまず _cm へつなぎ、つながらないときだけ --cm のウィンドウを起動する（server/connection.rs）。
pub fn start_cm_in_main_window() {
    #[cfg(feature = "flutter")]
    if cm_in_main_window() {
        crate::flutter::connection_manager::cm_init();
    }
}

/// リモート再起動の後、再起動を指示した担当者からの再接続を自動で承諾する期間。
const RESTART_TRUST_SECS: u64 = 15 * 60;
const RESTART_TRUST_FILE: &str = "azad_restart_trust";

fn now_secs() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

/// リモート再起動の直前に呼ぶ（server/connection.rs の RestartRemoteDevice）。
/// 次のサインインで Quick Support を1回だけ自動で起動し（RunOnce は実行後に自動で消える）、
/// 再起動を指示した担当者（controller_id）からの再接続を、RESTART_TRUST_SECS の間だけ自動で承諾できるようにする。
pub fn prepare_remote_restart(controller_id: &str) {
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        if let Ok(exe) = std::env::current_exe() {
            let cmd = format!("\"{}\" --quick_support", exe.display());
            let res = std::process::Command::new("reg")
                .args([
                    "add",
                    r"HKCU\Software\Microsoft\Windows\CurrentVersion\RunOnce",
                    "/v",
                    APP_NAME,
                    "/t",
                    "REG_SZ",
                    "/d",
                    &cmd,
                    "/f",
                ])
                .creation_flags(winapi::um::winbase::CREATE_NO_WINDOW)
                .status();
            hbb_common::log::info!("register RunOnce for remote restart: {:?}", res);
        }
    }
    let path = Config::path(RESTART_TRUST_FILE);
    if let Err(err) = std::fs::write(&path, format!("{}\n{}", controller_id, now_secs())) {
        hbb_common::log::error!("Failed to write {}: {}", path.display(), err);
    }
}

/// リモート再起動を指示した担当者からの再接続なら true（1回だけ）。期限切れの記録は消す。
pub fn take_restart_trust(peer_id: &str) -> bool {
    let path = Config::path(RESTART_TRUST_FILE);
    let Ok(content) = std::fs::read_to_string(&path) else {
        return false;
    };
    let mut lines = content.lines();
    let id = lines.next().unwrap_or_default();
    let ts = lines.next().and_then(|v| v.parse::<u64>().ok()).unwrap_or(0);
    let fresh = now_secs().saturating_sub(ts) <= RESTART_TRUST_SECS;
    let matched = fresh && !id.is_empty() && id == peer_id;
    if matched || !fresh {
        std::fs::remove_file(&path).ok();
    }
    matched
}

pub fn log_enabled() -> bool {
    std::env::var(LOG_ENV).map(|v| v == "1").unwrap_or(false)
}
