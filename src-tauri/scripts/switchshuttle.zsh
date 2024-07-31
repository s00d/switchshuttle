# Функция для получения списка команд из SwitchShuttle
function ss_commands() {
  curl -s http://127.0.0.1:8080/commands | jq -r '.[] | "\(.name)\t\(.description)"'
}

# Функция для выполнения команды
function ss_run() {
  local command_name=$1
  curl -s "http://127.0.0.1:9843/run/$command_name"
}

# Функция для автодополнения команд SwitchShuttle
function _ss_complete() {
  local -a commands
  commands=($(ss_commands))

  _describe 'command' commands
}

# Регистрация функции автодополнения для команды ss
compdef _ss_complete ss_run

# Добавление алиаса для выполнения команд SwitchShuttle
alias ss="ss_run"
