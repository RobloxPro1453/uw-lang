# KAPI PROTOKOLÜ — körlük, Kapı 1 ve Kapı 2

> Bu protokol sahibinin fikridir: mekanikleri biri detaylıca inceler ama **sistem kuruculara söylemez**;
> kurucular kendi genel sistemlerini kurduktan **sonra** mekanikler önlerine konur ve "sisteminiz bunları
> yapabiliyor mu?" diye sorulur.
>
> Protokolün amacı tek: **"verilen örneği amaç sanma"** hatasını yapısal olarak imkânsızlaştırmak.

## 1. Neden körlük

Örnek önceden gösterilirse tasarımcı farkında olmadan örneğe göre şekillendirir: "stealth için şu alan
gerek" der, alanı ekler, "tamam" der. Körlükte tasarımcı **hangi isteklerin geleceğini bilmez**; bu yüzden
yalnızca genel olan bir şey kurabilir. Mekanikler sonradan **sınav** olarak gelir, **gereksinim** olarak değil.

## 2. Neyin kapalı olduğu (ve neyin olmadığı)

**Kapalı olan yalnız "sınav kâğıdı"dır:**
1. `gizli/ANALIZ-A1.md`, `gizli/ANALIZ-A2.md` (analizcilerin çıktısı),
2. `light-and-darkness-report/mekanik-01..11.md` (sınavın kaynağı).

**Kapalı olmayan:** `Module_system 1.171/` (hatta zorunlu), `crates/`, `kit-meta/`, atlas, 6 örnek mod
(`ornek-modlar/`), eski meclis dosyaları, Praven kanıtı. Bunlar sınav kâğıdı değil, zemin bilgisidir;
okumamak planı kanıtsız bırakır.

`gizli/` klasörü talimatla korunur; dosya sistemi kilidi yok. Bu yüzden:

- Her koltuk çıktısının başında **okuduğu dosyaları** listeler.
- Moderatör, tasarım dosyalarında L&D mekanik adı / atfı arar (metin taraması).
- Tek bir sızma bulunursa o koltuğun çıktısı **tamamen** geçersiz sayılır; yeniden çalıştırılır.
- Körlük ihlali en ağır ihlaldir; "küçük bir bakış" yoktur.

## 3. Kapı 1 — mekanik sınavı

**Ne zaman:** B1-B4 "aklımızdaki sistem çalışıyor" dediğinde (tur 3).

**Süreç:**
1. Moderatör `gizli/ANALIZ-A1.md` ve `gizli/ANALIZ-A2.md`i açar, çelişkileri işaretler, tek bir
   **sınav listesi** yapar (A1 ve A2'nin birleşimi; çelişen satırlar ayrıca gösterilir).
2. Liste tasarımcılara **mekanik cümlesi** olarak verilir — uygulama ipucu, alan adı, çözüm önerisi yok.
3. Her tasarımcı **her mekanik** için dört kutudan birini doldurur (aşağıdaki tablo).
4. Moderatör her satırı denetler: verilen ilkel gerçekten var mı, gerçekten o işi yapıyor mu,
   yoksa isim benzerliği mi?

**Cevap kutuları:**

| Kutu | Anlamı | Kabul koşulu |
|---|---|---|
| **A** | "Mevcut ilkelle ifade edilir" | Hangi ilkel + modcunun yazacağı satır + derleyicinin üreteceği adım zinciri gösterilmeli |
| **B** | "Kısmen: ilkel var, şu adım eksik" | Eksik adımın **genel** olduğu gösterilmeli (tek mekaniğe özel değil) |
| **C** | "Motor sınırı — yapılamaz" | `Module_system 1.171/dosya:satır` ya da tekrarlanabilir deney ile kanıt |
| **D** | "Sistem eksik — şu **genel ilkel** yok" | Eksik ilkelin en az iki farklı mekanikte işe yaradığı gösterilmeli |

**Otomatik ret kalıpları** (hangi kelimeyle yazılırsa yazılsın):

| Ret kalıbı | Neden ret |
|---|---|
| "stealth için stealth modülü/alanı ekleyelim" | Örneği amaç sanmak; hastalığın ta kendisi |
| "bu mekanik için özel komut yazalım" | Tek isteğe özel çözüm, kapalılık yok |
| "şu örneği desteklemek için yeni alan" | Alan ekleyerek ilerleme = özellik fabrikası |
| "hack / özel durum kodu / if bloğu" | Genellik değil yama |
| "şimdilik böyle yapalım, sonra genelleştiririz" | Erteleme; genellik sonradan gelmez |
| Cevapsız / "yapılabilir" deyip göstermeden geçme | Kanıtsız iddia |

**Kapı 1 geçme ölçüsü (öneri, meclis tartışabilir):**
- Mekaniklerin **≥ %70'i A veya B** ise sistem "genel" iddiasını taşıyabilir.
- **D** (eksik genel ilkel) çıkması normaldir; her D, sentezde "kurulacak ilkel" listesine girer —
  ama **yalnız bir kez** ve **genel** olarak.
- Aynı mekanik için 3+ kez D çıkıyorsa sistem örnek başına ilkel istiyor demektir; **genel değildir**,
  tur 4'e geçilmez, tasarım yeniden kurulur.

**Tutanak:** `KAPI-TUTANAK.md` — tablo: mekanik · cevap kutusu · gösterilen ilkel · denetim sonucu
(kabul/ret) · gerekçe. Ret edilen satır sentezde **kullanılmaz**.

## 4. Kapı 2 — kapalılık testi (asıl sınav)

Kapı 1 "bilinen mekanikler"i sınar; asıl iddia **bilinmeyen** isteklerle sınanır.

**Süreç:**
1. Moderatör, tasarımcıların **hiç görmediği** kaynaklardan **3 istek** seçer
   (`SINAV-SETI.md` §2'deki 6 mod ve kendi ürettiği gerçekçi bir istek).
2. İstekler **davranış diliyle** verilir ("şu durumda şu olsun"), uygulama diliyle değil.
3. Tasarımcı, isteği **derleyiciye hiçbir şey eklemeden** ifade eder:
   - modcunun yazacağı UW satırları,
   - derleyicinin üreteceği adımlar,
   - mevcut ilkellerden hangilerini kullandığı.
4. Moderatör denetler: gerçekten mevcut ilkel mi, gizli bir "özel durum" var mı, üretilen adımlar
   tutarlı mı.

**Geçme ölçüsü (öneri):**
- 3 isteğin **≥ 2'si** yalnız mevcut ilkellerle ifade edilir → kapalılık iddiası ayakta.
- Kalan 1 istek **tek bir genel ilkel** eksiğiyle açıklanır → kabul (o ilkel listeye girer).
- Herhangi bir istek "şu özellik eklensin" ile çözülüyorsa → **kapalılık yok**; sentezde bu açıkça yazılır.

**Yasak:** Kapı 2 sonucunu "şu özellikler eklenecek" listesine çevirmek. Çıktı yalnız
**eksik genel ilkeller** listesidir.

## 5. Kapı 2 istek havuzu (moderatör seçer, önceden göstermez)

Havuz `SINAV-SETI.md` §2'deki modlardan ve sahibinin "sınırsız ihtimal" çerçevesinden üretilir.
Seçim kuralı: **istek** (davranış cümlesi) meclis tartışmasında geçmemiş olacak. Tasarımcıların o modları
okumuş olması sorun değil — test edilen şey kaynak değil, **isteğin kendisinin yeni olması**. Örnek biçim (bunlar örnek, aynen
kullanılmayacak):

- "Her gece ödenmemiş askerler kaçsın; ordu 20'nin altına düşünce lord yağmaya geçsin."
- "Bir yerleşimin adı oyuncunun krallığına göre değişsin."
- "Aynı karakter hem lord hem yoldaş olabilsin; hangisi olduğu oyuncunun seçimine bağlı olsun."

## 6. Kapı sonrası

- `KAPI-TUTANAK.md` + `KARAR-TASLAK.md` yazılır.
- Karar taslağı **"taslak — sahibi onayı bekliyor"** damgası taşır.
- Sahibi onaylamadan hiçbir ilkel ürün koduna girmez; girse bile `docs/tasks/` üzerinden, testleriyle.
- Hiçbir mekanik adı (Praven, stealth, CS 1.6) **kapsam maddesi** olarak ROADMAP'e yazılmaz.
