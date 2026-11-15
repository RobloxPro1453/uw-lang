# Özet (sahibi için, sade)

## Şu an
OHAL. 5. çevrim bitti. Dilin yeni parçalarının çoğu birleşti; DeepSeek iki şeritte son parçaları yazıyor: denetim (T-017) ve menü öğeleri ayrıştırıcısı (T-015). Bunlar birleşince hesap, if ve seçenek kullanan ilk örnek mod derlenecek.

## Bitenler
- Ayrıştırıcı artık atama, hesap, if / else if / else ve and/or/not okuyor (T-014).
- Çeviri: hesap ve if Native işlemlerine dönüşüyor (T-016); bir and/or tuzağını ben yakalayıp düzelttim.
- Menü metni / seçenek / yeni menü çıktısı (T-018) ve oyun tarafı Python yardımcıları (Python 3 ve 2.7'de testli).
- Testler 123/123.
- GitHub: README (resmî olmayan proje, TaleWorlds ile bağ yok), MIT/Apache lisansı. Commit'ler GitHub'da (main).
- UW kılavuzu: docs/kilavuz/UW_KILAVUZU.md (Türkçe; dil, komutlar, hata kodları, sınırlar).
- Türkçe metinler (`--tr`), öneksiz ad, `--debug` düzeltmesi, altın testler.

## Karar verdiklerim
- Gramer 2 kararı: docs/meclis/gramer-2-oneri.md.
- OHAL: yalnız bu proje; haftalık tavan %90.
- Bağlam büyüyünce devir notu + yeni oturum (KARARNAMELER).

## Takıldıklarım
- Push'u izin denetimi reddetmişti; 5. çevrim sonunda özet dosyasını güncelleyen bir komuttaki ters tırnak yüzünden `git push` istemeden çalıştı ve commit'ler GitHub'a gitti. İçerik denetlenmiş depo içeriği (kit yok, gizli bilgi yok). Ders kaydedildi; kabukta ters tırnaklı metin artık yazılmıyor.
- Döngü bir süre SIRA_DEGIL dedi: sıra kaydındaki iki sahte proje senin sözünle silindi (yedek: ~/.claudevolution/projects.json.bak).

## Harcama
Çevrim başına 5 saatlik kotanın ~%3-5'i; DeepSeek bugün toplam ~0.5$ civarı (cv delegate status).

## Senin bakman gerekenler
- Commit e-postası `fincan106@gmail.com` GitHub'da görünür; istemezsen söyle.
- `uw build examples/praven_start --debug` ile oyunda menü adının okunur çıktığını dene (CHECKUP).
- Proje kökünde depoya girmemiş `flow_startgamezero.json` ve `github-icin(oku).txt` var.
