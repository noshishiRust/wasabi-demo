target を指定して build する。

```bash
cargo build --target x86_64-unknown-uefi
```

生成した efi ファイルをマウント先に入れる。

```bash
cp target/x86_64-unknown-uefi/debug/wasabi-demo.efi mnt/EFI/BOOT/BOOTX64.EFI 
```

最初の UEFI を起動させる。

```bash
qemu-system-x86_64 -bios RELEASEX64_OVMF.fd
```

マウントドライブを指定して起動する。

```bash
qemu-system-x86_64 -bios third_party/ovmf/RELEASEX64_OVMF.fd -drive format=raw,file=fat:rw:mnt
```


文字コードを求める

```bash
echo 'A' | hexdump -C
00000000  41 0a                                             |A.| # 0a は改行
00000002
```
