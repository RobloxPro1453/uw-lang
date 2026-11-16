'use strict';

// T-110: `node editors/vscode/test/hover.test.js`. Çerçeve yok (npm paketi gerekmez): kontroller
// burada toplanır, biri bile düşerse çıkış kodu 1 olur.

const assert = require('assert');
const path = require('path');

const { parseExplain, hoverMarkdown, WORD_PATTERN } = require(path.join(__dirname, '..', 'extension.js'));

const checks = [];
function check(name, fn) {
  try {
    fn();
    checks.push({ name, ok: true });
  } catch (error) {
    checks.push({ name, ok: false, error });
  }
}

const PRAVEN = {
  name: 'p_town_6 (praven)',
  kind: 'party (parties #6)',
  native_id: 'p_town_6',
  text: 'p_town_6 (praven)  ·  "Praven"\n  what:     party (parties #6)\n  used by:  3 places (4 mentions)\n',
};

check('ad, tür, Native kimliği ve düz metin markdown olur', () => {
  const markdown = hoverMarkdown(PRAVEN);
  assert.ok(markdown.includes('**p_town_6 (praven)**'), markdown);
  assert.ok(markdown.includes('*party (parties #6)*'), markdown);
  assert.ok(markdown.includes('Native: `p_town_6`'), markdown);
  // Düz metnin her satırı kod bloğunun içinde kalır.
  assert.ok(
    markdown.includes('```text\n' + PRAVEN.text.replace(/\s+$/, '') + '\n```'),
    markdown,
  );
});

check('Native kimliği yoksa satır da yok (uydurulmaz)', () => {
  const markdown = hoverMarkdown({ name: '$g_player_luck', kind: 'global variable', native_id: null, text: 'x' });
  assert.ok(!markdown.includes('Native:'), markdown);
  assert.ok(!markdown.includes('null'), markdown);
  assert.ok(markdown.includes('**$g_player_luck** — *global variable*'), markdown);
});

check('gösterilecek bir şey yoksa undefined döner', () => {
  assert.strictEqual(hoverMarkdown(undefined), undefined);
  assert.strictEqual(hoverMarkdown(null), undefined);
  assert.strictEqual(hoverMarkdown({}), undefined);
  assert.strictEqual(hoverMarkdown({ name: '', kind: 'keyword', native_id: null, text: '' }), undefined);
});

check('eşleşme listesi (ad yok) tek başına metin olarak gösterilir', () => {
  const markdown = hoverMarkdown({
    name: 'cam',
    kind: '',
    native_id: null,
    text: "'cam' is not an exact name. Things that match (2):\n",
  });
  assert.ok(markdown.startsWith("**cam**\n"), markdown);
  assert.ok(markdown.includes('Things that match (2)'), markdown);
});

check('tek satırlık kelime ipucu da markdown olur', () => {
  const markdown = hoverMarkdown({ name: 'player.gold', kind: 'field', native_id: '', text: 'nesne alanı' });
  assert.ok(markdown.includes('**player.gold** — *field*'), markdown);
  assert.ok(markdown.includes('nesne alanı'), markdown);
});

check('explain çıktısındaki tek JSON nesnesi okunur', () => {
  const parsed = parseExplain(JSON.stringify(PRAVEN) + '\r\n');
  assert.strictEqual(parsed.name, 'p_town_6 (praven)');
  assert.strictEqual(parsed.native_id, 'p_town_6');
  // Boş ve JSON olmayan çıktı açıklama sayılmaz.
  assert.strictEqual(parseExplain(''), undefined);
  assert.strictEqual(parseExplain('uw 0.1.0\nusage: uw explain\n'), undefined);
});

check('kelime deseni noktalı adı, `$global`i ve `#dup` ekini tek parça bulur', () => {
  const found = (text) => {
    const re = new RegExp(WORD_PATTERN.source, 'g');
    const words = [];
    let match;
    while ((match = re.exec(text)) !== null) words.push(match[0]);
    return words;
  };
  assert.deepStrictEqual(found('display_message(player.gold)'), ['display_message', 'player.gold']);
  assert.deepStrictEqual(found('if $g_player_luck, 1'), ['if', '$g_player_luck']);
  assert.deepStrictEqual(found('override menu camp#1'), ['override', 'menu', 'camp#1']);
});

let failed = 0;
for (const result of checks) {
  if (result.ok) {
    console.log('ok   ' + result.name);
  } else {
    failed += 1;
    console.error('FAIL ' + result.name + ': ' + (result.error && result.error.message));
  }
}
console.log(`${checks.length - failed}/${checks.length} kontrol geçti`);
process.exit(failed === 0 ? 0 : 1);
