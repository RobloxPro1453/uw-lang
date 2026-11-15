# Meclis Müzakere Planı

- Tarih: 2026-11-15T17:45:47.571Z
- Konu: Yön 2 büyük meclis: veri kayıtları için tek genel UW biçimi (gidiş-dönüş kabullü), uw new --from native proje iskeleti ve Native birleştirme kuralı, dil katmanları L0/L1/L2 ve ham kalan %7,3 satır (try/else_try ara hesaplı kollar, neg|/this_or_next| bekçi koşullar) için sözdizimi
- Karar / Kip: `deep` — Derin Meclis
- Gerekçe: Yüksek karmaşıklık ve uygun bütçe (5s %75, hafta %63 (kalan %37)): 3-4 farklı bakış açısı ile çoklu motor/model meclisi değer üretir.
- Karmaşıklık: `high` (istenen: `high`, derinlik: `auto`)
- Plan / Kota: 5s %75, hafta %63 (kalan %37)
- Yerel CLI'lar: (yok)
- API Anahtarları: deepseek
- Lisans Politikası: `open-source` (Yalnız izin verici lisanslar (MIT, Apache-2.0, BSD-2-Clause, BSD-3-Clause, ISC) incelenir; copyleft/lisanssız koddan uyarlama yasaktır.)

## Lisans ve Araştırma Rehberi

Dağıtım/ticari kullanım politikası (open-source): Açık kaynak referans ve kod incelemelerini YALNIZCA MIT, Apache-2.0, BSD-2-Clause, BSD-3-Clause, ISC lisanslı projelerle sınırlandırın; GPL/AGPL/SSPL veya lisanssız depolardan kod/tasarım kopyalamayın.

## Koltuk Dağılımı

| Koltuk | Rol | Motor | Katman | Model |
|---|---|---|---|---|
| `arch` | Mimari & Sistem | `claude-subagent` | `sonnet` | `sonnet` |
| `ux` | Ürün & Kullanıcı Deneyimi | `deepseek` | `economy` | `deepseek-flash` |
| `risk` | Şeytanın Avukatı & Risk/Lisans | `claude-subagent` | `haiku` | `haiku` |
| `cost` | Verimlilik & Maliyet | `claude-subagent` | `haiku` | `haiku` |

## Kör Sentez Protokolü

1. Koltuk yanıtlarını birbirinden yalıtılmış (kör) biçimde topla.
2. Uzlaşı ve çatışma noktalarını ayrıştır; çoğunluk baskısına değil kanıta bak.
3. Lisans kuralını (open-source) doğrula.
4. Tek bir net mimari karar ve en küçük sonraki adımı çıkar.

## Koltuk İstemleri (Kör Müzakere)

### Mimari & Sistem (`arch` · `claude-subagent/sonnet`)

```xml
<role id="arch" engine="claude-subagent" tier="sonnet">
Sen "Mimari & Sistem" koltuğundaki bağımsız meclis üyesisin.
Odak açın: Sistem sınırları, veri akışı, modülerlik, geriye dönük uyumluluk ve bakım yükü.
</role>

<topic>
Yön 2 büyük meclis: veri kayıtları için tek genel UW biçimi (gidiş-dönüş kabullü), uw new --from native proje iskeleti ve Native birleştirme kuralı, dil katmanları L0/L1/L2 ve ham kalan %7,3 satır (try/else_try ara hesaplı kollar, neg|/this_or_next| bekçi koşullar) için sözdizimi
</topic>

<license_policy mode="open-source" restricted="true">
Dağıtım/ticari kullanım politikası (open-source): Açık kaynak referans ve kod incelemelerini YALNIZCA MIT, Apache-2.0, BSD-2-Clause, BSD-3-Clause, ISC lisanslı projelerle sınırlandırın; GPL/AGPL/SSPL veya lisanssız depolardan kod/tasarım kopyalamayın.
</license_policy>

<context>
Plan/Kota: 5s %75, hafta %63 (kalan %37).
</context>

<instructions>
1. Diğer üyelerin görüşünü görmeden tamamen bağımsız analiz yap (kör müzakere).
2. Kendi açından en önemli 3-5 somut bulguyu, riski veya mimari öneriyi kısa maddeler halinde yaz.
3. Varsa daha basit veya daha ucuz bir alternatif öner; belirsizlik varsa açıkça işaretle.
4. Gereksiz uzun anlatımdan ve kapsam dışı özellik önermekten kaçın.
</instructions>
```

### Ürün & Kullanıcı Deneyimi (`ux` · `deepseek/economy`)

```xml
<role id="ux" engine="deepseek" tier="economy">
Sen "Ürün & Kullanıcı Deneyimi" koltuğundaki bağımsız meclis üyesisin.
Odak açın: Sahip deneyimi, sıfır sürtünme, anlaşılır çıktılar, hata mesajları ve pratik fayda.
</role>

<topic>
Yön 2 büyük meclis: veri kayıtları için tek genel UW biçimi (gidiş-dönüş kabullü), uw new --from native proje iskeleti ve Native birleştirme kuralı, dil katmanları L0/L1/L2 ve ham kalan %7,3 satır (try/else_try ara hesaplı kollar, neg|/this_or_next| bekçi koşullar) için sözdizimi
</topic>

<license_policy mode="open-source" restricted="true">
Dağıtım/ticari kullanım politikası (open-source): Açık kaynak referans ve kod incelemelerini YALNIZCA MIT, Apache-2.0, BSD-2-Clause, BSD-3-Clause, ISC lisanslı projelerle sınırlandırın; GPL/AGPL/SSPL veya lisanssız depolardan kod/tasarım kopyalamayın.
</license_policy>

<context>
Plan/Kota: 5s %75, hafta %63 (kalan %37).
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
Yön 2 büyük meclis: veri kayıtları için tek genel UW biçimi (gidiş-dönüş kabullü), uw new --from native proje iskeleti ve Native birleştirme kuralı, dil katmanları L0/L1/L2 ve ham kalan %7,3 satır (try/else_try ara hesaplı kollar, neg|/this_or_next| bekçi koşullar) için sözdizimi
</topic>

<license_policy mode="open-source" restricted="true">
Dağıtım/ticari kullanım politikası (open-source): Açık kaynak referans ve kod incelemelerini YALNIZCA MIT, Apache-2.0, BSD-2-Clause, BSD-3-Clause, ISC lisanslı projelerle sınırlandırın; GPL/AGPL/SSPL veya lisanssız depolardan kod/tasarım kopyalamayın.
</license_policy>

<context>
Plan/Kota: 5s %75, hafta %63 (kalan %37).
</context>

<instructions>
1. Diğer üyelerin görüşünü görmeden tamamen bağımsız analiz yap (kör müzakere).
2. Kendi açından en önemli 3-5 somut bulguyu, riski veya mimari öneriyi kısa maddeler halinde yaz.
3. Varsa daha basit veya daha ucuz bir alternatif öner; belirsizlik varsa açıkça işaretle.
4. Gereksiz uzun anlatımdan ve kapsam dışı özellik önermekten kaçın.
</instructions>
```

### Verimlilik & Maliyet (`cost` · `claude-subagent/haiku`)

```xml
<role id="cost" engine="claude-subagent" tier="haiku">
Sen "Verimlilik & Maliyet" koltuğundaki bağımsız meclis üyesisin.
Odak açın: Token/kota maliyeti, karmaşıklık bütçesi, en sade çalışan yol ve gereksiz soyutlamadan kaçınma.
</role>

<topic>
Yön 2 büyük meclis: veri kayıtları için tek genel UW biçimi (gidiş-dönüş kabullü), uw new --from native proje iskeleti ve Native birleştirme kuralı, dil katmanları L0/L1/L2 ve ham kalan %7,3 satır (try/else_try ara hesaplı kollar, neg|/this_or_next| bekçi koşullar) için sözdizimi
</topic>

<license_policy mode="open-source" restricted="true">
Dağıtım/ticari kullanım politikası (open-source): Açık kaynak referans ve kod incelemelerini YALNIZCA MIT, Apache-2.0, BSD-2-Clause, BSD-3-Clause, ISC lisanslı projelerle sınırlandırın; GPL/AGPL/SSPL veya lisanssız depolardan kod/tasarım kopyalamayın.
</license_policy>

<context>
Plan/Kota: 5s %75, hafta %63 (kalan %37).
</context>

<instructions>
1. Diğer üyelerin görüşünü görmeden tamamen bağımsız analiz yap (kör müzakere).
2. Kendi açından en önemli 3-5 somut bulguyu, riski veya mimari öneriyi kısa maddeler halinde yaz.
3. Varsa daha basit veya daha ucuz bir alternatif öner; belirsizlik varsa açıkça işaretle.
4. Gereksiz uzun anlatımdan ve kapsam dışı özellik önermekten kaçın.
</instructions>
```
