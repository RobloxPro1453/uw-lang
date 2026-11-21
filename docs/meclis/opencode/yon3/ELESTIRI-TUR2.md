# E1 — Şeytanın avukatı: Tur 2 tasarımları (TASARIM-B1..B4, MIRAS, OLCUM, KAPALILIK, JOKER, SADELIK)

> Kanıt: okuduğum dosyalar: TASARIM-B1.md, TASARIM-B1-2.md, TASARIM-B2.md, TASARIM-B3.md, TASARIM-B4.md, MIRAS.md, OLCUM.md, OLCUM-2.md, KAPALILIK.md, JOKER.md, SADELIK.md, SADELIK-2.md. Kod doğrulaması: `crates/uw_lower/src/lib.rs`, `crates/uw_check/src/lib.rs`, `crates/uw_emit/src/lib.rs`, `crates/uw_cli/src/impact.rs`. Sınav kâğıdı (gizli/, mekanik-01..11) okumadım — evet.

## 1. Teşhis

Tasarımlar genel dil kullanıyor ama mekanizma düzeyinde birbirine indirgenmiyor: B1 (atlas tablo + kit-meta şema) ile B1-2 (kimlik/alan/bağ/depo bağlayıcısı) aynı yüzeyin iki yanı; B2 (katkı kümesi) ve B3 (yapı zinciri) birbirini varsayıyor; B4 "Kat 0" ham erişimi `raw:` olmadan iddia ediyor. F1 (Sadelik) "3 ilke", SADELIK-2 "5 ilke" diyor — aynı meclis aynı yüzeyi farklı sayıyla anlatıyor. OLCUM ve OLCUM-2 birebir aynı dosya: sentez öncesi temizlik yok.

## 2. Öneri — mekanizma düzeyinde

Tek bir kurucu yüzey seçilmeli (B1-2'nin dört ilkesi B1'in kit-meta şemasına indirgenemez; çelişki çözülmeden sentez yapılamaz). `raw:` gerçekten dile girene kadar B4'ün "hiçbir şey imkânsız değil" cümlesi tasarım iddiası olarak kalmalı. F1'in "etki raporu" iddiası doğrulanmalı: `Patch.origin`/`Op.origin` hesaplanıyor ve üretilen `.py` dosyasına yorum olarak basılıyor (uw_emit/src/lib.rs:71,153,159), ama gizli Native adımları (give_center_to_lord, assign_lords_to_empty_centers, original_faction) origin'de yok; origin kaynak satırı, gizli adım zinciri değil. JOKER'in "etki-zinciri türetimi yeni ilkel" iddiası therefore yarı doğru: veri var, gizli adımlar görünmez.

## 3. Nasıl çalışır

N/A — bu tur tasarım eleştirisi; mekanizma önerisi yok.

## 4. Neyi ifade EDEMEZ (eleştiriler)

**E1 — B1 vs B1-2 çelişkisi.** B1: "yeni varlık/ilişki → derleyici değişmeli? hayır: ref:<tablo> + kit-meta/records/*.toml allowlist". B1-2: "alan adı → numara bağlaması veri olabilir; tip ailesi değişince allowlist değişir". MIRAS B1-2'den bağımsız olarak kit-meta/records'ı bagaj sayıyor (MIRAS §3). Ama B1 mekanizması o şemaya dayanıyor. Hangisi canonical? Ne yapılmalı: birleştir (SADELIK-2 §5.1 zaten öneriyor) ve "tip ailesi değişince derleyici değişir" cümlesini dürüstçe yaz.

**E2 — Example-driven drift (B3).** B3 §2/§3 merkezde `center p_town_6 of faction fac_kingdom_1 lord trp_kingdom_1_lord` ve Praven. Sözdizimi `[İDDİA]` işaretli ama tasarımın kalbi yine Praven. BRIEF §4.2 "Praven düzeldi = bitti" yasak; B3 bunu yapı zinciri ilkesine çeviriyor (iyi) ama örnek cümlesi hâlâ merkez. Ne yapılmalı: örnek cümlesini "bir yerleşim + krallık + lord" şablonuna indir, Praven'i kanıt olarak KARAR.md'de bırak.

**E3 — Sahte genellik (B4).** B4 §2 "Kat 0 ham: 1060 op isimli fonksiyon; hiçbir şey imkânsız değil". Ama JOKER §5 kanıtladı `raw:` dilde yok (grep 0 sonuç). B4 §4 bunu "eksik genel ilkel: raw: henüz dilde yok" diye yazıyor ama §2 cümlesi tasarım düzeyinde "tam erişim" iddia ediyor. KAPALILIK §2 de "ham katman %100 ifade edilebilir" [İDDİA] diyor. Ne yapılmalı: B4 §2'yi "tasarım hedefi, henüz uygulanmadı" diye işaretle; ölçülemez kalsın.

**E4 — Sessiz bozulma riski (B1-2 slot sınırı).** B1-2 §4: "slot üst sınırı ve savegame kalıcılığı [İDDİA]"; numara haritası değişirse eski kayıt bozulur, uw.lock pinler. Ama pin yalnız UW deltası için (ARCHITECTURE §56); Native taban hash değişince pin ne yapar? Belirsiz. Eski savegame bozulması sessiz risk. Ne yapılmalı: savegame uyumsuzluğunu derleme hatası olarak raporla (B2'nin "dangling ref = hata" kuralıyla tutarlı).

**E5 — OLCUM/OLCUM-2 birebir aynı dosya.** İki çıktı dosyası aynı içerik; sentez öncesi temizlik yok. Ne yapılmalı: birini sil (moderatör işi; E1 sadece işaretler).

**E6 — SADELIK vs SADELIK-2 ilke sayısı çelişkisi.** SADELIK "3 ilke yetrer" [İDDİA]; SADELIK-2 "5 ilkel" ve JOKER'i "aşırı" diyor. Aynı meclis aynı yüzeyi farklı sayıyla anlatıyor. Ne yapılmalı: sentezde tek sayı seçilmeli, gerekçesiyle.

**E7 — F1 iddiası doğrulaması.** F1 (SADELIK-2 §2 İlkel 5): "etki raporu yeni mekanizma değil, Patch.origin verisi zaten hesaplanıyor, sadece basılmıyor." Doğrulama: `uw_check/src/lib.rs:159-171` Patch struct'ında `origin: String` var; `uw_emit/src/lib.rs:71,153,159` emitted `.py` dosyasına `# origin` yorumu olarak basılıyor. Yani veri hesaplanıyor ve üretilen dosyaya basılıyor. Ama (a) `uw_cli/src/impact.rs` "impact report" unreachable entries için, origin aggregation için değil; (b) origin kaynak satırı (örn. "start.uw:4"), gizli Native adımları (give_center_to_lord, assign_lords_to_empty_centers, original_faction) içermez. Sonuç: F1'in "sadece basılmıyor" iddiası yarı doğru — origin basılıyor, ama gizli adım zinciri origin'de yok. JOKER'in "etki-zinciri türetimi yeni ilkel" iddiası therefore yarı doğru: veri var, gizli adımlar görünmez. Ne yapılmalı: origin'i gizli adım zinciriyle eşleştir (B3'ün yapı zinciri ilkesi) — bu yeni ilkel değil, B3 ile birleşim.

**E8 — KAPALILIK "kağıt üstü" sınırı.** KAPALILIK §4: test kağıt üstünde tutarlı ifadeyi ölçer, gerçek derleme/oyun doğrulaması ayrı. Bu dürüst. Ama §2 "gizli özel durum/yama kabul edilmez" — denetim mekanizması moderatör yargısı; kanıtsız. Ne yapılmalı: kabul edilmez kuralını otomatik denetim (grep/metin taraması) ile bağla; talimat yetmez (Tur 1 E3 ile aynı sorun).

**E9 — B2 "like bileşik dünyada çözülmeli" implementasyon boşluğu.** B2 §2: like hedefi bugün atlasa bağlı (native kipe gömülü); project kipinde "boş dünyada sıfırdan kurma" için like bileşik dünyada çözülmeli. Bu mimari değişiklik değil, ad çözümleme hedefi. Ama BRIEF §6 "ürün kodu değiştirme yasak" ve meclis tasarım aşamasında; uygulanmadan "tek mekanizma" iddiası kanıtsız. Ne yapılmalı: [İDDİA] olarak işaretle, Kapı 1'de sınanacak iddia.

**E10 — MIRAS "6 mod bulunamadı" [İDDİA].** MIRAS §5: "ornek-modlar/ klasörü bu çalışma dizininde bulunamadı (glob boş)". Bu ortam sorunu; Native iddiası değil. Ama C2'nin mod okuma kanıtı eksik. Ne yapılmalı: moderatör doğrulamalı (sınav seti §2 klasörleri var).

## 5. Kanıt / [İDDİA] ayrımı

- Kanıtlı (dosya:satır): `uw_check/src/lib.rs:159-171` Patch.origin; `uw_emit/src/lib.rs:71,153,159` origin yorumu; `uw_lower/src/lib.rs:25-31` Op.origin; `uw_cli/src/impact.rs:1-70` unreachable entries (origin aggregation değil).
- [İDDİA]: B4 Kat 0 "hiçbir şey imkânsız değil" (raw: yok); B3 yapı zinciri sözdizimi; B1-2 slot üst sınırı/savegame; B2 like bileşik dünyada çözümü; SADELIK "3 ilke yetrer"; SADELIK-2 "5 ilkel"; MIRAS 6 mod bulunamadı.
- Çelişki kanıtı: SADELIK.md §2 "3 ilke", SADELIK-2.md §2 "5 ilkel".

## 6. Karşı çıktığın yaygın varsayım

"Patch.origin var, dolayısıyla etki raporu zaten var" varsayımı. Origin hesaplanıyor ve üretilen dosyaya yorum olarak basılıyor; ama (a) gizli Native adımları (give_center_to_lord, assign_lords_to_empty_centers, original_faction) origin'de yok, (b) origin aggregation komutu yok (impact.rs unreachable entries için), (c) modcu yüzünde origin Native adları görünür (bilgi kusanma yasağı VISION:94). Dolayısıyla "etki raporu yeni mekanizma değil" iddiası yarı doğru; gizli adım zinciri hâlâ eksik.