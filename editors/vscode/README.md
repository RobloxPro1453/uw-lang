# UW için VS Code eklentisi

## Kurulum (tek komut)

UW klasöründe:

```
python editors/vscode/pack.py
code --install-extension editors/vscode/uw-lang-0.1.0.vsix --force
```

Sonra VS Code'u yeniden başlat (ya da Ctrl+Shift+P → "Developer: Reload Window"). Bir `.uw` dosyası
açınca sağ altta **UW** yazmalı ve renkler görünmeli.

**Not:** klasörü elle `~/.vscode/extensions` içine kopyalamak yeni VS Code sürümlerinde çalışmaz
(VS Code onu kendi listesine kaydetmez, "Plain Text" kalır). Yukarıdaki `.vsix` yolunu kullan.


`.uw` dosyalarını renklendirir:

- anahtar kelimeler: `mod`, `override`, `in`, `new`, `troop`, `like`, `on`, `fn`, `save`, `every`,
  `if`, `else`, `for`, `require`, `attempt`, `option`, `when`, `after`, `before`, `text`, `change`,
  `remove`, `do`
- metinler (`"..."`), yorumlar (`//`, `#`, `/* ... */`) ve sayılar
- `$global` ve `:local` adları
- noktalı nesne adları (`player.gold`) ve operasyon çağrıları (`display_message(...)`)

Ayrıca `{ }` / `( )` parantez eşleme ve `//` ile satır yorumu açma-kapama çalışır; `/* */` blok
yorumu da tanımlıdır.

## Kurulum

Eklenti yalnız bu klasörden (yerel olarak) kurulur; Marketplace'te yayınlanmış sürümü yoktur. İki yol
vardır, ikisi de VS Code'u yeniden başlatmayı gerektirir.

### 1) Doğrudan klasörü kopyalayarak (internet gerekmez)

1. `editors/vscode/` klasörünü VS Code eklenti klasörüne `uw-lang-0.1.0` adıyla kopyalayın:
   - Linux/macOS: `~/.vscode/extensions/uw-lang-0.1.0/`
   - Windows: `%USERPROFILE%\.vscode\extensions\uw-lang-0.1.0\`
2. VS Code'u kapatıp açın.
3. Herhangi bir `.uw` dosyası açın: sağ altta dil "UW" görünür (`Ctrl+K M` ile de seçilebilir).

Kopyalama örneği (proje kökünden):

```sh
mkdir -p ~/.vscode/extensions
cp -r editors/vscode ~/.vscode/extensions/uw-lang-0.1.0
```

### 2) `.vsix` paketi üretip kurarak

Paketleme `npx` ile `@vscode/vsce` aracını indirir, yani **internet gerekir**. `editors/vscode/`
klasörünün içinde:

```sh
cd editors/vscode
npx @vscode/vsce package
code --install-extension uw-lang-0.1.0.vsix
```

`vsce package` bir `publisher` alanı ister; `package.json` içinde yer tutucu olarak `uw-lang`
yazılıdır, kendi yayıncı kimliğinizle değiştirebilirsiniz. Bu alan bir sır değildir; hiçbir anahtar
veya token bu klasörde tutulmaz.

## Hataları görme

Eklenti, `uw check --json <klasör>` komutunu `.uw` dosyası **açılınca ve her kaydedilişte** çalıştırır;
bulunan her hata/problems panelinde kırmızı dalgalı çizgi olarak görünür. Hatayı düzeltip dosyayı
kaydettiğinizde çizgi kaybolur.

- Klasör: dosyanın en yakın `uw.toml` içeren üst klasörü; yoksa dosyanın kendi klasörü.
- `uw` komutu `PATH` üzerinde olmalıdır (`cargo install --path crates/uw_cli`). Başka bir yerde ise
  ayarlardan yolunu verin: `uw.path` (varsayılan `uw`), yani `settings.json` içinde
  `"uw.path": "/tam/yol/uw"`.
- `uw` bulunamazsa eklenti çökmez; bir kez "uw bulunamadı" uyarısı gösterir.
- `uw check` henüz kurulu değilse hiçbir tanı üretilmez (çıktı boş sayılır).

## Otomatik tamamlama

`.uw` dosyasında yazarken eklenti `uw words --json <klasör>` komutunu çalıştırır ve projenin
adlarını önerir; her öneri tek satırlık bir ipucu taşır:

- **oyun adları** (`praven`, `swadia`, `king_harlaus`): projenin `names.uw` dosyasından, yoksa
  Native'in görünen adlarından `uw new` kuralıyla türetilir (ipucu, kaydın görünen adı ve türüdür);
- **kelimeler**: `lib/*.uw` ve projenin `fns/*.uw` dosyalarındaki `fn` adları (`say`, `give_gold`);
- **nesne alanları** (`player.gold`): adı `fn player.gold.get` / `.add` / `.remove` / `.set` olan
  fn'lerden türetilir;
- **anahtar kelimeler** (`mod`, `override`, `fn`, `if`, `option`, …) ve Türkçe ipuçları.

Klasör kuralı "Hataları görme" bölümündeki gibidir; komut `uw.path` ayarını kullanır. Liste klasör
başına **bir kez** çalışır: `names.uw` ya da `lib/` / `fns/` altındaki bir dosya kaydedilince
tazelenir. `uw` bulunamazsa tamamlama sessizce boş kalır (uyarıyı "Hataları görme" verir).

## Üstüne gelince açıklama

Bir adın üstüne gelince (imleci üstünde tutunca) kısa bir Türkçe açıklama görünür: ne olduğu, Native
kimliği (`p_town_6`) ve nerede tanımlı olduğu — `uw explain` ile aynı bilgi, markdown olarak.

- Ad, imlecin altındaki kelimedir: `praven`, `king_harlaus`, `$g_player_luck`, noktalı nesne adı
  (`player.gold`), `#dup` eki (`menu_camp#1`) ve `fn` kelimeleri (`say`).
- Uzun metin `uw explain --json <ad> <klasör>` çıktısından gelir ve ad başına **bir kez** çalışır
  (önbellek). `uw` adı bilmiyorsa (yerel bir `fn` gibi) "Otomatik tamamlama" listesindeki tek
  satırlık ipucu gösterilir.
- `names.uw`, `lib/` ya da `fns/` altındaki bir dosya kaydedilince iki önbellek de tazelenir.
- Klasör kuralı ve `uw.path` ayarı yukarıdaki bölümlerdeki gibidir. `uw` bulunamazsa hover sessizce
  boş kalır (uyarıyı "Hataları görme" verir).

`uw explain --json <ad>` tek bir JSON nesnesi yazar:
`{"name","kind","native_id","text"}` (`native_id`, Native'de karşılığı olmayan `$global` /
operasyon / sabit için `null`; `text` düz `uw explain` çıktısıdır).

## Denetim

Söz dizimi dosyalarının (`syntaxes/uw.tmLanguage.json`, `language-configuration.json`,
`package.json`) geçerli JSON olduğunu ve yukarıdaki anahtar kelimelerin hepsinin dil bilgisinde
bulunduğunu proje kökünde şu test kontrol eder:

```sh
cargo test -p uw_cli --test vscode_grammar
```

Test Node.js gerektirmez.

`extension.js` içindeki saf fonksiyonlar (`parseCheckOutput`, `parseWords`, `hoverMarkdown`) ayrıca
`node` ile denenir; bir örnek satır yanlış aralığa düşerse ya da hover metni yanlış çıkarsa test
sıfırdan farklı çıkış kodu verir:

```sh
node editors/vscode/test/parse.test.js
node editors/vscode/test/words.test.js
node editors/vscode/test/hover.test.js
```

## Dosyalar

- `package.json`: eklenti bildirimi (ad `uw-lang`, `.uw` uzantısı, dil bilgisi yolu, `main`, `uw.path`)
- `extension.js`: `uw check --json` çalıştırıp tanıları (dalgalı çizgi + Problems) gösteren,
  `uw words --json` ile otomatik tamamlama ve `uw explain --json` ile hover açıklaması sunan uzantı
- `test/parse.test.js`: `parseCheckOutput` testi (düz `node`, çerçeve yok)
- `test/words.test.js`: `parseWords` testi (düz `node`, çerçeve yok)
- `test/hover.test.js`: `hoverMarkdown` / `parseExplain` testi (düz `node`, çerçeve yok)
- `language-configuration.json`: yorumlar, parantezler, otomatik kapanan çiftler, kelime deseni
- `syntaxes/uw.tmLanguage.json`: TextMate dil bilgisi
