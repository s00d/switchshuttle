#!/bin/bash

# Скрипт для публикации SwitchShuttle на Homebrew Cask
# Использование: ./publish-to-homebrew.sh [version]

set -e

# Цвета для вывода
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Функция для вывода сообщений
log() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

warn() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Проверка аргументов
if [ $# -eq 0 ]; then
    # Получаем версию из package.json
    VERSION=$(node -p "require('./package.json').version")
    log "Используется версия из package.json: $VERSION"
else
    VERSION=$1
    log "Используется указанная версия: $VERSION"
fi

# Проверка наличия Homebrew
if ! command -v brew &> /dev/null; then
    error "Homebrew не установлен. Установите Homebrew: https://brew.sh"
    exit 1
fi

log "Начинаем процесс публикации SwitchShuttle v$VERSION на Homebrew Cask..."

# Проверяем, что релизы существуют на GitHub
log "Проверяем наличие релизов v$VERSION на GitHub..."

# URL для Intel (x64)
X64_URL="https://github.com/s00d/switchshuttle/releases/download/app-v$VERSION/switch-shuttle_${VERSION}_x64.dmg"
# URL для Apple Silicon (aarch64)
AARCH64_URL="https://github.com/s00d/switchshuttle/releases/download/app-v$VERSION/switch-shuttle_${VERSION}_aarch64.dmg"

# Проверяем Intel версию
if curl --output /dev/null --silent --head --fail "$X64_URL"; then
    log "Intel релиз найден: $X64_URL"
else
    error "Intel релиз не найден: $X64_URL"
    exit 1
fi

# Проверяем Apple Silicon версию
if curl --output /dev/null --silent --head --fail "$AARCH64_URL"; then
    log "Apple Silicon релиз найден: $AARCH64_URL"
else
    error "Apple Silicon релиз не найден: $AARCH64_URL"
    exit 1
fi

# Получаем SHA256 хеши
log "Получаем SHA256 хеши DMG файлов..."

X64_SHA256=$(curl -sL "$X64_URL" | shasum -a 256 | cut -d' ' -f1)
AARCH64_SHA256=$(curl -sL "$AARCH64_URL" | shasum -a 256 | cut -d' ' -f1)

log "Intel SHA256 хеш: $X64_SHA256"
log "Apple Silicon SHA256 хеш: $AARCH64_SHA256"

# Обновляем формулу Cask
log "Обновляем формулу Cask..."
sed -i.bak "s/version \"[^\"]*\"/version \"$VERSION\"/" Casks/switchshuttle.rb
sed -i.bak "s/sha256 \"[^\"]*\"/sha256 \"$X64_SHA256\"/" Casks/switchshuttle.rb
sed -i.bak "s/sha256 \"[^\"]*\"/sha256 \"$AARCH64_SHA256\"/" Casks/switchshuttle.rb


# Инструкции для пользователя
echo ""
log "✅ Формула Cask готова!"
echo ""
echo "Следующие шаги для публикации:"
echo ""
echo "1. Форкните репозиторий Homebrew Cask:"
echo "   https://github.com/Homebrew/homebrew-cask"
echo ""
echo "2. Клонируйте ваш форк:"
echo "   git clone https://github.com/YOUR_USERNAME/homebrew-cask.git"
echo "   cd homebrew-cask"
echo ""
echo "3. Создайте новую ветку:"
echo "   git checkout -b add-switchshuttle"
echo ""
echo "4. Скопируйте обновленную формулу:"
echo "   cp $PWD/Casks/switchshuttle.rb Casks/"
echo ""
echo "5. Протестируйте установку:"
echo "   brew install --cask Casks/switchshuttle.rb"
echo ""
echo "6. Создайте коммит и отправьте изменения:"
echo "   git add Casks/switchshuttle.rb"
echo "   git commit -m \"Add SwitchShuttle cask\""
echo "   git push origin add-switchshuttle"
echo ""
echo "7. Создайте Pull Request на:"
echo "   https://github.com/Homebrew/homebrew-cask"
echo ""
echo "Обновленная формула сохранена в: $PWD/Casks/switchshuttle.rb"
echo "Intel SHA256 хеш: $X64_SHA256"
echo "Apple Silicon SHA256 хеш: $AARCH64_SHA256"

# Очистка
rm -rf "$TEMP_DIR" 