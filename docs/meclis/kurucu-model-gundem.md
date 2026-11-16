# Meclis gündemi: Kurucu model — "sonsuz ihtimal"

Toplanma: haftalık kota yenilenince (21 Kasım 2026'dan sonra), uzun ve tam meclis. Sahibi o güne kadar yeni iş
başlatmıyor. Bu belge meclisin tek girdi dosyasıdır; kararlar `docs/meclis/` altına KARAR belgesi olarak,
özetleri VISION/ROADMAP/KARARNAMELER'e yazılır.

## 0. ÇERÇEVE — önce bunu oku (sahibi, 2026-11-16 gece, itirazla)

"Hâlâ çok spesifik şeylere takılıyorsun." Sahibi örnek vermek istemiyor, çünkü örnek görev gibi alınıyor:
o tek örnek yapılınca "her şey bitti" deniyor, oysa biten bir şey yok. Modcunun yapmak isteyeceği şeyin sınırı
yok (en azından teoride): Native'e yama da yapabilir, bomboş dünyada kendi haritasını da kurabilir, bütün
krallıkları silip sıfırdan krallık, lord, leydi ve yoldaş da yapabilir… **"Tek tek bakarsan sonsuza kadar sürer."**

Bu yüzden meclisin işi bir özellik listesi değil, **genel bir sistem**:
- Aşağıdaki §2 (Praven) ve §4 (envanter) **yalnız kanıt ve ilhamdır, kapsam değildir**. Bitti ölçüsü olarak
  kullanılmaz.
- Soru "şu alanı nasıl ekleriz" değil: "modcu Module System'in yapabildiği **her şeyi**, sıfırdan ya da yama
  olarak, Native bilmeden yapabiliyor mu ve bunu **sistematik** olarak nasıl garanti ve ölçeriz?"
- Tasarım tek tek durumlarla değil, genel mekanizmalarla cevaplanmalı. Örneğin: her kayıt türü için
  oluştur/değiştir/sil; Native'in kayıtlar arası bağları (oyun başı adımları dahil) için genel bir "başlangıç
  durumu" modeli; her kararda `auto`; her yerde ham kaçış kapısı, böylece hiçbir şey imkânsız olmaz.
- Bitti ölçüsü de genel olmalı: kapsama oranı (Module System'in ne kadarı ifade edilebiliyor), çok farklı
  modlarla stres testi (proje kökündeki `ornek mod mekanik arastirmasi (stres testi)/`), "her şeyi sil,
  sıfırdan kur" denemesi. Tek bir örnek kabul ölçüsü olamaz.

## 1. Sahibinin sözü (2026-11-16, oyun denemesinden sonra; aynen özet)

- "Sıfırdan birisi base açtığını düşün: şehir koydu bir yere, faction'ını kendi oluşturduğu faction'a bağladı,
  lordu da kendisi eklemek ister, kültürünü, lordunu vb. kendi belirlemek ister."
- "Yeni karakter de ekleyebilmek lazım: lord, leydi ya da yoldaş olarak; ismini, cismini, envanterini,
  statlarını, yüzünü, nerede dolaşır, nerede bulunabilir, ne sever… her şeyi ayarlayabilmesi lazım. Bu
  saydıklarımdan bile fazla şey var."
- "'Bilgi kusan' bir şey olmaması lazım. Bunların hepsini modcu kendisi ekleyebilmeli. İsterse faction'a
  lordları tanımlayacak, bir şehre de `auto` diyecek, Warband'ın kendi AI sistemi yapsın diyecek; belki kendi
  ekleyecek. Sonsuz ihtimal var, sonsuz ihtimali gerçekleştirebilecek sistem lazım."
- "Bu `lord` şeyi gerekliydi zaten, niye önceden yapmadık?" → Cevap (Claude): şimdiye kadar "Native'i al,
  var olanı değiştir" açısından tasarlandı; "sıfırdan kuran modcu" ölçü yapılmadı.

VISION.md (2026-11-16 maddesi) ve ROADMAP en üst maddesi "KURUCU MODEL" bu sözle yazıldı.

## 2. Kanıt: neden şimdiki model yetmedi (Praven deneyi, 2026-11-16)

- T-103 `owner vaegirs` → derlenen betik doğru: `give_center_to_faction_aux p_town_6 fac_kingdom_2`.
- Oyunda Praven turuncu (Swadia), Kral Harlaus'un. Sebep: Native `game_start` şehirleri **iki adımda** verir:
  1. `script_give_center_to_faction_aux` (Module_system 1.171/module_scripts.py:486-563)
  2. `script_give_center_to_lord` (aynı dosya :567-596) → şehir lordun krallığına geçer.
  3. Ardından `script_assign_lords_to_empty_centers` (:598) lordsuz merkezlere kendi krallığından lord verir.
  4. Sonra her merkezin "özgün krallığı" ve kültürü o anki krallıktan kaydedilir (:601-606:
     `store_faction_of_party` → `slot_center_original_faction`, `slot_faction_culture`).
- Ders: tek bir alan (sahip) Native'de birden çok gizli adıma dağılmış. Byte testleri bunu göremez; oyun
  denemesi gördü. Her kurucu alan için **gizli adım haritası** ve **oyun deneyi** şart.

## 3. İlke önerisi (meclis onaylasın/değiştirsin)

1. Ölçü sıfırdan kuran modcu: her varlık (krallık, yerleşim, karakter) boş tabandan tek dosyada tanımlanabilir.
2. Her karar ya modcunun ya açıkça `auto` (= Native'in kendi düzeni/AI'ı). Varsayılan `auto`; Native projesinde
   Native'in gerçek değerleri yazılır.
3. Gizli adımlar tek anlamlı satır: modcu `lord harlaus` yazar; derleyici `give_center_to_lord`, faction,
   original_faction, kültür vb. bütün adımları tutarlı üretir. Çelişki (yabancı lord) = Türkçe açıklamalı hata ya
   da açıkça söylenen `auto`.
4. Ham alan dökülmez: motor alanları (ör. `faction neutral` şehirde) gizlenir ya da tek cümleyle anlamlandırılır.
5. Kanıt: her parça bir oyun deneyiyle kapanır (CHECKUP), byte eşitliği ayrıca korunur (değişmeyen proje = Native).

## 4. Örnek envanter (KAPSAM DEĞİL, yalnız ilham; bkz. §0. "?" = nerede yaşadığı araştırılacak)

### Krallık (faction)
ad (+çeviri), renk, kültür (`slot_faction_culture`?), sancak/arma?, lider (`slot_faction_leader`, game_start'ta?),
lordlar listesi (troop kaydındaki `faction` alanı + slotlar?), başkent?, ilişkiler (diğer krallıklarla
başlangıç savaş/barış?), oyuncu katılabilir mi, AI kişiliği?, isyancı/hak iddiacısı (pretender)?.

### Yerleşim (şehir/kale/köy)
ad, konum, ikon, sahip (`owner`), lord (`lord` / `auto`, T-116), kültür (game_start :603 krallıktan türetiyor —
modcu ayrı seçebilmeli mi?), bağlı köyler (köy → merkez bağı?), sahneler (merkez, kale, meyhane, dükkan,
arena, hapishane, sokak; `scn_town_N_*`), garnizon (game_start :669 "add town garrisons"), pazar/ürünler?,
zenginlik/refah?, kasaba görevlileri (seneschal, arena ustası, tüccarlar `trp_town_N_*`), başlangıç menüsü
seçenekleri ("X'e giden kervana katıl" metni), yollar?.

### Karakter (kral, lord, leydi, yoldaş, sıradan birlik)
ad + çoğul ad (+çeviri), yüz kodları, beden/cilt (skin), stat (STR/AGI/INT/CHA), beceriler, silah
uzmanlıkları, seviye, envanter ve teçhizat (troop kaydında), krallık, ev/yurt (`slot_troop_home`?), nerede
dolaşır/bulunur (yoldaşlar meyhanede: `script_update_companion_candidates_in_taverns`?; leydiler kalede?),
ilişkiler (lord-lord, sevgi/nefret, yoldaş kişilik çatışmaları `slot_troop_personality_clash`?), kişilik
(onurlu, açgözlü…?), eş ve aile (`slot_troop_spouse`, baba/anne?), diyaloglar (ilk karşılaşma, geçmiş
hikâyesi — T-102 `dialog` var), maaş/katılma şartı (yoldaş), parti şablonu (lordun ordusu).

### Diğer (meclis eklesin)
birlik ağaçları (yükseltme), eşyalar, parti şablonları, haritadaki gruplar (haydutlar vb. doğma bölgeleri —
T-109 `area`), oyun başı (oyuncunun başlangıcı), görevler.

## 5. Meclisin cevaplayacağı sorular

0. **Genellik (en önce):** Hangi genel mekanizmalar, tek tek alan eklemeden, "modcu her şeyi yapabilir"
   hedefini sağlar? Kapsama sistematik olarak nasıl ölçülür (ör. Module System'in her tablosu, alanı ve işlemi
   için "UW'de ifade edilebilir mi" tablosu, otomatik)? Aşağıdaki sorular bu cevabın içinde yer alır.

1. **Gizli adım haritası:** her alan için Native'de hangi kayıt alanı + hangi game_start/başka betik adımı onu
   belirliyor? (Kanıt dosya:satır; araştırma işini DeepSeek'e "salt okuma rapor" olarak vermek ucuz — T-106
   denetim raporları gibi.)
2. **Yazım:** kurucu alanlar kayıt dosyasında mı (`party praven { owner … lord … culture … }`), yoksa yeni üst
   düzey yapılar mı (`kingdom yasin { leader …  lords [...] }`, `character …`)? İkisi birlikte mi?
3. **`auto` anlamı:** her alan için `auto` tam olarak hangi Native davranışı? (lord auto =
   assign_lords_to_empty_centers; kültür auto = krallıktan; yoldaş yeri auto = rastgele meyhane…)
4. **Tutarlılık:** çelişkiler (yabancı lord, krallıksız lord, lordsuz krallık, iki şehirde aynı lord) hata mı,
   uyarı mı, `auto` mu?
5. **Boş taban:** sıfırdan başlayan proje (D8 deneyi, `kit-meta/hardwired.toml` motor zorunluları) bu modelle
   nasıl birleşir? Hangi kayıtlar motor için zorunlu?
6. **Proje kipi + yeni dil:** `dialog`, `area`, `every`, `in menu`, `like` şu an yalnız yama kipinde; proje
   klasöründe de çalışmalı (ROADMAP maddesi). Kurucu model bunun üstüne mi kurulur?
7. **Sıra ve paketler:** önce hangi varlık (öneri: yerleşim → krallık → karakter), hangi parça Claude'da
   (tasarım, gizli adım birleştirme), hangisi DeepSeek'te (araştırma raporları, alan şemaları, testler).
8. **Kabul deneyi:** boş tabana elle yazılmış krallık + şehir + lord + leydi + yoldaş; oyunda adım adım ne
   görülmeli (CHECKUP maddesi olarak yazılsın).

## 6. Meclisten önce açık işler (aynı oturumda önce bunlar)

- **cv/T-115** (denetim tur 2: kimlik tekrarı, `rest N` → ad, tetik aralıkları adla, varsayılan alanları gizleme)
  ve **cv/T-116** (`lord` satırı / `auto`, Praven düzeltmesi): şeritte bitti ya da bitiyor; denetle → birleştir →
  tam ölçü (testler, `uw new --check` 12815/12815, `uw build-project --compare-native` 32/32) → push.
- T-115'in "varsayılanı gizle" altyapısıyla `faction neutral` yerleşim dosyalarından kalksın (sahibi 3. kez
  şikâyet etti).
- Sahibinin oyun deneyi: denememodum3'te `owner vaegirs` + `lord auto` → Praven Vaegir bayrağıyla.
  Kral adı için `name_tr` (troops) ve başlangıç menüsü metni (languages/tr/game_menus.csv:5) deneyi.
- Bu oturumda birleşenler: T-100..T-114 (özet docs/PROGRESS.md 2026-11-16 22:00).

## 7. Kaynaklar

docs/VISION.md (2026-11-16), docs/ROADMAP.md "KURUCU MODEL", docs/meclis/yon-2-proje-modeli.md,
docs/meclis/kolay-dil-taslak.md, docs/bilgi/anlam/*.md (anlam kütüphanesi: sehir-ve-krallik, asker-ve-birlik,
diyaloglar, bolgeler-ve-dogma), docs/bilgi/denetim/*.md (akıllı denetim raporları), docs/tasks/T-103.md, T-116.md.
