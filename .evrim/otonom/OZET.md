# Özet (sahibi için, sade)

## Şu an
OHAL. 6. çevrim bitti. **Dilin yeni hâli çalışıyor:** hesap, if, menü metni ve seçenekleri. Örnek mod `menu_demo` derlendi ve oyuna kuruldu; senin denemeni bekliyor. DeepSeek altın testleri yazıyor (T-019).

## Bitenler
- **Elle tutulur:** Warband başlatıcısında `menu_demo` modunu seç, yeni oyun aç, karakteri yarat. "Nereden başlamak istersin" menüsünde değişen metin, yeni adlı Praven seçeneği, yeni "Start rich" seçeneği görünmeli; Shariz kalkmış olmalı. Adımlar: docs/CHECKUP.md (Oyun deneyi kuyruğu).
- Kaynak: examples/menu_demo/menu_demo.uw (tek dosyada hepsi).
- Kılavuz yeni bölümler: 3.5 hesap, 3.6 if/and/or/not, 3.7 menü metni ve seçenekler, 3.8 yeni menü; hata kodları E0206-E0212 (docs/kilavuz/UW_KILAVUZU.md).
- Testler 141/141, Python yardımcıları 15/15.
- DeepSeek bugün ~0.5$ harcadı; işlerinin hepsi denetlendi, birinde gerçek bir hata yakalanıp düzeltildi.

## Karar verdiklerim
- Gramer 2 kararı: docs/meclis/gramer-2-oneri.md.
- OHAL: yalnız bu proje; haftalık tavan %90.
- Bağlam büyüyünce devir notu + yeni oturum (KARARNAMELER).

## Takıldıklarım
- **GitHub push'u otomatik izin denetimi reddediyor.** Son commit'ler yalnız bilgisayarında; GitHub'daki main geride. Çözüm: ya arada kendin `git push origin master:main` çalıştır ya da Claude Code izinlerine bu komutu ekle.
- 5. çevrimde özet yazarken bir ters tırnak yüzünden `git push` istemeden bir kez çalışmıştı (içerik temizdi). Ders kaydedildi.

## Harcama
Çevrim başına 5 saatlik kotanın ~%3-6'sı; haftalık %63 / tavan %90.

## Senin bakman gerekenler
1. `menu_demo`yu oyunda dene (yukarıda), sonucu docs/CHECKUP.md maddesinin altına yaz.
2. GitHub için push (yukarıda).
3. Commit e-postası `fincan106@gmail.com` GitHub'da görünür; istemezsen söyle.
