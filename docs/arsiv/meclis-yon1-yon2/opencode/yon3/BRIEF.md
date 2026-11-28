# BRIEF — her koltuğun okuduğu ortak metin

> Bu metni okumadan hiçbir koltuk işe başlamaz. Çelişki görürsen **bu metin** kazanır;
> çelişkiyi moderatöre bildir.

## 0. Görev tek cümle

**Hangi genel mekanizmalar, tek tek alan eklemeden, "modcu her şeyi yapabilir" hedefini sağlar —
ve bunu sistematik olarak nasıl ölçeriz?**

Senin işin bu soruya **mekanizma düzeyinde** cevap vermek. Özellik listesi vermek değil.

## 1. Hastalık ve semptom (bunu yanlış anlarsan bütün iş çöp)

**Hastalık:** Yön 2, "Native'i UW'ye çevir, üstüne yama yaz" çerçevesindeydi ve hâlâ "alan alan"
düşünüyordu: şu tablo, şu alan, şu kayıt. Her istek geldiğinde bir alan/özellik eklendi, o örnek
bitince "bitti" dendi. Oysa biten tek şey o örnekti. Sistem **özellik üretiyordu, kendisi üretmiyordu**.

**Semptom:** `owner vaegirs` yazıldı, dosyalar doğru üretildi, testler ve bayt karşılaştırması yeşildi —
ama oyunda Praven hâlâ Swadia'ydı. Çünkü Native bir merkezi tek alanla vermiyor: önce krallığa, sonra
bir lora veriyor. Praven **hedef değil, kanıt**.

**İstenen:** özellik fabrikası değil, **her şeyi yapabilme yeteneği**:

- modcu isterse Native'i yamalar,
- isterse boş dünyada kendi haritasını kurar,
- isterse bütün krallıkları/faction'ları silip sıfırdan krallık, lord, leydi, yoldaş yaratır,
- isterse bambaşka bir oyun yapar (örnek: bir modcu Module System ile CS 1.6 klonu yapmış).

**Sınır:** Modcunun isteyeceği şeyin sınırı teorik olarak yok. "Tek tek bakarsan sonsuza kadar sürer."
Bu yüzden örnek listesi yapmak yanlış: liste biter, ihtimal bitmez.

## 2. Sahibinin cümleleri (kural gücünde)

- "İster yama yapsın, ister bomboş dünyada kendi mapini yapsın, isterse tüm factionları krallıkları
  silsin sıfırdan kendisi krallıklar lordlar leydiler yoldaşlar yapsın."
- "Tek tek bakarsan sonsuza kadar sürer; o yüzden örnek vermek istemiyorum, çünkü ona takılıyorsun,
  onu görev gibi alıyorsun; o tek örneği yapınca 'her şeyi bitirdim' diyorsun."
- "ASLA şunu dememeliler: 'hmm stealth yapılamıyor, o zaman bir stealth modülü yazayım.' Bu yanlış
  düşünce, çünkü yine 'verilen örneği amaç sanmak' oluyor."
- "Bizim amacımız modcunun istediği şeyi istediği gibi yapabilmesi. Modcunun aklındakini hayal edemeyiz;
  biz onun istediğini yapabileceği bir yer kurmalıyız."

## 3. "Genel" ne demek: kapalılık

Bir sistem, **yarın beklenmedik bir istek geldiğinde derleyiciye yeni bir alan/özellik/modül
eklemeden** o isteği mevcut mekanizmaların bileşimiyle ifade edebiliyorsa geneldir.

- Her yeni istek "yeni bir alan" gerektiriyorsa → sistem hâlâ özellik üretiyor (hastalık sürüyor).
- Yeni istek mevcut ilkellerle ifade edilebiliyorsa → sistem üretilmiş.

Bu yüzden tasarımının sonunda şu cümleyi kurabilmelisin: *"Modcu X isterse, şu ilkelleri şöyle
birleştirir; derleyiciye dokunmaya gerek yok."* Kuramıyorsan eksik olan **genel ilkel**i söyle
(eksik *özellik* değil).

## 4. Yasak çıktılar (otomatik ret)

1. **"Şu özellik/alan/komut/modül eklensin."** Örnek adı geçen her çözüm reddedilir.
2. **Örnek = bitti ölçüsü.** "Praven düzelince biter", "stealth yapılınca biter" cümleleri yasak.
3. **Varlık envanterini yol haritası sanmak.** "Krallık, yerleşim, karakter için şu alanlar" listesi
   kapsam değildir; envanter yalnızca mekanizmayı **sınamak** için kullanılır.
4. **Ad uydurma.** Kitte olmayan hiçbir Native adı (menü, diyalog, kişi, işlem, slot) yazılmaz.
5. **Kanıtsız kesinlik.** Doğrulamadığın cümlenin sonuna `[İDDİA]` yaz.
6. **Ürün kodu / test / kit değiştirmek.** Yasak. Yalnız `.md` yazarsın.

## 5. Kanıt kuralı

- Native hakkında her iddia: `Module_system 1.171/<dosya>:<satır>` ya da **tekrarlanabilir komut**.
- Kit dışı (mod davranışı) iddialar: kaynağı yaz; kaynak yoksa `[İDDİA]`.
- Sayı (test sayısı, satır, oran) **ezberden yazılmaz**; komut çıktısından alınır.
- Bir mekaniğin "yapılabilir" olduğunu söylemek için: hangi UW ilkeliyle, hangi Native yapısına
  dayanarak yapıldığını yaz.

## 6. Yazma kuralları

- Kendi çıktı dosyana yaz; başka koltuğun dosyasını **değiştirme**.
- En fazla ~120 satır. Uzun ve süslü metin değil, yoğun ve denetlenebilir metin.
- Her dosya şu iskeletle başlar:

```md
# <Koltuk> — <konu>
> Kanıt: <okuduğun kit dosyaları / çalıştırdığın komutlar>
> Beyan: kapalı malzemeyi (L&D mekanik dosyaları, `gizli/`) okumadım — evet

## 1. Teşhis (2-5 cümle)
## 2. Öneri — mekanizma düzeyinde
## 3. Nasıl çalışır (modcu ne yazar, derleyici ne üretir)
## 4. Neyi ifade EDEMEZ (dürüst sınır) + eksik genel ilkel
## 5. Kanıt / [İDDİA] ayrımı
## 6. Karşı çıktığın yaygın varsayım
```

## 7. Tek kısıtlama: sınav kâğıdı (senin fikrin)

**Okuma sınırı yok.** Kit, derleyici kodu, atlas, şemalar, 6 örnek mod, eski meclis dosyaları, L&D raporunun
README'si — hepsini okuyabilirsin, okumalısın da. Bir sistemi Module System'i görmeden planlamak saçma;
kit'i açmadan yazılan her Native iddiası zaten `[İDDİA]`dır.

Tek istisna, sahibinin kendi kapı fikri: **sınav kâğıdı Kapı 1'e kadar B koltuklarına kapalıdır** —
yani `gizli/` (analizcilerin çıktısı) ve `light-and-darkness-report/mekanik-01..11.md` (sınavın kaynağı).
Sebep: tasarımcı mekanik listesini önceden görürse "stealth için şu alan" demeye başlar — yani
örneği amaç sanar. Mekanikler tasarımdan **sonra** sınav olarak gelir.

- A koltukları bulgularını `gizli/` altına yazar, tasarım turunda **kimseye söylemez**.
- B koltukları `gizli/` klasörünü ve `mekanik-01..11.md` dosyalarını **açmaz**.
- Körlük talimatla korunur, dosya sistemi kilidi yoktur: her koltuk çıktısının başında **ne okuduğunu**
  yazar; moderatör tasarım dosyalarında L&D mekanik adı arar. Tek sızma → o koltuğun çıktısı geçersiz.
- Tur 1'de kimse başka koltuğun taslağını okumaz (bağımsızlık için). Tur 2'de herkes her şeyi okur.

## 8. Neyi okumalısın (hepsi serbest)

**Zorunlu (en az bir kez):** `Module_system 1.171/` — iddialarını bağlayacağın yer burası.
`docs/VISION.md`, `docs/ARCHITECTURE.md`, `docs/KNOWLEDGE.md`, `docs/meclis/DEVIR.md`.

**Serbest ve teşvik edilir:**
- `crates/` — derleyicinin bugünkü kodu (ne var, ne yok, nerede sertleşmiş).
- `kit-meta/`, atlas çıktıları, şemalar — bugünkü kayıt modeli.
- `docs/arsiv/meclis-yon1-yon2/kurucu-model-gundem.md` — kanıt/ilham; §4 envanteri **kapsam değildir**.
- `ornek-modlar/` — 6 mod (1257AD, catplomacy, Diplado, medieval-conquests, Persistent-Kingdoms,
  pw_module_system). Okumak serbest; **kod kopyalamak yasak** (lisans + kural).
- `ornek mod mekanik arastirmasi (stres testi)/light-and-darkness-report/README.md` — kapsam, sınırlar ve
  okuma kuralı; herkes okur.
- `docs/arsiv/meclis-yon1-yon2/opencode/t116-lord/` — Praven iki adım kanıtı (konu arşiv, kanıt değerli).
- Önceki meclis dosyaları (`KARAR.md`, `tasarim-S1/S2.md`, `kanit-H1..H3.md`) — geçmiş kararlar.
- `docs/arsiv/meclis-yon1-yon2/opencode/yon3/` içindeki diğer dosyalar — bu meclisin kendi belgeleri.

**Kapalı — yalnız "sınav kâğıdı" (B koltukları için, Kapı 1'e kadar):**
`light-and-darkness-report/mekanik-01..11.md` ve `gizli/` (A koltuklarının analizi).
Bu bir "anlamama sınırı" değil, **sınav kuralıdır**: sınav kâğıdı sınavdan önce gösterilmez.
Kapı 1'den sonra hepsi açılır ve tartışılır. (Kapı 1'den önce okursan çıktın geçersiz sayılır.)

## 9. Meclisin çıkacağı yer (bitti ölçüsü)

1. **Kapsama:** Module System'in tablo/alan/işlem/kanca düzeyinde "UW'de ifade edilebilir mi" oranı —
   otomatik üretilebilir bir tablo.
2. **Stres:** (a) L&D mekanikleri, (b) 6 örnek mod, (c) uç hedef: CS 1.6 benzeri bir total conversion.
   Bunlar **hedef değil sınav**; puanları "kaç mekanik ifade edildi", "kaç yeni ilkel gerekti".
3. **Kapalılık:** görülmemiş istek → derleyici değişmeden ifade edilebiliyor mu?
4. **Yasak ölçü:** "Praven düzeldi", "stealth oldu", "şu özellik bitti".
