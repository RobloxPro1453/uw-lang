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

## 2026-11-15 16:14 · kota 5s %56 / hafta %61
Yaptım: OHAL ayarı (tavan %90, pay 3, 20 şerit); sıra kaydındaki test kayıtları silindi; T-011 kılavuz birleşti ve güncellendi; CHANGELOG
Maliyet: 5s %4 · commit: 5 · Değdi: evet
Sıradaki: uw_syntax AST (Stmt/Expr/Cond/MenuItem/New) + parser iskeleti Claude; ardından DeepSeek şeritleri (lower, emit, check, altın test, kılavuz)

## 2026-11-15 16:22 · kota 5s %61 / hafta %61
Yaptım: Gramer 2 iskeleti (AST, lower türleri, Action); T-014/016/017/018 DeepSeek şeritleri başladı
Maliyet: 5s %5 · commit: 3 · Değdi: evet
Sıradaki: uw_runtime yardımcıları (set_text, add_option, change_option, remove_option, add_entry) Claude; T-015 (menü öğeleri ayrıştırıcı) tarifi; biten dalları gözden geçir

## 2026-11-15 16:30 · kota 5s %68 / hafta %62
Yaptım: T-016, T-018, T-014 birleşti; lower this_or_next hatası düzeltildi; uw_runtime menü yardımcıları; T-015 başladı
Maliyet: 5s %3 · commit: 11 · Değdi: evet
Sıradaki: T-017 ve T-015 gözden geçir/birleştir; altın testler + örnek mod (if, option) + kılavuz bölümleri; release derle

## 2026-11-15 16:50 · kota 5s %75 / hafta %63
Yaptım: T-015 ve T-017 birleşti; menu_demo uçtan uca derlendi ve kuruldu; kılavuz 3.5-3.8; CHECKUP oyun deneyi; T-019 başladı; push izin denetiminde
Maliyet: 5s %5 · commit: 8 · Değdi: evet
Sıradaki: T-019 gözden geçir; bağlayıcı sırası (T-020) ve flow_json diyaloglar; bağlam 400k olunca devir
