# 03-tainted-paths — elle kurulan altyapı (infrastructure scan)

Büyük bir total-conversion Module System ağacı: tek-oyunculu dünya/kontrat sistemlerinin yanında tam bir
multiplayer co-op katmanı taşıyor (mesajlarda Medieval Conquests yolları geçiyor, ör.
`mods/03-tainted-paths/src/module_coop_scripts.py:4405`). Co-op katmanı üç ayrı dosyada ~19k satır yeni
Python olarak duruyor (`module_coop_scripts.py` 6373, `module_coop_mission_templates.py` 8004,
`module_coop_presentations.py` 4556 satır); bunların import'u `module_presentations.py:9-11`'de yorumlu,
yani bu kopyada ayrı dosya olarak tutuluyorlar. Ayrıca modun kendi `module_presentations.py` dosyası
31.205 satıra çıkmış (ilk presentation rastgele-alıntı ekranı), `module_simple_triggers.py` ve
`header_operations.py` de baştan düzenlenmiş.

| # | category | pattern (generic, one line) | evidence (file:line, 1-3) | approx lines | times reused |
|---|---|---|---|---|---|
| 1 | CONFIG | Ayar sistemi yok: bir seçenek ~5 yere elle kopyalanıyor (global overlay tutamacı, create, set_val, event dalı, ağ mesajı); yeni ayar tarifi "#STEP 2 ... çakışmasın" yorumuyla anlatılıyor | `mods/03-tainted-paths/src/module_coop_presentations.py:70-73`, `:933-941`; `module_coop_scripts.py:1082-1088` | ~1000 | ~40 widget |
| 2 | UI | HUD/panel çerçevesi yok: her widget için ayrı global overlay tutamacı, adı panele+numaraya gömülü; göster/gizle elle | `mods/03-tainted-paths/src/module_coop_presentations.py:25-55`, `:104-117`, `:929` | ~300 | ~40 |
| 3 | INPUT | "Tuş basılı mı" yok: bas-tut ayrımı elle — tık tetiği + 0.0/0.3 sn gecikmeli ikinci tetik + `game_key_is_down` + global durum (`$gk_order`, `$holdit`) | `mods/03-tainted-paths/src/module_coop_mission_templates.py:2529-2546`, `:2537-2538` | ~80 | 3 |
| 4 | INPUT | Tuş ataması veri değil: tuşlar global değişkende tutulup panelden değiştiriliyor, int olarak öbür istemcilere gönderiliyor | `mods/03-tainted-paths/src/module_coop_scripts.py:242-245`, `:1082-1088`, `:1164-1166` | ~40 | 3 |
| 5 | TIMER | Zamanlayıcı yok: periyot tetik alanında, geri sayım "bitiş zamanı globali + `store_mission_timer_a`" kıyasıyla | `mods/03-tainted-paths/src/module_coop_mission_templates.py:39-48` | ~20 | 4+ |
| 6 | TIMER | Cooldown: geçen süre timer'ı globalde saklanıp aşıldığında sıfırlanıyor (müzik/tur sınırı) | `mods/03-tainted-paths/src/module_coop_mission_templates.py:1416-1426` | ~15 | 5+ |
| 7 | TIMER | Tek periyodik tarama yolu simple trigger aralığı (6.0 sn); "iş yapıldı mı" kontrolü her adımda elle | `mods/03-tainted-paths/src/module_simple_triggers.py:23-30` | ~1970 | 1 liste |
| 8 | TIMER | Presentation kare zamanı trigger parametresinden; paneli tazelemek için "kapat + yeniden başlat" hilesi | `mods/03-tainted-paths/src/module_coop_presentations.py:2913-2934` | ~20 | 6+ |
| 9 | STATE | Enum + getter/setter script çifti: durum adı yok, int + yorum (0..5); durum dosyadan da okunuyor | `mods/03-tainted-paths/src/module_coop_scripts.py:93-97`, `:627-644` | ~40 | 10+ |
| 10 | STATE | Kalıcı nesne verisi motor slotlarında: id'ler elle rezerve ediliyor ("bu değerden sonra ~5 slot ayır"), kullanım yeri yorumla eşleştiriliyor | `mods/03-tainted-paths/src/module_constants.py:32-36`, `:959-962`; `module_coop_mission_templates.py:78-80` | ~200 | 20+ |
| 11 | STATE | Dosyaya kalıcı yazma: `dict_*` op'larıyla adlandırılmış dosya (kaydet / geri yükle / sil) | `mods/03-tainted-paths/src/module_coop_scripts.py:90-97`, `:631-648` | ~30 | 10+ |
| 12 | COLLECTION | Dizi yok: slot aralığı + `(id − taban)` indeks hilesi; item/imod paralel dizileri; elle takas sıralaması | `mods/03-tainted-paths/src/module_coop_scripts.py:2365-2380`, `:3672-3673`, `:2997-3007` | ~120 | 20+ |
| 13 | COLLECTION | Liste/kuyruk yerine parti taraması: dairesel sayaç + "her partiden bir asker çek" ile seçim | `mods/03-tainted-paths/src/module_coop_scripts.py:3664-3685` | ~40 | 4 |
| 14 | EVENT | Özel olay protokolü elle: numaralanmış olay kimlikleri + iç içe if (`event_type` → `event_subtype`) + script parametre 1..6 | `mods/03-tainted-paths/src/module_coop_scripts.py:1569-1581`, `:1672-1683`; `module_constants.py:1867-1918` (bu kopyada yorumlu) | ~900 | ~40 tip |
| 15 | NETWORK | Sunucu yetkili akış: her arayüz değişikliği ayrı "send_int_to_server" mesajı, sunucu tüm oyunculara yayınlıyor | `mods/03-tainted-paths/src/module_coop_presentations.py:938-944`; `module_coop_scripts.py:1681-1688` | ~200 | 40+ |
| 16 | NETWORK | Rol/yetki kontrolü her işleyicide tekrar ediyor (`multiplayer_is_server`, `player_is_admin`, `neg|multiplayer_is_dedicated_server`) | `mods/03-tainted-paths/src/module_coop_scripts.py:1677`; `module_coop_mission_templates.py:2531` | ~10 satır her yerde | 30+ |
| 17 | TEXT | Metin birleştirme yok: şablon string + `reg`/`s` register + `str_store_string` zinciriyle etiket kurma (her seçenek için elle) | `mods/03-tainted-paths/src/module_coop_scripts.py:955-975`, `:1000` | ~120 | 40+ |
| 18 | UI | Panel düzeni elle: çalışan y imleci, sihirli sayılar, tek satır text overlay, `tf_scrollable` konteyner | `mods/03-tainted-paths/src/module_coop_presentations.py:104-116`; `module_presentations.py:31-39` | ~400 | 9 panel |
| 19 | MATH | Sabit nokta ve konum matematiği elle (`set_fixed_point_multiplier`, `val_mul`, `position_transform_position_to_parent`, mesafe kıyası) | `mods/03-tainted-paths/src/module_coop_mission_templates.py:69`; `module_coop_scripts.py:3278-3285` | ~50 | 10+ |
| 20 | AGENT | Ajan sorgusu yok: `try_for_agents` + takım/durum filtresi her seferinde yeniden yazılıyor | `mods/03-tainted-paths/src/module_coop_scripts.py:3262-3273`, `:3302-3307` | ~60 | 10+ |
| 21 | AI | Bot yönlendirme elle: hedef koordinat ajan slotuna yazılıp `agent_set_scripted_destination` ile uygulanıyor, kadro sayısı sabit sayıyla sınırlı | `mods/03-tainted-paths/src/module_coop_scripts.py:3270-3291`, `:3298-3320` | ~60 | 5+ |
| 22 | CONFIG | Motor sürümü algılama: `neg|is_vanilla_warband` koruması her scriptte tekrar; eklenti yoksa özellik sessizce atlanıyor | `mods/03-tainted-paths/src/module_coop_scripts.py:27`, `:89`, `:631` | ~5 her yerde | 15 |
| 23 | BUILD | Derleyici girdisi elle bakılıyor: genişletilmiş opcode tablosu + blok derinliği listeleri (`depth_operations`) | `mods/03-tainted-paths/src/header_operations.py:1-2`, `:3060-3071`, `:3548-3562` | 3563 | 1 |
| 24 | DEBUG | Hata ayıklama yok: iz çıktısı `display_message` + `str_store_string` satırlarını açıp kapatmakla (`#Debug`) yapılıyor | `mods/03-tainted-paths/src/module_coop_scripts.py:938-951`; `module_coop_mission_templates.py:1417-1422` | ~15 her yerde | 20+ |
| 25 | OTHER:trigger-fragments | Yeniden kullanılabilir tetik parçaları modül düzeyinde tuple olarak tanımlanıp mission template listesine ekleniyor | `mods/03-tainted-paths/src/module_coop_mission_templates.py:39`, `:56`, `:87`, `:159`, `:322-325` | ~120 | 4 parça |

## Top 5 things a language should give built in

1. Key input as first-class state: `is_down` / `pressed` / `released` on named, rebindable actions — no hand-rolled tap-vs-hold state machines.
2. Declarative UI + settings: describe a control once (label, type, min/default/max, bound key) and get the panel, persistence and network sync for free.
3. Named objects with typed fields: per-object records with automatically allocated storage — no hand-numbered slot tables, no "reserve N slots after this".
4. Real collections: lists and dicts as values, with index / sort / find / length, and save/load to a file.
5. First-class scheduling and events: named timers and cooldowns plus typed event messages with a dispatcher — no event-id tables, no nested `if` on subtype.

## Native code the mod changed

- Native multiplayer panelleri `coop_*` adıyla yeniden yazılmış (9 panel): admin panel (kit `module_presentations.py:1743` → `module_coop_presentations.py:25`), welcome (kit `:2969` → `:1736`), team select (kit `:3154` → `:1483`), stats chart (kit `:6325` → `:2261`); aynı desenle troop select (`:1094`), item select (`:2942`), escape menu (`:1895`); iki panel de yeni (`:1251`, `:1700`).
- Native `simple_triggers` listesi baştan yazılmış: mod listesi 6.0 sn'lik faction-state taramasıyla başlıyor (`module_simple_triggers.py:22-30`), kit'te listenin başı farklı (`kit/module_simple_triggers.py:22-24`).
- Native opcode başlığı genişletilmiş tabloyla değiştirilmiş (`header_operations.py:1-2`, `:3548-3562`).
- Native başlangıç presentation'ı rastgele-alıntı ekranıyla değiştirilmiş (`module_presentations.py:22-39`; kit'te aynı isim yok).
- Native multiplayer mission-template tetik parçaları ve "common" script çağrıları bilinçli olarak devre dışı bırakılmış (`module_coop_mission_templates.py:328-330`, `:349-350`).
