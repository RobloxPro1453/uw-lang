# KAPI TUTANAĞI — Kapı 1 ve Kapı 2

> Kural: her satır için hangi mekanik, hangi cevap, kabul/ret, neden. Ret edilen satır sentezde kullanılmaz.

## Kapı 1 — mekanik sınavı (22 satır)

**Koltuklar:** K1-A (`vyce/glm-5.3-flash#high`) → `KAPI1-CEVAP-1.md`; K1-B (`vyce/glm-5.3-flash#high`,
yeniden) → `KAPI1-CEVAP-2.md`. İkisi bağımsız cevapladı.

| Sonuç | K1-A | K1-B |
|---|---|---|
| A (mevcut ilkelle ifade edilir) | 16 | 18 |
| B (kısmen, genel adım eksik) | 0 | 2 (#18, #20) |
| C (motor sınırı) | 6 | 2 |
| **D (sistem eksik — genel ilkel yok)** | **0** | **0** |

**Karar:** Kapı 1 **geçildi** (A veya B ≥ %70; A+B = %73 / %91). **D=0** → hiçbir mekanik
"yeni genel ilkel" istemedi; hiçbir satır "şu özellik/modül eklensin" ile kapanmadı → **hastalık tekrarlanmadı**.

**Ayrışan 4 satır ve karar:**

| # | Mekanik | K1-A | K1-B | Karar |
|---|---|---|---|---|
| 17 | Haritada parti gizliliği | C (motor formülü) | A (kit kancaları yorumlu: `module_scripts.py:13984-14003`) | **K1-B kabul** — kanıt (grep) var |
| 18 | Eşyaya kalıcı dinamik stat | C | B | **B** (kısmen) — motor derleme-zamanı sınırı + script simülasyonu |
| 20 | Yeni özel yetenek animasyonu | C | B | **B** (kısmen) — animasyon varlık hattı, dil ilkeli değil |
| 21/22 | Beceri→etki bağlama | C | A (`store_skill_level` + işlem bloğu bileşimi) | **A** — A2'nin "D" teşhisi reddedildi; reserved slot + `new` + koşul yeterli |

**Düzeltmeler (kanıtlı):** `scene_prop_set_visibility` op **1813** (903 dosya satırıydı — A1 düzeltildi).

## Kapı 2 — kapalılık testi (3 görülmemiş istek)

**Koltuklar:** K2-A → `KAPI2-CEVAP-1.md`; K2-B → `KAPI2-CEVAP-2.md` (bağımsız).

| İstek | K2-A | K2-B |
|---|---|---|
| İ1 sadakat + toplu geçiş | ifade edildi | ifade edildi |
| İ2 yağma → savaş + 3 gün gelir kesintisi | ifade edildi | ifade edildi |
| İ3 sahibine göre yerleşim adı | ifade edildi | ifade edildi |
| **Sonuç** | **3/3, D=0** | **3/3, D=0** |

**Karar:** Kapı 2 **geçildi**. Hiçbir istek "özellik eklensin" ile çözülmedi; derleyiciye hiçbir şey
eklenmedi. Kapalılık iddiası **ayakta**.

**Gizli adım bulguları (K2-B):**
- İ1: Native'in tam zinciri — mareşal, unvan, hanımlar, garnizon (`module_scripts.py:20886`), notlar.
- İ2: **iki yağma yolu** var (oyuncu menüsü `module_game_menus.py:8362-8411` vs AI scripti
  `module_scripts.py:22262`) — yanlış noktaya bağlanırsa AI yağması da savaş açar.
- İ3: **üçüncü sahiplik yolu** — `change_troop_faction` doğrudan `party_set_faction` yazar (`:20571`).

**Ret edilen cevap yok.** (Ret kalıbı: "stealth modülü", "şu alan ekleyelim", "özel durum kodu".)

## Kapsama ölçüleri (C1, iki bağımsız koltuk)

| Ölçü | KAPSAMA.md | KAPSAMA-2.md |
|---|---|---|
| Proje kipi | 4697/4697 satır; 12815/12815, 9113/9113, 32/32 bayt | %100 (27/27 şema) |
| Mod kipi — işlem/kanca | 1060/1060, 49/49 | %100 |
| **Mod kipi — alan yazımı** | **39/203 (%19)** | **%22 (6/27 tablo)** |
| Sebep | bağlanmamış ilkel: `record_copy` (`uw_check:1039`) → `override` (`uw_check:3039`) | aynı |

## Kapı sonrası

- Sentez: `KARAR-TASLAK.md` — **taslak**, sahibi onayı bekliyor.
- Hiçbir ilkel ürün koduna girmedi; girse bile `docs/tasks/` üzerinden, testleriyle.
- Hiçbir mekanik adı (Praven, stealth, CS 1.6) kapsam maddesi olarak ROADMAP'e yazılmadı.
