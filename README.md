# AZAD Quick Support

AZAD Quick Support is a remote support tool for Windows, used by AZAD Co., Ltd. to help its customers.
It is a modified version of [RustDesk](https://github.com/rustdesk/rustdesk) 1.4.9 and is licensed under the GNU AGPL-3.0.

AZAD Quick Support は、AZAD Co., Ltd. がお客様のPCのトラブル解決に使う、Windows 用の遠隔サポートソフトです。
オープンソースの遠隔操作ソフト [RustDesk](https://github.com/rustdesk/rustdesk) 1.4.9 を改変したもので、GNU AGPL-3.0 のもとで公開しています。

## How it works / 使い方

1. The customer downloads `azad_quick_support.exe` and runs it. No installation is needed.
2. The window shows a 6-digit ID. The customer tells the ID to the AZAD support staff by phone.
3. When the support staff connects, the customer clicks "承諾する" (Accept). The connection never starts without the customer's click.
4. The customer can end the connection at any time with "接続を終了する" or by closing the window.

1. お客様は `azad_quick_support.exe` をダウンロードして起動します（インストールは不要です）。
2. 画面に表示された6桁のIDを、電話などでサポート担当者に伝えます。
3. サポート担当者から接続の要求が来たら、「承諾する」を押します。お客様が押さない限り、接続は始まりません。
4. 「接続を終了する」を押すか、ウィンドウを閉じれば、いつでも接続を終えられます。

## Download / ダウンロード

Download the latest `azad_quick_support.exe` from [Releases](https://github.com/azad-co-jp/azad-quick-support/releases).
Please use it only when AZAD support staff asks you to.

最新の `azad_quick_support.exe` は [Releases](https://github.com/azad-co-jp/azad-quick-support/releases) からダウンロードできます。
AZAD のサポート担当者から案内があったときにお使いください。

## Changes from RustDesk / RustDesk からの主な変更

- Connects only to the AZAD server (`remote.azad.co.jp`) over WebSocket (TCP 443). No other ports are needed on the customer's network.
- Simple Japanese UI that shows the 6-digit ID; the customer accepts each support session in this window.
- Uses its own settings folder and name (`AZADQuickSupport`), so it does not interfere with an installed RustDesk.
- Writes no log files on the customer's PC.
- Supports UAC prompts through the elevated portable service, and reconnects automatically after a remote restart.

The server side (modified hbbs/hbbr) is published at [azad-rustdesk-server](https://github.com/azad-co-jp/azad-rustdesk-server).
All AZAD-specific values are collected in [`src/azad.rs`](src/azad.rs).

## Code signing policy / コード署名の方針

Free code signing provided by [SignPath.io](https://about.signpath.io), certificate by [SignPath Foundation](https://signpath.org).

- **What is signed:** `azad_quick_support.exe` and the executables and libraries packed inside it, built by the GitHub Actions workflow [`azad-build.yml`](.github/workflows/azad-build.yml) from the source code in this repository.
- **Committers and reviewers:** [Azad-hub-code](https://github.com/Azad-hub-code) (AZAD Co., Ltd.)
- **Approvers:** [Azad-hub-code](https://github.com/Azad-hub-code) (AZAD Co., Ltd.)
- Every signing request is approved manually by an approver. Only binaries built on GitHub-hosted runners from this repository are signed.

[SignPath.io](https://about.signpath.io) による無料のコード署名を利用し、証明書は [SignPath Foundation](https://signpath.org) が発行しています。
署名するのは、このリポジトリのソースから GitHub Actions でビルドした `azad_quick_support.exe` と、その中に含まれる実行ファイル・ライブラリだけです。署名のたびに承認者が承認します。

## Privacy policy / プライバシーポリシー

This program connects to the AZAD server (`remote.azad.co.jp`, operated by AZAD Co., Ltd.) to provide remote support.
The program will not transfer any information to other networked systems.

- **Sent to the AZAD server:** the 6-digit ID of the PC, the public IP address, and the time, duration and amount of data of each support session. These records are kept on the AZAD server to investigate problems and to confirm support work.
- **Not visible to the server:** screen images, keyboard and mouse input, clipboard and files are end-to-end encrypted between the customer's PC and the support staff's PC. The server only relays them.
- **On the customer's PC:** no log files are written. A small settings folder keeps the ID so that the ID stays the same.
- **How to stop:** close the program. It sends nothing while it is not running.

このソフトは、遠隔サポートのために AZAD のサーバー（`remote.azad.co.jp`、AZAD Co., Ltd. が運用）に接続します。それ以外の場所には情報を送りません。

- **サーバーに送る情報：** このPCの6桁のID、グローバルIPアドレス、サポートの日時・時間・データ量。トラブルの調査とサポート作業の確認のため、AZAD のサーバーに記録します。
- **サーバーでは見られない情報：** 画面、キーボード・マウスの操作、クリップボード、ファイルは、お客様のPCとサポート担当者のPCの間で暗号化され、サーバーは中継するだけです。
- **お客様のPCに残るもの：** ログは残しません。IDを毎回同じにするため、小さな設定フォルダーだけを残します。
- **止め方：** ソフトを終了してください。起動していない間は何も送りません。

## Changes to your system / PCへの変更

- On start, the program asks for administrator rights (UAC) so that the support staff can operate administrator windows. If you decline, it still works with limited functions.
- The program is extracted to `%LOCALAPPDATA%\AZADQuickSupport` and keeps its settings in `%APPDATA%\AZADQuickSupport`.
- When the support staff restarts your PC remotely, the program registers itself once in `HKCU\Software\Microsoft\Windows\CurrentVersion\RunOnce` so that it starts again after you sign in. Windows removes the entry after it runs.
- **To remove the program**, delete the downloaded `azad_quick_support.exe` and the two folders above.

- 起動すると、管理者の画面も操作できるよう、管理者権限（UAC）の確認が出ます。許可しなくても、機能を制限して動きます。
- ソフトは `%LOCALAPPDATA%\AZADQuickSupport` に展開され、設定は `%APPDATA%\AZADQuickSupport` に保存されます。
- サポート担当者がPCを遠隔で再起動するときは、サインイン後に1回だけ起動するよう `RunOnce` に登録します（実行後は Windows が自動で消します）。
- **削除するには**、ダウンロードした `azad_quick_support.exe` と、上の2つのフォルダーを削除してください。

## Build / ビルド

Windows x64 only. Releases are built by [`.github/workflows/azad-build.yml`](.github/workflows/azad-build.yml) on GitHub-hosted runners.
To build locally, install Visual Studio 2022 Build Tools (C++), Rust 1.97.1, LLVM 20.1.8, Flutter 3.27.4, Python 3 and vcpkg (`x64-windows-static`, see `vcpkg.json`), then run:

```powershell
powershell -ExecutionPolicy Bypass -File azad\build.ps1
```

The bridge files between Rust and Flutter are generated by `flutter_rust_bridge_codegen` 1.80.1 (see [`.github/workflows/bridge.yml`](.github/workflows/bridge.yml)).

## License / ライセンス

GNU Affero General Public License v3.0. See [LICENCE](LICENCE).
This program is a modified version of RustDesk. Copyright © RustDesk developers. Modifications © AZAD Co., Ltd.
Anyone may copy, modify and redistribute it under the terms of the AGPL-3.0.
It is provided without warranty under the AGPL-3.0, except as separately agreed in a maintenance contract with AZAD Co., Ltd.

GNU AGPL-3.0 のもとで公開しており、このライセンスの条件に従って、誰でも複製・改変・再配布できます。
AGPL-3.0 に基づき無保証で提供されます（AZAD Co., Ltd. との保守契約で別に定める場合を除きます）。
The original RustDesk README is kept at [docs/README-RustDesk.md](docs/README-RustDesk.md).
"RustDesk" is a trademark of its owner; AZAD Quick Support is not affiliated with or endorsed by the RustDesk project.
