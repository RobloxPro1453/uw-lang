# Günlük (sona eklenir, silinmez; çevrim başında yalnız son 10 satır okunur)

Her çevrim 3-6 satır, sade ve dürüst: ne yaptım, neye mal oldu (kota %, $), değdi mi, bir dahakine ne. Uyku çevrimi tek satır. Kayıtları `.evrim/cv cycle end` ekler; elle de eklenebilir.

<!-- Biçim:
## 2026-01-15 23:05 · kota 5s %67 / hafta %26
Yaptım: ... Maliyet: ~%2 kota, 0 $. Değdi: evet. Sıradaki: ...
-->

## 2026-11-15 16:01 · kota 5s %48 / hafta %60
Yaptım: 3 DeepSeek tarifi başlatıldı (kılavuz, TR metin, altın test); --debug ve çıplak ad düzeltildi; sahibinin bütçe/kılavuz notu kaydedildi
Maliyet: 5s %4, DeepSeek <0.1$ · commit: 4 · Değdi: evet
Sıradaki: gramer kararı (ifade, if, option, text, menu) + IR, ardından DeepSeek şeritleri; biten T-011..13 gözden geçir

## 2026-11-15 16:07 · kota 5s %51 / hafta %60
Yaptım: T-013 ve T-012 birleşti (altın test, resmî Türkçe metin); gramer 2 kararı hafif meclisle (risk koltuğu haiku) alındı
Maliyet: 5s %3 · commit: 8 · Değdi: evet
Sıradaki: uw_syntax AST (Stmt/Expr/Cond/MenuItem/New) Claude yazar; ardından DeepSeek şeritleri: parser, lower, emit, check, altın testler, kılavuz bölümleri; T-011 gözden geçir
