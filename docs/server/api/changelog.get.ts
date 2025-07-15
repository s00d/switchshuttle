import { readFileSync } from 'fs'
import { join } from 'path'
import { marked } from 'marked'

// Универсальная функция для генерации id
function toId(str: string) {
  return str
    .toLowerCase()
    .replace(/[^\w\s-]/g, '') // убрать всё кроме букв, цифр, _, пробелов и дефисов
    .replace(/\s+/g, '-')      // пробелы на дефис
}

export default defineEventHandler(async (event) => {
  try {
    const changelogPath = join(process.cwd(), '..', 'CHANGELOG.md')
    const changelogContent = readFileSync(changelogPath, 'utf-8')

    // Собираем TOC во время рендера
    const toc: Array<{ id: string, title: string, level: number }> = []

    marked.use({
      renderer: {
        heading(token: any) {
          const text = token.text.replace(/<[^>]*>/g, '')
          const level = token.depth
          let idText = text
          let displayText = text
          if (level === 2) {
            const versionMatch = text.match(/^\[([^\]]+)\]/)
            if (versionMatch) {
              idText = versionMatch[1]
              displayText = versionMatch[1]
            }
            // Только версии в TOC
            const id = toId(idText)
            toc.push({
              id: `heading-${id}`,
              title: displayText,
              level
            })
          }
          const id = `heading-${toId(idText)}`
          const anchor = toId(idText)
          return `\n<h${level} id=\"${id}\">\n  <a name=\"${anchor}\" class=\"anchor\" href=\"#${id}\">\n    <span class=\"header-link\"></span>\n  </a>\n  ${displayText}\n</h${level}>`
        }
      }
    })

    marked.setOptions({
      gfm: true,
      breaks: true
    })

    // Парсим markdown в HTML
    const html = await marked.parse(changelogContent)
    // Добавляем target="_blank" к внешним ссылкам
    const htmlWithLinks = html.replace(/<a href="(https?:\/\/[^"]+)">([^<]+)<\/a>/g, '<a href="$1" target="_blank" rel="noopener noreferrer">$2</a>')

    return {
      success: true,
      content: htmlWithLinks,
      toc
    }
  } catch (error) {
    console.error('Error reading changelog:', error)
    throw createError({
      statusCode: 500,
      statusMessage: 'Failed to load changelog'
    })
  }
}) 