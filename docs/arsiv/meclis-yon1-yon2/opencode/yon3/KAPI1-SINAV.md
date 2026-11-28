# KAPI 1 SINAV KÂĞIDI (moderatör hazırladı)

> Kaynak: `gizli/ANALIZ-A1.md` (L&D, ~70 mekanik → 3 kova), `gizli/ANALIZ-A2.md` (6 mod → beceri-etki).
> Bu liste **gereksinim değil, sınav**dır. Hiçbir satır "yapılacak iş" değildir.
>
> **Kural:** Her satır için dört kutudan biri doldurulur. "Şu özellik/alan/modül eklensin" cevabı **otomatik ret**.
>
> - **A** = mevcut ilkelle ifade edilir (hangi ilkel + modcunun satırı + derleyicinin ürettiği adımlar gösterilir)
> - **B** = kısmen: ilkel var, şu **genel** adım eksik
> - **C** = motor sınırı (kanıt: `Module_system 1.171/dosya:satır`)
> - **D** = sistem eksik: şu **genel ilkel** yok (ilkel en az iki farklı mekanikte işe yaramalı)

## Sınav listesi (22 satır)

### Kova (a) — görünürde basit, motor tarafında çok adımlı

| # | Mekanik (davranış dili) | Kaynak |
|---|---|---|
| 1 | "25 şu zırhı getir" → envanter sayımı, eşik, diyalog koşulu, ödül seçimi | A1 §2a (2.4) |
| 2 | Boss ölümsüz: dört kırılabilir prop kırılınca boss ölür, ödül düşer | A1 §2a (9.3) |
| 3 | Eşyaya kuşanınca geçici stat bonusu; çıkarınca geri al | A1 §2a (6.1) |
| 4 | Stamina barı + tuşla özel vuruş (ek hasar) | A1 §2a (4.2) |
| 5 | Hapishaneden kaçış: kontrol devri + süreli prop etkileşimi + yakalanınca takviye | A1 §2a (3.4) |
| 6 | 20+ adımlı ana görev zinciri: sayaçlar, eşikler, koşullu diyalog, harita olayı, sahne, ödül | A1 §2a (1.1-1.9) |
| 7 | At pazarı: parti avı sayacı → eşikte mini oyun açılır | A1 §2a (2.10) |
| 8 | Bilmece/blackjack: mission template + sayaç + rastgele | A1 §2a (2.6/2.7) |
| 9 | Yarış: bitiş sırasına göre ödül | A1 §2a (2.19) |
| 10 | Zindan: düşman seni görürse alarm; görmeden öldürürsen 1.5× XP | A1 §2a+§2b (3.2) |

### Kova (b) — motorun zaten sağladığı, modcunun bilmediği

| # | Mekanik | Kaynak |
|---|---|---|
| 11 | Gizli prop'u aynayla görünür kıl | A1 §2b (Mirror) |
| 12 | Bir parti oyuncuyu kovalasın / avı kessin | A1 §2b (1.3/7.8) |
| 13 | Killstreak sayacı ekranda gösterilsin | A1 §2b (10.4) |
| 14 | Diyalog bitişi görev durumunu belirlesin ("Tab'a basma" uyarısı) | A1 §2b (1.2) |
| 15 | Sahne geçişi + NPC ile konuşma başlat | A1 §2b |

### Kova (c) — gerçek motor sınırı adayları

| # | Mekanik | Kaynak |
|---|---|---|
| 16 | Yavaş çekim (bullet time) | A1 §2c (2003 "cheat mode only") |
| 17 | Haritada parti "gizlilik modu" (algı olasılığı) | A1 §2c (3.1) |
| 18 | Eşyaya **kalıcı** dinamik stat bonusu (kayıt/yükleme sonrası da) | A1 §2c (6.1) |
| 19 | Serbest metin girişli UI | A1 §2c (10.4) |
| 20 | Yeni özel yetenek animasyonu | A1 §2c (4.3) |

### A2'den — beceri/etki

| # | Mekanik | Kaynak |
|---|---|---|
| 21 | Yeni beceri ekle (reserved slot) ve **seviyesine bağlı etki** uygula | A2 §2-4 |
| 22 | Sabit 21 beceriden birinin motor etkisini kaldır | A2 §4 |

## Cevap tablosu (koltuklar doldurur)

| # | Kutu (A/B/C/D) | Hangi ilkel | Modcunun satırı | Derleyicinin ürettiği adımlar | Kanıt / [İDDİA] |
|---|---|---|---|---|---|
| 1 | | | | | |
| … | | | | | |
| 22 | | | | | |

## Kapı 1 geçme ölçüsü

- ≥ %70'i **A veya B** → sistem "genel" iddiasını taşıyabilir.
- **D** çıkması normaldir; her D, sentezde "kurulacak genel ilkel" listesine **bir kez** girer.
- Aynı mekanik için 3+ kez D → sistem örnek başına ilkel istiyor demektir; **genel değildir**.
- Cevabı "şu özellik/modül eklensin" olan satır **ret**; sentezde kullanılmaz.
