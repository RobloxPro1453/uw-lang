# Arşiv

> Durum: arşiv kuralı. Buradaki belgeler **yerini yenisine bırakmıştır**; bir oturum bunları varsayılan olarak okumaz.
> Neyin neden burada olduğu ve yerini neyin aldığı: `docs/HARITA.md` > "Arşiv".

- Arşivden bilgi gerekiyorsa (ör. eski bir kararın gerekçesi) `grep` ile o satır bulunur; dosya baştan okunmaz.
- Arşiv belgesi düzenlenmez (yalnız taşımada bağlantı yolu güncellenir). Hâlâ geçerli bir kural bulunursa canlı
  belgeye (KARARNAMELER, ARCHITECTURE, KNOWLEDGE) taşınır; arşivdeki metin olduğu gibi kalır.
- Taşıma `git mv` ile yapılır (geçmiş korunur) ve aynı commit'te HARITA güncellenir; bağlantılar betikle yeni yola çevrilir.
