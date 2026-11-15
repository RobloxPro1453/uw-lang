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
