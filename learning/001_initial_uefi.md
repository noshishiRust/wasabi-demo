# はじめての UEFI

### no_stdとno_main属性の役割

普段何気なく使っているRustの標準ライブラリ（std）は、実はOSの機能に大きく依存しています。
例えば、`println!`マクロ一つをとっても、OSごとの標準出力の違いを抽象化してくれています。
ブートローダーのような低レベルなプログラミングでは、このような抽象化は使えないため、
`#![no_std]`属性を使用してstdを無効化し、より低レベルな制御を行います。

### パニックハンドラの実装

組み込みシステムでは、パニック時の挙動も自前で実装する必要がある。

```rust
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
        hlt();
    }
}
```

### CPU制御の最適化 - HLT命令の重要性

単純な無限ループは、CPUリソースを無駄に消費してしまいます。
そこでHLT（Halt）命令を使用することで、次の割り込みやイベントが発生するまでCPUを休止状態にし、
電力消費を抑えることができます。

## UEFIプログラミングの基本

### エントリーポイントの構造

UEFIアプリケーションのエントリーポイントは以下の形式で定義されます：

```rust
fn efi_main(_image_handle: EfiHandle, efi_system_table: &EfiSystemTable)
```

これらの引数を通じて、UEFIファームウェアから必要な情報やサービスにアクセスできます。

## 実装のポイント

UEFIブート環境の最小構成を実装:

1. `#![no_std]`と`#![no_main]`による独立実行環境の構築
2. パニックハンドラの実装でエラー時の無限ループ
3. HLT命令による省電力制御

ベースとなるコードの構造:
```rust
#[no_mangle]
fn efi_main(_image_handle: EfiHandle, efi_system_table: &EfiSystemTable) {
    loop {
        hlt();
    }
}
```

## 今回の学びと次のステップ

### 得られた知見
- UEFIという現代的なブート環境の基本構造
- Rustでの低レベルプログラミングの実践的アプローチ
- 組み込みシステムにおける電力管理の重要性

### 次の課題
- UEFIが提供するサービスの具体的な活用方法の習得
- より複雑なブートローダー機能の実装

## 参考リソース
- [The Rust Reference - The no_std attribute](https://doc.rust-lang.org/reference/attributes/diagnostics.html#the-no_std-attribute)
- [UEFI Specification Version 2.11](https://uefi.org/specs/UEFI/2.11/04_EFI_System_Table.html)
- [Intel® 64 and IA-32 Architectures Software Developer's Manual - HLT instruction](https://www.intel.com/content/dam/www/public/us/en/documents/manuals/64-ia-32-architectures-software-developer-vol-2a-manual.pdf)
- [OS開発でのCPU省電力制御について](https://wiki.osdev.org/Power_Management)
