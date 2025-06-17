const fs = require('fs');
const { marked } = require('marked');

const renderer = new marked.Renderer();

// Функция транслитерации для конвертации символов в английские эквиваленты
const transliterate = (text) => {
    const translitMap = {
        // Русские буквы
        'а': 'a', 'б': 'b', 'в': 'v', 'г': 'g', 'д': 'd', 'е': 'e', 'ё': 'yo', 'ж': 'zh',
        'з': 'z', 'и': 'i', 'й': 'y', 'к': 'k', 'л': 'l', 'м': 'm', 'н': 'n', 'о': 'o',
        'п': 'p', 'р': 'r', 'с': 's', 'т': 't', 'у': 'u', 'ф': 'f', 'х': 'h', 'ц': 'ts',
        'ч': 'ch', 'ш': 'sh', 'щ': 'sch', 'ъ': '', 'ы': 'y', 'ь': '', 'э': 'e', 'ю': 'yu', 'я': 'ya',
        'А': 'A', 'Б': 'B', 'В': 'V', 'Г': 'G', 'Д': 'D', 'Е': 'E', 'Ё': 'Yo', 'Ж': 'Zh',
        'З': 'Z', 'И': 'I', 'Й': 'Y', 'К': 'K', 'Л': 'L', 'М': 'M', 'Н': 'N', 'О': 'O',
        'П': 'P', 'Р': 'R', 'С': 'S', 'Т': 'T', 'У': 'U', 'Ф': 'F', 'Х': 'H', 'Ц': 'Ts',
        'Ч': 'Ch', 'Ш': 'Sh', 'Щ': 'Sch', 'Ъ': '', 'Ы': 'Y', 'Ь': '', 'Э': 'E', 'Ю': 'Yu', 'Я': 'Ya',
        
        // Немецкие буквы
        'ä': 'ae', 'ö': 'oe', 'ü': 'ue', 'ß': 'ss',
        'Ä': 'Ae', 'Ö': 'Oe', 'Ü': 'Ue',
        
        // Японские символы (основные)
        'あ': 'a', 'い': 'i', 'う': 'u', 'え': 'e', 'お': 'o',
        'か': 'ka', 'き': 'ki', 'く': 'ku', 'け': 'ke', 'こ': 'ko',
        'さ': 'sa', 'し': 'shi', 'す': 'su', 'せ': 'se', 'そ': 'so',
        'た': 'ta', 'ち': 'chi', 'つ': 'tsu', 'て': 'te', 'と': 'to',
        'な': 'na', 'に': 'ni', 'ぬ': 'nu', 'ね': 'ne', 'の': 'no',
        'は': 'ha', 'ひ': 'hi', 'ふ': 'fu', 'へ': 'he', 'ほ': 'ho',
        'ま': 'ma', 'み': 'mi', 'む': 'mu', 'め': 'me', 'も': 'mo',
        'や': 'ya', 'ゆ': 'yu', 'よ': 'yo',
        'ら': 'ra', 'り': 'ri', 'る': 'ru', 'れ': 're', 'ろ': 'ro',
        'わ': 'wa', 'を': 'wo', 'ん': 'n',
        
        // Китайские символы (основные)
        '一': 'yi', '二': 'er', '三': 'san', '四': 'si', '五': 'wu',
        '六': 'liu', '七': 'qi', '八': 'ba', '九': 'jiu', '十': 'shi',
        '人': 'ren', '大': 'da', '小': 'xiao', '上': 'shang', '下': 'xia',
        '中': 'zhong', '国': 'guo', '家': 'jia', '学': 'xue', '生': 'sheng',
        '工': 'gong', '作': 'zuo', '用': 'yong', '开': 'kai', '关': 'guan',
        '安': 'an', '装': 'zhuang', '置': 'zhi', '设': 'she', '计': 'ji',
        '开': 'kai', '发': 'fa', '开': 'kai', '始': 'shi', '结': 'jie',
        '束': 'shu', '完': 'wan', '成': 'cheng', '使': 'shi', '用': 'yong'
    };
    
    return text
        .split('')
        .map(char => translitMap[char] || char)
        .join('')
        .toLowerCase()
        .replace(/[^\w\s-]/g, '') // Удаляем все кроме букв, цифр, пробелов и дефисов
        .replace(/\s+/g, '-') // Заменяем пробелы на дефисы
        .replace(/-+/g, '-') // Убираем множественные дефисы
        .replace(/^-|-$/g, ''); // Убираем дефисы в начале и конце
};

// Кастомный рендерер для заголовков с якорями
renderer.heading = (text, level) => {
    const id = transliterate(text);
    return `<h${level} id="${id}" class="heading-${level}">${text}</h${level}>`;
};

// Кастомный рендерер для кода с подсветкой
renderer.code = (code, language) => {
    const langClass = language ? `language-${language}` : '';
    return `<pre class="code-block"><code class="${langClass}">${code}</code></pre>`;
};

// Кастомный рендерер для блоков кода
renderer.blockquote = (quote) => {
    return `<blockquote class="blockquote">${quote}</blockquote>`;
};

// Путь к вашим README файлам
const readmeFiles = {
    'en': 'README.md',
    'de': 'README_DE.md',
    'ja': 'README_JA.md',
    'ru': 'README_RU.md',
    'zh': 'README_ZH.md'
};

const languageNames = {
    'en': 'English',
    'de': 'Deutsch', 
    'ja': '日本語',
    'ru': 'Русский',
    'zh': '中文'
};

const cleanMarkdown = (markdown) => {
    return markdown
        .replace(/## License[\s\S]*?(?=\n## |$)/g, '') // Удаление раздела License
        .replace(/<span class="locale">[\s\S]*?<\/span>/g, ''); // Удаление переключений языков
};

// Функция для извлечения заголовков из Markdown
const extractHeaders = (markdown) => {
    const lines = markdown.split('\n');
    const headers = lines
        .filter(line => line.startsWith('#'))
        .map(line => {
            const level = line.match(/^#+/)[0].length;
            const text = line.replace(/^#+\s*/, '');
            const id = transliterate(text);
            return { level, text, id };
        })
        .filter(header => header.level <= 2); // Только h1 и h2
    return headers;
};

// Генерация меню на основе заголовков
const generateMenu = (headers) => {
    const menuItems = headers.map(header => {
        const indentClass = header.level === 2 ? 'indent-1' : '';
        return `<li class="menu-item level-${header.level} ${indentClass}">
            <a href="#${header.id}" class="menu-link">
                <span class="menu-text">${header.text}</span>
            </a>
        </li>`;
    });
    return menuItems.join('\n');
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
        const headers = extractHeaders(data);
        const menuHtml = generateMenu(headers);

        // Генерация языкового меню
        const languageMenu = Object.entries(readmeFiles).map(([code, _]) => {
            const fileName = code === 'en' ? 'index.html' : `index_${code}.html`;
            const isActive = code === lang;
            return `<a href="${fileName}" class="lang-item ${isActive ? 'active' : ''}">${languageNames[code]}</a>`;
        }).join('');

        // Обертывание в современную структуру HTML
        const fullHtml = `<!DOCTYPE html>
<html lang="${lang}">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>SwitchShuttle Documentation - ${languageNames[lang]}</title>
    <link rel="stylesheet" href="styles.css">
    <link rel="preconnect" href="https://fonts.googleapis.com">
    <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin>
    <link href="https://fonts.googleapis.com/css2?family=Inter:wght@300;400;500;600;700&family=JetBrains+Mono:wght@400;500&display=swap" rel="stylesheet">
    <link rel="icon" type="image/png" href="https://github.com/s00d/switchshuttle/blob/main/icons/128x128_white.png?raw=true">
</head>
<body>
    <div class="app">
        <!-- Header -->
        <header class="header">
            <div class="header-content">
                <div class="logo">
                    <a href="https://github.com/s00d/switchshuttle/" class="logo-link">
                        <img src="https://github.com/s00d/switchshuttle/blob/main/icons/128x128_white.png?raw=true" alt="SwitchShuttle" class="logo-img">
                        <span class="logo-text">SwitchShuttle</span>
                    </a>
                </div>
                
                <nav class="nav">
                    <div class="nav-links">
                        <a href="https://github.com/s00d/switchshuttle/" class="nav-link">
                            <svg class="nav-icon" viewBox="0 0 24 24" fill="currentColor">
                                <path d="M12 0c-6.626 0-12 5.373-12 12 0 5.302 3.438 9.8 8.207 11.387.599.111.793-.261.793-.577v-2.234c-3.338.726-4.033-1.416-4.033-1.416-.546-1.387-1.333-1.756-1.333-1.756-1.089-.745.083-.729.083-.729 1.205.084 1.839 1.237 1.839 1.237 1.07 1.834 2.807 1.304 3.492.997.107-.775.418-1.305.762-1.604-2.665-.305-5.467-1.334-5.467-5.931 0-1.311.469-2.381 1.236-3.221-.124-.303-.535-1.524.117-3.176 0 0 1.008-.322 3.301 1.23.957-.266 1.983-.399 3.003-.404 1.02.005 2.047.138 3.006.404 2.291-1.552 3.297-1.23 3.297-1.23.653 1.653.242 2.874.118 3.176.77.84 1.235 1.911 1.235 3.221 0 4.609-2.807 5.624-5.479 5.921.43.372.823 1.102.823 2.222v3.293c0 .319.192.694.801.576 4.765-1.589 8.199-6.086 8.199-11.386 0-6.627-5.373-12-12-12z"/>
                            </svg>
                            GitHub
                        </a>
                        <a href="https://github.com/s00d/switchshuttle/issues" class="nav-link">
                            <svg class="nav-icon" viewBox="0 0 24 24" fill="currentColor">
                                <path d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm-2 15l-5-5 1.41-1.41L10 14.17l7.59-7.59L19 8l-9 9z"/>
                            </svg>
                            Issues
                        </a>
                    </div>
                    
                    <div class="language-selector">
                        <div class="language-dropdown">
                            <button class="language-btn">
                                <span class="current-lang">${languageNames[lang]}</span>
                                <svg class="dropdown-arrow" viewBox="0 0 24 24" fill="currentColor">
                                    <path d="M7 10l5 5 5-5z"/>
                                </svg>
                            </button>
                            <div class="language-menu">
                                ${languageMenu}
                            </div>
                        </div>
                    </div>
                </nav>
            </div>
        </header>

        <!-- Main Content -->
        <div class="main-container">
            <!-- Sidebar -->
            <aside class="sidebar">
                <div class="sidebar-header">
                    <h3 class="sidebar-title">Contents</h3>
                </div>
                <nav class="sidebar-nav">
                    <ul class="sidebar-menu">
                        ${menuHtml}
                    </ul>
                </nav>
            </aside>

            <!-- Content -->
            <main class="content">
                <div class="content-wrapper">
                    ${htmlContent}
                </div>
            </main>
        </div>
    </div>

    <script>
        // Плавная прокрутка к якорям
        document.querySelectorAll('a[href^="#"]').forEach(anchor => {
            anchor.addEventListener('click', function (e) {
                e.preventDefault();
                const target = document.querySelector(this.getAttribute('href'));
                if (target) {
                    target.scrollIntoView({
                        behavior: 'smooth',
                        block: 'start'
                    });
                }
            });
        });

        // Подсветка активного пункта меню через отслеживание скролла
        const updateActiveMenuItem = () => {
            const scrollPosition = window.scrollY + 100; // Небольшой отступ
            
            // Получаем все ссылки из меню
            const menuLinks = document.querySelectorAll('.sidebar-menu a');
            let activeLink = null;
            
            // Проверяем каждую ссылку из меню
            menuLinks.forEach(link => {
                const href = link.getAttribute('href');
                if (href && href.startsWith('#')) {
                    const targetId = href.substring(1);
                    const targetElement = document.getElementById(targetId);
                    
                    if (targetElement) {
                        const elementTop = targetElement.offsetTop;
                        
                        if (elementTop <= scrollPosition) {
                            activeLink = link;
                        }
                    }
                }
            });
            
            // Обновляем активный пункт меню
            menuLinks.forEach(link => {
                link.classList.remove('active');
            });
            
            if (activeLink) {
                activeLink.classList.add('active');
            }
        };

        // Добавляем обработчик скролла с throttling
        let ticking = false;
        window.addEventListener('scroll', () => {
            if (!ticking) {
                requestAnimationFrame(() => {
                    updateActiveMenuItem();
                    ticking = false;
                });
                ticking = true;
            }
        });

        // Инициализируем активный пункт при загрузке
        updateActiveMenuItem();

        // Переключение языкового меню
        const languageBtn = document.querySelector('.language-btn');
        const languageMenu = document.querySelector('.language-menu');
        
        if (languageBtn && languageMenu) {
            languageBtn.addEventListener('click', () => {
                languageMenu.classList.toggle('open');
            });

            document.addEventListener('click', (e) => {
                if (!languageBtn.contains(e.target) && !languageMenu.contains(e.target)) {
                    languageMenu.classList.remove('open');
                }
            });
        }

        // Мобильное меню
        const mobileMenuBtn = document.querySelector('.mobile-menu-btn');
        const sidebar = document.querySelector('.sidebar');
        
        if (mobileMenuBtn && sidebar) {
            mobileMenuBtn.addEventListener('click', () => {
                sidebar.classList.toggle('mobile-open');
            });
        }
    </script>
</body>
</html>`;

        // Запись в HTML файл
        fs.writeFile(outputPath, fullHtml, (err) => {
            if (err) {
                console.error(`Ошибка при записи файла ${outputPath}:`, err);
                return;
            }
            console.log(`✅ Файл успешно сохранен: ${outputPath}`);
        });
    });
};

// Генерация HTML файлов для всех языков
console.log('🚀 Начинаю генерацию документации...');
for (const [lang, inputPath] of Object.entries(readmeFiles)) {
    let outputPath = `docs/index_${lang}.html`;
    if(lang === 'en') {
        outputPath = `docs/index.html`
    }
    generateHtmlForLanguage(lang, inputPath, outputPath);
}
console.log('✨ Генерация документации завершена!');
