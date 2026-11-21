# Altyapı taraması — özet (dalga 1, 2026-11-21)

Kaynak: `warband-opensource-mods/mods/` altındaki 17 açık kaynak moddan 15'inin raporu (bu klasördeki
`NN-*.md`; 1257AD ve AI Chat raporu henüz yok). Her rapor, modun Module System'de bulamadığı için
**elle kurduğu genel altyapıyı** kanıt satırıyla listeler; burada her raporun "dilde hazır gelmeli"
ilk 5 maddesi sayıldı. Sayılar kabadır: bir modun ilk 5'inde anılan = 1. Kanıt satırları ayrı ayrı
kontrol edilmedi; denetim sırasında okunan örnekler doğru çıktı.

| # | Eksik genel olanak (dilde hazır gelmeli) | İlk 5'te anan mod | Modların elle yaptığı |
|---|---|---|---|
| 1 | **Nesnede adlandırılmış, tipli, kalıcı alan** (`town.wealth`), slot numarasını derleyici ayırır, kayıt uyumu dahil | 15/15 | elle numaralı slot tabloları, çakışma avı, "şu numaradan sonra N slot ayır" |
| 2 | **Gerçek koleksiyonlar**: liste/sözlük, ekle-sil-sırala-dolaş, kayda girer | 15/15 | kukla asker/parti + slot aritmetiği, "uzunluk slot 0'da" dizileri, paralel diziler |
| 3 | **Zaman**: "her N saat/saniye", "N sonra", cooldown, iptal edilebilir | 14/15 | tetik demeti + slotta zaman damgası + sayaç azaltma |
| 4 | **Bildirimsel arayüz/HUD**: bileşen, yerleşim, liste/kaydırma, değere bağlama, kendiliğinden yenileme | 13/15 | koordinatla overlay, globalde tutamaç, id karşılaştırma zinciri |
| 5 | **Metin değeri**: ara değer gömme (`"{name} {n} altın"`), yerelleştirme | 11/15 | s0..s9 / reg kaydırma, tek dev biçim dizgisi |
| 6 | **Girdi eylemleri**: basıldı / basılı / bırakıldı, adlı ve yeniden atanabilir tuş | 9/15 | kenar algılama durum makinesi, ham tuş numarası |
| 7 | **Olaylar**: adlı, tipli olay + işleyici (yerel ve ağ) | 9/15 | olay-id tabloları, global değişkenle argüman taşıma |
| 8 | **Modül/eklenti**: import, başkasının menü/tetik/sunumunu override + sarma, çakışma denetimi | 4/15 | Native dosyalarına ekleme, modmerger, elle id tabloları |
| 9 | **Ayar ekranı bildirimden doğar** (ad, tür, min/varsayılan/max, kayıt) | 4/15 | her ayar için elle sunum + global + kayıt |
| 10 | Ağ eşitleme (sunucu↔istemci alan kopyası) | 2/15 (MP modları) | alan başına bir gönderme çağrısı, elle olay numarası |
| 11 | Gerçek aritmetik (sabit nokta, üs, sıkıştırma) | 1/15 | tekrar eden mul/div |

## UW için anlamı

- İlk 7 madde "motor" katmanıdır: dilin kendisi olarak gelir, hiçbir mekaniğe özel değildir. Bu, VISION
  "2026-11-21 akşam" sınır kuralıyla birebir örtüşür: modcu fikri yazar, altyapı dilden gelir.
- 1-3 en temeldir ve birbirine bağlıdır: alan + koleksiyon + zaman, slot ve tetik katmanının üstüne
  derleyicinin kurduğu tek bir "durum" çekirdeği gibi tasarlanmalı.
- 8 (import/override/sarma) azınlıkta görünse de büyük modların hepsinde çıkıyor; `main.uw` + `import`
  kararıyla aynı iş.
- Ayrıntılı desenler ve konu konu inceleme: dalga 2 (`docs/bilgi/maden/`).
