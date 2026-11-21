# Kenar durum avcısı — T-116 `lord` satırı

> Bu dosya yalnız T-116'nin kenar durumlarını listeler. Kod değiştirilmez. Kanıt yoksa `[İDDİA]`.

## Özet (≤5 satır)

T-116, `owner` değiştiğinde merkezin lordunu da tutarlı kılar: `lord <ad>` veya `lord auto`.
Köy/kale/şehir, oyuncu krallığı, pretender, ölü lord, aynı lorda iki merkez, çağrısız merkez,
çift çağrı, lord-owner çelişkisi ve kayıtlı oyun kenar durumları aşağıda listelenir.

## Bulgular

### 1. Köy / kale / şehir ayrımı
- Beklenen davranış: `lord` yalnız şehir ve kale için geçerlidir. Köylerde `lord` satırı yok; köyün kendi lordu yoktur (bağlı olduğu merkezin lordu geçerlidir) ve `lord auto` köyde atama yapmaz.
- Kanıt: `Module_system 1.171/module_scripts.py:567-596` — `script_give_center_to_lord` yalnız `p_town_1..22` ve üç kale (`p_castle_29/30/35`) için; hiçbir `p_village_*` yok. Köyde lord yok / ne olur = [İDDİA].

### 2. Oyuncu krallığı
- Beklenen davranış: oyuncu faciyonuna bağlı merkezde `lord auto` → oyuncu kendisi veya oyuncunun lordu atanır; `lord <ad>` ile seçilebilir. Oyuncu faciyonuna ait olmayan bir lordun oyuncu merkezinde seçilmesi = hata veya `auto`.
- Kanıt: yok — oyuncu faciyonu ve davranışı kitten okunmadı = [İDDİA].

### 3. Pretender / isyancı
- Beklenen davranış: pretender faciyonuna verilen merkezde `lord auto` → pretender faciyonun bir lordu atanır; pretender'ın kendi (yeni) merkezi = [İDDİA].
- Kanıt: yok — pretender faciyonu ve davranışı kitten okunmadı = [İDDİA].

### 4. Ölü lord
- Beklenen davranış: ölü (`trp_*_lord` öldü) lorde atanmış merkezde `lord auto` → canlı başka bir lord atanır; ölü lord manuel seçilirse hata veya `auto`.
- Kanıt: yok — lord ölümü / canlılık kontrolü incelenmedi = [İDDİA].

### 5. Aynı lorda iki merkez
- Beklenen davranış: bir lord iki farklı merkezin lordu olabilir (çakışma yok); `lord auto` iki merkeze aynı lordu verebilir.
- Kanıt: Native'de bir lordun birden fazla merkeze sahip olabilmesi = [İDDİA] (567-596'de her merkez bir kez, ama aynı lordun tekrarı kontrol edilmedi).

### 6. Çağrısı olmayan merkez
- Beklenen davranış: `lord auto` (veya çağrı yok) → `script_assign_lords_to_empty_centers` merkezin kendi krallığından bir lord atar.
- Kanıt: `Module_system 1.171/module_scripts.py:598` — `script_assign_lords_to_empty_centers` çağrısı; `docs/tasks/T-116.md` Goal — `lord auto` çağrıyı kaldırır, oyun atar (taslak hedefi, henüz uygulanmadı = [İDDİA] kod tarafı).

### 7. Çift çağrı (iki `lord` / iki `give_center_to_lord` aynı merkezde)
- Beklenen davranış: hata (merkez zaten verilmiş) veya tek geçerli çağrı; sessizce ikincisini kabul etmemeli.
- Kanıt: Native'de her merkez en fazla bir kez = [İDDİA] (567-596'de her `p_town_*` bir kez); çift-çağrı davranışı = [İDDİA].

### 8. Lordun krallığı ile owner çelişkisi
- Beklenen davranış: `owner` değişip `lord` eski (farklı faciyonun) lordunda kalmışsa otomatik `auto` + Türkçe mesaj; manuel `lord <yabancı ad>` → Türkçe hata + dosya:satır.
- Kanıt: `docs/tasks/T-116.md` Goal — "owner changed and lord's faction differs → auto + Turkish message"; "chosen lord's faction differs from owner → Turkish error with file:line". Uygulama yok = [İDDİA].

### 9. Kayıtlı oyun (save game)
- Beklenen davranış: kaydı yüklemede lord ataması tekrar çalışmaz; mevcut lord korunur. `lord` yalnız yeni oyun başında etkili.
- Kanıt: yok — save/load akışı incelenmedi = [İDDİA].

## Öneri

- `lord` uygulanmadan önce her kenar durumu için tek bir oyun deneyi yazılsın (CHECKUP kuyruğu).
- `lord auto` ve `lord <ad>` ayrı kod yolu olsun; çelişki durumunda sessiz `auto` yerine Türkçe mesaj verilsin.
- Köylerde `lord` reddedilsin veya yoksayılsın; sessiz atama yapılmasın.
- Byte eşitliği (32/32, 12815/12815) değişmeyen projede korunmaya devam etsin.

## Açık sorular / [İDDİA]

- Köyün `lord` satırı varsa ne olur? [İDDİA]
- Oyuncu krallığı, pretender, ölü lord davranışları nedir? [İDDİA]
- Çift çağrıda hangi çağrı kazanır, yoksa hata mı? [İDDİA]
- Kayıtlı oyun yüklemesinde lord bozulur mu? [İDDİA]

## Kaynaklar (okuduğum dosyalar)

- `docs/arsiv/meclis-yon1-yon2/opencode/t116-lord/BRIEF.md`
- `docs/tasks/T-116.md` (Goal, Known facts, Tests)
- `docs/arsiv/meclis-yon1-yon2/kurucu-model-gundem.md` §2 (Praven dersi), §4 (envanter, yalnız ilham)
- `Module_system 1.171/module_scripts.py:486-563, 567-598, 601-613`
- `crates/uw_decomp/src/project/mod.rs` (`apply_owners`, `take_owner`, `owner_line`)