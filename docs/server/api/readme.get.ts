import { readFileSync } from 'fs'
import { join } from 'path'
import { marked } from 'marked'

export default defineEventHandler(async (event) => {
    const query = getQuery(event)
    const locale = query.locale as string

    if (!locale) {
        return {
            success: false,
            error: 'Locale parameter is required'
        }
    }

    // Функция для очистки markdown перед парсингом
    function cleanMarkdown(markdown: string): string {
        // Удаляем блок с badges и центрированием
        let cleaned = markdown.replace(
            /<div align="center">[\s\S]*?<\/div>/g,
            ''
        )

        // Удаляем блок с ссылками на локали
        cleaned = cleaned.replace(
            /<div class="locale">[\s\S]*?<\/div>/g,
            ''
        )

        // Удаляем горизонтальные линии
        cleaned = cleaned.replace(/^---$/gm, '')

        // Удаляем пустые строки в начале документа
        cleaned = cleaned.replace(/^\s+/, '')

        // Удаляем множественные пустые строки
        cleaned = cleaned.replace(/\n\s*\n\s*\n/g, '\n\n')

        return cleaned
    }

    // Функция для очистки HTML после парсинга
    function cleanHtml(html: string): string {
        // Удаляем лишние закрывающие div в начале документа
        let cleaned = html.replace(/^<\/div>\s*/, '')

        // Удаляем пустые строки и переносы в начале документа
        cleaned = cleaned.replace(/^[\s\n\r]+/, '')

        // Удаляем множественные пустые строки
        cleaned = cleaned.replace(/\n\s*\n\s*\n/g, '\n\n')

        return cleaned
    }

    // Функция для парсинга markdown
    async function parseMarkdown(markdown: string): Promise<string> {
        // Настраиваем marked
        marked.setOptions({
            breaks: true,
            gfm: true
        })

        // Парсим markdown в HTML
        const html = await marked.parse(markdown)

        // Очищаем HTML от ненужных блоков
        const cleanedHtml = cleanHtml(html)

        // Добавляем якоря к заголовкам (как на GitHub)
        const htmlWithAnchors = cleanedHtml.replace(/<h([1-6])([^>]*)>(.*?)<\/h[1-6]>/g, (match: string, level: string, attrs: string, text: string) => {
            const escapedText = text.toLowerCase().replace(/[^\w]+/g, '-')
            const id = `heading-${escapedText}`

            return `
        <h${level} id="${id}"${attrs}>
          <a name="${escapedText}" class="anchor" href="#${escapedText}">
            <span class="header-link"></span>
          </a>
          ${text}
        </h${level}>`
        })

        // Добавляем target="_blank" к внешним ссылкам
        const htmlWithLinks = htmlWithAnchors.replace(/<a href="(https?:\/\/[^"]+)">([^<]+)<\/a>/g, '<a href="$1" target="_blank" rel="noopener noreferrer">$2</a>')

        // Улучшаем оформление таблиц
        const htmlWithTables = htmlWithLinks.replace(
            /<table>/g,
            '<table class="console-table">'
        )

        // Добавляем data-language к блокам кода
        const htmlWithCodeLang = htmlWithTables.replace(/<pre><code class="language-(\w+)">/g, '<pre class="code-block" data-language="$1"><code class="language-$1">')

        // Добавляем класс code-block к обычным блокам кода без указанного языка
        const htmlWithAllCodeBlocks = htmlWithCodeLang.replace(/<pre><code>/g, '<pre class="code-block"><code>')

        // Форматируем структуру проекта
        const htmlWithProjectStructure = htmlWithAllCodeBlocks.replace(
            /<pre><code>([\s\S]*?SwitchShuttle\/[\s\S]*?)<\/code><\/pre>/g,
            (match: string, content: string) => {
                const lines = content.split('\n').filter(line => line.trim())
                const formattedLines = lines.map(line => {
                    const trimmed = line.trim()
                    const indent = line.length - trimmed.length
                    const indentClass = `indent-${Math.min(Math.floor(indent / 2), 5)}`

                    if (trimmed.endsWith('/')) {
                        // Папка
                        return `<div class="item folder ${indentClass}">${trimmed}</div>`
                    } else {
                        // Файл
                        const extension = trimmed.split('.').pop() || ''
                        const fileClass = `file ${extension}`
                        return `<div class="item ${fileClass} ${indentClass}">${trimmed}</div>`
                    }
                }).join('')

                return `<div class="project-structure">${formattedLines}</div>`
            }
        )

        return htmlWithProjectStructure
    }

    // Функция для генерации оглавления из markdown
    function generateToc(markdown: string): Array<{id: string, title: string, level: number}> {
        // Очищаем markdown от badges для генерации TOC
        let cleanedMarkdown = markdown.replace(
            /<div align="center">[\s\S]*?<\/div>/g,
            ''
        )

        const toc: Array<{id: string, title: string, level: number}> = []
        const lines = cleanedMarkdown.split('\n')

        for (const line of lines) {
            // Ищем заголовки markdown (# ## ### и т.д.)
            const headingMatch = line.match(/^(#{1,6})\s+(.+)$/)
            if (headingMatch) {
                const level = headingMatch[1].length
                const title = headingMatch[2].trim()

                // Создаем ID как в HTML (lowercase, заменяем пробелы на дефисы)
                const id = title.toLowerCase().replace(/[^\w]+/g, '-')

                toc.push({
                    id: `heading-${id}`,
                    title,
                    level
                })
            }
        }

        return toc
    }

    try {
        // Определяем имя файла в зависимости от локали
        const readmeFile = locale === 'en' ? 'README.md' : `README_${locale.toUpperCase()}.md`
        const filePath = join(process.cwd(), '..', readmeFile)

        // Читаем файл
        const rawContent = readFileSync(filePath, 'utf-8')

        // Очищаем markdown перед парсингом
        const cleanedRawContent = cleanMarkdown(rawContent)

        // Парсим markdown в HTML
        const html = await parseMarkdown(cleanedRawContent)
        const toc = generateToc(cleanedRawContent)

        return {
            success: true,
            content: html,
            toc,
            locale
        }
    } catch (error) {
        console.error(`Error reading README for locale ${locale}:`, error)

        // Если файл не найден, возвращаем английский README
        try {
            const fallbackPath = join(process.cwd(), '..', 'README.md')
            const fallbackContent = readFileSync(fallbackPath, 'utf-8')

            // Очищаем markdown перед парсингом
            const cleanedFallbackContent = cleanMarkdown(fallbackContent)

            // Парсим fallback markdown в HTML
            const html = await parseMarkdown(cleanedFallbackContent)
            const toc = generateToc(fallbackContent)

            return {
                success: true,
                content: html,
                toc,
                locale: 'en',
                fallback: true
            }
        } catch (fallbackError) {
            console.error('Error reading fallback README:', fallbackError)

            return {
                success: false,
                error: 'Failed to load documentation',
                locale
            }
        }
    }
})