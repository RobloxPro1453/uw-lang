# Universal Warband — meclis sonrası bağımsız inceleme paketi

**Hazırlayan:** Manus AI  
**Tarih:** 15 Kasım 2026  
**Kapsam:** Yüklenen `universal_warband-Kopya.zip`, içindeki vizyon/mimari/yol haritası/meclis belgeleri, Manus ve Perplexity notları ve erişilebilir teknik kaynaklar.

## Bu pakette

1. [Ana karar ve bağımsız değerlendirme](01-ana-karar-raporu.md) — meclis üyesi perspektifi, tasarım seçenekleri, karar gerekçesi, karşı savlar ve açık kapılar.
2. [Teknik risk ve kanıt matrisi](02-risk-ve-kanit-matrisi.md) — iddiaları kanıt sınıfı, etki, doğrulama ve çıkış ölçütüyle ayırır.
3. [Kapılı yürütme planı](03-kapili-yurutme-plani.md) — D1–D10 sırasını, bağımlılıkları, durma/geri dönüş koşullarını ve önerilen teslim ölçülerini belirler.
4. [Claude için bağımsız inceleme prompt’u](04-claude-inceleme-promptu.md) — sonucu baştan dikte etmeyen, karşı kanıt ve test tasarımı isteyen kısa prompt.
5. [Kaynak ve yöntem notu](05-kaynak-ve-yontem.md) — hangi kaynağın neyi desteklediğini ve sınırlarını açıklar.

## Tek cümlelik karar

**Koşullu devam:** `Native + fark` (değer düzeyinde ince wrapper/yama) yönünü yalnızca D1–D3 teknik kapıları geçerse küçük bir Praven dikey diliminde sürdür; D4 oyun davranışını ise sahibinin açık onayı olmadan test etme. Bu sonuç tam derleyici mimarisinin veya motor davranışının kanıtlandığı anlamına gelmez.

## Kanıt etiketleri

- **Projede kayıtlı:** proje/meclis belgesinde yazılı; bu etiket tek başına bağımsız doğrulama değildir.
- **Dış kaynaktan teyitli:** belirtilen dış kaynağın söylediği dar kapsamda teyit edildi.
- **Bu incelemede yeniden doğrulanmadı:** önceki kayıt var ama ilgili kit/oyun davranışı bu teslim turunda yeniden çalıştırılmadı.
- **Motor deneyi gerekir:** Warband 1.174 çalıştırması veya gerçek kit üzerinde kontrollü deney gerekir.
- **Önerilen karar:** kanıt değil; risk yönetimi için bu raporun tavsiyesi.

## Güven ilkesi

Bana, Claude’a, proje sahibine veya meclise yüzde yüz otorite muamelesi yapmayın. Her birimiz yanılabiliriz. Bu paket de kararın kendisi değil; iddia, kaynak, belirsizlik ve bir sonraki ölçülebilir deneyi görünür kılma aracıdır. Önce kanıt, sonra kesinleştirme.
