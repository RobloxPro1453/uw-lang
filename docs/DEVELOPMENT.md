# Geliştirme

## Derleme ve test

Komutlar `.evrim/config.json` içindedir (`build`, `test`); beceriler ve ucuz model oradan okur. Burada yalnız insan için özet:

```
cargo build --workspace
```

```
cargo test --workspace
```

- Süre: kit derlemesi ~27 s (D1); `cargo test --workspace` gerçek kit testiyle ~30 s. Kit ya da `C:\Python27` yoksa o test kendini atlar.
- Atlas üretimi (kit değişince): `python py/kitrun.py --kit "Module_system 1.171" --staging target/uw-staging/d1`, sonra `target/uw-staging/d1` içinde `C:/Python27/python.exe -B -E -s ../../../py/atlas_dump.py ../atlas.json` → `target/uw-staging/atlas.json` (depoya girmez). Deneyler: `py/d2_empty_patch.py`, `py/d3_reprint.py`.
- Testler geçici staging klasörü kullanır; kit klasörüne ve sahibinin Warband mod klasörlerine dokunmaz (oyun deneyleri ayrı test modu klasöründe).

## Yeni özellik ekleme

1. <Hangi dosyaya ne eklenir; ör. "Komutu `src/commands.js` listesine ekle.">
2. <Arayüze nasıl bağlanır.>
3. Davranış için test ekle (<test dosyası>); test, davranış bozulunca düşmeli.

## Test yazma kuralları

- Her test başladığı durumu geri bırakır (açtığını kapatır, değiştirdiği ayarı eski hâline getirir). Tek tek geçen testler birlikte koşunca bu yüzden kırılır.
- Test süreçleri sahibinin açık penceresinden ayırt edilebilir olmalı (ayrı profil, ayrı pencere adı).

## Araçlar

| Komut | Ne yapar |
|---|---|
| `.evrim/cv status` | tur, kota, şeritler, hafıza özeti |
| `.evrim/cv doctor` | gözetimsiz çalışma önkoşulları |
| `.evrim/cv delegate ...` | ucuz model ajanı ve şeritler (`calis` becerisi kullanır) |
