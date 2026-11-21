# Meclis brief'i — T-116: `owner` şehrin lordunu da taşımalı

> **ARŞİV — KULLANILMAZ (2026-11-17).** Bu meclisin konusu **yanlıştı**: tek bir alanı (`lord`) hedef
> yapıyordu, yani "örneği amaç sanma" hatasının kendisiydi. Sahibinin kararıyla Yön 2'nin çerçevesi
> düştü; güncel hazırlık `../yon3/README.md`. Bu dosya yalnız "Praven iki adım" kanıtı için saklanır.
>
> Bu dosya meclis koltuklarının ortak girdisidir. Her koltuk yalnız kendi çıktı dosyasını yazar.
> Karar değil, **taslak** üretilir; mimari değişiklik sahibi + gerçek meclis onayı ister.

## 0. Amaç

T-103 `owner` alanını ekledi ve `give_center_to_faction_aux` satırını doğru değiştirdi; ama oyunda Praven
Swadia'da kaldı. Çünkü Native `game_start` şehri iki adımda veriyor: önce krallık, sonra lord. T-116'nın işi:
`owner` değişince merkezin **lordunu** da tutarlı hale getirmek; modcu `lord <ad>` ya da `lord auto` yazabilsin.

## 1. Doğrulanmış olgular (kit satırları)

Kaynak: `Module_system 1.171/module_scripts.py` (oyun başı `game_start` bloğu).

- `:486-507` — 22 şehir için `script_give_center_to_faction_aux`; Praven `:491` → `fac_kingdom_1`.
- `:509-563` — 48 kale için aynı çağrı.
- `:567-596` — şehir/kale için `script_give_center_to_lord`; Praven `:572` → `trp_kingdom_1_lord`.
- `:598` — `script_assign_lords_to_empty_centers`: lordu olmayan merkezlere kendi krallığından lord atar.
- `:601-607` — her merkez için `slot_center_original_faction`, `slot_center_culture`, `slot_center_ex_faction`
  o anki krallıktan türetilir.
- `:609-614` — bazı kalelerde `ex_faction` "hak iddiası" olarak elle değiştirilir (Praven değil).
- `:669-685` — duvarlı merkezlere servet + garnizon eklenir (lord atamasından sonra).
- T-116 tarifi ve kabul ölçütleri: `docs/tasks/T-116.md`.
- Praven iki adım dersi: `docs/meclis/kurucu-model-gundem.md` §2.

Proje kodu (salt okunur): `crates/uw_decomp/src/project/mod.rs` (`apply_owners`, `take_owner`, owner satırı
dışa aktarımı), `crates/uw_decomp/src/project/load.rs` (`load_with_owners`),
`crates/uw_cli/src/build_project.rs` (değişiklikleri basar).

## 2. Cevaplanacak sorular

- **Q1 — Kapsam:** T-116 tarifi tam mı? Hangi gizli adım eksik (kültür, `original_faction`, `ex_faction`/hak
  iddiası, garnizon, lordun kendi `faction` alanı, ilişki, AI, pretender)?
- **Q2 — Mekanizma:** (A) alan bazlı yama (`owner`, `lord` ayrı ayrı) mı, (B) oyun başı "initial state"
  katmanı mı? İkisinin artı/eksisi; şimdi hangisi, kurucu modelde hangisi?
- **Q3 — `lord auto`:** Çağrısı olan merkezde çağrıyı silmek yeterli mi? `assign_lords_to_empty_centers`
  gerçekten istenen lordu verir mi; hangi durumda yanlış/eksik kalır?
- **Q4 — Çelişki:** `owner` değişip `lord` eski lorda aitse otomatik `auto` + Türkçe mesaj doğru mu?
  Hangi durumda hata olmalı, hangi durumda sessiz `auto`?
- **Q5 — Merkez türleri:** Köylerde lord var mı? Hangi türler `lord` alır? Oyuncu krallığı, isyancı
  (pretender), ölü lord, aynı lorda iki merkez gibi kenar durumlar.
- **Q6 — Bayt eşitliği:** Değişmeyen projede 32/32 bayt ve 12815/12815 okuma nasıl korunur? Ekleme/silme
  sırasında hangi boşluk/sıra riskleri var?
- **Q7 — Kanıt:** Hangi testler (tiny atlas fixture, gerçek atlas, oyun deneyi)? Oyun deneyinde tam olarak
  ne görülmeli; byte testinin göremediği ne?
- **Q8 — En küçük v1:** Şimdi ne yapılmalı, ne kurucu modelin genel katmanına bırakılmalı?

## 3. Kanıt kuralı (pazarlıksız)

- Her olgusal cümle `dosya:satır` ya da komut çıktısı taşır. Taşımıyorsa `[İDDİA]` yazılır.
- Native adı **uydurulmaz**; emin değilsen "doğrulanmadı" yaz.
- Sayı ezberden yazılmaz.
- Ürün kodu, test, kit ve `.evrim` değiştirilmez. Yalnız kendi `.md` dosyan yazılır.
- `cargo`, `uw build`, `git`, `cv` çalıştırılmaz. Gerekirse salt okunur arama/okuma yapılır.

## 4. Çıktı biçimi (kendi dosyan)

```
# <Rol> — T-116
## Özet (en fazla 5 satır)
## Bulgular
| # | İddia | Kanıt (dosya:satır) | Güven |
## Öneri (madde madde)
## Açık sorular / [İDDİA]
## Kaynaklar (okuduğun dosyalar)
```

- En fazla ~120 satır. Türkçe yaz; teknik terim İngilizce kalabilir.
- Kendi dosyan dışına yazma; başka koltuğun dosyasını silme/değiştirme.

## 5. Okuma listesi

1. Bu brief.
2. `docs/tasks/T-116.md`.
3. `docs/meclis/kurucu-model-gundem.md` §2 (Praven dersi) ve §4 (envanter, yalnız ilham).
4. Kit: `Module_system 1.171/module_scripts.py` (yukarıdaki aralıklar).
5. Proje: `crates/uw_decomp/src/project/mod.rs`, `.../load.rs`, `crates/uw_cli/src/build_project.rs`.
6. Rolün gerektirdiği kadar `module_parties.py`, `module_troops.py`, `module_factions.py`.

Gereksiz yere bütün kiti okuma; dar oku.
