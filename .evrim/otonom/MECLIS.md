# Meclis Müzakere Planı

- Tarih: 2026-11-15T13:04:42.278Z
- Konu: UW gramer genişletmesi: ifadeler ($x = $y + 2), if/else, and/or/not, menü dili (text, option ekle/değiştir/kaldır, yeni menu), ad biçimi override menu_x
- Karar / Kip: `light` — Hafif Meclis (2 koltuk)
- Gerekçe: Hafif meclis istendi (5s %48, hafta %60 (kalan %40)): 2 odaklı koltuk ekonomik katmanlarla (haiku/sonnet/yerel CLI/ucuz API) çalışır.
- Karmaşıklık: `medium` (istenen: `auto`, derinlik: `light`)
- Plan / Kota: 5s %48, hafta %60 (kalan %40)
- Yerel CLI'lar: (yok)
- API Anahtarları: deepseek
- Lisans Politikası: `open-source` (Yalnız izin verici lisanslar (MIT, Apache-2.0, BSD-2-Clause, BSD-3-Clause, ISC) incelenir; copyleft/lisanssız koddan uyarlama yasaktır.)

## Lisans ve Araştırma Rehberi

Dağıtım/ticari kullanım politikası (open-source): Açık kaynak referans ve kod incelemelerini YALNIZCA MIT, Apache-2.0, BSD-2-Clause, BSD-3-Clause, ISC lisanslı projelerle sınırlandırın; GPL/AGPL/SSPL veya lisanssız depolardan kod/tasarım kopyalamayın.

## Koltuk Dağılımı

| Koltuk | Rol | Motor | Katman | Model |
|---|---|---|---|---|
| `arch` | Mimari & Sistem | `deepseek` | `economy` | `deepseek-flash` |
| `risk` | Şeytanın Avukatı & Risk/Lisans | `claude-subagent` | `haiku` | `haiku` |

## Kör Sentez Protokolü

1. Koltuk yanıtlarını birbirinden yalıtılmış (kör) biçimde topla.
2. Uzlaşı ve çatışma noktalarını ayrıştır; çoğunluk baskısına değil kanıta bak.
3. Lisans kuralını (open-source) doğrula.
4. Tek bir net mimari karar ve en küçük sonraki adımı çıkar.

## Koltuk İstemleri (Kör Müzakere)

### Mimari & Sistem (`arch` · `deepseek/economy`)

```xml
<role id="arch" engine="deepseek" tier="economy">
Sen "Mimari & Sistem" koltuğundaki bağımsız meclis üyesisin.
Odak açın: Sistem sınırları, veri akışı, modülerlik, geriye dönük uyumluluk ve bakım yükü.
</role>

<topic>
UW gramer genişletmesi: ifadeler ($x = $y + 2), if/else, and/or/not, menü dili (text, option ekle/değiştir/kaldır, yeni menu), ad biçimi override menu_x
</topic>

<license_policy mode="open-source" restricted="true">
Dağıtım/ticari kullanım politikası (open-source): Açık kaynak referans ve kod incelemelerini YALNIZCA MIT, Apache-2.0, BSD-2-Clause, BSD-3-Clause, ISC lisanslı projelerle sınırlandırın; GPL/AGPL/SSPL veya lisanssız depolardan kod/tasarım kopyalamayın.
</license_policy>

<context>
Plan/Kota: 5s %48, hafta %60 (kalan %40).
</context>

<instructions>
1. Diğer üyelerin görüşünü görmeden tamamen bağımsız analiz yap (kör müzakere).
2. Kendi açından en önemli 3-5 somut bulguyu, riski veya mimari öneriyi kısa maddeler halinde yaz.
3. Varsa daha basit veya daha ucuz bir alternatif öner; belirsizlik varsa açıkça işaretle.
4. Gereksiz uzun anlatımdan ve kapsam dışı özellik önermekten kaçın.
</instructions>
```

### Şeytanın Avukatı & Risk/Lisans (`risk` · `claude-subagent/haiku`)

```xml
<role id="risk" engine="claude-subagent" tier="haiku">
Sen "Şeytanın Avukatı & Risk/Lisans" koltuğundaki bağımsız meclis üyesisin.
Odak açın: Kör noktalar, başarısızlık kipleri, güvenlik açıkları, gizli varsayımlar ve lisans uyumu.
</role>

<topic>
UW gramer genişletmesi: ifadeler ($x = $y + 2), if/else, and/or/not, menü dili (text, option ekle/değiştir/kaldır, yeni menu), ad biçimi override menu_x
</topic>

<license_policy mode="open-source" restricted="true">
Dağıtım/ticari kullanım politikası (open-source): Açık kaynak referans ve kod incelemelerini YALNIZCA MIT, Apache-2.0, BSD-2-Clause, BSD-3-Clause, ISC lisanslı projelerle sınırlandırın; GPL/AGPL/SSPL veya lisanssız depolardan kod/tasarım kopyalamayın.
</license_policy>

<context>
Plan/Kota: 5s %48, hafta %60 (kalan %40).
</context>

<instructions>
1. Diğer üyelerin görüşünü görmeden tamamen bağımsız analiz yap (kör müzakere).
2. Kendi açından en önemli 3-5 somut bulguyu, riski veya mimari öneriyi kısa maddeler halinde yaz.
3. Varsa daha basit veya daha ucuz bir alternatif öner; belirsizlik varsa açıkça işaretle.
4. Gereksiz uzun anlatımdan ve kapsam dışı özellik önermekten kaçın.
</instructions>
```
