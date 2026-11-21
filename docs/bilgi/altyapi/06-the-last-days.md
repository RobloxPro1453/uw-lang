# Altyapı taraması: 06-the-last-days

**Mod:** The Last Days (TLD) — Mount&Blade/Warband için Orta Dünya tam dönüşümü; tek kaynak hem 1.011 hem Warband'e derlenir (`module_info.py:9-17`). 25+ Python modül dosyası, ~110 bin satır (ana dosyalar: `module_scripts.py` 35.2k, `module_dialogs.py` 15.5k, `module_game_menus.py` 14.5k) ve yüzlerce script girdisi; ayrıca FormAI/PBOD gibi hazır alt sistemleri kendi `module_scripts_*` / `module_mission_templates_*` dosyalarına kopyalayıp genişletmiş. Kanıt yolları `target/ref/mods/06-the-last-days/ModuleSystem/` altındadır; kit `target/ref/kit/` ile karşılaştırılarak "mod ekledi" ayrımı yapıldı. Satır ve kullanım sayıları yaklaşıktır.

## Genel altyapı kalıpları

| # | category | pattern (generic, one line) | evidence (file:line, 1-3) | approx lines | times reused |
|---|---|---|---|---|---|
| 1 | STATE | Her nesne türü için elle numaralanmış slot kaydı: alan = sabit tamsayı; tip yok, çakışan numaralar takma adla yeniden kullanılıyor | mods/06-the-last-days/ModuleSystem/module_constants.py:176-260, mods/06-the-last-days/ModuleSystem/module_constants.py:403-455 | ~800 | tüm modüller |
| 2 | COLLECTION | Parti slotlarını dizi/yığın olarak kullanan genel kütüphane (oluştur/sil/uzunluk/başlat/sırala/rastgele); slot 0 = eleman sayısı | mods/06-the-last-days/ModuleSystem/module_scripts_common_warp.py:29-44, mods/06-the-last-days/ModuleSystem/module_scripts_common_warp.py:113-125 | ~700 | ~40 script |
| 3 | COLLECTION | Sabit boyutlu diziler ardışık slot numaralarıyla taklit ediliyor; begin/end sabitleri ve 9'luk "stride" ile ikinci boyut | mods/06-the-last-days/ModuleSystem/module_constants.py:586-606, mods/06-the-last-days/ModuleSystem/module_constants.py:2803-2863 | ~120 | onlarca alan |
| 4 | INPUT | "Basılı tutuldu / bırakıldı" olayı yok: 0,5 sn'lik sayaç trigger'ı + basılı-tutma bayrağı + ayrı bırakma trigger'ı ile debounce | mods/06-the-last-days/ModuleSystem/module_mission_templates_TLD.py:30-44, mods/06-the-last-days/ModuleSystem/module_mission_templates_TLD.py:47-63 | ~45 | 2 emir script'i |
| 5 | UI | Presentation kalıcı HUD katmanı olarak kullanılıyor; overlay tutamaçları boş bir troop'un slotlarında dizi gibi saklanıyor | mods/06-the-last-days/ModuleSystem/module_presentations.py:6601-6607, mods/06-the-last-days/ModuleSystem/module_presentations.py:6659 | ~170 | 1 HUD |
| 6 | CONFIG | Çalışma-zamanı ayar ekranı: check/number-box overlay'leri global değişkene bağlanır, event_state_change geri yazar | mods/06-the-last-days/ModuleSystem/module_presentations.py:6774-6797, mods/06-the-last-days/ModuleSystem/module_presentations.py:6875-6892 | ~120 | 2 ekran |
| 7 | TIMER | Saatlik aralık trigger'ı + nesnede "şu saate kadar" mutlak saati tutup akım saatle karşılaştırma | mods/06-the-last-days/ModuleSystem/module_simple_triggers.py:30-38, mods/06-the-last-days/ModuleSystem/module_constants.py:448-449 | ~10/kullanım | birçok yerde |
| 8 | TIMER | Kişi başına cooldown/sayaç slotu (agent, faction, troop) | mods/06-the-last-days/ModuleSystem/module_constants.py:204, mods/06-the-last-days/ModuleSystem/module_constants.py:362, mods/06-the-last-days/ModuleSystem/module_constants.py:927 | ~5 | 3+ |
| 9 | EVENT | Kendi olay kuyruğu: üretici troop slotlarına yazar, 0 aralıklı trigger her turda birini çekip kalanları kaydırır | mods/06-the-last-days/ModuleSystem/module_simple_triggers.py:98-118 | ~21 | 3 kuyruk |
| 10 | BUILD | Python tarafında dosya parçalama + derleme hedefi anahtarıyla (WB/1.011) koşullu import | mods/06-the-last-days/ModuleSystem/module_scripts.py:18-25, mods/06-the-last-days/ModuleSystem/module_mission_templates_TLD.py:12-15 | ~20 | 6 dosya |
| 11 | BUILD | Derleme anında opcode bloğu üreten Python fonksiyonları: veri tablosundan binlerce satırlık toplu init ve bitfield | mods/06-the-last-days/ModuleSystem/module_scripts.py:35-47, mods/06-the-last-days/ModuleSystem/module_scripts.py:50-69 | ~60 | 3 üretici |
| 12 | BUILD | Bitfield sabiti üreten yardımcı fonksiyonlar (silah yeterliliği bayrakları) | mods/06-the-last-days/ModuleSystem/module_troops.py:40-57 | ~18 | tüm troop kayıtları |
| 13 | DEBUG | Derleme anahtarına bağlı koordinat/teşhis presentation'ı; yorum satırına alınmış hata ayıklama kısayolları | mods/06-the-last-days/ModuleSystem/module_presentations.py:27-53, mods/06-the-last-days/ModuleSystem/module_triggers.py:35-36 | ~30 | 1 |
| 14 | AI | Merkezî AI zamanlayıcısı: tiyatro döngüsü + "şu saate kadar scriptli" slotuyla faction AI'sini askıya alma | mods/06-the-last-days/ModuleSystem/module_scripts_ai.py:20-39 | ~40 | her saat |
| 15 | TEXT | Adlandırılmış yer tutuculu ({reg14}, {s14}) biçim dizeleri; birleştirme API'si yerine sabit format + str_store | mods/06-the-last-days/ModuleSystem/module_constants.py:35-40, mods/06-the-last-days/ModuleSystem/module_strings.py:3-25 | ~40 | onlarca mesaj |
| 16 | TEXT | String dizisi yerine s1..s5 register'ları arasından döngüyle seçim | mods/06-the-last-days/ModuleSystem/module_presentations.py:6633-6660 | ~28 | 1 |
| 17 | MATH | Yüzde buff/debuff tablosunu kod değil veri olarak tutmak | mods/06-the-last-days/ModuleSystem/module_constants.py:2967-2980 | ~14 | etki sistemi |
| 18 | UI | Python'da üretilen sabitler + kod parçacıklarıyla sayfalı liste menüsü | mods/06-the-last-days/ModuleSystem/module_game_menus.py:44-58 | ~15 | 1 menü |
| 19 | STATE | Tek slotta bitmask ile yara/ölüm durumu | mods/06-the-last-days/ModuleSystem/module_constants.py:801-806 | ~6 | tüm kahraman yara kontrolleri |
| 20 | AGENT | Agent'tan agent'a geri referans slotları (binek ↔ binici) | mods/06-the-last-days/ModuleSystem/module_constants.py:210-213, mods/06-the-last-days/ModuleSystem/module_constants.py:303-304 | ~6 | binek mekanikleri |
| 21 | STATE | Eski kayıtlarla uyum için global'e doğrudan erişmeyip tembel kuran erişim script'i | mods/06-the-last-days/ModuleSystem/module_scripts_common_warp.py:84-106 | ~12 | 1 |

## Top 5 things a language should give built in

1. Her nesne türüne (agent, parti, şehir, troop, faction, prop) tip denetimli, otomatik ayrılan alanlar; elle slot numarası kaydı, takma ad ve çakışma derdi olmasın.
2. Birinci sınıf büyüyebilir koleksiyonlar (liste, sözlük, yığın, sıralama, rastgele seçim); "boş parti doğurup slotla bellek ayırma" hilesi ve çöp partisi temizliği olmasın.
3. Olay tabanlı girdi: "basıldı / basılı tutuldu / bırakıldı" geri çağrıları; kenar algılama ve debounce kütüphanede olsun.
4. Veriden üretilen, duruma bağlı arayüz: HUD ve ayar ekranları; overlay tutamacı elle saklanmasın, bileşen değeri duruma bağlansın.
5. Zamanlayıcı ve kuyruk ilkelleri: "her N saat", "şu saate kadar bekle", kişi başına cooldown ve olay kuyruğu; her şey trigger yoklamasıyla kurulmasın.

## Native code the mod changed

- Native `anyone|start` diyalog girişi baştan yazıldı; ilişki/parti/zaman önbellekleri script çağrılarına taşındı (`module_dialogs.py:52-92`).
- Beceri tablosu yeniden yazıldı: adlar/anlamlar değişti, `engineer` etkisizleştirildi, `reserved_*` yuvalar korundu (`module_skills.py:41-84`).
- Native görev listesi budandı; kullanılmayan görevler yorum satırına alındı (`module_quests.py:13-45`).
- Menü dosyasına üretilmiş sabitler/kod parçacıkları eklendi (sayfalama, şehir arka planı, metin rengi) (`module_game_menus.py:44-58`).
- `module_items.py` ve `module_troops.py` dışarıdaki bir üreticiyle yeniden üretilmiş dosyalar; `#OUTPUT` işaretleri duruyor (`module_items.py:1`, `module_troops.py:1`).
