'use strict';

// T-101: `.uw` dosyaları açılınca ve kaydedilince `uw check --json <klasör>` çalışır, çıktısındaki her
// hata kırmızı dalgalı çizgi ve Problems panelinde görünür. Düz CommonJS'tir: npm paketi gerekmez ve
// `vscode` yalnızca `activate` içinde çağrılır, böylece bu dosya `node` ile de yüklenebilir
// (editors/vscode/test/parse.test.js).

const { spawn } = require('child_process');
const fs = require('fs');
const path = require('path');

const SOURCE = 'uw';
const DEFAULT_COMMAND = 'uw';
/// İmlecin altındaki ad: `$global`, `:local`, noktalı nesne adı (`player.gold`), `#dup` eki
/// (`language-configuration.json` içindeki kelime deseniyle aynı).
const WORD_PATTERN = /(?:[$:][A-Za-z0-9_]+)|(?:[A-Za-z_][A-Za-z0-9_]*(?:#[0-9]+)?(?:\.[A-Za-z_][A-Za-z0-9_]*)*)/;

/// `uw check --json` çıktısı: satır başına bir JSON nesnesi, son satır `{"summary":{...}}`.
/// Nesneleri VS Code'un beklediği biçime çevirir: satır/sütun 0 tabanlı, `help` mesaja eklenir.
function parseCheckOutput(text) {
  const diagnostics = [];
  for (const raw of String(text == null ? '' : text).split(/\r?\n/)) {
    const line = raw.trim();
    if (line === '') continue;
    let item;
    try {
      item = JSON.parse(line);
    } catch (e) {
      continue; // tanı değil (kullanım metni, yarım satır): yoksay
    }
    if (typeof item !== 'object' || item === null) continue;
    // `{"summary":{...}}` satırında `file` yoktur; böyle satırlar tanı üretmez.
    if (typeof item.file !== 'string' || typeof item.line !== 'number') continue;
    const startLine = Math.max(item.line - 1, 0);
    const startCharacter = Math.max((typeof item.column === 'number' ? item.column : 1) - 1, 0);
    let endLine = typeof item.end_line === 'number' ? Math.max(item.end_line - 1, 0) : startLine;
    let endCharacter = typeof item.end_column === 'number' ? Math.max(item.end_column - 1, 0) : startCharacter;
    if (endLine < startLine || (endLine === startLine && endCharacter < startCharacter)) {
      endLine = startLine;
      endCharacter = startCharacter;
    }
    const message = typeof item.message === 'string' ? item.message : '';
    diagnostics.push({
      file: item.file,
      range: {
        start: { line: startLine, character: startCharacter },
        end: { line: endLine, character: endCharacter },
      },
      severity: typeof item.severity === 'string' ? item.severity : 'error',
      code: item.code == null ? undefined : String(item.code),
      message: typeof item.help === 'string' && item.help !== '' ? message + '\n' + item.help : message,
    });
  }
  return diagnostics;
}

/// `uw words --json` çıktısı: satır başına bir JSON nesnesi `{"label","kind","detail"}`.
/// Tamamlama listesi için sade nesnelere çevirir; boş ve JSON olmayan satırları yoksayar.
function parseWords(text) {
  const words = [];
  for (const raw of String(text == null ? '' : text).split(/\r?\n/)) {
    const line = raw.trim();
    if (line === '') continue;
    let item;
    try {
      item = JSON.parse(line);
    } catch (e) {
      continue; // kelime değil (kullanım metni, yarım satır): yoksay
    }
    if (typeof item !== 'object' || item === null) continue;
    if (typeof item.label !== 'string' || item.label === '') continue;
    if (typeof item.kind !== 'string' || item.kind === '') continue;
    words.push({
      label: item.label,
      kind: item.kind,
      detail: typeof item.detail === 'string' ? item.detail : '',
    });
  }
  return words;
}

/// `uw explain --json` çıktısı: tek bir JSON nesnesi `{"name","kind","native_id","text"}`.
/// Boş ya da JSON olmayan çıktıda `undefined` döner (açıklama yok).
function parseExplain(text) {
  const stdout = String(text == null ? '' : text).trim();
  if (stdout === '') return undefined;
  let item;
  try {
    item = JSON.parse(stdout);
  } catch (e) {
    return undefined; // JSON değil (kullanım metni, yarım çıktı): açıklama yok
  }
  return typeof item === 'object' && item !== null ? item : undefined;
}

/// Hover'da görünecek markdown: `uw explain --json` nesnesinden (`{name,kind,native_id,text}`) adı,
/// ne olduğunu, Native kimliğini ve düz metni yazar. `vscode` çağırmaz; gösterilecek bir şey yoksa
/// `undefined` döner.
function hoverMarkdown(obj) {
  if (typeof obj !== 'object' || obj === null) return undefined;
  const name = typeof obj.name === 'string' ? obj.name : '';
  const kind = typeof obj.kind === 'string' ? obj.kind : '';
  const id = typeof obj.native_id === 'string' ? obj.native_id : '';
  const text = typeof obj.text === 'string' ? obj.text : '';
  if (name === '' && text === '') return undefined;
  const lines = [];
  if (name !== '') lines.push(kind === '' ? `**${name}**` : `**${name}** — *${kind}*`);
  else if (kind !== '') lines.push(`*${kind}*`);
  if (id !== '') lines.push(`Native: \`${id}\``);
  if (text !== '') {
    if (lines.length > 0) lines.push('');
    lines.push('```text', text.replace(/\s+$/, ''), '```');
  }
  return lines.join('\n');
}

/// Dosyanın en yakın `uw.toml` içeren üst klasörü; yoksa dosyanın kendi klasörü.
function projectFolder(file) {
  const own = path.dirname(file);
  let dir = own;
  for (;;) {
    if (fs.existsSync(path.join(dir, 'uw.toml'))) return dir;
    const parent = path.dirname(dir);
    if (parent === dir) return own;
    dir = parent;
  }
}

/// `uw check --json <folder>`: çözülünce stdout (çıkış kodu 1 olsa bile hatalar oradadır).
function runCheck(command, folder) {
  return new Promise((resolve, reject) => {
    const child = spawn(command, ['check', '--json', folder], { windowsHide: true });
    let stdout = '';
    child.stdout.setEncoding('utf8');
    child.stdout.on('data', (chunk) => {
      stdout += chunk;
    });
    child.stderr.on('data', () => {});
    child.on('error', reject);
    child.on('close', () => resolve(stdout));
  });
}

/// `uw words --json`: çözülünce kelimelerin stdout'u. Komut projenin klasöründe çalışır, böylece
/// CLI atlası klasörün üst klasörlerinde bulur.
function runWords(command, folder) {
  return new Promise((resolve, reject) => {
    const child = spawn(command, ['words', '--json'], { cwd: folder, windowsHide: true });
    let stdout = '';
    child.stdout.setEncoding('utf8');
    child.stdout.on('data', (chunk) => {
      stdout += chunk;
    });
    child.stderr.on('data', () => {});
    child.on('error', reject);
    child.on('close', () => resolve(stdout));
  });
}

/// `uw explain --json <ad> <klasör>`: çözülünce stdout. Komut projenin klasöründe çalışır ve klasörü
/// de verir, böylece CLI projenin kendi `names.uw` dosyasını okur.
function runExplain(command, name, folder) {
  return new Promise((resolve, reject) => {
    const child = spawn(command, ['explain', '--json', name, folder], { cwd: folder, windowsHide: true });
    let stdout = '';
    child.stdout.setEncoding('utf8');
    child.stdout.on('data', (chunk) => {
      stdout += chunk;
    });
    child.stderr.on('data', () => {});
    child.on('error', reject);
    child.on('close', () => resolve(stdout));
  });
}

function activate(context) {
  const vscode = require('vscode');
  const collection = vscode.languages.createDiagnosticCollection('uw');
  context.subscriptions.push(collection);

  const severities = {
    error: vscode.DiagnosticSeverity.Error,
    warning: vscode.DiagnosticSeverity.Warning,
    info: vscode.DiagnosticSeverity.Information,
    hint: vscode.DiagnosticSeverity.Hint,
  };
  let missingToolWarned = false;

  const check = async (document) => {
    if (!document || document.languageId !== 'uw' || document.uri.scheme !== 'file') return;
    const configured = vscode.workspace.getConfiguration('uw').get('path', DEFAULT_COMMAND);
    const command = configured || DEFAULT_COMMAND;
    let stdout;
    try {
      stdout = await runCheck(command, projectFolder(document.uri.fsPath));
    } catch (error) {
      if (error && error.code === 'ENOENT') {
        if (!missingToolWarned) {
          missingToolWarned = true;
          vscode.window.showWarningMessage(
            `uw bulunamadı ("${command}"). uw'yu PATH'e ekleyin veya "uw.path" ayarını doldurun.`,
          );
        }
      } else {
        vscode.window.showWarningMessage(`uw check çalıştırılamadı: ${(error && error.message) || error}`);
      }
      return;
    }

    const byFile = new Map();
    for (const item of parseCheckOutput(stdout)) {
      const { start, end } = item.range;
      const diagnostic = new vscode.Diagnostic(
        new vscode.Range(start.line, start.character, end.line, end.character),
        item.message,
        severities[item.severity] || vscode.DiagnosticSeverity.Error,
      );
      diagnostic.source = SOURCE;
      if (item.code !== undefined) diagnostic.code = item.code;
      const list = byFile.get(item.file);
      if (list) list.push(diagnostic);
      else byFile.set(item.file, [diagnostic]);
    }

    collection.clear();
    for (const [file, list] of byFile) collection.set(vscode.Uri.file(file), list);
  };

  context.subscriptions.push(
    vscode.workspace.onDidOpenTextDocument(check),
    vscode.workspace.onDidSaveTextDocument(check),
  );

  // T-104: otomatik tamamlama. `uw words --json` klasör başına bir kez çalışır (önbellek);
  // `names.uw`, `lib/` ya da `fns/` altındaki bir dosya kaydedilince liste tazelenir.
  const words = new Map();
  const completionKinds = {
    name: vscode.CompletionItemKind.Variable,
    word: vscode.CompletionItemKind.Function,
    field: vscode.CompletionItemKind.Field,
    keyword: vscode.CompletionItemKind.Keyword,
  };
  const wordsFor = (folder) => {
    if (!words.has(folder)) {
      const configured = vscode.workspace.getConfiguration('uw').get('path', DEFAULT_COMMAND);
      const command = configured || DEFAULT_COMMAND;
      words.set(folder, runWords(command, folder).then(parseWords).catch(() => []));
    }
    return words.get(folder);
  };

  // T-110: üstüne gelince açıklama. `uw explain --json <ad> <klasör>` ad başına bir kez çalışır
  // (önbellek); CLI adı bilmiyorsa (yerel bir `fn` gibi) kelime önbelleğinin tek satırı gösterilir.
  const explanations = new Map();
  const explainFor = (folder, name) => {
    const key = folder + '\u0000' + name;
    if (!explanations.has(key)) {
      const configured = vscode.workspace.getConfiguration('uw').get('path', DEFAULT_COMMAND);
      const command = configured || DEFAULT_COMMAND;
      explanations.set(key, runExplain(command, name, folder).then(parseExplain).catch(() => undefined));
    }
    return explanations.get(key);
  };
  context.subscriptions.push(
    vscode.languages.registerHoverProvider('uw', {
      async provideHover(document, position) {
        if (!document || document.uri.scheme !== 'file') return undefined;
        const range = document.getWordRangeAtPosition(position, WORD_PATTERN);
        if (!range) return undefined;
        const name = document.getText(range);
        if (name === '') return undefined;
        const folder = projectFolder(document.uri.fsPath);
        let markdown = hoverMarkdown(await explainFor(folder, name));
        if (markdown === undefined) {
          const found = (await wordsFor(folder)).find((word) => word.label === name);
          if (!found) return undefined;
          markdown = hoverMarkdown({ name, kind: found.kind, native_id: '', text: found.detail });
        }
        if (markdown === undefined) return undefined;
        return new vscode.Hover(new vscode.MarkdownString(markdown), range);
      },
    }),
    vscode.languages.registerCompletionItemProvider('uw', {
      async provideCompletionItems(document) {
        const folder = projectFolder(document.uri.fsPath);
        const found = await wordsFor(folder);
        return found.map((word) => {
          const item = new vscode.CompletionItem(
            word.label,
            completionKinds[word.kind] || vscode.CompletionItemKind.Text,
          );
          if (word.detail !== '') item.detail = word.detail;
          return item;
        });
      },
    }),
    vscode.workspace.onDidSaveTextDocument((document) => {
      if (!document || !document.uri || document.uri.scheme !== 'file') return;
      const file = document.uri.fsPath;
      const name = path.basename(file);
      const parent = path.basename(path.dirname(file));
      if (name === 'names.uw' || parent === 'lib' || parent === 'fns') {
        words.clear();
        explanations.clear();
      }
    }),
  );
}

function deactivate() {}

module.exports = {
  activate,
  deactivate,
  parseCheckOutput,
  parseWords,
  parseExplain,
  hoverMarkdown,
  WORD_PATTERN,
};
