# AZAD QUICK SUPPORT のビルド（Windows x64）
#
# 必要なもの（2026-10-21 時点で動作確認した版）
#   Visual Studio 2022 Build Tools（C++ によるデスクトップ開発）
#   Rust 1.97.1（rustup default 1.97.1）
#   LLVM 20.1.8（C:\Program Files\LLVM）
#   Flutter 3.27.4（C:\work\flutter）
#   Python 3.12（brotli を使う）
#   vcpkg（C:\work\vcpkg、x64-windows-static に依存ライブラリをインストール済み）
#
# 使い方:
#   powershell -ExecutionPolicy Bypass -File azad\build.ps1            # Rust → Flutter → ポータブル版
#   powershell -ExecutionPolicy Bypass -File azad\build.ps1 -SkipRust  # Flutter から
#
# 成果物: dist\<日時>\azad_quick_support.exe（と SHA-256）
param(
  [switch]$SkipRust,
  [switch]$SkipFlutter
)
$ErrorActionPreference = 'Stop'
$root = Split-Path -Parent $PSScriptRoot
Set-Location $root

$env:VCPKG_ROOT = 'C:\work\vcpkg'
$env:LIBCLANG_PATH = 'C:\Program Files\LLVM\bin'
$py = Join-Path $env:LOCALAPPDATA 'Programs\Python\Python312'
$env:Path = "$env:USERPROFILE\.cargo\bin;C:\work\flutter\bin;$py;$py\Scripts;C:\Program Files\LLVM\bin;$env:Path"

function Step($name, [scriptblock]$block) {
  Write-Host "==== $name ($(Get-Date -Format 'HH:mm:ss'))"
  & $block
  if ($LASTEXITCODE -ne 0) { throw "$name failed (exit $LASTEXITCODE)" }
}

if (-not $SkipRust) {
  Step 'cargo build (librustdesk.dll)' { cargo build --locked --features flutter --lib --release }
}

$release = Join-Path $root 'flutter\build\windows\x64\runner\Release'
if (-not $SkipFlutter) {
  Push-Location flutter
  try {
    # 古いファイル（以前の exe や .bak）がポータブル版に入らないよう、Release を空にしてから作る
    if (Test-Path $release) { Remove-Item $release -Recurse -Force }
    # .dart_tool はPCごとのパスを含むので、毎回このPC用に作り直す（pubspec.lock の版は変えない）
    Step 'flutter pub get' { flutter pub get --enforce-lockfile }
    Step 'flutter build windows' { flutter build windows --release }
  } finally { Pop-Location }
  Copy-Item target\release\deps\dylib_virtual_display.dll $release -Force
}

# ポータブル版：Release フォルダを data.bin に詰め、libs\portable の起動用 exe に埋め込む。
# 起動用 exe の名前が azad_quick_support.exe のとき、--quick_support として起動する（libs/portable/src/main.rs）。
# 中の本体も azad_quick_support.exe にする。管理者権限で動く子プロセスもこの名前で AZAD 用と判定する（src/azad.rs）。
$stage = Join-Path $root 'target\azad_portable_input'
if (Test-Path $stage) { Remove-Item $stage -Recurse -Force }
Copy-Item $release $stage -Recurse
Rename-Item (Join-Path $stage 'rustdesk.exe') 'azad_quick_support.exe'
Push-Location libs\portable
try {
  Step 'pip install' { python -m pip install -q -r requirements.txt }
  Step 'generate data.bin' { python .\generate.py -f "$stage" -o . -e "$stage\azad_quick_support.exe" }
  Step 'cargo build (portable packer)' { cargo build --locked --release }
} finally { Pop-Location }

$stamp = Get-Date -Format 'yyyyMMdd_HHmm'
$out = Join-Path $root "dist\$stamp"
New-Item -ItemType Directory -Force $out | Out-Null
$exe = Join-Path $out 'azad_quick_support.exe'
Copy-Item target\release\rustdesk-portable-packer.exe $exe
$dll = Join-Path $release 'librustdesk.dll'
$lines = foreach ($f in @($exe, $dll)) {
  $h = Get-FileHash $f -Algorithm SHA256
  "{0}  {1}  {2}" -f $h.Hash, (Get-Item $f).Length, $f
}
$lines | Set-Content (Join-Path $out 'SHA256.txt') -Encoding utf8
$lines
Write-Host "output: $exe"
