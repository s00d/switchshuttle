<template>
  <div class="calculator">
    <div class="calculator-display">
      <div class="calculator-history">{{ history }}</div>
      <div class="calculator-current">{{ current || '0' }}</div>
    </div>
    
    <div class="calculator-buttons">
      <!-- Первый ряд -->
      <button class="calc-btn calc-btn-secondary" @click="clear">C</button>
      <button class="calc-btn calc-btn-secondary" @click="toggleSign">±</button>
      <button class="calc-btn calc-btn-secondary" @click="percentage">%</button>
      <button class="calc-btn calc-btn-operator" @click="setOperator('÷')">÷</button>
      
      <!-- Второй ряд -->
      <button class="calc-btn" @click="appendNumber('7')">7</button>
      <button class="calc-btn" @click="appendNumber('8')">8</button>
      <button class="calc-btn" @click="appendNumber('9')">9</button>
      <button class="calc-btn calc-btn-operator" @click="setOperator('×')">×</button>
      
      <!-- Третий ряд -->
      <button class="calc-btn" @click="appendNumber('4')">4</button>
      <button class="calc-btn" @click="appendNumber('5')">5</button>
      <button class="calc-btn" @click="appendNumber('6')">6</button>
      <button class="calc-btn calc-btn-operator" @click="setOperator('-')">−</button>
      
      <!-- Четвертый ряд -->
      <button class="calc-btn" @click="appendNumber('1')">1</button>
      <button class="calc-btn" @click="appendNumber('2')">2</button>
      <button class="calc-btn" @click="appendNumber('3')">3</button>
      <button class="calc-btn calc-btn-operator" @click="setOperator('+')">+</button>
      
      <!-- Пятый ряд -->
      <button class="calc-btn calc-btn-zero" @click="appendNumber('0')">0</button>
      <button class="calc-btn" @click="appendDecimal">.</button>
      <button class="calc-btn calc-btn-equals" @click="calculate">=</button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'

const current = ref('')
const history = ref('')
const operator = ref('')
const previousValue = ref(0)
const waitingForOperand = ref(false)

const appendNumber = (num: string) => {
  if (waitingForOperand.value) {
    current.value = num
    waitingForOperand.value = false
  } else {
    if (current.value === '0' && num !== '.') {
      current.value = num
    } else {
      current.value += num
    }
  }
}

const appendDecimal = () => {
  if (waitingForOperand.value) {
    current.value = '0.'
    waitingForOperand.value = false
  } else if (current.value.indexOf('.') === -1) {
    current.value += '.'
  }
}

const clear = () => {
  current.value = ''
  history.value = ''
  operator.value = ''
  previousValue.value = 0
  waitingForOperand.value = false
}

const toggleSign = () => {
  if (current.value !== '') {
    current.value = current.value.startsWith('-') 
      ? current.value.substring(1) 
      : '-' + current.value
  }
}

const percentage = () => {
  if (current.value !== '') {
    const value = parseFloat(current.value)
    current.value = (value / 100).toString()
  }
}

const setOperator = (newOperator: string) => {
  if (current.value !== '') {
    if (operator.value && !waitingForOperand.value) {
      calculate()
    }
    previousValue.value = parseFloat(current.value)
    operator.value = newOperator
    history.value = `${current.value} ${operator.value}`
    waitingForOperand.value = true
  } else if (operator.value && waitingForOperand.value) {
    operator.value = newOperator
    history.value = history.value.slice(0, -1) + newOperator
  }
}

const calculate = () => {
  if (current.value === '' || operator.value === '') return
  
  const currentValue = parseFloat(current.value)
  let result = 0
  
  switch (operator.value) {
    case '+':
      result = previousValue.value + currentValue
      break
    case '−':
      result = previousValue.value - currentValue
      break
    case '×':
      result = previousValue.value * currentValue
      break
    case '÷':
      if (currentValue === 0) {
        current.value = 'Error'
        return
      }
      result = previousValue.value / currentValue
      break
  }
  
  history.value = `${previousValue.value} ${operator.value} ${current.value} =`
  current.value = result.toString()
  operator.value = ''
  waitingForOperand.value = true
}
</script>

<style scoped>
.calculator {
  width: 100%;
  height: 100%;
  background: #1c1c1e;
  border-radius: 0;
  padding: 20px;
  display: flex;
  flex-direction: column;
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
}

.calculator-display {
  flex: 1;
  display: flex;
  flex-direction: column;
  justify-content: flex-end;
  margin-bottom: 20px;
  min-height: 80px;
}

.calculator-history {
  color: #8e8e93;
  font-size: 14px;
  text-align: right;
  margin-bottom: 8px;
  min-height: 20px;
  overflow: hidden;
  text-overflow: ellipsis;
}

.calculator-current {
  color: #ffffff;
  font-size: 48px;
  font-weight: 300;
  text-align: right;
  line-height: 1;
  overflow: hidden;
  text-overflow: ellipsis;
}

.calculator-buttons {
  display: grid;
  grid-template-columns: repeat(4, 1fr);
  gap: 12px;
  flex: 1;
}

.calc-btn {
  border: none;
  border-radius: 50%;
  font-size: 24px;
  font-weight: 400;
  cursor: pointer;
  transition: all 0.2s ease;
  display: flex;
  align-items: center;
  justify-content: center;
  min-height: 60px;
  user-select: none;
}

.calc-btn:hover {
  transform: scale(1.05);
  filter: brightness(1.1);
}

.calc-btn:active {
  transform: scale(0.95);
}

.calc-btn:not(.calc-btn-operator):not(.calc-btn-equals):not(.calc-btn-secondary) {
  background: #333333;
  color: #ffffff;
}

.calc-btn-operator {
  background: #ff9500;
  color: #ffffff;
}

.calc-btn-equals {
  background: #ff9500;
  color: #ffffff;
}

.calc-btn-secondary {
  background: #a5a5a5;
  color: #000000;
}

.calc-btn-zero {
  grid-column: span 2;
  border-radius: 30px;
  justify-content: flex-start;
  padding-left: 24px;
}

/* Анимации */
@keyframes buttonPress {
  0% { transform: scale(1); }
  50% { transform: scale(0.95); }
  100% { transform: scale(1); }
}

.calc-btn:active {
  animation: buttonPress 0.1s ease;
}

/* Адаптивность */
@media (max-width: 400px) {
  .calculator {
    padding: 16px;
  }
  
  .calc-btn {
    font-size: 20px;
    min-height: 50px;
  }
  
  .calculator-current {
    font-size: 36px;
  }
}
</style> 