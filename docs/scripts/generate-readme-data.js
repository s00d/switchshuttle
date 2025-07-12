import { readFileSync, writeFileSync, existsSync } from 'fs'
import { join } from 'path'
import { marked } from 'marked'

// Функция для очистки HTML после парсинга
function cleanHtml(html) {
  // Удаляем блок с badges и центрированием
  let cleaned = html.replace(
    /<div align="center">[\s\S]*?<\/div>/g,
    ''
  )
  
  // Удаляем блок с ссылками на локали
  cleaned = cleaned.replace(
    /<div class="locale">[\s\S]*?<\/div>/g,
    ''
  )
  
  // Удаляем пустые строки в начале документа
  cleaned = cleaned.replace(/^\s+/, '')
  
  return cleaned
}

// Функция для парсинга markdown
async function parseMarkdown(markdown) {
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
  const htmlWithAnchors = cleanedHtml.replace(/<h([1-6])([^>]*)>(.*?)<\/h[1-6]>/g, (match, level, attrs, text) => {
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
  
  return htmlWithAllCodeBlocks
}

// Функция для генерации оглавления из markdown
function generateToc(markdown) {
  // Очищаем markdown от badges для генерации TOC
  let cleanedMarkdown = markdown.replace(
    /<div align="center">[\s\S]*?<\/div>/g,
    ''
  )
  
  const toc = []
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

// Основная функция генерации
async function generateReadmeData() {
  const locales = ['en', 'ru', 'de', 'ja', 'zh']
  const dataDir = join(process.cwd(), 'data', 'readme')
  
  // Создаем папку если не существует
  if (!existsSync(dataDir)) {
    const fs = await import('fs')
    fs.mkdirSync(dataDir, { recursive: true })
  }
  
  for (const locale of locales) {
    try {
      // Определяем имя файла в зависимости от локали
      const readmeFile = locale === 'en' ? 'README.md' : `README_${locale.toUpperCase()}.md`
      const filePath = join(process.cwd(), '..', readmeFile)
      
      if (!existsSync(filePath)) {
        console.log(`Skipping ${readmeFile} - file not found`)
        continue
      }
      
      // Читаем файл
      const rawContent = readFileSync(filePath, 'utf-8')
      
      // Парсим markdown в HTML
      const html = await parseMarkdown(rawContent)
      const toc = generateToc(rawContent)
      
      // Создаем объект данных
      const data = {
        success: true,
        content: html,
        toc,
        locale
      }
      
      // Записываем в файл
      const outputPath = join(dataDir, `${locale}.json`)
      writeFileSync(outputPath, JSON.stringify(data, null, 2))
      
      console.log(`✅ Generated ${locale}.json`)
      
    } catch (error) {
      console.error(`❌ Error processing ${locale}:`, error.message)
      
      // Если файл не найден, создаем fallback с английским README
      try {
        const fallbackPath = join(process.cwd(), '..', 'README.md')
        if (existsSync(fallbackPath)) {
          const fallbackContent = readFileSync(fallbackPath, 'utf-8')
          const html = await parseMarkdown(fallbackContent)
          const toc = generateToc(fallbackContent)
          
          const data = {
            success: true,
            content: html,
            toc,
            locale: 'en',
            fallback: true
          }
          
          const outputPath = join(dataDir, `${locale}.json`)
          writeFileSync(outputPath, JSON.stringify(data, null, 2))
          
          console.log(`✅ Generated ${locale}.json (fallback)`)
        }
      } catch (fallbackError) {
        console.error(`❌ Error creating fallback for ${locale}:`, fallbackError.message)
      }
    }
  }
  
  console.log('🎉 README data generation completed!')
}

// Запускаем генерацию
generateReadmeData().catch(console.error) 