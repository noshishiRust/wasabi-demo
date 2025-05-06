# UEFIメモリマップの取得

## メモリ管理の基礎

### メモリマップとは

UEFIシステムでは、利用可能なメモリ領域の情報をメモリマップという形で提供します。
このマップには以下のような情報が含まれます：

- メモリ領域の開始アドレス
- メモリ領域のサイズ
- メモリの種類（通常のRAM、予約領域、ACPIなど）

### 実装例

```rust
let mut memory_map = MemoryMapHolder::new();
let status = efi_system_table
    .boot_services
    .get_memory_map(&mut memory_map);

for e in memory_map.iter() {
    if e.memory_type() == EfiMemoryType::CONVENTIONAL_MEMORY {
        total_memory_pages += e.number_of_pages();
    }
}
```

### メモリマップの重要性

1. 利用可能なメモリ領域の把握
2. OSのメモリ管理機能の初期化
3. ページングの設定
4. メモリ保護機能の実装

## 004からの主な変更点

1. メモリマップ機能の追加:
   - UEFIのメモリマップ取得
   - メモリ種別の判定ロジック

2. メモリ情報の表示:
   - テキスト描画機能を活用した情報表示
   - 利用可能メモリ量の計算と表示

主要な差分:
```rust
let mut memory_map = MemoryMapHolder::new();
efi_system_table.boot_services.get_memory_map(&mut memory_map);
```

## 振り返り

### 分かったこと
- UEFIのメモリマップ取得方法
- メモリ種別の区分け方法
- ブートローダーでのメモリ管理の重要性
- ページサイズと総メモリ量の計算方法

### 次の課題
- メモリマップに基づくメモリ管理システムの実装
- ページングの設定
- メモリアロケータの実装

## 参考リソース
- [UEFI Memory Map](https://uefi.org/specs/UEFI/2.10/07_Services_Boot_Services.html#memory-allocation-services)
- [OSDev Wiki - Memory Management](https://wiki.osdev.org/Memory_Management)
- [Writing an OS in Rust - Memory Management](https://os.phil-opp.com/memory-management/)
