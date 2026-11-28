# Meclis: "Maden" — UW dil çekirdeği (2026-11-21)

Bu meclis mimariyi değiştirir (AGENTS.md kural 5). Koltuklar birbirini görmeden yazar; sentez `KARAR.md`.

## Sahibinin sabit kararları (tartışılmaz; tam metin `docs/VISION.md` "2026-11-21 akşam")

- MS = assembly, UW = Python. **Mekanik başına hazır parça yok** (stamina, gizlilik, HUD yöneticisi yok).
  Derleyicinin bildiği sonlu: (1) kitten otomatik çıkarılan motor, (2) genel dil kavramları, (3) MS çilesinin
  gizlenmesi (yazmaç, slot numarası, try blokları, id). Turnusol: "derleyiciyi yazan stamina/HUD/Praven
  duymamış olsaydı Alex denemesi yine yazılabilir miydi?"
- Sınır kuralı: her oyun motorunda zaten bulunan genel olanaklar dilde hazır gelir (stat/beceri, olay/tetik,
  zaman, arayüz parçaları ~%80-90 güçlü özelleştirmeyle); modun tasarım kararı modcunun kodudur. Alan ekleme
  **mekanizması** dilde, eklenen alan modcunun.
- Modüler: konu başına dosya, `main.uw` `import` ile alır. Süslü parantez. Nesne yazımı `agent.health`,
  takma ad (`alias`). Adlar motordan çıkar, uydurulmaz. Kendi türünü tanımlama (`class`) var.
- Native = dille yazılmış, içe aktarılabilir, değiştirilebilir, kısmen bırakılabilir kod. `town.lord` Native'in
  alanıdır, dilin değil. Overhaul da sıfırdan dünya da desteklenir.
- Ham MS kaçışı var ama gerekmemeli. Tek oyunculu önce; **çok oyunculu kapsam dışı**.
- Bitiş ölçüsü: sahibi aklına gelen her şeyi derleyiciye dokunmadan yalnız UW yazarak yapabiliyorsa iş bitmiştir.

## Kanıt (gerçek modlardan; okuyun, kanıt satırları doğrulandı)

- `docs/bilgi/altyapi/OZET.md`: 15 modun elle kurduğu ortak altyapı. Kısaca: nesnede adlı tipli kalıcı alan
  15/15, liste/sözlük 15/15, zaman/cooldown 14/15, bildirimsel arayüz 13/15, metin değeri 11/15, girdi eylemleri
  9/15, olaylar 9/15, modül/override/sarma 4/15, ayar ekranı 4/15.
- `docs/bilgi/maden/*.md`: 11 konu, 17 mod, 1006 kanıt satırı. Her birinin sonunda "For UW" bölümü var:
  ui-presentations, battle-triggers, dialogs, scripts-style, data-modeling, world-simulation, python-tooling,
  engine-quirks, strings-text, records-authoring (multiplayer kapsam dışı ama okunabilir).
- Mevcut durum: `docs/dil/CORE-v0.md` (dil taslağı), `docs/arsiv/meclis-yon3/dil/KARAR-DIL.md` (önceki karar; §2b
  esneklik), `crates/uw_core/` (ayrıştırıcı + alt çeviri, Alex örneği `crates/uw_core/tests/alex/`),
  `kit-meta/easy/` (kitten otomatik söz dağarcığı: 933 işlem otomatik + 127 elle, sabitler, slotlar).

## Karar verilecek sorular

1. **Çekirdek katmanlar.** Kanıttaki 11 olanaktan hangisi (a) dilin kendisi (sözdizimi/tip), (b) UW ile yazılmış
   standart kütüphane (`std`), (c) Native kodu, (d) modcunun kodu? Sınır kuralına ve turnusola uyan net bir tablo.
2. **Durum modeli.** Varlık başına adlı alan + kayıt (struct) + liste/sözlük, motorda neyle saklanır (slot,
   kukla asker/parti, global)? Slot numarasını derleyici nasıl kararlı ayırır; kayıt dosyası uyumu (alan eklenince
   eski kayıt bozulmasın) nasıl sağlanır? Ayırma/serbest bırakma kimde?
3. **Zaman ve olaylar.** `every`, `after`, `cooldown`, `on <olay>`, tuş eylemleri: kaç gizli tetik/dağıtıcıya
   derlenir? Modcunun kendi olayını tanımlayıp yayınlaması (`event` + `emit`) nasıl olur?
4. **Arayüz.** `screen`/`hud` bildirimi + bileşen durumu + değere bağlama: neyi dil, neyi `std` sağlar? Tek
   presentation mı, bileşen başına mı?
5. **`class` ve Native.** Modcunun türü neye derlenir (alan seti + yöntemler = betikler)? Native'in var olan
   varlıkları (`town`, `lord`) bu `class` mekanizmasıyla mı tanımlanır, yani Native = `import native` ile gelen
   sıradan UW kodu mu? Override ve sarma (wrap) — başkasının menüsünü/tetiğini/diyaloğunu değiştirme — nasıl?
6. **Metin.** Ara değer gömme, kararlı metin kimliği, yazmaç havuzu: dil mi std mi?
7. **Uygulama sırası.** Yukarıdakilerin hangi sırayla kodlanacağı; her adımın kabul testi. Kabul testi olarak
   "gerçek mod sistemlerinin UW'ye çevirisi" (ROADMAP M3) nasıl kullanılır? İlk dikey dilim ne olmalı?

## Cevap biçimi (her koltuk, en çok ~60 satır)

Soru başına: karar önerisi (1-3 satır) + gerekçe (kanıt dosyası adıyla) + risk/belirsizlik. Sonda: en küçük
ilk adım ve "bunu yapma" listesi. Ad uydurma: Native/kit adı gerekiyorsa kitten/atlas'tan olduğunu söyle.
