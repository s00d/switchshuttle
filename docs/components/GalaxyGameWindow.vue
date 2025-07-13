<template>
  <div class="galaxy-game">
    <!-- Game Menu -->
    <div v-if="!gameStarted" class="game-menu">
      <div class="menu-content">
        <h1 class="game-title">🚀 Galaxy Game</h1>
        <div class="menu-buttons">
          <button @click="startGame" class="menu-btn start-btn">Start Game</button>
          <button @click="showHighScores" class="menu-btn scores-btn">High Scores</button>
          <button @click="showInstructions" class="menu-btn instructions-btn">Instructions</button>
        </div>
      </div>
    </div>

    <!-- Game Over Screen -->
    <div v-else-if="gameOver" class="game-over">
      <div class="game-over-content">
        <h2 class="game-over-title">Game Over!</h2>
        <div class="final-score">Final Score: {{ score }}</div>
        <div class="high-score" v-if="isNewHighScore">🎉 New High Score! 🎉</div>
        <div class="game-over-buttons">
          <button @click="restartGame" class="menu-btn restart-btn">Play Again</button>
          <button @click="backToMenu" class="menu-btn menu-btn-secondary">Main Menu</button>
        </div>
      </div>
    </div>

    <!-- Game Canvas -->
    <canvas 
      v-else
      ref="gameCanvas" 
      class="game-canvas"
      @mousemove="handleMouseMove"
      @click="handleClick"
      :style="{ border: '1px solid red' }"
    ></canvas>

    <!-- Game UI -->
    <div v-if="gameStarted && !gameOver" class="game-ui">
      <div class="ui-left">
        <div class="score">Score: {{ score }}</div>
        <div class="lives">Lives: {{ lives }}</div>
        <div class="level">Level: {{ level }}</div>
        <div class="controls">Controls: Mouse/A/D or ←/→</div>
      </div>
      <div class="ui-center">
        <div class="combo" v-if="combo > 0">Combo: {{ Math.floor(combo) }}x</div>
        <div class="power-up" v-if="isPowerUpActive">
          {{ powerUp }}: {{ powerUpTimeLeft }}s
        </div>
      </div>
      <div class="ui-right">
        <div class="max-combo">Max Combo: {{ maxCombo }}</div>
        <div class="fps">FPS: {{ Math.round(1000 / 16) }}</div>
      </div>
    </div>

    <!-- Pause Menu -->
    <div v-if="paused && gameStarted && !gameOver" class="pause-menu">
      <div class="pause-content">
        <h3>Game Paused</h3>
        <button @click="resumeGame" class="menu-btn">Resume</button>
        <button @click="backToMenu" class="menu-btn menu-btn-secondary">Main Menu</button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted, nextTick, computed } from 'vue'
import { useRuntimeConfig } from 'nuxt/app'

const config = useRuntimeConfig()

// Game state
const gameStarted = ref(false)
const gameOver = ref(false)
const paused = ref(false)
const score = ref(0)
const lives = ref(3)
const level = ref(1)
const isNewHighScore = ref(false)
const powerUp = ref('')
const powerUpTime = ref(0)
const combo = ref(0)
const maxCombo = ref(0)

// Computed properties
const powerUpTimeLeft = computed(() => {
  if (!powerUp.value || powerUpTime.value <= Date.now()) return 0
  return Math.ceil((powerUpTime.value - Date.now()) / 1000)
})

const isPowerUpActive = computed(() => {
  return powerUp.value && powerUpTime.value > Date.now()
})

// Canvas
const gameCanvas = ref<HTMLCanvasElement>()

// Game objects
let player: Player
let meteors: Meteor[] = []
let bullets: Bullet[] = []
let particles: Particle[] = []
let powerUps: PowerUp[] = []
let enemies: Enemy[] = []
let animationId: number
let gameLoopStarted = false

// SVG Logo
let logoImage: HTMLImageElement | null = null

// Mouse position
let mouseX = 0
let mouseY = 0

// Player movement
let playerMovingLeft = false
let playerMovingRight = false
const playerSpeed = 3

// Auto-fire for rapid fire
let lastShot = 0
const rapidFireInterval = 150 // milliseconds

// Game classes
class Player {
  x: number
  y: number
  size: number
  shield: boolean
  rapidFire: boolean
  tripleShot: boolean

  constructor(x: number, y: number) {
    this.x = x
    this.y = y
    this.size = 40
    this.shield = false
    this.rapidFire = false
    this.tripleShot = false
  }

  update() {
    // Убрали вращение
  }

  updateMovement(canvasWidth: number) {
    // Handle keyboard movement
    if (playerMovingLeft) {
      this.x = Math.max(this.size / 2, this.x - playerSpeed)
    }
    if (playerMovingRight) {
      this.x = Math.min(canvasWidth - this.size / 2, this.x + playerSpeed)
    }
  }

  draw(ctx: CanvasRenderingContext2D) {
    ctx.save()
    ctx.translate(this.x, this.y)
    ctx.rotate(-36 * Math.PI / 180) // Поворот на 35 градусов влево (-35°)
    
    // Draw shield
    if (this.shield) {
      ctx.strokeStyle = '#00FFFF'
      ctx.lineWidth = 3
      ctx.beginPath()
      ctx.arc(0, 0, this.size + 10, 0, Math.PI * 2)
      ctx.stroke()
    }
    
    // Draw SwitchShuttle logo
    if (logoImage) {
      // Draw SVG logo
      const logoSize = this.size * 1.5
      ctx.drawImage(logoImage, -logoSize/2, -logoSize/2, logoSize, logoSize)
    } else {
      // Fallback to original design
      ctx.fillStyle = '#007AFF'
      ctx.fillRect(-this.size/2, -this.size/2, this.size, this.size)
      ctx.fillStyle = '#5AC8FA'
      ctx.fillRect(-this.size/2 + 5, -this.size/2 + 5, this.size - 10, this.size - 10)
    }
    
    // Draw power-up indicators
    if (this.rapidFire) {
      ctx.fillStyle = '#FFD700'
      ctx.fillRect(-this.size/2 - 5, -this.size/2 - 5, 5, 5)
    }
    if (this.tripleShot) {
      ctx.fillStyle = '#FF6B6B'
      ctx.fillRect(this.size/2, -this.size/2 - 5, 5, 5)
    }
    
    ctx.restore()
  }
}

class PowerUp {
  x: number
  y: number
  vx: number
  vy: number
  type: string
  size: number
  rotation: number
  color: string

  constructor(x: number, y: number, type: string) {
    this.x = x
    this.y = y
    this.type = type
    this.size = 20
    this.rotation = 0
    this.vx = (Math.random() - 0.5) * 2
    this.vy = Math.random() * 2 + 1
    this.color = '#FFFFFF' // Default color
    
    switch (type) {
      case 'shield':
        this.color = '#00FFFF'
        break
      case 'rapidFire':
        this.color = '#FFD700'
        break
      case 'tripleShot':
        this.color = '#FF6B6B'
        break
      case 'life':
        this.color = '#00FF00'
        break
      case 'bomb':
        this.color = '#FF00FF'
        break
    }
  }

  update() {
    this.x += this.vx
    this.y += this.vy
    this.rotation += 0.05
  }

  draw(ctx: CanvasRenderingContext2D) {
    ctx.save()
    ctx.translate(this.x, this.y)
    ctx.rotate(this.rotation)
    
    ctx.fillStyle = this.color
    ctx.shadowColor = this.color
    ctx.shadowBlur = 10
    ctx.fillRect(-this.size/2, -this.size/2, this.size, this.size)
    ctx.shadowBlur = 0
    
    // Draw type indicator
    ctx.fillStyle = 'white'
    ctx.font = '12px Arial'
    ctx.textAlign = 'center'
    ctx.fillText(this.type.charAt(0).toUpperCase(), 0, 4)
    
    ctx.restore()
  }
}

class Enemy {
  x: number
  y: number
  vx: number
  vy: number
  size: number
  health: number
  maxHealth: number
  type: string
  lastShot: number

  constructor(x: number, y: number, type: string) {
    this.x = x
    this.y = y
    this.type = type
    this.lastShot = 0
    this.vx = 0
    this.vy = 0
    this.size = 30
    this.health = 1
    this.maxHealth = 1
    
    switch (type) {
      case 'shooter':
        this.size = 30
        this.health = 3
        this.maxHealth = 3
        this.vx = (Math.random() - 0.5) * 1 // Уменьшено с 2
        this.vy = Math.random() * 0.5 + 0.3 // Уменьшено с 1 + 0.5
        break
      case 'tank':
        this.size = 50
        this.health = 8
        this.maxHealth = 8
        this.vx = (Math.random() - 0.5) * 0.5 // Уменьшено с 1
        this.vy = Math.random() * 0.3 + 0.1 // Уменьшено с 0.5 + 0.2
        break
      case 'speedster':
        this.size = 20
        this.health = 1
        this.maxHealth = 1
        this.vx = (Math.random() - 0.5) * 2 // Уменьшено с 4
        this.vy = Math.random() * 1.5 + 1 // Уменьшено с 3 + 2
        break
    }
  }

  update() {
    this.x += this.vx
    this.y += this.vy
    
    // Bounce off edges
    if (this.x < this.size || this.x > 800 - this.size) this.vx *= -1
    if (this.y < this.size || this.y > 600 - this.size) this.vy *= -1
  }

  draw(ctx: CanvasRenderingContext2D) {
    // Draw enemy
    ctx.fillStyle = this.getColor()
    ctx.beginPath()
    ctx.arc(this.x, this.y, this.size, 0, Math.PI * 2)
    ctx.fill()
    
    // Draw health bar
    if (this.health < this.maxHealth) {
      const barWidth = this.size * 2
      const barHeight = 4
      const healthPercent = this.health / this.maxHealth
      
      ctx.fillStyle = '#FF0000'
      ctx.fillRect(this.x - barWidth/2, this.y - this.size - 10, barWidth, barHeight)
      ctx.fillStyle = '#00FF00'
      ctx.fillRect(this.x - barWidth/2, this.y - this.size - 10, barWidth * healthPercent, barHeight)
    }
  }

  getColor(): string {
    switch (this.type) {
      case 'shooter': return '#FF6B6B'
      case 'tank': return '#8B4513'
      case 'speedster': return '#FFD700'
      default: return '#FF0000'
    }
  }

  shoot(): Bullet | null {
    const now = Date.now()
    if (now - this.lastShot > 3000) { // Увеличено с 2000 до 3000 мс
      this.lastShot = now
      return new Bullet(this.x, this.y, player.x, player.y, true)
    }
    return null
  }
}

class Meteor {
  x: number
  y: number
  vx: number
  vy: number
  size: number
  color: string
  health: number

  constructor(x: number, y: number, targetX: number, targetY: number) {
    this.x = x
    this.y = y
    this.size = Math.random() * 20 + 10
    this.color = `hsl(${Math.random() * 60 + 10}, 70%, 50%)`
    this.health = Math.floor(this.size / 5) + 1
    
    // Calculate velocity towards target
    const dx = targetX - x
    const dy = targetY - y
    const distance = Math.sqrt(dx * dx + dy * dy)
    const speed = Math.random() * 2 + 1
    this.vx = (dx / distance) * speed
    this.vy = (dy / distance) * speed
  }

  update() {
    this.x += this.vx
    this.y += this.vy
  }

  draw(ctx: CanvasRenderingContext2D) {
    ctx.fillStyle = this.color
    ctx.beginPath()
    ctx.arc(this.x, this.y, this.size, 0, Math.PI * 2)
    ctx.fill()
    
    // Add glow effect
    ctx.shadowColor = this.color
    ctx.shadowBlur = 10
    ctx.fill()
    ctx.shadowBlur = 0
  }

  takeDamage(): boolean {
    this.health--
    return this.health <= 0
  }
}

class Bullet {
  x: number
  y: number
  vx: number
  vy: number
  size: number
  isEnemy: boolean
  damage: number

  constructor(x: number, y: number, targetX: number, targetY: number, isEnemy: boolean = false) {
    this.x = x
    this.y = y
    this.size = isEnemy ? 6 : 4
    this.isEnemy = isEnemy
    this.damage = isEnemy ? 1 : 1
    
    // Calculate velocity towards target
    const dx = targetX - x
    const dy = targetY - y
    const distance = Math.sqrt(dx * dx + dy * dy)
    const speed = isEnemy ? 2 : 8 // Уменьшено с 4 до 2 для врагов
    this.vx = (dx / distance) * speed
    this.vy = (dy / distance) * speed
  }

  update() {
    this.x += this.vx
    this.y += this.vy
  }

  draw(ctx: CanvasRenderingContext2D) {
    if (this.isEnemy) {
      ctx.fillStyle = '#FF0000'
      ctx.shadowColor = '#FF0000'
    } else {
      ctx.fillStyle = '#FFD700'
      ctx.shadowColor = '#FFD700'
    }
    ctx.shadowBlur = 5
    ctx.beginPath()
    ctx.arc(this.x, this.y, this.size, 0, Math.PI * 2)
    ctx.fill()
    ctx.shadowBlur = 0
  }
}

class Particle {
  x: number
  y: number
  vx: number
  vy: number
  life: number
  maxLife: number
  color: string
  size: number

  constructor(x: number, y: number, color: string) {
    this.x = x
    this.y = y
    this.color = color
    this.size = Math.random() * 3 + 1
    this.life = 60
    this.maxLife = 60
    this.vx = (Math.random() - 0.5) * 8
    this.vy = (Math.random() - 0.5) * 8
  }

  update() {
    this.x += this.vx
    this.y += this.vy
    this.life--
    this.vx *= 0.98
    this.vy *= 0.98
  }

  draw(ctx: CanvasRenderingContext2D) {
    const alpha = this.life / this.maxLife
    ctx.globalAlpha = alpha
    ctx.fillStyle = this.color
    ctx.shadowColor = this.color
    ctx.shadowBlur = 10
    ctx.beginPath()
    ctx.arc(this.x, this.y, this.size, 0, Math.PI * 2)
    ctx.fill()
    ctx.shadowBlur = 0
    ctx.globalAlpha = 1
  }
}

// Sound effects (using Web Audio API)
let audioContext: AudioContext | null = null
let gainNode: GainNode | null = null

function initAudio() {
  if (typeof window !== 'undefined' && 'AudioContext' in window) {
    audioContext = new AudioContext()
    gainNode = audioContext.createGain()
    gainNode.connect(audioContext.destination)
    gainNode.gain.value = 0.3
  }
}

function playSound(frequency: number, duration: number, type: OscillatorType = 'sine') {
  if (!audioContext || !gainNode) return
  
  const oscillator = audioContext.createOscillator()
  const gain = audioContext.createGain()
  
  oscillator.connect(gain)
  gain.connect(gainNode)
  
  oscillator.frequency.value = frequency
  oscillator.type = type
  
  gain.gain.setValueAtTime(0.1, audioContext.currentTime)
  gain.gain.exponentialRampToValueAtTime(0.01, audioContext.currentTime + duration)
  
  oscillator.start(audioContext.currentTime)
  oscillator.stop(audioContext.currentTime + duration)
}

function playExplosionSound() {
  playSound(100, 0.3, 'sawtooth')
  setTimeout(() => playSound(80, 0.2, 'sawtooth'), 50)
  setTimeout(() => playSound(60, 0.1, 'sawtooth'), 100)
}

function playPowerUpSound() {
  playSound(800, 0.2, 'sine')
  setTimeout(() => playSound(1000, 0.2, 'sine'), 100)
  setTimeout(() => playSound(1200, 0.2, 'sine'), 200)
}

function playShootSound() {
  playSound(400, 0.1, 'square')
}

function getLogoPath() {
  const baseURL = config.app?.baseURL || ''
  
  // Если baseURL пустой или равен '/', используем относительный путь
  if (!baseURL || baseURL === '/') {
    return '/switchshuttle.svg'
  }
  
  // Иначе добавляем baseURL
  return `${baseURL}/switchshuttle.svg`
}

function loadLogoImage() {
  logoImage = new Image()
  logoImage.onload = () => {
    console.log('Logo image loaded successfully')
  }
  logoImage.onerror = () => {
    console.error('Failed to load logo image, trying alternative method')
    createLogoSVG()
  }
  logoImage.src = getLogoPath()
}

function createLogoSVG() {
  // Create a canvas to draw the SVG
  const canvas = document.createElement('canvas')
  const ctx = canvas.getContext('2d')
  if (!ctx) return
  
  canvas.width = 64
  canvas.height = 64
  
  // Create a simple SwitchShuttle logo as fallback
  ctx.fillStyle = '#007AFF'
  ctx.fillRect(0, 0, 64, 64)
  
  ctx.fillStyle = '#5AC8FA'
  ctx.fillRect(8, 8, 48, 48)
  
  // Add some details to make it look more like the original logo
  ctx.fillStyle = '#FFFFFF'
  ctx.fillRect(16, 16, 32, 32)
  
  ctx.fillStyle = '#007AFF'
  ctx.fillRect(20, 20, 24, 24)
  
  // Create image from canvas
  logoImage = new Image()
  logoImage.onload = () => {
    console.log('Fallback logo created successfully')
  }
  logoImage.src = canvas.toDataURL()
}

// Game functions
function startGame() {
  console.log('Starting game...')
  
  // Reset debug flag
  gameLoopStarted = false
  
  // Load logo if not loaded
  if (!logoImage) {
    loadLogoImage()
  }
  
  // Сначала устанавливаем состояние
  gameStarted.value = true
  gameOver.value = false
  paused.value = false
  score.value = 0
  lives.value = 3
  level.value = 1
  combo.value = 0
  maxCombo.value = 0
  powerUp.value = ''
  powerUpTime.value = 0
  
  // Ждем следующего тика для обновления DOM
  nextTick(() => {
    console.log('DOM updated, looking for canvas...')
    
    // Initialize audio
    initAudio()
    
    // Initialize player
    const canvas = gameCanvas.value
    console.log('Canvas ref:', canvas)
    
    if (!canvas) {
      console.error('Canvas not found!')
      return
    }
    
    console.log('Canvas found, initializing...')
    console.log('Canvas dimensions before:', canvas.width, 'x', canvas.height)
    
    canvas.width = 800
    canvas.height = 600
    
    console.log('Canvas dimensions after:', canvas.width, 'x', canvas.height)
    
    player = new Player(canvas.width / 2, canvas.height - 100)
    console.log('Player created at:', player.x, player.y)
    
    // Clear arrays
    meteors = []
    bullets = []
    particles = []
    powerUps = []
    enemies = []
    
    // Start game loop
    console.log('Starting game loop...')
    gameLoop()
  })
}

function restartGame() {
  startGame()
}

function backToMenu() {
  gameStarted.value = false
  gameOver.value = false
  paused.value = false
  if (animationId) {
    cancelAnimationFrame(animationId)
  }
}

function resumeGame() {
  paused.value = false
  gameLoop()
}

function showHighScores() {
  // TODO: Implement high scores
  alert('High Scores feature coming soon!')
}

function showInstructions() {
  alert(`Instructions:
- Move mouse or use A/D or ←/→ keys to move
- Click to shoot
- Destroy meteors and enemies before they hit you
- Collect power-ups for special abilities
- Survive as long as possible!`)
}

function handleMouseMove(event: MouseEvent) {
  const canvas = gameCanvas.value
  if (!canvas) return
  
  const rect = canvas.getBoundingClientRect()
  mouseX = event.clientX - rect.left
  mouseY = event.clientY - rect.top
  
  // Update player position to follow mouse horizontally (only if not using keyboard)
  if (player && !playerMovingLeft && !playerMovingRight) {
    player.x = Math.max(player.size / 2, Math.min(canvas.width - player.size / 2, mouseX))
  }
  
  // Auto-fire when rapid fire is active
  if (gameStarted.value && !gameOver.value && !paused.value && player && player.rapidFire) {
    const now = Date.now()
    if (now - lastShot > rapidFireInterval) {
      handleClick()
      lastShot = now
    }
  }
}

function handleClick() {
  if (!gameStarted.value || gameOver.value || paused.value) return
  
  if (!player) return
  
  // Play shoot sound
  playShootSound()
  
  // Create bullets based on power-ups
  if (player.tripleShot) {
    // Triple shot
    bullets.push(new Bullet(player.x, player.y, mouseX - 20, mouseY, false))
    bullets.push(new Bullet(player.x, player.y, mouseX, mouseY, false))
    bullets.push(new Bullet(player.x, player.y, mouseX + 20, mouseY, false))
  } else {
    // Single shot
    bullets.push(new Bullet(player.x, player.y, mouseX, mouseY, false))
  }
  
  lastShot = Date.now()
}

function spawnMeteor() {
  const canvas = gameCanvas.value
  if (!canvas) return
  
  if (!player) return
  
  const x = Math.random() * canvas.width
  const y = -50
  
  meteors.push(new Meteor(x, y, player.x, player.y))
}

function spawnPowerUp() {
  const canvas = gameCanvas.value
  if (!canvas) return
  
  const types = ['shield', 'rapidFire', 'tripleShot', 'life', 'bomb']
  const type = types[Math.floor(Math.random() * types.length)]
  const x = Math.random() * canvas.width
  const y = -50
  
  powerUps.push(new PowerUp(x, y, type))
}

function spawnEnemy() {
  const canvas = gameCanvas.value
  if (!canvas) return
  
  const types = ['shooter', 'tank', 'speedster']
  const type = types[Math.floor(Math.random() * types.length)]
  const x = Math.random() * canvas.width
  const y = -50
  
  enemies.push(new Enemy(x, y, type))
}

function activatePowerUp(type: string) {
  powerUp.value = type
  powerUpTime.value = Date.now() + 10000 // 10 seconds
  
  // Play power-up sound
  playPowerUpSound()
  
  switch (type) {
    case 'shield':
      player.shield = true
      setTimeout(() => {
        player.shield = false
        powerUp.value = ''
        powerUpTime.value = 0
      }, 10000)
      break
    case 'rapidFire':
      player.rapidFire = true
      setTimeout(() => {
        player.rapidFire = false
        powerUp.value = ''
        powerUpTime.value = 0
      }, 10000)
      break
    case 'tripleShot':
      player.tripleShot = true
      setTimeout(() => {
        player.tripleShot = false
        powerUp.value = ''
        powerUpTime.value = 0
      }, 10000)
      break
    case 'life':
      lives.value = Math.min(lives.value + 1, 5)
      // Очищаем сразу для мгновенных бонусов
      powerUp.value = ''
      powerUpTime.value = 0
      break
    case 'bomb':
      // Destroy all enemies and meteors
      enemies.forEach(enemy => {
        for (let i = 0; i < 15; i++) {
          particles.push(new Particle(enemy.x, enemy.y, enemy.getColor()))
        }
      })
      meteors.forEach(meteor => {
        for (let i = 0; i < 10; i++) {
          particles.push(new Particle(meteor.x, meteor.y, meteor.color))
        }
      })
      enemies = []
      meteors = []
      score.value += 50
      playExplosionSound()
      // Очищаем сразу для мгновенных бонусов
      powerUp.value = ''
      powerUpTime.value = 0
      break
  }
}

function checkCollisions() {
  // Check bullet-meteor collisions
  bullets.forEach((bullet, bulletIndex) => {
    meteors.forEach((meteor, meteorIndex) => {
      const dx = bullet.x - meteor.x
      const dy = bullet.y - meteor.y
      const distance = Math.sqrt(dx * dx + dy * dy)
      
      if (distance < meteor.size + bullet.size) {
        // Collision detected
        bullets.splice(bulletIndex, 1)
        
        if (meteor.takeDamage()) {
          meteors.splice(meteorIndex, 1)
          score.value += 10
          combo.value++
          maxCombo.value = Math.max(maxCombo.value, combo.value)
          
          // Create explosion particles
          for (let i = 0; i < 10; i++) {
            particles.push(new Particle(meteor.x, meteor.y, meteor.color))
          }
          
          // Play explosion sound
          playExplosionSound()
          
          // Chance to spawn power-up
          if (Math.random() < 0.1) {
            spawnPowerUp()
          }
        }
      }
    })
  })
  
  // Check bullet-enemy collisions
  bullets.forEach((bullet, bulletIndex) => {
    enemies.forEach((enemy, enemyIndex) => {
      const dx = bullet.x - enemy.x
      const dy = bullet.y - enemy.y
      const distance = Math.sqrt(dx * dx + dy * dy)
      
      if (distance < enemy.size + bullet.size) {
        // Collision detected
        bullets.splice(bulletIndex, 1)
        enemy.health--
        
        if (enemy.health <= 0) {
          enemies.splice(enemyIndex, 1)
          score.value += 25
          combo.value++
          maxCombo.value = Math.max(maxCombo.value, combo.value)
          
          // Create explosion particles
          for (let i = 0; i < 15; i++) {
            particles.push(new Particle(enemy.x, enemy.y, enemy.getColor()))
          }
          
          // Play explosion sound
          playExplosionSound()
          
          // Chance to spawn power-up
          if (Math.random() < 0.2) {
            spawnPowerUp()
          }
        }
      }
    })
  })
  
  // Check enemy bullet-player collisions
  bullets.forEach((bullet, bulletIndex) => {
    if (bullet.isEnemy) {
      const dx = bullet.x - player.x
      const dy = bullet.y - player.y
      const distance = Math.sqrt(dx * dx + dy * dy)
      
      if (distance < player.size / 2 + bullet.size) {
        bullets.splice(bulletIndex, 1)
        
        if (!player.shield) {
          lives.value--
          
          // Create explosion particles
          for (let i = 0; i < 15; i++) {
            particles.push(new Particle(player.x, player.y, '#FF6B6B'))
          }
          
          if (lives.value <= 0) {
            gameOver.value = true
            if (animationId) {
              cancelAnimationFrame(animationId)
            }
          }
        }
      }
    }
  })
  
  // Check meteor-player collisions
  meteors.forEach((meteor, index) => {
    const dx = meteor.x - player.x
    const dy = meteor.y - player.y
    const distance = Math.sqrt(dx * dx + dy * dy)
    
    if (distance < meteor.size + player.size / 2) {
      // Player hit
      meteors.splice(index, 1)
      
      if (!player.shield) {
        lives.value--
        
        // Create explosion particles
        for (let i = 0; i < 15; i++) {
          particles.push(new Particle(player.x, player.y, '#FF6B6B'))
        }
        
        if (lives.value <= 0) {
          gameOver.value = true
          if (animationId) {
            cancelAnimationFrame(animationId)
          }
        }
      }
    }
  })
  
  // Check enemy-player collisions
  enemies.forEach((enemy, index) => {
    const dx = enemy.x - player.x
    const dy = enemy.y - player.y
    const distance = Math.sqrt(dx * dx + dy * dy)
    
    if (distance < enemy.size + player.size / 2) {
      // Player hit
      enemies.splice(index, 1)
      
      if (!player.shield) {
        lives.value--
        
        // Create explosion particles
        for (let i = 0; i < 15; i++) {
          particles.push(new Particle(player.x, player.y, '#FF6B6B'))
        }
        
        if (lives.value <= 0) {
          gameOver.value = true
          if (animationId) {
            cancelAnimationFrame(animationId)
          }
        }
      }
    }
  })
  
  // Check power-up-player collisions
  powerUps.forEach((powerUp, index) => {
    const dx = powerUp.x - player.x
    const dy = powerUp.y - player.y
    const distance = Math.sqrt(dx * dx + dy * dy)
    
    if (distance < powerUp.size + player.size / 2) {
      // Power-up collected
      powerUps.splice(index, 1)
      activatePowerUp(powerUp.type)
    }
  })
  
  // Reset combo if no hits
  if (combo.value > 0) {
    combo.value = Math.max(0, combo.value - 0.1)
  }
}

function gameLoop() {
  if (paused.value || gameOver.value) return
  
  const canvas = gameCanvas.value
  if (!canvas) {
    console.error('Canvas not found in gameLoop')
    return
  }
  
  const ctx = canvas.getContext('2d')
  if (!ctx) {
    console.error('Context not found in gameLoop')
    return
  }
  
  // Check if player exists
  if (!player) {
    console.error('Player not found in gameLoop')
    return
  }
  
  // Debug: log first frame
  if (!gameLoopStarted) {
    console.log('First game loop frame')
    console.log('Canvas size:', canvas.width, 'x', canvas.height)
    console.log('Player position:', player.x, player.y)
    gameLoopStarted = true
  }
  
  // Clear canvas
  ctx.fillStyle = '#0A0A0A'
  ctx.fillRect(0, 0, canvas.width, canvas.height)
  
  // Draw stars background
  ctx.fillStyle = '#FFFFFF'
  for (let i = 0; i < 100; i++) {
    const x = (i * 37) % canvas.width
    const y = (i * 73) % canvas.height
    ctx.fillRect(x, y, 1, 1)
  }
  
  // Update and draw player
  player.update()
  player.updateMovement(canvas.width) // Pass canvas width to updateMovement
  player.draw(ctx)
  
  // Spawn meteors
  if (Math.random() < 0.01 + level.value * 0.002 && meteors.length < 8) { // Ограничение максимум 8 метеоритов
    spawnMeteor()
  }
  
  // Spawn enemies
  if (Math.random() < 0.005 + level.value * 0.001 && enemies.length < 5) { // Ограничение максимум 5 врагов
    spawnEnemy()
  }
  
  // Update and draw meteors
  meteors.forEach((meteor, index) => {
    meteor.update()
    meteor.draw(ctx)
    
    // Remove meteors that are off screen
    if (meteor.x < -100 || meteor.x > canvas.width + 100 || 
        meteor.y < -100 || meteor.y > canvas.height + 100) {
      meteors.splice(index, 1)
    }
  })
  
  // Update and draw enemies
  enemies.forEach((enemy, index) => {
    enemy.update()
    enemy.draw(ctx)
    
    // Enemy shooting
    const enemyBullet = enemy.shoot()
    if (enemyBullet) {
      bullets.push(enemyBullet)
    }
    
    // Remove enemies that are off screen
    if (enemy.x < -100 || enemy.x > canvas.width + 100 || 
        enemy.y < -100 || enemy.y > canvas.height + 100) {
      enemies.splice(index, 1)
    }
  })
  
  // Update and draw power-ups
  powerUps.forEach((powerUp, index) => {
    powerUp.update()
    powerUp.draw(ctx)
    
    // Remove power-ups that are off screen
    if (powerUp.x < -50 || powerUp.x > canvas.width + 50 || 
        powerUp.y < -50 || powerUp.y > canvas.height + 50) {
      powerUps.splice(index, 1)
    }
  })
  
  // Update and draw bullets
  bullets.forEach((bullet, index) => {
    bullet.update()
    bullet.draw(ctx)
    
    // Remove bullets that are off screen
    if (bullet.x < 0 || bullet.x > canvas.width || 
        bullet.y < 0 || bullet.y > canvas.height) {
      bullets.splice(index, 1)
    }
  })
  
  // Update and draw particles
  particles.forEach((particle, index) => {
    particle.update()
    particle.draw(ctx)
    
    if (particle.life <= 0) {
      particles.splice(index, 1)
    }
  })
  
  // Check collisions
  checkCollisions()
  
  // Level up
  if (score.value > level.value * 100) {
    level.value++
  }
  
  animationId = requestAnimationFrame(gameLoop)
}

function handleKeyDown(event: KeyboardEvent) {
  if (!gameStarted.value || gameOver.value || paused.value) return
  
  switch (event.code) {
    case 'ArrowLeft':
    case 'KeyA':
      playerMovingLeft = true
      break
    case 'ArrowRight':
    case 'KeyD':
      playerMovingRight = true
      break
  }
}

function handleKeyUp(event: KeyboardEvent) {
  switch (event.code) {
    case 'ArrowLeft':
    case 'KeyA':
      playerMovingLeft = false
      break
    case 'ArrowRight':
    case 'KeyD':
      playerMovingRight = false
      break
  }
}

// Lifecycle
onMounted(() => {
  // Canvas будет инициализирован при старте игры
  console.log('GalaxyGameWindow mounted')
  
  // Load logo image
  loadLogoImage()
  
  // Add global keyboard listeners
  document.addEventListener('keydown', handleKeyDown)
  document.addEventListener('keyup', handleKeyUp)
})

onUnmounted(() => {
  if (animationId) {
    cancelAnimationFrame(animationId)
  }
  
  // Remove global keyboard listeners
  document.removeEventListener('keydown', handleKeyDown)
  document.removeEventListener('keyup', handleKeyUp)
})
</script>

<style lang="scss" scoped>
.galaxy-game {
  width: 100%;
  height: 100%;
  background: #0A0A0A;
  position: relative;
  overflow: hidden;
  font-family: 'Orbitron', monospace;
}

.game-menu {
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  display: flex;
  align-items: center;
  justify-content: center;
  background: linear-gradient(135deg, #0A0A0A 0%, #1A1A2E 50%, #16213E 100%);
  
  &::before {
    content: '';
    position: absolute;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    background: 
      radial-gradient(circle at 20% 20%, rgba(0, 122, 255, 0.1) 0%, transparent 50%),
      radial-gradient(circle at 80% 80%, rgba(90, 200, 250, 0.1) 0%, transparent 50%);
    pointer-events: none;
  }
}

.menu-content {
  text-align: center;
  color: white;
  z-index: 1;
  position: relative;
}

.game-title {
  font-size: 4rem;
  margin-bottom: 3rem;
  text-shadow: 
    0 0 20px #007AFF,
    0 0 40px #007AFF,
    0 0 60px #007AFF;
  animation: glow 2s ease-in-out infinite alternate;
  font-weight: 900;
  letter-spacing: 0.2em;
  
  @keyframes glow {
    from { 
      text-shadow: 
        0 0 20px #007AFF,
        0 0 40px #007AFF,
        0 0 60px #007AFF;
    }
    to { 
      text-shadow: 
        0 0 30px #007AFF,
        0 0 50px #007AFF,
        0 0 70px #007AFF,
        0 0 90px #007AFF;
    }
  }
}

.menu-buttons {
  display: flex;
  flex-direction: column;
  gap: 1.5rem;
}

.menu-btn {
  padding: 1rem 2rem;
  font-size: 1.2rem;
  border: none;
  border-radius: 0.5rem;
  cursor: pointer;
  transition: all 0.3s cubic-bezier(0.25, 0.46, 0.45, 0.94);
  background: linear-gradient(135deg, #007AFF 0%, #5AC8FA 100%);
  color: white;
  font-weight: 600;
  text-transform: uppercase;
  letter-spacing: 0.1em;
  box-shadow: 
    0 4px 15px rgba(0, 122, 255, 0.3),
    inset 0 1px 0 rgba(255, 255, 255, 0.2);
  
  &:hover {
    transform: translateY(-3px) scale(1.05);
    box-shadow: 
      0 8px 25px rgba(0, 122, 255, 0.5),
      inset 0 1px 0 rgba(255, 255, 255, 0.3);
  }
  
  &:active {
    transform: translateY(-1px) scale(1.02);
  }
  
  &.menu-btn-secondary {
    background: linear-gradient(135deg, #666 0%, #999 100%);
    box-shadow: 
      0 4px 15px rgba(102, 102, 102, 0.3),
      inset 0 1px 0 rgba(255, 255, 255, 0.2);
    
    &:hover {
      box-shadow: 
        0 8px 25px rgba(102, 102, 102, 0.5),
        inset 0 1px 0 rgba(255, 255, 255, 0.3);
    }
  }
}

.game-canvas {
  width: 100%;
  height: 100%;
  cursor: crosshair;
  display: block;
  background: #0A0A0A;
}

.game-ui {
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  pointer-events: none;
  padding: 1.5rem;
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  
  .ui-left, .ui-center, .ui-right {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }
  
  .ui-center {
    align-items: center;
    justify-content: center;
    height: 100%;
  }
  
  .ui-right {
    align-items: flex-end;
  }
  
  .score, .lives, .level, .combo, .power-up, .max-combo, .fps, .controls {
    color: white;
    font-size: 1.1rem;
    font-weight: 600;
    text-shadow: 
      0 0 10px rgba(0, 0, 0, 0.8),
      0 2px 4px rgba(0, 0, 0, 0.5);
    padding: 0.5rem 1rem;
    background: rgba(0, 0, 0, 0.6);
    border-radius: 0.5rem;
    backdrop-filter: blur(10px);
    border: 1px solid rgba(255, 255, 255, 0.1);
  }
  
  .controls {
    color: #FFD700;
    font-size: 0.9rem;
    opacity: 0.8;
  }
  
  .combo {
    color: #FFD700;
    font-size: 1.5rem;
    font-weight: 700;
    animation: pulse 1s ease-in-out infinite;
    
    @keyframes pulse {
      0%, 100% { opacity: 1; transform: scale(1); }
      50% { opacity: 0.8; transform: scale(1.05); }
    }
  }
  
  .power-up {
    color: #00FFFF;
    font-size: 1.2rem;
    font-weight: 700;
    animation: glow 1s ease-in-out infinite alternate;
    
    @keyframes glow {
      from { text-shadow: 0 0 10px #00FFFF; }
      to { text-shadow: 0 0 20px #00FFFF, 0 0 30px #00FFFF; }
    }
  }
}

.game-over {
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  display: flex;
  align-items: center;
  justify-content: center;
  background: rgba(0, 0, 0, 0.9);
  backdrop-filter: blur(10px);
}

.game-over-content {
  text-align: center;
  color: white;
  background: rgba(0, 0, 0, 0.8);
  padding: 3rem;
  border-radius: 1rem;
  border: 1px solid rgba(255, 255, 255, 0.1);
  box-shadow: 0 20px 40px rgba(0, 0, 0, 0.5);
}

.game-over-title {
  font-size: 3rem;
  margin-bottom: 1.5rem;
  color: #FF6B6B;
  text-shadow: 0 0 20px #FF6B6B;
  animation: shake 0.5s ease-in-out;
  
  @keyframes shake {
    0%, 100% { transform: translateX(0); }
    25% { transform: translateX(-5px); }
    75% { transform: translateX(5px); }
  }
}

.final-score {
  font-size: 1.5rem;
  margin-bottom: 0.5rem;
  color: #FFD700;
}

.high-score {
  font-size: 1.2rem;
  color: #FFD700;
  margin-bottom: 2rem;
  animation: pulse 1s ease-in-out infinite;
  
  @keyframes pulse {
    0%, 100% { opacity: 1; }
    50% { opacity: 0.7; }
  }
}

.game-over-buttons {
  display: flex;
  gap: 1rem;
  justify-content: center;
}

.pause-menu {
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  display: flex;
  align-items: center;
  justify-content: center;
  background: rgba(0, 0, 0, 0.8);
  backdrop-filter: blur(10px);
}

.pause-content {
  text-align: center;
  color: white;
  background: rgba(0, 0, 0, 0.8);
  padding: 2rem;
  border-radius: 1rem;
  border: 1px solid rgba(255, 255, 255, 0.1);
  
  h3 {
    font-size: 2rem;
    margin-bottom: 2rem;
    color: #007AFF;
  }
}

// Responsive design
@media (max-width: 768px) {
  .game-title {
    font-size: 2.5rem;
  }
  
  .menu-btn {
    padding: 0.8rem 1.5rem;
    font-size: 1rem;
  }
  
  .game-ui {
    padding: 1rem;
    
    .score, .lives, .level, .combo, .power-up, .max-combo, .fps {
      font-size: 0.9rem;
      padding: 0.3rem 0.8rem;
    }
    
    .combo {
      font-size: 1.2rem;
    }
    
    .power-up {
      font-size: 1rem;
    }
  }
  
  .game-over-title {
    font-size: 2rem;
  }
  
  .final-score {
    font-size: 1.2rem;
  }
}
</style> 