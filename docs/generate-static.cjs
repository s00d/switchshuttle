const fs = require('fs')
const path = require('path')

// Создаем папку docs если её нет
const docsDir = path.join(__dirname, '../docs')
if (!fs.existsSync(docsDir)) {
  fs.mkdirSync(docsDir, { recursive: true })
}

// Копируем файлы из .output/public в docs (в корень)
const publicDir = path.join(__dirname, '.output/public')

if (fs.existsSync(publicDir)) {
  // Рекурсивно копируем все файлы
  function copyDir(src, dest) {
    if (!fs.existsSync(dest)) {
      fs.mkdirSync(dest, { recursive: true })
    }
    
    const items = fs.readdirSync(src)
    for (const item of items) {
      const srcPath = path.join(src, item)
      const destPath = path.join(dest, item)
      
      if (fs.statSync(srcPath).isDirectory()) {
        copyDir(srcPath, destPath)
      } else {
        fs.copyFileSync(srcPath, destPath)
      }
    }
  }
  
  copyDir(publicDir, docsDir)
  console.log('✅ Статические файлы скопированы в docs/')
} else {
  console.log('❌ Папка .output/public не найдена. Сначала выполните pnpm build')
} 