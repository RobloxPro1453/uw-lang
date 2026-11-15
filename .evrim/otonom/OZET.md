# Özet (sahibi için, sade)

## Şu an
OHAL. 4. çevrim bitti. Dilin yeni ağaç yapısı yazıldı; DeepSeek 4 şeritte çalışıyor: ayrıştırıcı (T-014), hesaplama/if çevirisi (T-016), denetim (T-017), menü çıktısı (T-018).

## Bitenler
- Gramer 2 iskeleti: hesaplama, if, menü seçenekleri ve yeni menü için türler hazır (testler 92/92).
- UW kılavuzu hazır: docs/kilavuz/UW_KILAVUZU.md (Türkçe; dil, komutlar, hata kodları, sınırlar). Bugünkü yenilikler de işlendi.
- Dilin yeni yazım kuralları kararı: docs/meclis/gramer-2-oneri.md (örneklerle). Kısa risk denetiminden geçti; iki gerçek tuzak (and/or önceliği, "not a < b") karara eklendi.
- Türkçe metinler: uw explain / browse artık oyunun resmî Türkçesini de gösteriyor ve Türkçe kelimeyle arıyor.
- Altın testler: her dil özelliği için örnek .uw ve beklenen çıktı.
- `--debug` düzeltildi: menü adı artık `uw: menu-start-phase-2` gibi sarı yazılmalı ve mesaj günlüğünde kalmalı. **Oyunda denenmedi** (aşağıda).
- `uw explain start_phase_2` (ön eksiz ad) artık doğrudan `menu_start_phase_2`'yi buluyor; `graph` ve `browse` da aynı.
- Senin notun kaydedildi: DeepSeek için paralel şerit 12, günlük tavan 5$; kılavuz kuralı kalıcı karar oldu (her yeni özellik kılavuza aynı turda eklenir).

## Karar verdiklerim
- Kılavuzun yeri: `docs/kilavuz/UW_KILAVUZU.md`. İlk hâlini DeepSeek yazıyor, ben denetleyeceğim.
- Dilin büyük adımı (ifadeler, `if`, menü seçenekleri) önce benim gramer kararımı bekliyor; sonraki çevrimde başlıyorum, sonra DeepSeek'e çok sayıda paralel iş açılacak.
- Remote Control'ü açmadım (araç yalnız senin isteğinle açılmasını istiyor).

## Takıldıklarım
- Döngü bir süre SIRA_DEGIL dedi: makinedeki sıra kaydında paket testlerinden kalan iki sahte proje vardı; senin sözünle silindi (yedek: ~/.claudevolution/projects.json.bak).

## Harcama
Bu çevrim: 5 saatlik kotanın ~%4'ü; DeepSeek şu ana kadar birkaç sent.

## Senin bakman gerekenler
- `uw build examples/praven_start --debug` ile oyunda menü adının okunur çıktığını dene (CHECKUP).
- Proje kökünde depoya girmemiş `flow_startgamezero.json` var; ben dokunmadım.
