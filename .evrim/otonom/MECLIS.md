# Meclis Müzakere Planı

- Tarih: 2026-11-15T09:47:02.854Z
- Konu: UW sözdizimi: süslü parantez mi girinti mi; Faz 1 için en küçük dil alt kümesi (override menu, on game.start, Native op çağrısı)
- Karar / Kip: `light` — Hafif Meclis (2 koltuk)
- Gerekçe: Orta karmaşıklık + sıkı kota/plan (5s %100, hafta %100 (kalan %0)): 4 ağır Opus koltuğu yerine 2 odaklı ekonomik koltuk (haiku/sonnet/yerel CLI/API) seçildi.
- Karmaşıklık: `medium` (istenen: `medium`, derinlik: `auto`)
- Plan / Kota: 5s %100, hafta %100 (kalan %0)
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
UW sözdizimi: süslü parantez mi girinti mi; Faz 1 için en küçük dil alt kümesi (override menu, on game.start, Native op çağrısı)
</topic>

<license_policy mode="open-source" restricted="true">
Dağıtım/ticari kullanım politikası (open-source): Açık kaynak referans ve kod incelemelerini YALNIZCA MIT, Apache-2.0, BSD-2-Clause, BSD-3-Clause, ISC lisanslı projelerle sınırlandırın; GPL/AGPL/SSPL veya lisanssız depolardan kod/tasarım kopyalamayın.
</license_policy>

<context>
Plan/Kota: 5s %100, hafta %100 (kalan %0).
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
UW sözdizimi: süslü parantez mi girinti mi; Faz 1 için en küçük dil alt kümesi (override menu, on game.start, Native op çağrısı)
</topic>

<license_policy mode="open-source" restricted="true">
Dağıtım/ticari kullanım politikası (open-source): Açık kaynak referans ve kod incelemelerini YALNIZCA MIT, Apache-2.0, BSD-2-Clause, BSD-3-Clause, ISC lisanslı projelerle sınırlandırın; GPL/AGPL/SSPL veya lisanssız depolardan kod/tasarım kopyalamayın.
</license_policy>

<context>
Plan/Kota: 5s %100, hafta %100 (kalan %0).
</context>

<instructions>
1. Diğer üyelerin görüşünü görmeden tamamen bağımsız analiz yap (kör müzakere).
2. Kendi açından en önemli 3-5 somut bulguyu, riski veya mimari öneriyi kısa maddeler halinde yaz.
3. Varsa daha basit veya daha ucuz bir alternatif öner; belirsizlik varsa açıkça işaretle.
4. Gereksiz uzun anlatımdan ve kapsam dışı özellik önermekten kaçın.
</instructions>
```
