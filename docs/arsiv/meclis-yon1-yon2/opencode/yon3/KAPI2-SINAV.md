# KAPI 2 SINAVI — görülmemiş istekler (kapalılık testi)

> Amaç: **bilinen** mekanikleri değil, **hiç konuşulmamış** istekleri sınamak.
> Kural: tasarımcı, isteği **derleyiciye hiçbir şey eklemeden** mevcut ilkellerle ifade eder.
> "Şu özellik/alan/modül eklensin" cevabı **ret**; sonuç yalnız "eksik **genel ilkel**" olabilir.

## İstekler (moderatör seçti; meclis tartışmasında geçmedi)

**İ1 — Sadakat ve toplu geçiş.** Her lordun bir "sadakat" puanı olsun. Puan bir eşiğin altına
düşünce lord başka bir krallığa geçsin; lord geçerken **elindeki bütün merkezler de** onunla geçsin
ve o merkezlerin "özgün krallık" ve kültür kayıtları tutarlı kalsın.

**İ2 — Yağma sonrası savaş ve gelir kesintisi.** Oyuncu bir köyü yağmaladığında o köyün krallığı
oyuncuya savaş açsın ve köyün geliri **3 gün** boyunca sıfır olsun; 3 gün sonra kendiliğinden düzelsin.

**İ3 — Sahibine göre değişen yerleşim adı.** Bir yerleşimin oyunda görünen adı, onu elinde tutan
krallığa göre değişsin; oyuncu kaydedip yükledikten sonra da doğru ad görünsün.

## Cevap biçimi (her istek için)

1. **Modcunun yazacağı satırlar** (uydurma Native adı yok; yeni sözdizimi öneriyorsa açıkça "öneri" yaz).
2. **Derleyicinin üreteceği adımlar** (hangi Native yapısı; `dosya:satır` ya da `[İDDİA]`).
3. **Kullandığı mevcut ilkeller** (çekirdeğin hangileri).
4. **Gizli adım var mı** — modcunun tek satırının tetiklediği, görünmeyen Native adımları; yapı
   zinciri/etki raporu bunu kapsıyor mu?
5. **Sonuç:** `ifade edildi` / `kısmen` / `edilemedi` + edilemediyse **eksik genel ilkel** (özellik değil).

## Geçme ölçüsü

- 3 isteğin **≥ 2'si** yalnız mevcut ilkellerle ifade edilirse kapalılık iddiası ayakta.
- Kalan 1 istek **tek bir genel ilkel** eksiğiyle açıklanırsa kabul.
- Herhangi bir istek "şu özellik eklensin" ile çözülüyorsa → **kapalılık yok**; sentezde açıkça yazılır.
