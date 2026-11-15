# Özet (sahibi için, sade)

## Şu an
Gözetimsiz döngü çalışıyor (1. çevrim bitti). DeepSeek'te 3 iş sürüyor: UW kılavuzunun ilk hâli, oyunun resmî Türkçe metinlerini araçlara bağlama, "altın testler" (her dil özelliği için örnek .uw → beklenen çıktı).

## Bitenler
- `--debug` düzeltildi: menü adı artık `uw: menu-start-phase-2` gibi sarı yazılmalı ve mesaj günlüğünde kalmalı. **Oyunda denenmedi** (aşağıda).
- `uw explain start_phase_2` (ön eksiz ad) artık doğrudan `menu_start_phase_2`'yi buluyor; `graph` ve `browse` da aynı.
- Senin notun kaydedildi: DeepSeek için paralel şerit 12, günlük tavan 5$; kılavuz kuralı kalıcı karar oldu (her yeni özellik kılavuza aynı turda eklenir).

## Karar verdiklerim
- Kılavuzun yeri: `docs/kilavuz/UW_KILAVUZU.md`. İlk hâlini DeepSeek yazıyor, ben denetleyeceğim.
- Dilin büyük adımı (ifadeler, `if`, menü seçenekleri) önce benim gramer kararımı bekliyor; sonraki çevrimde başlıyorum, sonra DeepSeek'e çok sayıda paralel iş açılacak.
- Remote Control'ü açmadım (araç yalnız senin isteğinle açılmasını istiyor).

## Takıldıklarım
(yok)

## Harcama
Bu çevrim: 5 saatlik kotanın ~%4'ü; DeepSeek şu ana kadar birkaç sent.

## Senin bakman gerekenler
- `uw build examples/praven_start --debug` ile oyunda menü adının okunur çıktığını dene (CHECKUP).
- Proje kökünde depoya girmemiş `flow_startgamezero.json` var; ben dokunmadım.
