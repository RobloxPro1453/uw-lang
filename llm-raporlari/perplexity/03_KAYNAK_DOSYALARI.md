# İncelenecek kaynak dosyaları
Module dosyaları oyun içeriğini tanımlar; header_operations.py geçerli operation'ları ve sayısal karşılıklarını içerir. [S01][S04]

Aşağıdaki liste hedef dağıtımda kontrol edilecek dosya envanteridir; her sürümde aynı dosya setinin bulunduğu garanti edilmez.

| Alan | Kontrol edilecek dosyalar | Çıkarılacak bilgiler |
|---|---|---|
| Derleme | build_module.bat, module_info.py, process_*.py | Çalıştırma sırası ve export yolu |
| Operandlar | header_common.py, process_operations.py | Etiketleme ve yerel/global çözümü |
| Komutlar | header_operations.py | Opcode, imza, açıklama, bayraklar |
| Mantık | module_scripts.py | Script ID, parametre ve gövde |
| Zamanlama | module_simple_triggers.py, module_triggers.py | Farklı kayıt şemaları |
| Diyalog | module_dialogs.py | Konuşmacı, durum, koşul, metin, sonraki durum, sonuç |
| Menü | module_game_menus.py | Menü ve seçenek şemaları |
| Savaş | module_mission_templates.py | Entry/spawn ve mission trigger şemaları |
| UI | module_presentations.py | Presentation flag'leri ve event gövdeleri |
| Varlık | module_troops.py, module_items.py | Flag ve paketlenmiş özellikler |
| Dünya | module_parties.py, module_party_templates.py, module_factions.py | Konum, ilişki, stack ve şablon |
| Görev | module_quests.py | Veri tanımı ile çalışma zamanı mantığı ayrımı |
| Sahne | module_scenes.py, module_scene_props.py | Sahne metaverisi ve prop event'leri |
| Metin | module_strings.py | String referansları |
| Kaynak | module_meshes.py, module_sounds.py, module_music.py | Kaynak isimleri ve bayrakları |
| Görsel | module_animations.py, module_particle_systems.py, module_map_icons.py, module_skins.py | Kaynak/asset şemaları |
| Sabitler | module_constants.py, ID_*.py | Proje sabitleri ile üretilmiş ID ayrımı |

Önemli: Slot adlarının hepsi motorun yerleşik alanları değildir. Projeye ait slot sabitlerini hedef kaynak ağacında ara. Bir modda kullanılan `slot_*` adını evrensel API kabul etme.
