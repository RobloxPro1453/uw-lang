# Kaynağı açık örnek modlar (`ornek-modlar/`)

Yerel referans, **depoya girmez** (lisanslar karışık). Yalnız kaynak dosyaları (`*.py`, `*.txt`, README, LICENSE) seyrek klonlandı (2026-11-15). Kaynak: Manus raporu `llm-raporlari/manus-ai/09-acik-kaynakli-modlar.md` **[İDDİA]**; depo varlığı ve commit'ler doğrulandı.

| Mod | Commit | Tür | Lisans (kendi değişiklikleri) |
|---|---|---|---|
| catplomacy | b12ae7c (2021-10-15) | Diplomacy türevi | yok / bilinmiyor |
| Diplado | d560bd4 (2019-10-27) | Diplomacy 4.3+ fork, 1.174 | yok / bilinmiyor |
| 1257AD | 612dd60 (2018-03-16) | tarihî total conversion | forum duyurusu, net değil |
| medieval-conquests | 113e13e (2019-08-13) | 1257AD tabanı + ko-op | MIT (TaleWorlds dışı kısım) |
| pw_module_system | a35fd5d (2017-07-15) | Persistent World (MP) | BSD benzeri (PW değişiklikleri) |
| Persistent-Kingdoms | 7725744 (2020-06-23) | PW tabanı (MP) | PW lisansı + ek madde |

## UW için ne işe yarar

1. **Dayanıklılık deneme seti:** "Native + fark" yaklaşımı (atlas dökümü, değer düzeyinde yama) yalnız Native'de değil, büyük gerçek modlarda da çalışmalı. Manus raporu **[İDDİA]**: PW ve Medieval Conquests kayıtları yardımcı Python fonksiyonları/ek import'larla üretiyor → atlas'ın dosyayı metin olarak değil **değer** olarak okuması bu yüzden doğru seçim; deneyle sınanacak.
2. **Yan ürün fikri:** Değer düzeyinde yama herhangi bir Module System kaynağının üstüne uygulanabilir → ileride `base: <başka modun kaynağı>` (ör. Diplomacy üstüne mod). Öncelik değil; sahibi "var olan modları almak şimdilik gereksiz" dedi.
3. **Mekanik kalıp örnekleri** (sahibinin kendi mekaniklerini tasarlarken ve UW ilkellerinin yeterliliğini sınarken): Diplado savaşta tuşla at çağırma/taunt [İDDİA], PW kaynak toplama/ağaç kesme sahne nesneleri + kalıcı kayıt [İDDİA], PK sunucu log script'leri [İDDİA].
4. Kod kopyalama: yalnız izin verici lisanslı kısımlardan (MC'nin MIT kısmı, PW'nin BSD benzeri kısmı) ve lisans notu korunarak; diğerleri yalnız fikir için okunur.

## Stres testi girdisi: Light & Darkness – Heroes of Calradia (`ornek mod mekanik arastirmasi (stres testi)/`)

Sahibinin eklediği araştırma raporu (2026-11-16, 13 dosya, ~2600 satır, Türkçe): modun **kodu değil**, oynanıştan çıkarılmış mekanik envanteri (11 başlık, ~100 alt mekanik, kaynaklı). Derleyici bitince her mekanik UW ile yazılıp "tam / kısmen / motor sınırı / doğrulanmadı" diye ölçülecek (ROADMAP "Stres testi"). Tasarımda şimdiden gözetilecek ilkeller:

- **Görev durum makinesi:** sıralı aşamalar, sayaç eşikleri ("25 zırh topla"), gecikmeli/koşullu tetik, görev bitmeden eylem kilidi → `quest` + aşama + sayaç sözü gerekir.
- **Kalıcı seçim/rota durumu:** diyalog cevabı → kalıcı bayrak (Işık/Karanlık), önceki cevaba bağlı seçenek, ilişki eşiği → adlı global durum + diyalog koşulları.
- **Harita olayları:** parti doğurma/takip/av, kamp baskını, liman/tekne, harita gizliliği → `spawn`, `on enter area`, `every … hour` (Y6).
- **Sahne/mission akışı:** düello, boss, dalga sayacı, gizlilik, tuzak, tuşla yetenek (sprint, enerji vuruşu), sahne prop'u etkileşimi, sırayla vurulan prop bulmacası → mission template tetikleri + tuş + prop olayları (Y8).
- **Eşya:** artefakt, kristalle yükseltme/geri alma, ganimet, eşyayla beceri açma → eşya değişimi + slot.
- **Yoldaş:** katılma/ayrılma, ilişki, otonom parti, rotaya göre ayrılma (Y5).
- **Arayüz:** HUD göstergesi, karakter yaratma soruları, beceri tavanı, parti kapasitesi → presentation + karakter yaratma menüsü.

Depo: rapor kendi sözleriyle yazılmış, TaleWorlds/mod kodu içermez; yine de 3. taraf rehberlerin özeti olduğundan **sahibi karar verene kadar depoya alınmadı**.
