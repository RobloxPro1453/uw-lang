# A2 — Genel beceri (skill) kalıbı, 6 örnek mod + Native
> Kanıt: `Module_system 1.171/header_skills.py:1-6,9-41,474,52-471`; `ID_skills.py:1-42`; `module_skills.py:5-10,38-39,41-84`; `kit-meta/records/skills.toml:5-39`; `crates/uw_decomp/tests/readable_t062.rs:100-111`; `crates/uw_cli/src/model.rs:104`; `kit-meta/aciklama/ops.json:3504,6308,7071`; `ornek-modlar/{1257AD,catplomacy,Diplado,medieval-conquests,Persistent-Kingdoms,pw_module_system}` skill dosyaları (sadece okundu, kopyalanmadı); `light-and-darkness-report/{README.md,sources.md,mekanik-01.md,mekanik-06.md,mekanik-10.md}` (sadece okundu, oyun-gözlem tabanlı)
> Beyan: analizciyim (A2), bulgularımı kimseye söylemedim; `gizli/ANALIZ-A1.md` (A1 çıktısı) okumadım, kendi analizimi çıkardım.

## 1. Teşhis (2-5 cümle)

Altı modda beceri (skill) modeli Native'nin `module_skills.py` alan düzeniyle aynı: `(id, isim, flags{temel_nitelik | parti_etkisi | pasif}, max_seviye, açıklama)`. Dört mod (1257AD, catplomacy, Diplado, medieval-conquests) Native'nin 21 sabit becerisiyle birebir aynı; değişiklik yok. Persistent-Kingdoms ve pw_module_system ayrıca `reserved` slot'lara `labouring`, `sailing`, `tailoring`, `herding` ekler (PK'ye ek olarak `musician`). Bu dört beceri iki modda tekrar eder; ortak desen: "yeni, motorda etkisi yok olan bir beceri = reserved slot'u almak, demetle tanımlamak, etkisini script'te seviye okunarak uygulamak".

## 2. Öneri — mekanizma düzeyinde

UW'de beceri **bildirimi** (kayıt) zaten ifade edilebilir: `skills.toml` Native'nin `module_skills.py` alan düzenini aynen taşır. Eksik olan genel ilkel: **herhangi bir becerinin (özellikle reserved/özel) seviyesine bağlanmış, derleyiciye yeni alan/özellik eklenmeden çalışan "beceri-etki / koşullu bonus" bağlama mekanizması**. Modcu şunu ifade edebilmeli: "beceri X seviye N olduğunda etki Y" — mevcut UW ilkeleriyle (kayıt + işlem + koşul bileşimi) henüz tek bir genel ilkel olarak yok.

## 3. Nasıl çalışır (modcu ne yazar, derleyici ne üretir)

Modcu `skills` kaydına yeni bir satır yazar (reserved id veya yeni id), flags ile temel nitelik (str/agi/int/cha) ve parti/pasif bitini seçer, `max_seviye` ve `açıklama` girer. Derleyici bunu `skills.txt` atlas kaydına ve `knows_*_N` bit masasına dökümler (`header_skills.py:52-471`, `num_skill_words = 6` satır 474). Etki ise `module_scripts.py`'deki `store_skill_level` / `party_get_skill_level` / `troop_raise_skill` işlemleriyle (`ops.json:3504,6308,7071`) okunup script koşulunda uygulanır. Sabit 21 becerinin etkisi motorda sabittir, sadece `sf_inactive` ile kapatılabilir (`module_skills.py:38-39`).

## 4. Neyi ifade EDEMEZ (dürüst sınır) + eksik genel ilkel

- Sabit 21 becerinin motor etkisini değiştirmek/kaldırmak: sadece `sf_inactive` ile devre dışı bırakılabilir (`module_skills.py:38-39`).
- Reserved/özel bir becerinin "seviye N → etki E" bağlamını derleyiciye dokunmadan tanımlamak: UW `skills.toml` sadece bildirimi (id, isim, flags, max_seviye, açıklama) tutar; etki bağlama yok.
- Eksik genel ilkel: **beceriye bağımlı koşullu etki/bonus ilki** (skill-bound conditional effect), her beceri için geçerli, yeni alan/özellik istemeden.

## 5. Tekrar eden genel beceriler (Native yapı + UW durumu)

| Genel beceri | Modlarda (tekrar) | Native yapı (dosya:satır) | UW'de bildirim | UW'de etki | Eksik ilkel |
|---|---|---|---|---|---|
| labouring | PK, PW | reserved slot, `sf_base_att_str`, `module_skills.py:42-84` demet | Evet (`skills.toml`) | Hayır | beceri-etki bağlama |
| sailing | PK, PW | reserved slot, `sf_base_att_str` | Evet | Hayır | beceri-etki bağlama |
| tailoring | PK, PW | reserved slot, `sf_base_att_agi` | Evet | Hayır | beceri-etki bağlama |
| herding | PK, PW | reserved slot, `sf_base_att_agi` | Evet | Hayır | beceri-etki bağlama |
| 21 sabit beceri | 6 mod | `module_skills.py:42-79`, `header_skills.py:9-41` | Evet | motor-sabit, sadece `sf_inactive` ile kapanır | — |

`musician` sadece PK'de (tek mod), tekrar etmediği için genel kalıp dışı. `medieval-conquests` Native'den yalnız `reserved_14..18`'de `sf_inactive`'yi tek başına kullanır (`module_skills.py:116-124`), beceri sayısında değişiklik yok.

## 6. Kanıt / [İDDİA] ayrımı

Native iddiaları yukarıda `dosya:satır` ile gösterildi. UW tarafında `skills.toml` alan eşleşmesi ve decomp testleri (`readable_t062.rs`) okundu; "UW'de etki bağlama yok" iddiası model incelemesine dayanıyor ancak tüm koşul/etkin dili taranmadığı için **[İDDİA]**. Mod tarafındaki custom beceri isimleri (`labouring`, `sailing`, `tailoring`, `herding`, `musician`) mod dosyalarından okundu, kopyalanmadı. Sayılar (42 beceri, 10 seviye, 6 kelime) dosya içeriğinden alındı, ezberden yazılmadı.

## 7. Karşı çıktığın yaygın varsayım

"Yeni beceri eklemek = yeni bir beceri alanı/modülü eklemek" yanlış: Native'de zaten `reserved` slot var ve beceri etkisi scriptte yaşar. Bunu "labouring modülü" diye ele almak örnek=amaç hatasıdır (BRIEF §4). Genel etki-bağlama ilki eksikse, her yeni meslek için yeni alan açmak zorunlu kalır — bu, Yön 2'nin "alan alan" hastalığının aynen tekrarı olur.