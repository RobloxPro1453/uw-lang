# MODEL DURUMU — 2026-11-17 testi

## Ne test edildi, ne kanıtlar

Her modele tek satırlık bir görev verildi: *"Sadece `OK <model>` döndür; araç çağırma, dosya okuma/yazma
yok."* Amaç erişim ve yanıt kontrolü. **Dürüst sınır:** bu test yalnız modelin erişilebilir olduğunu ve
yanıt verdiğini kanıtlar. Araç kullanımını, uzun çıktı kalitesini, Türkçe kalitesini ve kanıt disiplinini
**kanıtlamaz**. Gerçek kapasite tur 1'de görülür; zayıf koltuk bir kez yeniden denenir, olmazsa model değişir.

## Çalışanlar (13)

| Model | Sonuç | Not |
|---|---|---|
| `opencode/hy4-flash-free#high` | ✅ `OK hy4-flash-free` | |
| `opencode/iceman-free` | ✅ `OK iceman-free` | varyantsız |
| `opencode/inkling-2-small-free#high` | ✅ | |
| `opencode/longcat-2.5-free` | ✅ | varyantsız |
| `opencode/flint-40b-free#high` | ✅ | |
| `opencode/deepseek-v4.1-flash-free#high` | ✅ | **moderatörün de modeli** (ayrı örnek) |
| `opencode/nemotron-3.5-super-vl-free#high` | ✅ | |
| `opencode/beam-free#high` | ✅ | |
| `opencode/mimo-v2.6-flash-free` | ✅ | varyantsız |
| `opencode/glm-5.3-flash-free#high` | ✅ | |
| `opencode/big-pickle` | ✅ | varyantsız |
| `vyce/glm-5.3-flash#high` | ✅ `OK glm-5.3-flash` | |
| `vyce/nemotron-3.5-super-vl#high` | ✅ `OK nemotron-3.5-super-vl` | |

13 çalışan model, 13 koltuk → her koltuk **farklı** model. Moderatör `opencode/deepseek-v4.1-flash-free`
(B4 ile aynı model, ayrı örnek; sorun değil, ikisi farklı görevde).

## Çalışmayanlar (3)

| Model | Hata | Durum |
|---|---|---|
| `vyce/deepseek-v4-flash-lr#high` | `You do not have access to this resource.` | Kalıcı görünüyor; önceki meclis denemesinde de aynı hata |
| `vyce/grok-4.6#high` | `This model is currently in maintenance. Please try again later.` | Geçici; açılınca **kırmızı takım** koltuğu |
| `vyce/qwen3.8-27b#high` | (sonuç gelmeden iptal edildi) | Sahibi "çalışanlar yeter" dedi; istenirse sonra denenir |

## Kullanılmayan yol

- `cv council` (paket komutu) **kullanılmadı ve kullanılmayacak**: motor olarak DeepSeek API'sini çağırır,
  yani sahibinin bakiyesinden harcar. Bu meclis yalnız oturum içi ücretsiz modellerle çalışır.
- Claude kotası kullanılmadı.

## Tekrar test (gerekirse)

Tek satırlık görevle, koltuğun çıktısını beklemeden:

```
subagent(model: "<model>", prompt: "Model testi. Sadece tek satır döndür: `OK <model>`. Başka hiçbir şey yapma.")
```

Sonuç gelmezse ya da hata verirse o koltuğu **yedek modelle** aç; boş/yarım çıktıyı kabul etme.
