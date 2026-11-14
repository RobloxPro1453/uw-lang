# Sorular (ajan yazar, sahibi cevaplar; cevap beklenmez)

Ajan bir kararda emin değilse burada sorar ve **güvenli varsayılanla** yoluna devam eder. Sahibi cevabını sorunun altına yazar; bir sonraki çevrim okur, uygular ve soruyu "Kapananlar"a taşır.

## Açık

### 2026-11-15 · Meclis: sahibinin onayı gereken 5 karar (`docs/meclis/KARAR.md` §9)
Şimdilik yaptığım (güvenli varsayılan): beşini de meclisin önerdiği gibi kabul edip Faz 0'ı buna göre sıraladım; oyunu açma (D4) senin "evet"in olmadan yapılmaz.
1. Native'in karakter yaratma ve prolog kodu Faz 1'de silinmiyor, yalnız girişleri kesiliyor (oyunda görünmez, dosyada durur). Uygun mu?
2. Faz 0'da bir kez oyunu açıp 3-4 şeye bakmak (karakter ekranı atlanıyor mu, Praven'de mi başlıyorsun, Türkçe harfler, log dosyası). "Oyunu açmak doğrulama değil" kuralına tek seferlik istisna; kabul mü?
3. İlk sürüm yalnız `base: native`; tamamen boş dünya (`base: empty`) deneyle kanıtlanınca Faz 1'den sonra. Uygun mu?
4. Derleme Python 2.7 gerektiriyor. Modculara "Python 2.7 kur" demek kabul mü, yoksa ileride kurulumu `uw` mu halletsin?
5. Üretilen dosyalar "Native + fark": Native'in kendi kodu UW'ye çevrilmiyor, içi eskisi kadar zor okunur kalır; ona `uw explain` ile bakarsın. Kabul mü?
**Cevap:**

### 2026-11-14 · `base: empty` gerçekten ne kadar boş olabilir?
Neden soruyorum: Motor bazı kayıtları sırayla bekliyor (ana parti, oyuncu birliği, ilk menü, bazı scriptler). Şimdilik yaptığım: Faz 0'da kitten "hardwired" listesini çıkarıp en küçük iskeleti deneyle bulmak. Seçenekler: A) tamamen düz zemin + tek parti, B) Native haritası ama içi boş (şehir/lord yok).
**Cevap:**

### 2026-11-14 · Topluluk harita editörü hangisi?
Neden soruyorum: `import_map` fikri için arazi dosyasını üretecek araç lazım; lisansı/kullanılabilirliği araştırılacak. Şimdilik: Faz 2'ye kadar Native haritası.
**Cevap:**

<!-- Biçim:
### 2026-01-15 · <kısa soru>
Neden soruyorum: ... Şimdilik yaptığım (güvenli varsayılan): ... Seçenekler: A) ... B) ...
**Cevap:** (sahibi buraya yazar)
-->

## Kapananlar
