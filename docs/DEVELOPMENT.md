# Geliştirme

## Derleme ve test

Komutlar `.evrim/config.json` içindedir (`build`, `test`); beceriler ve ucuz model oradan okur. Burada yalnız insan için özet:

```
cargo build --workspace
```

```
cargo test --workspace
```

- Derleme/test süresi: Faz 0'da ölçülüp buraya yazılacak (D1). Uzun koşular arka planda çalıştırılır, süre sınırı konur.
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
