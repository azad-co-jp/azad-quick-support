# AZAD QUICK SUPPORT の仮アイコン（青地に白の「A」）を作る。正式なロゴができたら差し替える。
# 出力: res\icon.ico（ポータブル版の起動用 exe）と flutter\windows\runner\resources\app_icon.ico（本体）
Add-Type -AssemblyName System.Drawing
$root = Split-Path -Parent $PSScriptRoot
$sizes = 16, 24, 32, 48, 64, 128, 256
$pngs = foreach ($s in $sizes) {
  $bmp = New-Object System.Drawing.Bitmap $s, $s
  $g = [System.Drawing.Graphics]::FromImage($bmp)
  $g.SmoothingMode = 'AntiAlias'
  $g.TextRenderingHint = 'AntiAliasGridFit'
  $g.Clear([System.Drawing.Color]::Transparent)
  # 角の丸い四角
  $r = [Math]::Max(2, [int]($s * 0.18))
  $path = New-Object System.Drawing.Drawing2D.GraphicsPath
  $path.AddArc(0, 0, 2 * $r, 2 * $r, 180, 90)
  $path.AddArc($s - 2 * $r - 1, 0, 2 * $r, 2 * $r, 270, 90)
  $path.AddArc($s - 2 * $r - 1, $s - 2 * $r - 1, 2 * $r, 2 * $r, 0, 90)
  $path.AddArc(0, $s - 2 * $r - 1, 2 * $r, 2 * $r, 90, 90)
  $path.CloseFigure()
  $g.FillPath((New-Object System.Drawing.SolidBrush ([System.Drawing.Color]::FromArgb(255, 0x1F, 0x5F, 0xBF))), $path)
  # 白の「A」
  $font = New-Object System.Drawing.Font 'Segoe UI', ([float]($s * 0.62)), ([System.Drawing.FontStyle]::Bold), ([System.Drawing.GraphicsUnit]::Pixel)
  $fmt = New-Object System.Drawing.StringFormat
  $fmt.Alignment = 'Center'; $fmt.LineAlignment = 'Center'
  $g.DrawString('A', $font, [System.Drawing.Brushes]::White, (New-Object System.Drawing.RectangleF 0, ([float]($s * 0.03)), $s, $s), $fmt)
  $g.Dispose()
  $ms = New-Object System.IO.MemoryStream
  $bmp.Save($ms, [System.Drawing.Imaging.ImageFormat]::Png)
  $bmp.Dispose()
  , $ms.ToArray()
}
# ICO（PNG を格納する形式）を組み立てる
$out = New-Object System.IO.MemoryStream
$w = New-Object System.IO.BinaryWriter $out
$w.Write([uint16]0); $w.Write([uint16]1); $w.Write([uint16]$sizes.Count)
$offset = 6 + 16 * $sizes.Count
for ($i = 0; $i -lt $sizes.Count; $i++) {
  $s = $sizes[$i]; $len = $pngs[$i].Length
  $w.Write([byte]($(if ($s -ge 256) { 0 } else { $s }))); $w.Write([byte]($(if ($s -ge 256) { 0 } else { $s })))
  $w.Write([byte]0); $w.Write([byte]0); $w.Write([uint16]1); $w.Write([uint16]32)
  $w.Write([uint32]$len); $w.Write([uint32]$offset)
  $offset += $len
}
foreach ($p in $pngs) { $w.Write($p) }
$w.Flush()
foreach ($dst in "$root\res\icon.ico", "$root\flutter\windows\runner\resources\app_icon.ico") {
  [System.IO.File]::WriteAllBytes($dst, $out.ToArray())
  "written: $dst ($($out.Length) bytes)"
}
# 確認用の PNG（256px）
[System.IO.File]::WriteAllBytes("$root\azad\icon_preview.png", $pngs[-1])
