# kit-meta/corpus — gerçek modların kullanım profili (ROADMAP M1)

`uw corpus warband-opensource-mods/mods` ile üretilir (modlar git'e girmez; burada yalnız sayılar var, mod kodu yok).

- `ops.json`: kitin her işlemi (1060) için toplam kullanım, kaç modda geçtiği ve mod başına sayı; en yaygın önce.
  `uses = 0` olanlar hiçbir modda geçmez.
- `ti.json`: `ti_*` tetik türleri için aynısı.
- `summary.json`: mod başına dosya, satır, işlem kullanımı, farklı işlem sayısı, kullanılan `ti_*` türü,
  motorun çağırdığı betikler (atlas hooks) ve Native'de olmayan betik/sunum sayısı.

Dikkat: tarama metinseldir (`(ad,` ve `(neg|ad,`); yorumlar atlanır, header/process/ID dosyaları sayılmaz. Çoğu mod
Native'in kopyası üstüne kurulduğu için Native'in kullandığı işlemler hemen her modda görünür; "modun eklediği"
kullanımı ayırmak için bu sayıyı Native'in kendi profiliyle karşılaştırın.
