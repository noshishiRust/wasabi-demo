# UEFIでの画面表示

## 画面表示の基本

### VRAM とは
Video RAM（VRAM）は、画面に表示する内容を保持するための特殊なメモリ領域です。
UEFIでは、GOP（Graphics Output Protocol）を通じてVRAMにアクセスし、ピクセル単位で画面の制御が可能です。

### UEFI Protocol の取得

UEFIでは、様々な機能をProtocolという形で提供しています。画面制御に必要なGOPも、以下のように`locate_protocol`を使用して取得します：

```rust
let gop = unsafe {
    efi_system_table
        .boot_services
        .locate_protocol::<GraphicsOutputProtocol>(guid, c_void, c_void)
        .expect("Failed to get GOP")
};
```

このように取得したGOPを通じて、VRAMへのアクセスが可能になります。Protocol取得時にunsafeブロックが必要なのは、UEFIのシステムテーブルへの直接アクセスを行うためです。

### 実装のポイント

```rust
let mut vram = init_vram(efi_system_table).expect("Failed to initialize VRAM");
fill_rect(&mut vram, Color::White as u32, 0, 0, vw, vh).expect("fill_rect failed");
```

## 001からの主な変更点

1. GOPの取得と初期化:
   - `locate_protocol`を使用してGOPを取得
   - VRAMへのアクセス権を確立

2. 画面描画機能の追加:
   - 画面全体を白で塗りつぶす機能
   - ピクセル単位の描画抽象化

主要な差分:
```rust
let mut vram = init_vram(efi_system_table).expect("Failed to initialize VRAM");
fill_rect(&mut vram, Color::White as u32, 0, 0, vw, vh);
```

## 振り返り

### 分かったこと
- UEFIのGOPを使用した画面制御の基本
- VRAMの構造と操作方法
- Rustでのグラフィック処理における安全性の確保
- Protocol取得の仕組みとunsafeな操作の必要性

### 次の課題
- より複雑な図形の描画
- フォント描画の実装
- ダブルバッファリングなどの画面ちらつき防止
- UEFIの他のProtocolの活用方法の調査

## 参考リソース
- [UEFI Graphics Output Protocol](https://uefi.org/specs/UEFI/2.10/12_Protocols_Console_Support.html#graphics-output-protocol)
- [OSDev Wiki - VRAM](https://wiki.osdev.org/Video_RAM)
