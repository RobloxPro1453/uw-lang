# UW dili için VS Code eklentisi (v0)

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

## Denetim

Söz dizimi dosyalarının (`syntaxes/uw.tmLanguage.json`, `language-configuration.json`,
`package.json`) geçerli JSON olduğunu ve yukarıdaki anahtar kelimelerin hepsinin dil bilgisinde
bulunduğunu proje kökünde şu test kontrol eder:

```sh
cargo test -p uw_cli --test vscode_grammar
```

Test Node.js gerektirmez.

## Dosyalar

- `package.json`: eklenti bildirimi (ad `uw-lang`, `.uw` uzantısı, dil bilgisi yolu)
- `language-configuration.json`: yorumlar, parantezler, otomatik kapanan çiftler, kelime deseni
- `syntaxes/uw.tmLanguage.json`: TextMate dil bilgisi
