# Sorular (ajan yazar, sahibi cevaplar; cevap beklenmez)

Ajan bir kararda emin değilse burada sorar ve **güvenli varsayılanla** yoluna devam eder. Sahibi cevabını sorunun altına yazar; bir sonraki çevrim okur, uygular ve soruyu "Kapananlar"a taşır.

## Açık

### 2026-11-15 · (5. soru, sade anlatım) Native'in kodu UW'ye çevrilsin mi, olduğu gibi mi kalsın?
Neden soruyorum: Sahibi ilk sorulduğunda "anlamadım" dedi; sade hali şu. Native Warband'ın kendi kodu (lordlar, ekonomi, karakter yaratma… on binlerce satır) bugün Python dosyalarında duruyor. İki yol var:
- A) **Olduğu gibi bırak** (meclisin seçimi). Sen yalnız *kendi değişikliklerini* UW ile yazarsın; UW Native'in dosyasına dokunmaz, yanına "şunu değiştir, şunu ekle" diye kısa bir fark dosyası koyar. Native'in içini merak edersen `uw explain lord_ai` gibi bir komut "bu nerede tanımlı, kim kullanıyor, kaynağı şu satır" diye gösterir. Artısı: güvenli ve hızlı kurulur, Native'i bozma riski en düşük. Eksisi: Native'in kendi kodu yine eski, zor okunan Python olarak kalır.
- B) **Native'in tamamını UW diline çevir.** Artısı: her şey tek dilde, Native'i de UW gibi okursun. Eksisi: on binlerce satırı hatasız çeviren ikinci bir derleyici yazmak gerekir; aylar sürer ve çeviri hatası Native'i sessizce bozabilir.
Şimdilik yaptığım (güvenli varsayılan): A ile başlamak; B'yi ileride yalnız "ölçmek için" (`uw lift`, ör. eşyalar gibi basit veriler) açık tutmak. A seçilse bile istediğin Native parçasını (ör. bir menüyü, bir script'i) UW'de baştan yazıp Native'inkinin yerine koyabilirsin; yasak olan yalnız toplu çeviri.
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

### 2026-11-15 · Meclis: sahibinin onayı gereken 5 karar — KAPANDI (5. soru yeniden soruldu)
1. **Giriş kesme, silme yok:** Kabul. Sahibinin düzeltmesi: "Modlamak zaten sistemi istediğin gibi bükmek. Ama hâlâ benim farazi hikâyeme göre yapıyorsun; prolog silici, sprint ekleyici gibi built-in fonksiyonlar yapmıyoruz, evrensel bir derleyici yapıyoruz; Module System için belli bir amacı yok, istenen her mod yapılabilmeli." → Uygulandı: `disable native.prologue` gibi amaca özel kısayollar kaldırıldı; yerine genel mekanizma (herhangi bir Native girdisini `override`/`disable` etmek). praven_start yalnız kabul testi, dilin özelliği değil (KARARNAMELER).
2. **Oyunu açmak:** "Nihai doğrulama olmamalı, oyunda küçük sessiz hatayı göremezsin; ama genel olarak oyuna girip bakmak küçük şeyler için doğrulama sayılabilir." → D4-D7 oyun sondaları onaylandı; oyunda bakmak tamamlayıcı kanıt, otomatik testlerin yerini tutmaz (KARARNAMELER güncellendi).
3. **Önce `base: native`:** "Evet." → Faz 1 yalnız `base: native`; `base: empty` D8 sonrası.
4. **Python 2.7:** "Şu an gerek yok, ileriki sürümlerde ekleriz. Bütün iş yükümüz mimariyi doğru ve kapsamlı kurmak; topluluk işleri en son." → Python 2.7 gerekliliği şimdilik kabul; kurulum kolaylığı/Python'dan kurtulma sonraki sürümlere.
5. Sahibi anlamadı → sade anlatımla "Açık"ta yeniden soruldu.
