# Sorular (ajan yazar, sahibi cevaplar; cevap beklenmez)

Ajan bir kararda emin değilse burada sorar ve **güvenli varsayılanla** yoluna devam eder. Sahibi cevabını sorunun altına yazar; bir sonraki çevrim okur, uygular ve soruyu "Kapananlar"a taşır.

## Açık

<!-- Biçim:
### 2026-01-15 · <kısa soru>
Neden soruyorum: ... Şimdilik yaptığım (güvenli varsayılan): ... Seçenekler: A) ... B) ...
**Cevap:** (sahibi buraya yazar)
-->

## Kapananlar

### 2026-11-15 · Native'in kodu UW'ye çevrilsin mi? → **A (olduğu gibi kalsın, "Native + fark")**
Sahibi: "A olsun, aşırı iş yüküne gerek yok, gücümüzü daha önemli yerlere harcayalım." Meclis kararı onaylandı.

### 2026-11-14 · `base: empty` ne kadar boş? → **Tek seçenek değil, taban ön ayarları**
Sahibi: "İkisi de olsun, hatta daha fazlası." İşlenişi: `base` tek bir sabit değil, genişletilebilir ön ayar listesi (`native`, `native-map-empty` = Native haritası içi boş, `empty-flat` = düz zemin + tek parti, ileride `custom-map`). Her biri D8 deneyiyle kanıtlanınca açılır; Faz 1'de yalnız `native`.

### 2026-11-14 · Harita editörü? → **Yükseklik haritası + arazi boyama**
Sahibi: yükseklik haritası "muazzam olur"; deniz, göl, dağ, kayalık her şey belli olmalı; 3B modelleme bilmiyor; haritayı Claude da çizebilmeli. İşlenişi: `docs/bilgi/harita.md` (map.txt biçimi kitle/Native'le uyumlu okundu) + ROADMAP Faz 0b.

### 2026-11-15 · Meclis: sahibinin onayı gereken 5 karar — KAPANDI (5. soru yeniden soruldu)
1. **Giriş kesme, silme yok:** Kabul. Sahibinin düzeltmesi: "Modlamak zaten sistemi istediğin gibi bükmek. Ama hâlâ benim farazi hikâyeme göre yapıyorsun; prolog silici, sprint ekleyici gibi built-in fonksiyonlar yapmıyoruz, evrensel bir derleyici yapıyoruz; Module System için belli bir amacı yok, istenen her mod yapılabilmeli." → Uygulandı: `disable native.prologue` gibi amaca özel kısayollar kaldırıldı; yerine genel mekanizma (herhangi bir Native girdisini `override`/`disable` etmek). praven_start yalnız kabul testi, dilin özelliği değil (KARARNAMELER).
2. **Oyunu açmak:** "Nihai doğrulama olmamalı, oyunda küçük sessiz hatayı göremezsin; ama genel olarak oyuna girip bakmak küçük şeyler için doğrulama sayılabilir." → D4-D7 oyun sondaları onaylandı; oyunda bakmak tamamlayıcı kanıt, otomatik testlerin yerini tutmaz (KARARNAMELER güncellendi).
3. **Önce `base: native`:** "Evet." → Faz 1 yalnız `base: native`; `base: empty` D8 sonrası.
4. **Python 2.7:** "Şu an gerek yok, ileriki sürümlerde ekleriz. Bütün iş yükümüz mimariyi doğru ve kapsamlı kurmak; topluluk işleri en son." → Python 2.7 gerekliliği şimdilik kabul; kurulum kolaylığı/Python'dan kurtulma sonraki sürümlere.
5. Sahibi anlamadı → sade anlatımla "Açık"ta yeniden soruldu.

- [2026-11-15] Döngü SIRA_DEGIL diyor: ~/.claudevolution/projects.json içinde paketin testlerinden kalmış iki sahte "proj" kaydı (AppData/Local/Temp/cv-qm-test-*) sırayı tutuyor. Silme izni otomatik denetime takıldı. Sahibi silebilir ya da kayıtlar 2026-11-16 ~15:00 (TR) kendiliğinden düşer.
