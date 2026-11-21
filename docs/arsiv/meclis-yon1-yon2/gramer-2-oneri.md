# Gramer 2 önerisi (mimar: Claude, 2026-11-15) — ifadeler, if, menü dili

Faz 1 sözdizimine (süslü parantez, `ad(a, b)` Native işlemleri) eklenir; eski her `.uw` aynen geçerli kalır.
Bütün Native adları kitten: aşağıdaki işlem adları `Module_system 1.171/header_operations.py`'de var
(eq 41, gt 42, ge 40, lt/neq/le 444-446 `neg|`, is_between 44, assign 1191, val_add..val_mod 1139-1147,
store_add/sub/mul/div/mod 1166-1174, try_begin/else_try/try_end 16-19, neg/this_or_next 440-441).

## 1. Değerler ve atama

```uw
$gold = 100                 // assign($gold, 100)
:x = $gold + 2 * :y         // store_mul(:_uw0, 2, :y); store_add(:x, $gold, :_uw0)
$gold += 50                 // val_add($gold, 50)   (-= *= /= %= -> val_sub/mul/div/mod)
```
- İşleçler: `+ - * / %`, parantez, tekli `-`. Öncelik Python'daki gibi. Yalnız tamsayı (motor tamsayı).
- Ara sonuç yerelleri `:_uw0, :_uw1…` (alt çizgiyle başlayan yerel adlar kullanıcıya yasak → çakışma yok).
- Hedef yalnız `$global`, `:yerel` ya da `reg0`-`reg65` gibi kit sabiti olan bir yazmaç.
- Değer bir kit sabiti / Native adı olabilir (`p_town_6`, `tf_male`): bugünkü `Arg` kuralları.

## 2. Koşullar ve `if`

```uw
if $gold >= 100 and not party_is_active(p_town_6) {
    $gold -= 100
} else if $gold > 0 {
    ...
} else {
    ...
}
```
- Karşılaştırma: `== != > >= < <=` → `eq neq gt ge lt le`; iki taraf ifade olabilir (önce ara yerele).
- Koşul olarak Native koşul işlemi doğrudan yazılır: `party_is_active(p_town_6)`.
- `and` → art arda koşullar; `or` → `this_or_next|` (her iki taraf tek işlemse); `not` → `neg|`
  (tek işleme). İndirgenemeyen karışık durum (`(a and b) or c`) ara bayrak yereli ile:
  `:_uwN = 0; try_begin a; b; :_uwN = 1; try_end; ...` sonra `eq(:_uwN, 1)`.
- `if` → `try_begin … else_try … try_end`.
- Bloğun en üstünde çıplak koşul işlemi (bugünkü gibi `eq($x, 1)`) Native anlamını korur: başarısızsa
  bloğun kalanı çalışmaz. Kılavuz bunu açıkça anlatır.

## 3. Ad biçimi (sahibinin isteği)

- Kanonik: tam kimlik `override menu_start_phase_2 { … }`; tür önekten (atlas `id_prefix`).
- Eski `override menu start_phase_2` kabul edilmeye devam eder (tek eşleşme kuralı).

## 4. Menü dili

```uw
override menu_start_phase_2 {
    text "Yeni giriş metni"                  // alan 2 (menü metni) değişir
    on enter { ... }                         // bugünkü gibi: giriş gövdesi değişir

    option uw_skip "Doğrudan başla" {        // YENİ seçenek, sona eklenir
        jump_to_menu(mnu_start_game_0)
    }
    option uw_rich "Zengin başla" when $gold < 10 after uw_skip {
        $gold = 1000
    }
    option uw_gate "…" when { store_troop_gold(:g, trp_player); ge(:g, 5) } { … }

    change option <native_id> {              // var olan seçeneği değiştir (alanları ayrı ayrı)
        text "…"
        when $x > 0                         // koşul bloğunu değiştirir
        do { … }                            // sonuç bloğunu değiştirir
    }
    remove option <native_id>
}

new menu_uw_ask "Nereden geldin?" {            // YENİ menü (yeni kayıt; ID sırası uw_plan)
    on enter { … }
    option a "Kuzeyden" { jump_to_menu(mnu_start_phase_2) }
}
```
- Katı: `option X` Native'de zaten olan id ise hata ("var olanı değiştirmek için `change option X`");
  `change/remove option X` yoksa hata + yakın ad önerisi. Seçenek kimliği ASCII.
- Konum: varsayılan sona; `after <id>` / `before <id>`.
- `new menu_x`: kimlik `menu_` önekiyle yazılır, Native'de varsa hata. Bayraklar şimdilik 0, mesh "none".
- Metin: ASCII (D5 deneyi bitene dek), `_` oyunda boşluk görünür → kılavuzda not.

## 5. Bağlayıcı sırası

Aynı kancaya ya da aynı kayda birden çok dosya dokunursa: dosya adı alfabetik, dosya içinde yazım sırası.
Aynı menüye iki `on enter` / iki `text` → hata (iki yer gösterilir).

## 6. IR (uw_check → uw_lower → uw_emit)

- `uw_syntax`: `Stmt = Call | Assign{target, op, expr} | If{arms, else}`; `Expr`, `Cond` ağaçları;
  menü öğeleri `MenuItem = Text | OnEnter | Option | ChangeOption | RemoveOption`; `Item::New`.
- `uw_check`: adları çözer, `Stmt`'leri değer türleriyle (`Value`) taşır.
- `uw_lower`: `Stmt` ağacı → düz `Op` listesi (ifade, `if`, `and/or/not`), ara yerel sayacı.
- `Patch.action`: `ReplaceBody | Append | ReplaceField{value} | InsertOption{pos} | ChangeOption |
  RemoveOption | NewEntry`; emit `uw_runtime`'ın hazır işlevlerine (`replace_field`, `insert_after`,
  `insert_before`, `remove_where`, `extend`) gider.

## 7. Risk koltuğu (haiku) ve çözümler — KARAR (2026-11-15, hafif meclis: mimar + risk, oybirliği)

1. **Menü öneki:** kit girdi adını çıplak yazar (`"start_phase_2"`), derlenmiş kimlik `menu_start_phase_2`,
   işlem içindeki başvuru `"mnu_start_phase_2"` (atlas ikisini de çözüyor: `entry_by_id`, `resolve_ref`).
   Karar: bildirimde kimlik (`override menu_x`, `new menu_x`), işlem argümanında ikisi de kabul; emit her
   zaman kitin biçimini yazar (`"mnu_x"`). Çift önek hatası derleyicide yakalanır (`menu_menu_x` → hata).
2. **Seçenek kimliği:** kitteki gibi öneksiz yazılır (`option uw_skip`, `remove option camp_wait_here`
   (uydurma örnek)); `mno_` yazılırsa hata + düzeltme önerisi.
3. **Öncelik:** `not` > `and` > `or` (Python). `this_or_next|` yalnız bir `and` zincirinin öğesi olan saf
   `or` grubunda ve grubun her öğesi tek koşul işlemiyse; aksi her durumda bayrak yereli. Karışık
   and/or altın testleri zorunlu.
4. **Çıplak koşul işlemi** bloğun herhangi yerinde Native anlamını korur (bloğun kalanını keser); bu Native
   davranışı, yasak değil. Kılavuz bunu örnekle anlatır.
5. **Satır devamı:** parantez içinde ve satır bir ikili işleç / `and` / `or` / `not` / `=` ile bitiyorsa satır
   sürer. `when {` her zaman blok biçimidir; `when <ifade>` gövdenin `{`'sine kadar. `else if` desteklenir,
   `elif` / `else:` tek satırlık düzeltme önerisiyle hata.
6. **Ara yereller:** her ara yerel kullanılmadan önce aynı blokta yazılır (store_* ya da `= 0`); okunmamış
   değer taşınmaz. Kullanıcı `:_` ile başlayan yerel yazamaz.
7. **`not` ile ters karşılaştırma:** kitte `lt = neg|ge`, `neq = neg|eq`, `le = neg|gt` (header_operations.py
   444-446); `neg|lt` yine `lt` olur (bit zaten açık). Karar: `not` karşılaştırmaya uygulanınca işleç
   tersine çevrilir (`not a < b` → `ge`), `neg|` yalnız karşılaştırma olmayan koşul işlemlerine.
