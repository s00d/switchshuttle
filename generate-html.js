const fs = require('fs');
const { marked } = require('marked');

const renderer = new marked.Renderer();

renderer.heading = (text, level) => {
    const id = text.toLowerCase().replace(/\s+/g, '-');
    return `<h${level} id="${id}">${text}</h${level}>`;
};

// Путь к вашим README файлам
const readmeFiles = {
    'en': 'README.md',
    'de': 'README_DE.md',
    'ja': 'README_JA.md',
    'ru': 'README_RU.md',
    'zh': 'README_ZH.md'
};

const cleanMarkdown = (markdown) => {
    return markdown
        .replace(/## License[\s\S]*?(?=\n## |$)/g, '') // Удаление раздела License
        .replace(/<span class="locale">[\s\S]*?<\/span>/g, ''); // Удаление переключений языков
};

// Функция для генерации HTML для каждого языка
const generateHtmlForLanguage = (lang, inputPath, outputPath) => {
    fs.readFile(inputPath, 'utf8', (err, data) => {
        if (err) {
            console.error(`Ошибка при чтении файла ${inputPath}:`, err);
            return;
        }

        data = cleanMarkdown(data);

        // Преобразование Markdown в HTML
        const htmlContent = marked(data, { renderer });

        // Функция для извлечения заголовков из Markdown
        const extractHeaders = (markdown) => {
            const lines = markdown.split('\n');
            const headers = lines
                .filter(line => line.startsWith('#'))
                .map(line => {
                    const level = line.match(/^#+/)[0].length;
                    const text = line.replace(/^#+\s*/, '');
                    const id = text.toLowerCase().replace(/\s+/g, '-');
                    return { level, text, id };
                });
            return headers;
        };

        const headers = extractHeaders(data);

        // Генерация меню на основе заголовков
        const generateMenu = (headers) => {
            const menuItems = headers.map(header => {
                return `<li class="level-${header.level}"><a href="#${header.id}">${header.text}</a></li>`;
            });
            return menuItems.join('\n');
        };

        const menuHtml = generateMenu(headers);

        // Обертывание в базовую структуру HTML
        const fullHtml = `
<!DOCTYPE html>
<html lang="${lang}">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>SwitchShuttle Documentation</title>
    <link rel="stylesheet" href="styles.css">
</head>
<body>
<header>
    <div class="logo">
        <a href="https://github.com/s00d/switchshuttle/">
            <img src="https://github.com/s00d/switchshuttle/blob/main/icons/128x128_white.png?raw=true" alt="SwitchShuttle Logo">
        </a>
    </div>
    <nav>
        <ul>
            <li><a href="https://github.com/s00d/switchshuttle/">GitHub</a></li>
            <li><a href="https://github.com/s00d/switchshuttle/issues">Issues</a></li>
            <li class="dropdown">
                <a href="javascript:void(0)" class="dropbtn">Language</a>
                <div class="dropdown-content">
                    <a href="index.html" class="${lang === 'en' ? 'selected' : ''}">English</a>
                    <a href="index_de.html" class="${lang === 'de' ? 'selected' : ''}">Deutsch</a>
                    <a href="index_ja.html" class="${lang === 'ja' ? 'selected' : ''}">日本語</a>
                    <a href="index_ru.html" class="${lang === 'ru' ? 'selected' : ''}">Русский</a>
                    <a href="index_zh.html" class="${lang === 'zh' ? 'selected' : ''}">中文</a>
                </div>
            </li>
        </ul>
    </nav>
</header>
<div class="container">
    <aside>
        <ul id="aside-menu">
            ${menuHtml}
        </ul>
    </aside>
    <main id="content">
        ${htmlContent}
    </main>
</div>
</body>
</html>
`;

        // Запись в HTML файл
        fs.writeFile(outputPath, fullHtml, (err) => {
            if (err) {
                console.error(`Ошибка при записи файла ${outputPath}:`, err);
                return;
            }
            console.log(`Файл успешно сохранен: ${outputPath}`);
        });
    });
};

// Генерация HTML файлов для всех языков
for (const [lang, inputPath] of Object.entries(readmeFiles)) {
    let outputPath = `docs/index_${lang}.html`;
    if(lang === 'en') {
        outputPath = `docs/index.html`
    }
    generateHtmlForLanguage(lang, inputPath, outputPath);
}
