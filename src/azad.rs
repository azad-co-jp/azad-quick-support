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

pub fn log_enabled() -> bool {
    std::env::var(LOG_ENV).map(|v| v == "1").unwrap_or(false)
}
