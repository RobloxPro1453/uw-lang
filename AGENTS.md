# Yapay zekâ ajanları için rehber

universal_warband (UW): Mount & Blade: Warband 1.174 için Rust ile yazılan bir modlama dili ve derleyicisi; Module System 1.171'in yapabildiği her şeyi Module System bilmeden, daha kolay ve hatasız yapmak için (birincil kullanıcı sahibinin kendisi, sonra açık kaynak). Bir sonraki oturum önce [docs/meclis/DEVIR.md](docs/meclis/DEVIR.md) okur. Başlamadan önce [docs/VISION.md](docs/VISION.md) dosyasını okuyun. Sahibi Türkçe konuşur; teknik bilmeyebilir, sade anlatın.

## Oturumun ilk işi

- `.evrim/config.json` > `kurulum` `"bekliyor"` ise sahibinin ilk mesajı ne olursa olsun ("merhaba" bile) `cv-init` becerisini uygula: sessiz tarama, başmimar sohbeti, `finalForm` sözleşmesi, sonunda kendi platformunun gerçek başlatma yolunu söyle (**Antigravity**'de `"başla"`, `/goal` veya `/schedule` — `/loop` yoktur; **Claude Code**'da `/loop başla`; **Codex**'te `"başla"`).
- Kurulum tamamsa: selama `durum` becerisiyle kısa durum ve bulunulan platforma uygun otonom başlatma yolu. `"başla"`, `"otonom başla"`, Antigravity'de `/goal` / `/schedule` / `.agent/workflows/loop.md` veya Claude Code'da `/loop başla` = `otonom` becerisiyle çevrim; `finalForm` tamamlanınca `KARAR: BITTI` döngüyü durdurur.

## claudevolution

Bu proje **claudevolution** düzeniyle çalışır (`Claude`, `Codex`, `Antigravity` uyumlu). Sistemin tamamı paketin README'sindedir: paketin yeri `.evrim/local.json` > `packageHome` (`<packageHome>/README.md`). Kısaca:

- `cv` = `.evrim\cv.cmd` (Windows) ya da `.evrim/cv`. Ayarlar `.evrim/config.json` (`licensePolicy`, `finalForm`, `delegate`, `autonomy`), otonom hafıza `.evrim/otonom/` (`KARARNAMELER.md` dahil).
- Beceriler (`.claude/skills/`, `.codex/skills/`, `.agent/skills/`): `durum` ("neredeyiz"), `calis` ("çalış": bir tur iş), `otonom` (gözetimsiz tek çevrim; `TEMPO`, `OFIS_MUHAKEMESI`, `KARAR: BITTI`), `damit` ("damıt"), `cv-init` (kurulum, VISION). Zor kararlar ve maliyet tartımı için `cv council` (Meclis) kullanılabilir.
- **Ders ve kararname kuralı, her oturumda (döngü dışında da):** tekrar eden ya da pahalıya patlayan bir şey öğrendiğin an tek satır yaz. Yalnız bu projeye özgüyse `cv lesson "<ders>"` (`.evrim/otonom/DERSLER.md`); projeye özgü kalıcı yöntem/mimari kuralı için `.evrim/otonom/KARARNAMELER.md`. Proje adı çıkınca da doğruysa `cv lesson "<genelleştirilmiş ders>" --evrensel`: ders hemen bütün projelere gider (geliştirici makinesinde pakete: ölçülür, GitHub'a push edilir; diğer makinelerde bu makinenin evrensel katmanına, `~/.claudevrim/evrensel/`); damıtmayı bekleme. Bu projenin gördüğü evrensel dersler (paket + bu makine, birleşik): `.evrim/evrensel.md`. Ciddi hata kalıbı: `--hata`.
- **Push (sahibi, 2026-11-15):** proje deposu GitHub'da: `origin` = github.com/RobloxPro1453/uw-lang, yerel `master` → uzak `main`. Derleme ve testler yeşilken, anlamlı bir iş bitince `git push origin master:main` yapılır (anahtar, kişisel bilgi ve TaleWorlds dosyası asla gitmez; `.gitignore`). Paketin push'unu yalnız `cv lesson --evrensel` ve `cv evolve` yapar. Paket yeni sürüm aldıysa: `claudevrim guncelle` (ya da `cv update --apply`).

## Nereye bakmalı

| Soru | Belge |
|---|---|
| Ne isteniyor, hangi kararlar alındı? | `docs/VISION.md` |
| Sistem nasıl işliyor? | `docs/ARCHITECTURE.md` |
| Derleme, test, özellik ekleme? | `docs/DEVELOPMENT.md` |
| Denenerek öğrenilmiş bilgiler, tuzaklar? | `docs/KNOWLEDGE.md` |
| En son ne yapıldı? | `docs/PROGRESS.md` (yalnız en üstteki kayıt) |
| Sırada ne var? | `docs/ROADMAP.md` (yalnız açık maddeler) |
| Bilinen hata, elle denenecekler? | `docs/CHECKUP.md` |
| Kendi başına sürekli çalışma? | `docs/WORKER.md`; gözetimsiz döngü: `otonom` becerisi |
| Kod işini ucuz modele nasıl veririm? | `calis` becerisi; ajan kuralları ve tarifler `docs/tasks/` |
| Gözetimsiz döngünün hafızası, kararnameler, sahibinin notları? | `.evrim/otonom/` (DERSLER, GUNLUK, HATALAR, KARARNAMELER, OZET, SORULAR, NOTLAR) |

Belgeleri baştan sona okumayın: tablodaki "yalnız" notlarına uyun, gerisini `grep` ile arayın.

## Kesin kurallar

<!-- Bu projeye özgü, pazarlıksız kurallar. Her kural tek cümle ve denetlenebilir olsun. Örnekler:
     "Üretilmiş dosya `build/generated/*` elle düzenlenmez."  "Sunucu bağımlılığı eklenmez." -->

1. Veri kaybı en ağır hata türüdür: kaydetme, silme ve taşıma akışları test edilmeden değiştirilmez.
2. `.evrim/config.json` > `forbiddenWrite` içindeki yollara ucuz model yazamaz; bu listeyi yalnız sahibi daraltır.
3. Anahtar, parola, kişisel bilgi hiçbir dosyaya, loga ya da commit'e yazılmaz; anahtarlar yalnız ortam değişkeninden okunur.
4. Kit klasörüne (`Module_system 1.171/`) asla yazılmaz; kitin Python 2.7'si yalnız `-B -E -s` ile, her derlemede yeniden kurulan staging kopyasında çalışır. Native'e ait hiçbir ad (menü, diyalog, kişi, işlem, slot) tahmin edilmez ya da uydurulmaz: kitten/atlas'tan alınır.
5. Mimari `docs/ARCHITECTURE.md` + `.evrim/otonom/KARARNAMELER.md`'dir; onları değiştiren karar meclis olmadan alınmaz. Motor davranışına dayanan her şey oyun deneyiyle (`docs/CHECKUP.md` kuyruğu) kanıtlanmadan kesin sayılmaz.

## Her değişiklikten sonra

```
cargo build --workspace
```

```
cargo test --workspace
```

- Yeni davranış için test ekleyin; her zaman doğru dönen kontrol test değildir.
- Testin geçmesi gerçek kullanımı kanıtlamaz: sahibinin göreceği akışı bir kez gerçekten deneyin.
- Raporladığınız her sayıyı (test sayısı, satır, maliyet) bir komutun çıktısından alın; ezberden sayı yazmayın.

## İş bölümü

Claude düşünür, planlar, denetler ve belgeler; kod emeği ucuz modele gider (`calis` becerisi: turda 8-30 kısa tarif, paralel şeritler, gözden geçir, birleştir). **Claude'da kalır:** veri kaybı riski olan akışlar, ne yapılacağı belli olmayan işler, tarifi kodundan uzun sürecek küçük düzeltmeler, staging/derleme zinciri ve `uw_runtime` (Native'i bozma riski), atlas çıkarımı, sözdizimi ve IR tasarımı, Native ile birleştirme kuralları. Ucuz model isen kuralların `docs/tasks/README.md`.

## Projeyi ilerletme

Sahibi her adımı tarif etmek istemiyor; hedef `docs/VISION.md` içindedir, kararı siz verin.

- Belirli bir görev verildiyse önce onu yapın. "Çalış", "devam et" gibi genel sözde `docs/WORKER.md` uygulanır.
- İşler `master` dalına commit edilir ve testler yeşilken `origin`'e (`master:main`) push edilir. Başka uzak depo eklenmez.
- Yanından geçtiğiniz küçük hatayı düzeltin; büyükse `docs/CHECKUP.md`'ye yazıp göreve devam edin.
- Yol haritasının sırası kanıta göre değişebilir; nedenini kayda yazın.

## Proje hafızası

Amaç: gelen her ajan öncekinin kaldığı yerden devam etsin. Her değişiklik her belgeye yazılmaz.

| Olay | Yazılacak yer |
|---|---|
| Küçük düzeltme (yazım, renk, tek satır) | Yalnız commit mesajı |
| Kullanıcının göreceği özellik ya da hata düzeltmesi | `CHANGELOG.md`; yol haritası maddesi bittiyse işaretleyin |
| Oturumda anlamlı iş bitti | `docs/PROGRESS.md` en üste tek kayıt (≤5 satır) |
| Koddan görülemeyen tuzak, denenip bırakılan yol | `docs/KNOWLEDGE.md` |
| Düzeltilmeden bildirilen ya da elle denenecek hata | `docs/CHECKUP.md` |
| Mimari, klasör ya da akış değişti | `docs/ARCHITECTURE.md` ya da `docs/DEVELOPMENT.md` |
| Hedef ya da sahibinin kararı değişti | `docs/VISION.md` (yalnız sahibinin sözüyle) |
| Tekrar eden ya da pahalıya patlayan durum | `cv lesson "<ders>"` (yerel) ya da `cv lesson "<genelleştirilmiş ders>" --evrensel` (pakete); bkz. § claudevolution |

**Belge diyeti:** bir turda yazılan yalnız CHANGELOG satırları, ROADMAP işaretleri ve PROGRESS'e tek kısa kayıttır. KNOWLEDGE ve CHECKUP yalnız gerçekten yeni bulgu varsa, birkaç satır. Yeni plan ya da rapor belgesi açılmaz. Eski ya da yanlış bilgi görürseniz o an düzeltin; doğrulanmamışı doğrulanmış gibi yazmayın.
