# E1 — Şeytanın avukatı: meclisin kendisini vurur

> Kanıt: okuduğum dosyalar: BRIEF.md, README.md, PLAN.md, KOLTUKLAR.md, KAPI-PROTOKOLU.md, SINAV-SETI.md, docs/arsiv/meclis-yon1-yon2/KARAR.md, docs/arsiv/meclis-yon1-yon2/kurucu-model-gundem.md. Kit dosyası okumadım (yöntem eleştirisi; Native iddiası yok). Komut çalıştırılmadı.
> Beyan: sınav kâğıdını (gizli/ + mekanik-01..11) okumadım — evet

## 1. Teşhis

Meclis kendi hastalığını teşhis etti ("özellik fabrikası") ama yöntemi hâlâ örnek odaklı: merkezde Praven, sınav setinde L&D 11 mekanik, uç hedef CS 1.6. "Genellik" tanımı döngüsel ve ölçülemez; körlük talimatla korunuyor, dosya sistemi kilidi yok; bitti kriteri moderatörün seçtiği 3 isteğe ve keyfi eşiklere dayanıyor. Metodoloji iddiaları kanıt kuralının kapsamı dışında.

## 2. Öneri — mekanizma düzeyinde

Yöntemi kanıta bağla: körlüğü yapısal izolasyona çevir, genelliği adversarial karşı-örnek kümesiyle ölç, bitti kriterini sahibinin onayına bağla. (Bu tur tasarım yok; öneri metodoloji düzeyinde.)

## 3. Nasıl çalışır

N/A — bu tur tasarım üretmiyor; meclis yöntemi eleştiriliyor.

## 4. Neyi ifade EDEMEZ (eleştiriler)

**E1 — Örnek odaklı kayma (README.md:16-25, KOLTUKLAR B bloğu).** Cümle: "Praven (owner değişti, oyunda Swadia kaldı) bu hastalığın semptomu". Yasak çıktı listesi (BRIEF §4.2) "Praven düzeldi = bitti"yi yasaklar ama merkezde Praven tutulur; B koltukları "Praven kanıtı" okuyabilir. Neden yanlış: yazılı yasak davranış değiştirmez, yapısal risk devam. Ne yapılmalı: Praven'i koltuk talimatlarından çıkar; kanıt olarak KARAR.md'de kalsın.

**E2 — Körlük sızıntısı (SINAV-SETI §4 tablosu + kurucu-model-gundem.md).** B koltukları okuma listesinde kit, 6 mod, eski meclis dosyaları var; kurucu-model-gundem.md §2/§4 Praven iki adım, owner vaegirs, lord auto içerir. B koltukları bu dosyayı okuyarak mekanikleri dolaylı görür. Neden yanlış: körlük talimatla korunuyor, dosya sistemi kilidi yok. Ne yapılmalı: B koltukları okuma listesinden kurucu-model-gundem.md §2/§4 çıkar veya "kapalı" işaretle.

**E3 — Körlük denetimi kanıtsız (KAPI-PROTOKOLU §2).** "Tek sızma → geçersiz" ama kanıt mekanizması yok: moderatör metin taramasıyla mekanik adı arar. Tasarımcı mekanik adını kullanmadan aynı içeriği ifade edebilir → sızma tespit edilemez. Ne yapılmalı: körlüğü yapısal izolasyona çevir; A çıktısı ayrı dizinde, B koltuklarına verilmez; erişim moderatör dışında kilitli.

**E4 — Kapı 2 "görülmemiş istek" çelişkisi (KAPI-PROTOKOLU §4.1 vs SINAV-SETI §2).** B koltukları 6 örnek modu okumuş (SINAV-SETI §2: herkes okuyabilir). Test istekleri bu modlardan seçilecek → istekler "görülmemiş" değil. Protokol "tasarımcıların hiç görmediği kaynaklardan 3 istek seçer" der. Ne yapılmalı: istek havuzu tasarımcıların okumadığı kaynaklardan veya moderatörün ürettiği gerçekçi isteklerden; mod okuma izni test sonrası.

**E5 — "Genellik" tanımı döngüsel (BRIEF §3).** Genel = görülmemiş istek ifade edilebilir; test moderatör seçer; seçim moderatörün hayal gücünü ölçer, modcununkini değil. BRIEF §2 "Modcunun aklındakini hayal edemeyiz" ile çelişir. Ne yapılmalı: genellik tanımına adversarial karşı-örnek kümesi ekle; modcu topluluğundan gerçek istek topla (Kapı 2 sonrası, sahibi onaylı).

**E6 — Keyfi eşikler (KAPI-PROTOKOLU §3).** Kapı 1 geçme ölçüsü %70 A/B; "aynı mekanik için 3+ D → genel değil". Eşikler moderatör önerisi, "meclis tartışabilir". Kanıt yok. Ne yapılmalı: eşikleri kanıta bağla veya "ölçülemez" işaretle; aksi halde sahte kesinlik.

**E7 — "Otomatik" ölçüm kanıtsız (BRIEF §9.1, KOLTUKLAR D1).** Kapsama tablosu "otomatik üretilebilir" ama C1/D1 henüz yok; D1 "ölçümün kendisi özellik eklemeyi gerektirmemeli" ama satır üretme kuralı elle. Ne yapılmalı: otomatik ölçüm kanıtlanana dek "manuel, geçici" işaretle.

**E8 — Metodoloji iddiaları kanıt kuralı dışında (BRIEF §5).** Kanıt kuralı Native için (dosya:satır). "Körlük işe yarar", "genellik ölçülebilir", "tek sızma → geçersiz" kanıtsız metodoloji iddiaları. Ne yapılmalı: metodoloji iddialarına da [İDDİA] etiketi ve doğrulama yolu.

**E9 — Sorulmayan sorular.** Modcu gerçekten "her şeyi" ister mi, iyi varsayılan + az kaldıraç mı? (E2 joker tek koltuğa bırakılmış, en sona.) Kapalılık derleyici düzeyinde mi runtime düzeyinde mi? (B4 ham katman motor sınırını söyler ama runtime davranış derleyici ifade edemez.) Öğrenme eğrisi/maliyet ölçümü yok. Geriye uyumluluk (yeni ilkel eklenince eski projeler) ölçümü yok. Veri kaybı riski (KARAR §7 kural 1) meclis ölçmüyor.

**E10 — Meta-eleştiri tek koltuğa bırakılmış (PLAN §7, KOLTUKLAR E1).** E1 tur 1'de meclisin kendisini vuruyor; tur 2'de B/C/D'yi vuracak. E1 çıktısı senteze (tur 5) girmiyor. Risk: eleştiriler görmezden gelinir. Ne yapılmalı: E1 çıktısı Kapı 1 öncesi moderatör tarafından okunmalı; KARAR-TASLAK'ta "reddedilen eleştiri" bölümü olmalı.

## 5. Kanıt / [İDDİA] ayrımı

Yukarıdaki eleştiriler metodoloji iddiasıdır; Native iddiası yok. [İDDİA]: körlük sızıntısının gerçekten meydana geldiği (B koltukları tur 1'de kurucu-model-gundem.md okudu mu) henüz kanıtlanmadı — okuma listesinde var, okunduğu kanıtlanmadı. [İDDİA]: Kapı 2 isteklerinin gerçekten modlardan seçileceği — protokolde yazılı, uygulanmadı.

## 6. Karşı çıktığın yaygın varsayım

"Örnek = hedef değil" kuralı yazıldığı için meclis örnek odaklı kaymadı varsayımı. Yazılı kural davranış değiştirmez; merkezde Praven ve L&D mekanikleri var.