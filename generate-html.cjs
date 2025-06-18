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

const cleanMarkdownForHeaders = (markdown) => {
    return markdown
        // Удаление HTML-блоков с командами и примерами
        .replace(/```bash[\s\S]*?```/g, '') // Удаляем блоки кода bash
        .replace(/```json[\s\S]*?```/g, '') // Удаляем блоки кода json
        .replace(/```[\s\S]*?```/g, '') // Удаляем все остальные блоки кода
        .replace(/<div[\s\S]*?<\/div>/g, '') // Удаляем div блоки
        .replace(/<span[\s\S]*?<\/span>/g, '') // Удаляем span блоки
        .replace(/<img[\s\S]*?>/g, '') // Удаляем img теги
        .replace(/<a[\s\S]*?<\/a>/g, '') // Удаляем ссылки
        .replace(/<svg[\s\S]*?<\/svg>/g, '') // Удаляем SVG
        .replace(/<path[\s\S]*?>/g, '') // Удаляем path теги
        .replace(/<button[\s\S]*?<\/button>/g, '') // Удаляем button теги
        .replace(/<nav[\s\S]*?<\/nav>/g, '') // Удаляем nav блоки
        .replace(/<header[\s\S]*?<\/header>/g, '') // Удаляем header блоки
        .replace(/<aside[\s\S]*?<\/aside>/g, '') // Удаляем aside блоки
        .replace(/<main[\s\S]*?<\/main>/g, '') // Удаляем main блоки
        .replace(/<footer[\s\S]*?<\/footer>/g, '') // Удаляем footer блоки
        .replace(/<script[\s\S]*?<\/script>/g, '') // Удаляем script блоки
        .replace(/<style[\s\S]*?<\/style>/g, '') // Удаляем style блоки
        .replace(/<link[\s\S]*?>/g, '') // Удаляем link теги
        .replace(/<meta[\s\S]*?>/g, '') // Удаляем meta теги
        .replace(/<title[\s\S]*?<\/title>/g, '') // Удаляем title теги
        .replace(/<head[\s\S]*?<\/head>/g, '') // Удаляем head блоки
        .replace(/<body[\s\S]*?<\/body>/g, '') // Удаляем body блоки
        .replace(/<html[\s\S]*?<\/html>/g, '') // Удаляем html блоки
        .replace(/<!DOCTYPE[\s\S]*?>/g, '') // Удаляем DOCTYPE
        .replace(/<!--[\s\S]*?-->/g, '') // Удаляем комментарии
        // Удаление разделов, которые не должны быть в меню
        .replace(/## License[\s\S]*?(?=\n## |$)/g, '') // Удаление раздела License
        .replace(/## Contributing[\s\S]*?(?=\n## |$)/g, '') // Удаление раздела Contributing
        .replace(/## Support[\s\S]*?(?=\n## |$)/g, '') // Удаление раздела Support
        .replace(/## Acknowledgments[\s\S]*?(?=\n## |$)/g, '') // Удаление раздела Acknowledgments
        .replace(/## Download[\s\S]*?(?=\n## |$)/g, '') // Удаление раздела Download
        .replace(/## Building[\s\S]*?(?=\n## |$)/g, '') // Удаление раздела Building
        .replace(/## Prerequisites[\s\S]*?(?=\n## |$)/g, '') // Удаление раздела Prerequisites
        .replace(/## Steps[\s\S]*?(?=\n## |$)/g, '') // Удаление раздела Steps
        .replace(/## Build Steps[\s\S]*?(?=\n## |$)/g, '') // Удаление раздела Build Steps
        .replace(/## Platform-Specific Notes[\s\S]*?(?=\n## |$)/g, '') // Удаление раздела Platform-Specific Notes
        .replace(/## Development Guidelines[\s\S]*?(?=\n## |$)/g, '') // Удаление раздела Development Guidelines
        .replace(/<span class="locale">[\s\S]*?<\/span>/g, ''); // Удаление переключений языков
};

const cleanMarkdownForContent = (markdown) => {
    return markdown
        // Удаляем только переключения языков, но сохраняем все остальное
        .replace(/<span class="locale">[\s\S]*?<\/span>/g, '') // Удаление переключений языков
        .replace(/<div class="locale">[\s\S]*?<\/div>/g, ''); // Удаление переключений языков
};

// Функция для извлечения заголовков из Markdown
const extractHeaders = (markdown) => {
    const lines = markdown.split('\n');
    const headers = lines
        .filter(line => {
            // Проверяем, что строка начинается с # и содержит текст
            const isHeader = line.match(/^#{1,6}\s+/);
            if (!isHeader) return false;
            
            // Извлекаем текст заголовка
            const text = line.replace(/^#{1,6}\s+/, '').trim();
            
            // Игнорируем пустые заголовки
            if (!text) return false;
            
            // Игнорируем заголовки, которые содержат только специальные символы
            if (/^[^\wа-яёА-ЯЁ]*$/.test(text)) return false;
            
            // Игнорируем заголовки, которые выглядят как команды или пути
            if (text.includes('```') || text.includes('npm') || text.includes('git') || 
                text.includes('chmod') || text.includes('xattr') || text.includes('codesign')) {
                return false;
            }
            
            return true;
        })
        .map(line => {
            const level = line.match(/^#+/)[0].length;
            const text = line.replace(/^#+\s*/, '').trim();
            const id = transliterate(text);
            return { level, text, id };
        })
        .filter(header => header.level <= 2 && header.text.length > 0); // Только h1 и h2 с непустым текстом
    
    return headers;
};

// Генерация меню на основе заголовков
const generateMenu = (headers) => {
    const menuItems = headers.map(header => {
        const indentClass = header.level === 2 ? 'level-1' : '';
        return `<li class="menu-item ${indentClass}">
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

        // Очищаем markdown для извлечения заголовков (удаляем HTML)
        const cleanDataForHeaders = cleanMarkdownForHeaders(data);
        const headers = extractHeaders(cleanDataForHeaders);
        const menuHtml = generateMenu(headers);

        // Очищаем markdown для контента (сохраняем изображения)
        const cleanDataForContent = cleanMarkdownForContent(data);

        // Преобразование Markdown в HTML
        const htmlContent = marked(cleanDataForContent, { renderer });

        // Генерация языкового меню
        const languageMenu = Object.entries(readmeFiles).map(([code, _]) => {
            const fileName = code === 'en' ? 'index.html' : `index_${code}.html`;
            const isActive = code === lang;
            return `<a href="${fileName}" class="nav-link ${isActive ? 'active' : ''}">${languageNames[code]}</a>`;
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
    <link href="https://fonts.googleapis.com/css2?family=JetBrains+Mono:wght@400;500;600;700;800&display=swap" rel="stylesheet">
    <link rel="icon" type="image/png" href="https://github.com/s00d/switchshuttle/blob/main/icons/128x128_white.png?raw=true">
    <meta name="description" content="SwitchShuttle - Cross-platform system tray application for managing terminal configurations and commands">
    <meta name="keywords" content="terminal, configuration, system tray, cross-platform, development tools">
    <meta name="author" content="SwitchShuttle Team">
    <meta property="og:title" content="SwitchShuttle Documentation">
    <meta property="og:description" content="Cross-platform system tray application for managing terminal configurations and commands">
    <meta property="og:type" content="website">
    <meta property="og:url" content="https://github.com/s00d/switchshuttle">
    <meta property="og:image" content="https://github.com/s00d/switchshuttle/blob/main/icons/128x128_white.png?raw=true">
</head>
<body>
    <div class="app">
        <header class="header" role="banner">
            <div class="header-content">
                <a href="https://github.com/s00d/switchshuttle/" class="logo-link" aria-label="SwitchShuttle на GitHub">
                    <img src="https://github.com/s00d/switchshuttle/blob/main/icons/128x128_white.png?raw=true" alt="SwitchShuttle" class="logo-img">
                    <span class="logo-text">SwitchShuttle</span>
                </a>
                <nav class="nav" aria-label="Основная навигация">
                    <ul class="nav-links">
                        <li><a href="https://github.com/s00d/switchshuttle/" class="nav-link">GitHub</a></li>
                        <li><a href="https://github.com/s00d/switchshuttle/issues" class="nav-link">Issues</a></li>
                        <li><a href="https://github.com/s00d/switchshuttle/releases" class="nav-link">Releases</a></li>
                    </ul>
                </nav>
                <button class="mobile-menu-btn" aria-label="Открыть меню" aria-expanded="false">
                    <span class="hamburger-line"></span>
                    <span class="hamburger-line"></span>
                    <span class="hamburger-line"></span>
                </button>
            </div>
        </header>
        
        <div class="main-container">
            <aside class="sidebar" role="complementary" aria-label="Содержание">
                <div class="sidebar-header">
                    <h3 class="sidebar-title">📋 Содержание</h3>
                </div>
                <nav class="sidebar-nav" aria-label="Навигация по разделам">
                    <ul class="sidebar-menu">
                        ${menuHtml}
                    </ul>
                </nav>
                <div class="sidebar-footer">
                    <div class="language-selector">
                        <button class="language-btn" aria-label="Переключить язык">
                            <span class="current-lang">🌐 ${languageNames[lang]}</span>
                            <svg class="dropdown-arrow" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                <polyline points="6,9 12,15 18,9"></polyline>
                            </svg>
                        </button>
                        <div class="language-menu">
                            ${languageMenu}
                        </div>
                    </div>
                </div>
            </aside>
            
            <main class="content" id="main-content" tabindex="-1" role="main">
                <section class="content-wrapper">
                    ${htmlContent}
                </section>
            </main>
        </div>
        
        <!-- Мобильный оверлей -->
        <div class="mobile-overlay" aria-hidden="true"></div>
        
        <footer class="footer" role="contentinfo">
            <div class="footer-content">
                <div class="footer-left">
                    <span>© ${new Date().getFullYear()} SwitchShuttle</span>
                </div>
                <div class="footer-right">
                    <a href="https://github.com/s00d/switchshuttle/" class="footer-link">GitHub</a>
                    <a href="https://boosty.to/s00d" class="footer-link">Donate</a>
                </div>
            </div>
        </footer>
    </div>
    
    <script>
        // Плавная прокрутка к якорям
        document.querySelectorAll('a[href^="#"]').forEach(anchor => {
            anchor.addEventListener('click', function (e) {
                const href = this.getAttribute('href');
                if (href && href.startsWith('#')) {
                    e.preventDefault();
                    const target = document.querySelector(href);
                    if (target) {
                        target.scrollIntoView({ behavior: 'smooth', block: 'start' });
                    }
                }
            });
        });
        
        // Подсветка активного пункта меню через отслеживание скролла
        const updateActiveMenuItem = () => {
            const scrollPosition = window.scrollY + 100;
            const menuLinks = document.querySelectorAll('.menu-link');
            let activeLink = null;
            
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
            
            menuLinks.forEach(link => link.classList.remove('active'));
            if (activeLink) activeLink.classList.add('active');
        };
        
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
        updateActiveMenuItem();
        
        // Мобильное меню
        const mobileMenuBtn = document.querySelector('.mobile-menu-btn');
        const sidebar = document.querySelector('.sidebar');
        const overlay = document.querySelector('.mobile-overlay');
        
        if (mobileMenuBtn && sidebar) {
            mobileMenuBtn.addEventListener('click', () => {
                const isOpen = sidebar.classList.contains('open');
                sidebar.classList.toggle('open');
                mobileMenuBtn.classList.toggle('active');
                mobileMenuBtn.setAttribute('aria-expanded', !isOpen);
                
                // Управление оверлеем
                if (overlay) {
                    if (!isOpen) {
                        overlay.classList.add('active');
                    } else {
                        overlay.classList.remove('active');
                    }
                }
                
                document.body.style.overflow = isOpen ? '' : 'hidden';
            });

            // Закрытие меню при клике на оверлей
            if (overlay) {
                overlay.addEventListener('click', () => {
                    sidebar.classList.remove('open');
                    mobileMenuBtn.classList.remove('active');
                    mobileMenuBtn.setAttribute('aria-expanded', 'false');
                    overlay.classList.remove('active');
                    document.body.style.overflow = '';
                });
            }

            // Закрытие меню при клике на ссылку в мобильном меню
            document.querySelectorAll('.menu-link').forEach(link => {
                link.addEventListener('click', () => {
                    if (window.innerWidth <= 768) {
                        sidebar.classList.remove('open');
                        mobileMenuBtn.classList.remove('active');
                        mobileMenuBtn.setAttribute('aria-expanded', 'false');
                        if (overlay) {
                            overlay.classList.remove('active');
                        }
                        document.body.style.overflow = '';
                    }
                });
            });
        }
        
        // Простая подсветка кода
        document.querySelectorAll('.code-block').forEach(block => {
            const code = block.querySelector('code');
            if (code) {
                let text = code.textContent;
                
                // Подсветка ключевых слов
                const keywords = ['let', 'const', 'var', 'function', 'return', 'if', 'else', 'for', 'while', 'switch', 'case', 'break', 'import', 'from', 'export', 'class', 'new', 'await', 'async', 'try', 'catch', 'throw', 'console', 'log', 'error', 'warn', 'info', 'debug', 'require', 'module', 'exports', 'default', 'static', 'public', 'private', 'protected', 'extends', 'implements', 'interface', 'type', 'enum', 'namespace', 'declare', 'abstract', 'final', 'super', 'this', 'null', 'undefined', 'true', 'false', 'NaN', 'Infinity'];
                
                keywords.forEach(keyword => {
                    const regex = new RegExp('\\\\b' + keyword + '\\\\b', 'gi');
                    text = text.replace(regex, '<span class="keyword">' + keyword + '</span>');
                });
                
                code.innerHTML = text;
            }
        });
        
        // Анимация появления элементов при скролле
        const observerOptions = {
            threshold: 0.1,
            rootMargin: '0px 0px -50px 0px'
        };
        
        const observer = new IntersectionObserver((entries) => {
            entries.forEach(entry => {
                if (entry.isIntersecting) {
                    // Анимация только для заголовков
                    if (entry.target.tagName === 'H1') {
                        entry.target.classList.add('animate-on-scroll', 'fade-in-right');
                    } else if (entry.target.tagName === 'H2') {
                        entry.target.classList.add('animate-on-scroll', 'fade-in-right');
                    } else if (entry.target.tagName === 'H3') {
                        entry.target.classList.add('animate-on-scroll', 'fade-in-right');
                    }
                }
            });
        }, observerOptions);
        
        // Наблюдаем только за заголовками
        document.querySelectorAll('h1, h2, h3').forEach(el => {
            observer.observe(el);
        });
        
        // Переключение языка
        const languageBtn = document.querySelector('.sidebar .language-btn');
        const languageMenu = document.querySelector('.sidebar .language-menu');
        
        if (languageBtn && languageMenu) {
            languageBtn.addEventListener('click', (e) => {
                e.stopPropagation();
                languageMenu.classList.toggle('open');
            });
            
            // Закрытие меню при клике вне его
            document.addEventListener('click', (e) => {
                if (!languageBtn.contains(e.target) && !languageMenu.contains(e.target)) {
                    languageMenu.classList.remove('open');
                }
            });
            
            // Закрытие меню при клике на пункт меню
            languageMenu.querySelectorAll('.lang-item').forEach(item => {
                item.addEventListener('click', () => {
                    languageMenu.classList.remove('open');
                });
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

