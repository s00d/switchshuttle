<template>
  <div class="music-player">
    <div class="player-header">
      <div class="song-info">
        <div class="song-title">{{ currentSong.title }}</div>
        <div class="song-artist">{{ currentSong.artist }}</div>
        <div class="song-genre">{{ currentSong.genre }}</div>
      </div>
      <div class="visualizer">
        <canvas ref="visualizerCanvas" width="300" height="60"></canvas>
      </div>
    </div>
    
    <div class="player-controls">
      <div class="control-buttons">
        <button class="control-btn" @click="previousSong" :disabled="!isPlaying">
          <span>⏮</span>
        </button>
        <button class="control-btn play-btn" @click="togglePlay">
          <span>{{ isPlaying ? '⏸' : '▶' }}</span>
        </button>
        <button class="control-btn" @click="nextSong" :disabled="!isPlaying">
          <span>⏭</span>
        </button>
      </div>
      
      <div class="progress-container">
        <div class="progress-bar">
          <div class="progress-fill" :style="{ width: progress + '%' }"></div>
        </div>
        <div class="time-display">
          {{ formatTime(currentTime) }} / {{ formatTime(duration) }}
        </div>
      </div>
    </div>
    
    <div class="generator-settings">
      <h3>🎵 Professional Music Generator</h3>
      
      <div class="settings-grid">
        <div class="setting-group">
          <label>Genre:</label>
          <select v-model="selectedGenre" @change="updateGenre">
            <option value="ambient">Ambient</option>
            <option value="chill">Chill</option>
            <option value="electronic">Electronic</option>
            <option value="jazz">Jazz</option>
            <option value="classical">Classical</option>
            <option value="rock">Rock</option>
            <option value="lofi">Lo-Fi</option>
          </select>
        </div>
        
        <div class="setting-group">
          <label>Tempo (BPM):</label>
          <input 
            type="range" 
            v-model="tempo" 
            min="60" 
            max="180" 
            @input="updateTempo"
          >
          <span>{{ tempo }} BPM</span>
        </div>
        
        <div class="setting-group">
          <label>Volume:</label>
          <input 
            type="range" 
            v-model="volume" 
            min="0" 
            max="100" 
            @input="updateVolume"
          >
          <span>{{ volume }}%</span>
        </div>
        
        <div class="setting-group">
          <label>Complexity:</label>
          <select v-model="complexity" @change="updateComplexity">
            <option value="simple">Simple</option>
            <option value="medium">Medium</option>
            <option value="complex">Complex</option>
          </select>
        </div>
      </div>
      
      <div class="advanced-settings">
        <div class="setting-group">
          <label>Instruments:</label>
          <div class="instrument-toggles">
            <label class="toggle">
              <input type="checkbox" v-model="instruments.piano" @change="updateInstruments">
              <span>🎹 Piano</span>
            </label>
            <label class="toggle">
              <input type="checkbox" v-model="instruments.strings" @change="updateInstruments">
              <span>🎻 Strings</span>
            </label>
            <label class="toggle">
              <input type="checkbox" v-model="instruments.synth" @change="updateInstruments">
              <span>🎛️ Synth</span>
            </label>
            <label class="toggle">
              <input type="checkbox" v-model="instruments.drums" @change="updateInstruments">
              <span>🥁 Drums</span>
            </label>
            <label class="toggle">
              <input type="checkbox" v-model="instruments.bass" @change="updateInstruments">
              <span>🎸 Bass</span>
            </label>
            <label class="toggle">
              <input type="checkbox" v-model="instruments.brass" @change="updateInstruments">
              <span>🎺 Brass</span>
            </label>
            <label class="toggle">
              <input type="checkbox" v-model="instruments.flute" @change="updateInstruments">
              <span>🎵 Flute</span>
            </label>
            <label class="toggle">
              <input type="checkbox" v-model="instruments.organ" @change="updateInstruments">
              <span>🎹 Organ</span>
            </label>
          </div>
        </div>
        
        <div class="setting-group">
          <label>Mood:</label>
          <select v-model="mood" @change="updateMood">
            <option value="calm">Calm</option>
            <option value="energetic">Energetic</option>
            <option value="melancholic">Melancholic</option>
            <option value="uplifting">Uplifting</option>
          </select>
        </div>
      </div>
      
      <div class="action-buttons">
        <button class="action-btn" @click="generateNewSong">
          🎼 Generate New Song
        </button>
        <button class="action-btn" @click="toggleLoop" :class="{ active: isLooping }">
          🔄 {{ isLooping ? 'Loop Off' : 'Loop On' }}
        </button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted, watch } from 'vue'

// Состояние плеера
const isPlaying = ref(false)
const isLooping = ref(false)
const currentTime = ref(0)
const duration = ref(180) // 3 минуты
const progress = ref(0)
const volume = ref(50)
const tempo = ref(120)

// Настройки генератора
const selectedGenre = ref('ambient')
const complexity = ref('medium')
const mood = ref('calm')
const instruments = ref({
  piano: true,
  strings: true,
  synth: false,
  drums: false,
  bass: false,
  brass: false,
  flute: false,
  organ: false
})

// Текущая песня
const currentSong = ref({
  title: 'Ambient Dreams',
  artist: 'AI Composer',
  genre: 'Ambient'
})

// Canvas для визуализации
const visualizerCanvas = ref<HTMLCanvasElement>()

// Web Audio API
let audioContext: AudioContext | null = null
let oscillator: OscillatorNode | null = null
let gainNode: GainNode | null = null
let analyser: AnalyserNode | null = null
let animationId: number | null = null

// Таймеры для каждого слоя
let melodyTimer: ReturnType<typeof setTimeout> | null = null
let bassTimer: ReturnType<typeof setTimeout> | null = null
let chordTimer: ReturnType<typeof setTimeout> | null = null
let rhythmTimer: ReturnType<typeof setTimeout> | null = null

// Индексы для каждого слоя
let melodyIndex = 0
let bassIndex = 0
let chordIndex = 0
let rhythmIndex = 0

// Генераторы названий и артистов
const songTitles = {
  ambient: ['Ambient Dreams', 'Ethereal Flow', 'Cosmic Drift', 'Serene Waters', 'Mystic Echo', 'Floating Clouds', 'Deep Space', 'Zen Garden'],
  chill: ['Chill Vibes', 'Lazy Afternoon', 'Gentle Breeze', 'Peaceful Mind', 'Soft Glow', 'Sunset Drive', 'Ocean Waves', 'Mountain Air'],
  electronic: ['Digital Pulse', 'Neon Lights', 'Cyber Dreams', 'Electric Soul', 'Future Beat', 'Synthwave', 'Digital Rain', 'Electric Dreams'],
  jazz: ['Midnight Jazz', 'Smooth Groove', 'Blue Notes', 'Jazz Cafe', 'Swing Time', 'Late Night Blues', 'Smooth Operator', 'Jazz Fusion'],
  classical: ['Symphony No. 1', 'Moonlight Sonata', 'Spring Awakening', 'Elegant Waltz', 'Royal March', 'Concerto in C', 'Adagio', 'Prelude'],
  rock: ['Electric Storm', 'Rock Anthem', 'Power Drive', 'Rebel Heart', 'Thunder Road', 'Wild Fire', 'Rock Revolution', 'Heavy Metal'],
  lofi: ['Lo-Fi Beats', 'Study Session', 'Rainy Day', 'Coffee Shop', 'Late Night', 'Homework', 'Chill Study', 'Night Vibes']
}

const artistNames = {
  ambient: ['Ambient Mind', 'Ethereal Composer', 'Cosmic Drift', 'Serenity Project'],
  chill: ['Chill Master', 'Lazy Composer', 'Peaceful Mind', 'Gentle Soul'],
  electronic: ['Digital Pulse', 'Neon Composer', 'Cyber Mind', 'Electric Soul'],
  jazz: ['Jazz Master', 'Smooth Operator', 'Blue Note', 'Swing Time'],
  classical: ['Classical Composer', 'Symphony Master', 'Elegant Mind', 'Royal Composer'],
  rock: ['Rock Master', 'Electric Storm', 'Power Drive', 'Rebel Heart'],
  lofi: ['Lo-Fi Master', 'Study Beats', 'Rainy Day', 'Coffee Shop']
}

// Инициализация Web Audio API
function initAudio() {
  if (!audioContext) {
    audioContext = new (window.AudioContext || (window as any).webkitAudioContext)()
    analyser = audioContext.createAnalyser()
    analyser.fftSize = 256
    gainNode = audioContext.createGain()
    gainNode.connect(analyser)
    analyser.connect(audioContext.destination)
  }
}

// Генерация новой песни
function generateNewSong() {
  // Сначала останавливаем текущую музыку
  if (isPlaying.value) {
    stopAudio()
  }
  
  const genre = selectedGenre.value
  const titles = songTitles[genre as keyof typeof songTitles] || songTitles.ambient
  const artists = artistNames[genre as keyof typeof artistNames] || artistNames.ambient
  
  currentSong.value = {
    title: titles[Math.floor(Math.random() * titles.length)],
    artist: artists[Math.floor(Math.random() * artists.length)],
    genre: genre.charAt(0).toUpperCase() + genre.slice(1)
  }
  
  // Обновляем темп в зависимости от жанра (реалистичные темпы)
  const genreTempos = {
    ambient: 60,
    chill: 80,
    electronic: 120,
    jazz: 100,
    classical: 90,
    rock: 140,
    lofi: 85
  }
  
  tempo.value = genreTempos[genre as keyof typeof genreTempos] || 120
  
  // Запускаем новую музыку с небольшой задержкой
  setTimeout(() => {
    if (isPlaying.value) {
      startAudio()
    }
  }, 200)
}

// Создание осциллятора с улучшенным звуком
function createOscillator(frequency: number, type: OscillatorType = 'sine') {
  if (!audioContext) return null
  
  const osc = audioContext.createOscillator()
  const gain = audioContext.createGain()
  
  // Добавляем фильтр для более мягкого звука
  const filter = audioContext.createBiquadFilter()
  filter.type = 'lowpass'
  filter.frequency.setValueAtTime(frequency * 2, audioContext.currentTime)
  filter.Q.setValueAtTime(0.5, audioContext.currentTime)
  
  osc.frequency.setValueAtTime(frequency, audioContext.currentTime)
  osc.type = type
  
  osc.connect(filter)
  filter.connect(gain)
  gain.connect(gainNode!)
  
  return { oscillator: osc, gain: gain, filter: filter }
}

// Создание инструментов с реалистичными звуками
function createInstrumentSound(frequency: number, duration: number, volume: number = 0.3, instrumentType: string = 'piano') {
  if (!audioContext) return null
  
  const currentTime = audioContext.currentTime
  
  switch (instrumentType) {
    case 'piano':
      return createPianoSound(frequency, duration, volume)
    case 'strings':
      return createStringsSound(frequency, duration, volume)
    case 'synth':
      return createSynthSound(frequency, duration, volume)
    case 'bass':
      return createBassSound(frequency, duration, volume)
    case 'brass':
      return createBrassSound(frequency, duration, volume)
    case 'flute':
      return createFluteSound(frequency, duration, volume)
    case 'organ':
      return createOrganSound(frequency, duration, volume)
    default:
      return createRichSound(frequency, duration, volume)
  }
}

// Создание звука пианино (MIDI-стиль)
function createPianoSound(frequency: number, duration: number, volume: number) {
  if (!audioContext) return null
  
  const currentTime = audioContext.currentTime
  
  // Основной тон с быстрой атакой и резким затуханием (как MIDI)
  const mainOsc = createOscillator(frequency, 'sine')
  if (mainOsc) {
    mainOsc.gain.gain.setValueAtTime(0, currentTime)
    mainOsc.gain.gain.linearRampToValueAtTime(volume * 0.9, currentTime + 0.005) // Быстрая атака
    mainOsc.gain.gain.exponentialRampToValueAtTime(0.01, currentTime + duration * 0.1) // Быстрое затухание
    mainOsc.oscillator.start(currentTime)
    mainOsc.oscillator.stop(currentTime + duration)
  }
  
  // Обертоны для богатства звука (как в MIDI)
  const harmonics = [2, 3, 4]
  harmonics.forEach((harmonic, index) => {
    const harmonicOsc = createOscillator(frequency * harmonic, 'triangle')
    if (harmonicOsc) {
      const harmonicVolume = volume * (0.4 - index * 0.1)
      harmonicOsc.gain.gain.setValueAtTime(0, currentTime)
      harmonicOsc.gain.gain.linearRampToValueAtTime(harmonicVolume, currentTime + 0.005)
      harmonicOsc.gain.gain.exponentialRampToValueAtTime(0.01, currentTime + duration * 0.1)
      harmonicOsc.oscillator.start(currentTime)
      harmonicOsc.oscillator.stop(currentTime + duration)
    }
  })
  
  return { main: mainOsc }
}

// Создание звука струнных (MIDI-стиль)
function createStringsSound(frequency: number, duration: number, volume: number) {
  if (!audioContext) return null
  
  const currentTime = audioContext.currentTime
  
  // Основной тон с легким вибрато (как в MIDI)
  const mainOsc = createOscillator(frequency, 'sine')
  if (mainOsc) {
    // Добавляем легкое вибрато
    const vibratoOsc = audioContext.createOscillator()
    vibratoOsc.frequency.setValueAtTime(5, currentTime) // 5 Hz вибрато
    const vibratoGain = audioContext.createGain()
    vibratoGain.gain.setValueAtTime(1, currentTime) // 1 полутон вибрато
    
    vibratoOsc.connect(vibratoGain)
    vibratoGain.connect(mainOsc.oscillator.frequency)
    
    mainOsc.gain.gain.setValueAtTime(0, currentTime)
    mainOsc.gain.gain.linearRampToValueAtTime(volume * 0.8, currentTime + 0.01)
    mainOsc.gain.gain.exponentialRampToValueAtTime(0.01, currentTime + duration * 0.2)
    
    vibratoOsc.start(currentTime)
    mainOsc.oscillator.start(currentTime)
    vibratoOsc.stop(currentTime + duration)
    mainOsc.oscillator.stop(currentTime + duration)
  }
  
  // Добавляем второй осциллятор для богатства
  const secondOsc = createOscillator(frequency, 'triangle')
  if (secondOsc) {
    secondOsc.gain.gain.setValueAtTime(0, currentTime)
    secondOsc.gain.gain.linearRampToValueAtTime(volume * 0.3, currentTime + 0.01)
    secondOsc.gain.gain.exponentialRampToValueAtTime(0.01, currentTime + duration * 0.2)
    secondOsc.oscillator.start(currentTime)
    secondOsc.oscillator.stop(currentTime + duration)
  }
  
  return { main: mainOsc, second: secondOsc }
}

// Создание звука синтезатора (MIDI-стиль)
function createSynthSound(frequency: number, duration: number, volume: number) {
  if (!audioContext) return null
  
  const currentTime = audioContext.currentTime
  
  // Основной тон с быстрой атакой (как в MIDI)
  const mainOsc = createOscillator(frequency, 'square')
  if (mainOsc) {
    // Добавляем фильтр с быстрой огибающей
    const filter = audioContext.createBiquadFilter()
    filter.type = 'lowpass'
    filter.frequency.setValueAtTime(frequency * 6, currentTime)
    filter.frequency.exponentialRampToValueAtTime(frequency * 2, currentTime + duration * 0.1)
    
    mainOsc.oscillator.connect(filter)
    filter.connect(mainOsc.gain)
    
    mainOsc.gain.gain.setValueAtTime(0, currentTime)
    mainOsc.gain.gain.linearRampToValueAtTime(volume * 0.7, currentTime + 0.005)
    mainOsc.gain.gain.exponentialRampToValueAtTime(0.01, currentTime + duration * 0.1)
    
    mainOsc.oscillator.start(currentTime)
    mainOsc.oscillator.stop(currentTime + duration)
  }
  
  // Добавляем второй осциллятор для богатства
  const secondOsc = createOscillator(frequency * 2, 'sawtooth')
  if (secondOsc) {
    secondOsc.gain.gain.setValueAtTime(0, currentTime)
    secondOsc.gain.gain.linearRampToValueAtTime(volume * 0.3, currentTime + 0.005)
    secondOsc.gain.gain.exponentialRampToValueAtTime(0.01, currentTime + duration * 0.1)
    secondOsc.oscillator.start(currentTime)
    secondOsc.oscillator.stop(currentTime + duration)
  }
  
  return { main: mainOsc, second: secondOsc }
}

// Создание звука баса (MIDI-стиль)
function createBassSound(frequency: number, duration: number, volume: number) {
  if (!audioContext) return null
  
  const currentTime = audioContext.currentTime
  
  // Основной тон баса с быстрой атакой
  const mainOsc = createOscillator(frequency, 'square')
  if (mainOsc) {
    mainOsc.gain.gain.setValueAtTime(0, currentTime)
    mainOsc.gain.gain.linearRampToValueAtTime(volume * 0.9, currentTime + 0.005)
    mainOsc.gain.gain.exponentialRampToValueAtTime(0.01, currentTime + duration * 0.1)
    mainOsc.oscillator.start(currentTime)
    mainOsc.oscillator.stop(currentTime + duration)
  }
  
  // Суббас с быстрой атакой
  const subOsc = createOscillator(frequency * 0.5, 'sine')
  if (subOsc) {
    subOsc.gain.gain.setValueAtTime(0, currentTime)
    subOsc.gain.gain.linearRampToValueAtTime(volume * 0.5, currentTime + 0.01)
    subOsc.gain.gain.exponentialRampToValueAtTime(0.01, currentTime + duration * 0.1)
    subOsc.oscillator.start(currentTime)
    subOsc.oscillator.stop(currentTime + duration)
  }
  
  return { main: mainOsc, sub: subOsc }
}

// Создание звука духовых (MIDI-стиль)
function createBrassSound(frequency: number, duration: number, volume: number) {
  if (!audioContext) return null
  
  const currentTime = audioContext.currentTime
  
  // Основной тон с быстрой атакой
  const mainOsc = createOscillator(frequency, 'square')
  if (mainOsc) {
    mainOsc.gain.gain.setValueAtTime(0, currentTime)
    mainOsc.gain.gain.linearRampToValueAtTime(volume * 0.8, currentTime + 0.005)
    mainOsc.gain.gain.exponentialRampToValueAtTime(0.01, currentTime + duration * 0.1)
    mainOsc.oscillator.start(currentTime)
    mainOsc.oscillator.stop(currentTime + duration)
  }
  
  // Обертоны для богатства звука (как в MIDI)
  [2, 3].forEach((harmonic, index) => {
    const harmonicOsc = createOscillator(frequency * harmonic, 'triangle')
    if (harmonicOsc) {
      const harmonicVolume = volume * (0.3 - index * 0.1)
      harmonicOsc.gain.gain.setValueAtTime(0, currentTime)
      harmonicOsc.gain.gain.linearRampToValueAtTime(harmonicVolume, currentTime + 0.005)
      harmonicOsc.gain.gain.exponentialRampToValueAtTime(0.01, currentTime + duration * 0.1)
      harmonicOsc.oscillator.start(currentTime)
      harmonicOsc.oscillator.stop(currentTime + duration)
    }
  })
  
  return { main: mainOsc }
}

// Создание звука флейты (MIDI-стиль)
function createFluteSound(frequency: number, duration: number, volume: number) {
  if (!audioContext) return null
  
  const currentTime = audioContext.currentTime
  
  // Основной тон с быстрой атакой
  const mainOsc = createOscillator(frequency, 'sine')
  if (mainOsc) {
    mainOsc.gain.gain.setValueAtTime(0, currentTime)
    mainOsc.gain.gain.linearRampToValueAtTime(volume * 0.8, currentTime + 0.005)
    mainOsc.gain.gain.exponentialRampToValueAtTime(0.01, currentTime + duration * 0.1)
    mainOsc.oscillator.start(currentTime)
    mainOsc.oscillator.stop(currentTime + duration)
  }
  
  // Второй осциллятор для богатства
  const secondOsc = createOscillator(frequency * 2, 'triangle')
  if (secondOsc) {
    secondOsc.gain.gain.setValueAtTime(0, currentTime)
    secondOsc.gain.gain.linearRampToValueAtTime(volume * 0.2, currentTime + 0.005)
    secondOsc.gain.gain.exponentialRampToValueAtTime(0.01, currentTime + duration * 0.1)
    secondOsc.oscillator.start(currentTime)
    secondOsc.oscillator.stop(currentTime + duration)
  }
  
  return { main: mainOsc, second: secondOsc }
}

// Создание звука органа (MIDI-стиль)
function createOrganSound(frequency: number, duration: number, volume: number) {
  if (!audioContext) return null
  
  const currentTime = audioContext.currentTime
  
  // Основной тон с быстрой атакой
  const mainOsc = createOscillator(frequency, 'square')
  if (mainOsc) {
    mainOsc.gain.gain.setValueAtTime(0, currentTime)
    mainOsc.gain.gain.linearRampToValueAtTime(volume * 0.9, currentTime + 0.005)
    mainOsc.gain.gain.exponentialRampToValueAtTime(0.01, currentTime + duration * 0.1)
    mainOsc.oscillator.start(currentTime)
    mainOsc.oscillator.stop(currentTime + duration)
  }
  
  // Регистры органа (8', 4', 2') с быстрой атакой
  const registers = [1, 2, 4]
  registers.forEach((register, index) => {
    const registerOsc = createOscillator(frequency * register, 'square')
    if (registerOsc) {
      const registerVolume = volume * (0.5 - index * 0.1)
      registerOsc.gain.gain.setValueAtTime(0, currentTime)
      registerOsc.gain.gain.linearRampToValueAtTime(registerVolume, currentTime + 0.005)
      registerOsc.gain.gain.exponentialRampToValueAtTime(0.01, currentTime + duration * 0.1)
      registerOsc.oscillator.start(currentTime)
      registerOsc.oscillator.stop(currentTime + duration)
    }
  })
  
  return { main: mainOsc }
}

// Fallback функция для создания базового звука (MIDI-стиль)
function createRichSound(frequency: number, duration: number, volume: number = 0.3) {
  if (!audioContext) return null
  
  const currentTime = audioContext.currentTime
  
  // Основной осциллятор с быстрой атакой
  const mainOsc = createOscillator(frequency, 'sine')
  if (mainOsc) {
    mainOsc.gain.gain.setValueAtTime(0, currentTime)
    mainOsc.gain.gain.linearRampToValueAtTime(volume * 0.8, currentTime + 0.005)
    mainOsc.gain.gain.exponentialRampToValueAtTime(0.01, currentTime + duration * 0.1)
    mainOsc.oscillator.start(currentTime)
    mainOsc.oscillator.stop(currentTime + duration)
  }
  
  // Второй осциллятор для обертонов с быстрой атакой
  const harmonicOsc = createOscillator(frequency * 2, 'triangle')
  if (harmonicOsc) {
    harmonicOsc.gain.gain.setValueAtTime(0, currentTime)
    harmonicOsc.gain.gain.linearRampToValueAtTime(volume * 0.3, currentTime + 0.005)
    harmonicOsc.gain.gain.exponentialRampToValueAtTime(0.01, currentTime + duration * 0.1)
    harmonicOsc.oscillator.start(currentTime)
    harmonicOsc.oscillator.stop(currentTime + duration)
  }
  
  return { main: mainOsc, harmonic: harmonicOsc }
}

// Расширенные музыкальные константы с строгими правилами
const MUSICAL_SCALES = {
  major: [0, 2, 4, 5, 7, 9, 11, 12], // До, Ре, Ми, Фа, Соль, Ля, Си, До
  minor: [0, 2, 3, 5, 7, 8, 10, 12], // До, Ре, Ми-бемоль, Фа, Соль, Ля-бемоль, Си-бемоль, До
  pentatonic: [0, 2, 4, 7, 9, 12], // Пентатоника
  blues: [0, 3, 5, 6, 7, 10, 12], // Блюзовая гамма
  dorian: [0, 2, 3, 5, 7, 9, 10, 12], // Дорийский лад
  lydian: [0, 2, 4, 6, 7, 9, 11, 12], // Лидийский лад
  mixolydian: [0, 2, 4, 5, 7, 9, 10, 12], // Миксолидийский лад
  phrygian: [0, 1, 4, 5, 7, 8, 11, 12], // Фригийский лад
  locrian: [0, 1, 3, 5, 6, 8, 10, 12] // Локрийский лад
}

// Музыкальные правила и ограничения
const MUSICAL_RULES = {
  // Максимальный интервал между соседними нотами
  maxInterval: 7, // Большая септима
  
  // Предпочтительные интервалы для мелодии
  preferredIntervals: [1, 2, 3, 4, 5, 7, 8, 12], // Прима, секунда, терция, кварта, квинта, септима, октава
  
  // Избегаемые интервалы (тритоны)
  avoidedIntervals: [6], // Тритон
  
  // Правила для аккордов
  chordRules: {
    // Минимальное расстояние между голосами
    minVoiceSpacing: 3,
    // Максимальное расстояние между голосами
    maxVoiceSpacing: 12,
    // Избегаемые параллели
    avoidParallels: ['fifths', 'octaves']
  },
  
  // Правила для баса
  bassRules: {
    // Бас должен быть ниже мелодии минимум на октаву
    minOctaveBelow: 1,
    // Предпочтительные интервалы с мелодией
    preferredIntervals: [1, 3, 5, 8, 10, 12]
  },
  
  // Правила для ритма
  rhythmRules: {
    // Синкопа не должна быть слишком частой
    maxSyncopation: 0.3,
    // Минимальная длительность ноты
    minNoteDuration: 0.25,
    // Максимальная длительность ноты
    maxNoteDuration: 4.0
  }
}

// Гармонические функции аккордов
const CHORD_FUNCTIONS = {
  tonic: [0, 3, 6], // I, iii, vi
  dominant: [4, 7], // V, vii°
  subdominant: [1, 4], // ii, IV
  leading: [6, 7] // vi, vii°
}

// Функции для проверки музыкальных правил
function checkMelodicInterval(note1: number, note2: number): boolean {
  const interval = Math.abs(note2 - note1)
  return interval <= MUSICAL_RULES.maxInterval && 
         !MUSICAL_RULES.avoidedIntervals.includes(interval)
}

function checkHarmonicProgression(chord1: number, chord2: number): boolean {
  // Проверяем, что прогрессия следует правилам гармонии
  const functions1 = Object.entries(CHORD_FUNCTIONS).find(([_, chords]) => 
    chords.includes(chord1 % 7)
  )
  const functions2 = Object.entries(CHORD_FUNCTIONS).find(([_, chords]) => 
    chords.includes(chord2 % 7)
  )
  
  // Tonic -> Dominant -> Tonic (правильная прогрессия)
  if (functions1?.[0] === 'tonic' && functions2?.[0] === 'dominant') return true
  if (functions1?.[0] === 'dominant' && functions2?.[0] === 'tonic') return true
  
  // Subdominant -> Dominant -> Tonic (правильная прогрессия)
  if (functions1?.[0] === 'subdominant' && functions2?.[0] === 'dominant') return true
  
  return false
}

function validateChordSpacing(chord: number[]): boolean {
  // Проверяем расстояние между голосами аккорда
  for (let i = 0; i < chord.length - 1; i++) {
    const interval = chord[i + 1] - chord[i]
    if (interval < MUSICAL_RULES.chordRules.minVoiceSpacing || 
        interval > MUSICAL_RULES.chordRules.maxVoiceSpacing) {
      return false
    }
  }
  return true
}

function getValidNextNote(currentNote: number, scale: number[], previousNote?: number): number {
  const validNotes = []
  
  for (let i = 0; i < scale.length; i++) {
    const candidateNote = scale[i]
    
    // Проверяем интервал с предыдущей нотой
    if (previousNote !== undefined) {
      if (!checkMelodicInterval(previousNote, candidateNote)) {
        continue
      }
    }
    
    // Предпочитаем ноты из гаммы
    if (scale.includes(candidateNote)) {
      validNotes.push(candidateNote)
    }
  }
  
  return validNotes.length > 0 ? validNotes[Math.floor(Math.random() * validNotes.length)] : scale[0]
}

// Выбор гаммы в зависимости от жанра
const GENRE_SCALES = {
  ambient: 'dorian',
  chill: 'pentatonic',
  electronic: 'mixolydian',
  jazz: 'blues',
  classical: 'major',
  rock: 'minor',
  lofi: 'pentatonic'
}

// Расширенные аккордовые прогрессии
const CHORD_PROGRESSIONS = {
  pop: [0, 5, 3, 4], // I-V-vi-IV
  jazz: [2, 5, 1, 4], // ii-V-I-IV
  rock: [0, 4, 0, 5], // I-V-I-V
  ambient: [0, 3, 4, 0], // I-iii-IV-I
  classical: [0, 4, 5, 3], // I-IV-V-vi
  lofi: [0, 3, 4, 0], // I-iii-IV-I
  electronic: [0, 4, 3, 4] // I-IV-iii-IV
}

// Типы аккордов
const CHORD_TYPES = {
  major: [0, 4, 7], // Мажорный аккорд
  minor: [0, 3, 7], // Минорный аккорд
  diminished: [0, 3, 6], // Уменьшенный аккорд
  augmented: [0, 4, 8], // Увеличенный аккорд
  sus2: [0, 2, 7], // Sus2
  sus4: [0, 5, 7], // Sus4
  maj7: [0, 4, 7, 11], // Мажорный септаккорд
  min7: [0, 3, 7, 10], // Минорный септаккорд
  dom7: [0, 4, 7, 10] // Доминантсептаккорд
}

// Мелодические паттерны
const MELODY_PATTERNS = {
  ambient: ['ascending', 'descending', 'repetitive'],
  chill: ['stepwise', 'arpeggio', 'sustained'],
  electronic: ['repetitive', 'ascending', 'descending'],
  jazz: ['chromatic', 'arpeggio', 'stepwise'],
  classical: ['ascending', 'descending', 'ornate'],
  rock: ['repetitive', 'ascending', 'descending'],
  lofi: ['stepwise', 'repetitive', 'sustained']
}

// Ритмические паттерны
const RHYTHM_PATTERNS = {
  ambient: [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], // Очень медленный
  chill: [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], // Медленный
  electronic: [1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0], // Быстрый
  jazz: [1, 0, 0, 1, 0, 1, 0, 0, 1, 0, 0, 1, 0, 1, 0, 0], // Свинг
  classical: [1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0], // Торжественный
  rock: [1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0], // Рок
  lofi: [1, 0, 0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 1] // Lo-fi
}

// Генерация мелодии с соблюдением строгих музыкальных правил
function generateMelody() {
  if (!audioContext) return
  
  const baseFreq = 220 // A3
  const notes = []
  
  // Выбираем тональность
  const key = Math.floor(Math.random() * 12) // 0-11 полутонов
  const keyFreq = baseFreq * Math.pow(2, key / 12)
  
  // Выбираем гамму в зависимости от жанра
  const scaleName = GENRE_SCALES[selectedGenre.value as keyof typeof GENRE_SCALES] || 'major'
  const selectedScale = MUSICAL_SCALES[scaleName as keyof typeof MUSICAL_SCALES] || MUSICAL_SCALES.major
  
  // Выбираем паттерн мелодии
  const patterns = MELODY_PATTERNS[selectedGenre.value as keyof typeof MELODY_PATTERNS] || MELODY_PATTERNS.ambient
  const pattern = patterns[Math.floor(Math.random() * patterns.length)]
  
  // Создаем структуру мелодии с соблюдением правил
  const melodyLength = 32
  let currentNote = 0
  let octave = 1
  let previousNote = undefined
  
  for (let i = 0; i < melodyLength; i++) {
    let noteIndex = 0
    let frequency = 0
    
    // Применяем строгие музыкальные правила
    if (i === 0) {
      // Начинаем с тоники (первая нота гаммы)
      currentNote = 0
    } else if (i === melodyLength - 1) {
      // Заканчиваем на тонике
      currentNote = 0
    } else {
      // Используем функцию для получения валидной следующей ноты
      currentNote = getValidNextNote(currentNote, selectedScale, previousNote)
    }
    
    // Применяем разные паттерны в зависимости от жанра
    switch (pattern) {
      case 'ascending':
        if (i < melodyLength / 2) {
          // Восходящая мелодия с проверкой интервалов
          const targetNote = Math.min(selectedScale.length - 1, Math.floor(i / 2))
          if (checkMelodicInterval(currentNote, targetNote)) {
            currentNote = targetNote
          }
        } else {
          // Нисходящая мелодия с проверкой интервалов
          const targetNote = Math.max(0, selectedScale.length - 1 - Math.floor((i - melodyLength / 2) / 2))
          if (checkMelodicInterval(currentNote, targetNote)) {
            currentNote = targetNote
          }
        }
        break
        
      case 'stepwise':
        // Пошаговое движение с проверкой интервалов
        if (i > 0) {
          const direction = Math.random() > 0.5 ? 1 : -1
          const candidateNote = Math.max(0, Math.min(selectedScale.length - 1, currentNote + direction))
          if (checkMelodicInterval(currentNote, candidateNote)) {
            currentNote = candidateNote
          }
        }
        break
        
      case 'arpeggio':
        // Арпеджио по аккордам с проверкой интервалов
        const chordNotes = [0, 2, 4, 7] // Основные ноты аккорда
        const targetNote = chordNotes[i % chordNotes.length]
        if (checkMelodicInterval(currentNote, targetNote)) {
          currentNote = targetNote
        }
        break
        
      case 'repetitive':
        // Повторяющиеся паттерны с вариациями
        const patternLength = 4
        const baseNote = selectedScale[i % patternLength]
        if (checkMelodicInterval(currentNote, baseNote)) {
          currentNote = baseNote
        }
        break
        
      case 'sustained':
        // Длительные ноты с редкими переходами
        if (i % 4 === 0) {
          const randomNote = Math.floor(Math.random() * selectedScale.length)
          if (checkMelodicInterval(currentNote, randomNote)) {
            currentNote = randomNote
          }
        }
        break
        
      default:
        // Используем валидную ноту из гаммы
        currentNote = getValidNextNote(currentNote, selectedScale, previousNote)
    }
    
    // Добавляем октавные переходы с проверкой интервалов
    if (i % 8 === 0 && i > 0) {
      const newOctave = Math.random() > 0.5 ? octave + 1 : Math.max(0, octave - 1)
      const octaveInterval = Math.abs((newOctave - octave) * 12)
      if (octaveInterval <= MUSICAL_RULES.maxInterval) {
        octave = newOctave
      }
    }
    
    // Создаем частоту с учетом октавы
    frequency = keyFreq * Math.pow(2, (selectedScale[currentNote] + (octave - 1) * 12) / 12)
    
    // Добавляем паузы для ритмичности (не чаще чем разрешено)
    if (Math.random() < 0.03) { // Уменьшили вероятность пауз
      notes.push(0) // Пауза
    } else {
      notes.push(frequency)
    }
    
    // Сохраняем предыдущую ноту для проверки интервалов
    previousNote = currentNote
  }
  
  return notes
}

// Генерация басовой линии
function generateBassLine() {
  if (!audioContext) return
  
  const baseFreq = 110 // A2
  const scaleName = GENRE_SCALES[selectedGenre.value as keyof typeof GENRE_SCALES] || 'major'
  const selectedScale = MUSICAL_SCALES[scaleName as keyof typeof MUSICAL_SCALES] || MUSICAL_SCALES.major
  const notes = []
  
  // Бас следует за аккордовой прогрессией
  const progression = CHORD_PROGRESSIONS[selectedGenre.value as keyof typeof CHORD_PROGRESSIONS] || CHORD_PROGRESSIONS.pop
  
  for (let i = 0; i < 4; i++) {
    const chordRoot = progression[i % progression.length]
    const frequency = baseFreq * Math.pow(2, chordRoot / 12)
    notes.push(frequency)
  }
  
  return notes
}

// Генерация аккордов с различными типами
function generateChords() {
  if (!audioContext) return
  
  const baseFreq = 220 // A3
  const scaleName = GENRE_SCALES[selectedGenre.value as keyof typeof GENRE_SCALES] || 'major'
  const selectedScale = MUSICAL_SCALES[scaleName as keyof typeof MUSICAL_SCALES] || MUSICAL_SCALES.major
  const chords = []
  
  // Используем аккордовую прогрессию
  const progression = CHORD_PROGRESSIONS[selectedGenre.value as keyof typeof CHORD_PROGRESSIONS] || CHORD_PROGRESSIONS.pop
  
  // Выбираем типы аккордов в зависимости от жанра
  const genreChordTypes = {
    ambient: ['major', 'sus4', 'maj7'],
    chill: ['major', 'minor', 'sus2'],
    electronic: ['major', 'minor', 'dom7'],
    jazz: ['maj7', 'min7', 'dom7'],
    classical: ['major', 'minor', 'diminished'],
    rock: ['major', 'minor', 'power'],
    lofi: ['major', 'minor', 'sus4']
  }
  
  const chordTypes = genreChordTypes[selectedGenre.value as keyof typeof genreChordTypes] || ['major', 'minor']
  
  for (let i = 0; i < 8; i++) { // Увеличиваем количество аккордов
    const chordRoot = progression[i % progression.length]
    const chordType = chordTypes[i % chordTypes.length]
    
    // Создаем аккорд в зависимости от типа
    let chordNotes: number[] = []
    
    switch (chordType) {
      case 'major':
        chordNotes = CHORD_TYPES.major
        break
      case 'minor':
        chordNotes = CHORD_TYPES.minor
        break
      case 'diminished':
        chordNotes = CHORD_TYPES.diminished
        break
      case 'augmented':
        chordNotes = CHORD_TYPES.augmented
        break
      case 'sus2':
        chordNotes = CHORD_TYPES.sus2
        break
      case 'sus4':
        chordNotes = CHORD_TYPES.sus4
        break
      case 'maj7':
        chordNotes = CHORD_TYPES.maj7
        break
      case 'min7':
        chordNotes = CHORD_TYPES.min7
        break
      case 'dom7':
        chordNotes = CHORD_TYPES.dom7
        break
      case 'power':
        // Power chord (только тоника и квинта)
        chordNotes = [0, 7]
        break
      default:
        chordNotes = CHORD_TYPES.major
    }
    
    // Создаем частоты для каждой ноты аккорда
    const chordFrequencies = chordNotes.map(note => {
      return baseFreq * Math.pow(2, (chordRoot + note) / 12)
    })
    
    chords.push(chordFrequencies)
  }
  
  return chords
}

// Генерация ритмического паттерна с улучшенными паттернами
function generateRhythm() {
  // Используем расширенные ритмические паттерны
  const pattern = RHYTHM_PATTERNS[selectedGenre.value as keyof typeof RHYTHM_PATTERNS] || RHYTHM_PATTERNS.ambient
  
  // Добавляем вариации в зависимости от сложности
  let rhythmPattern = [...pattern]
  
  if (complexity.value === 'complex') {
    // Добавляем дополнительные удары для сложности
    for (let i = 0; i < rhythmPattern.length; i++) {
      if (Math.random() < 0.2) {
        rhythmPattern[i] = 1
      }
    }
  } else if (complexity.value === 'simple') {
    // Упрощаем паттерн
    for (let i = 0; i < rhythmPattern.length; i++) {
      if (i % 2 === 1) {
        rhythmPattern[i] = 0
      }
    }
  }
  
  return rhythmPattern
}

// Воспроизведение аудио с улучшенной структурой
function startAudio() {
  if (!audioContext) {
    initAudio()
  }
  
  if (audioContext?.state === 'suspended') {
    audioContext.resume()
  }
  
  isPlaying.value = true
  currentTime.value = 0
  progress.value = 0
  
  // Генерируем все слои музыки
  const melody = generateMelody()
  const bassLine = generateBassLine()
  const chords = generateChords()
  const rhythm = generateRhythm()
  
  if (!melody || !melody.length) return
  
  // Разные длительности для разных слоев (более длинные звуки)
  const melodyDuration = 60 / tempo.value * 4 // 4/4 такт для мелодии
  const bassDuration = 60 / tempo.value * 8 // 8/4 такт для баса
  const chordDuration = 60 / tempo.value * 16 // 16/4 такт для аккордов
  const rhythmDuration = 60 / tempo.value * 2 // 2/4 такт для ритма
  
  // Структура композиции
  const sectionDuration = 30 // 30 секунд на секцию
  let currentSection = 'intro' // intro, verse, chorus, outro
  
  // Сбрасываем индексы при начале воспроизведения
  melodyIndex = 0
  bassIndex = 0
  chordIndex = 0
  rhythmIndex = 0
  
  // Запускаем все слои одновременно с разными интервалами
  
  // Слой 1: Мелодия (с выбором инструмента)
  function playMelody() {
    if (!isPlaying.value) return
    
    const currentTime = audioContext!.currentTime
    const melodyFreq = melody![melodyIndex % melody!.length]
    
    // Пропускаем паузы (частота 0)
    if (melodyFreq > 0) {
      // Выбираем инструмент для мелодии
      let instrumentType = 'piano'
      if (instruments.value.strings) instrumentType = 'strings'
      else if (instruments.value.synth) instrumentType = 'synth'
      else if (instruments.value.flute) instrumentType = 'flute'
      else if (instruments.value.brass) instrumentType = 'brass'
      else if (instruments.value.organ) instrumentType = 'organ'
      
      // Используем выбранный инструмент
      const instrumentSound = createInstrumentSound(melodyFreq, melodyDuration * 2, volume.value / 100, instrumentType)
      
      // Добавляем реверберацию для более естественного звука
      if (instrumentSound && instrumentSound.main) {
        const reverbGain = audioContext!.createGain()
        reverbGain.gain.setValueAtTime(0.1 * (volume.value / 100), currentTime)
        reverbGain.gain.exponentialRampToValueAtTime(0.01, currentTime + melodyDuration * 3)
        instrumentSound.main.gain.connect(reverbGain)
        reverbGain.connect(gainNode!)
      }
    }
    
    melodyIndex++
    
    melodyTimer = setTimeout(() => {
      if (isPlaying.value) {
        playMelody()
      }
    }, melodyDuration * 1000)
  }
  
  // Слой 2: Бас (с выбором инструмента)
  function playBass() {
    if (!isPlaying.value) return
    
    const currentTime = audioContext!.currentTime
    
    if (bassLine && bassLine.length > 0) {
      const bassFreq = bassLine[bassIndex % bassLine.length]
      
      // Выбираем инструмент для баса
      let instrumentType = 'bass'
      if (instruments.value.bass) instrumentType = 'bass'
      else if (instruments.value.brass) instrumentType = 'brass'
      else if (instruments.value.organ) instrumentType = 'organ'
      
      // Используем выбранный инструмент
      const bassSound = createInstrumentSound(bassFreq, bassDuration * 2, volume.value / 100, instrumentType)
      
      // Добавляем дополнительный суббас если выбран бас
      if (instrumentType === 'bass' && bassSound) {
        const subBassFreq = bassFreq * 0.5
        const subBassSound = createInstrumentSound(subBassFreq, bassDuration * 2, volume.value / 100 * 0.5, 'bass')
      }
    }
    
    bassIndex++
    
    bassTimer = setTimeout(() => {
      if (isPlaying.value) {
        playBass()
      }
    }, bassDuration * 1000)
  }
  
  // Слой 3: Аккорды (улучшенные)
  function playChords() {
    if (!isPlaying.value) return
    
    const currentTime = audioContext!.currentTime
    
    if (chords && chords.length > 0) {
      const chord = chords[chordIndex % chords.length]
      
      // Создаем мягкие аккорды с арпеджио
      chord.forEach((freq: number, index: number) => {
        const delay = index * 0.1 // Небольшая задержка для арпеджио
        
        const chordOsc = createOscillator(freq, 'sine')
        if (chordOsc) {
          chordOsc.gain.gain.setValueAtTime(0, currentTime + delay)
          chordOsc.gain.gain.linearRampToValueAtTime(0.15 * (volume.value / 100), currentTime + delay + 0.3)
          chordOsc.gain.gain.exponentialRampToValueAtTime(0.01, currentTime + delay + chordDuration * 3)
          chordOsc.oscillator.start(currentTime + delay)
          chordOsc.oscillator.stop(currentTime + delay + chordDuration * 3)
        }
        
        // Добавляем гармоники для богатства звука
        const harmonicOsc = createOscillator(freq * 2, 'triangle')
        if (harmonicOsc) {
          harmonicOsc.gain.gain.setValueAtTime(0, currentTime + delay)
          harmonicOsc.gain.gain.linearRampToValueAtTime(0.05 * (volume.value / 100), currentTime + delay + 0.4)
          harmonicOsc.gain.gain.exponentialRampToValueAtTime(0.01, currentTime + delay + chordDuration * 3)
          harmonicOsc.oscillator.start(currentTime + delay)
          harmonicOsc.oscillator.stop(currentTime + delay + chordDuration * 3)
        }
      })
    }
    
    chordIndex++
    
    chordTimer = setTimeout(() => {
      if (isPlaying.value) {
        playChords()
      }
    }, chordDuration * 1000)
  }
  
  // Слой 4: Ритм (улучшенный)
  function playRhythm() {
    if (!isPlaying.value) return
    
    const currentTime = audioContext!.currentTime
    
    if (rhythm && rhythm.length > 0 && instruments.value.drums) {
      const shouldPlay = rhythm[rhythmIndex % rhythm.length]
      if (shouldPlay) {
        // Создаем мягкий ритм вместо резкого
        const rhythmOsc = createOscillator(150, 'sine')
        if (rhythmOsc) {
          rhythmOsc.gain.gain.setValueAtTime(0, currentTime)
          rhythmOsc.gain.gain.linearRampToValueAtTime(0.08 * (volume.value / 100), currentTime + 0.05)
          rhythmOsc.gain.gain.exponentialRampToValueAtTime(0.01, currentTime + 0.3)
          rhythmOsc.oscillator.start(currentTime)
          rhythmOsc.oscillator.stop(currentTime + 0.3)
        }
        
        // Добавляем мягкий хай-хэт
        const hihatOsc = createOscillator(800, 'triangle')
        if (hihatOsc) {
          hihatOsc.gain.gain.setValueAtTime(0, currentTime)
          hihatOsc.gain.gain.linearRampToValueAtTime(0.03 * (volume.value / 100), currentTime + 0.02)
          hihatOsc.gain.gain.exponentialRampToValueAtTime(0.01, currentTime + 0.2)
          hihatOsc.oscillator.start(currentTime)
          hihatOsc.oscillator.stop(currentTime + 0.2)
        }
      }
    }
    
    rhythmIndex++
    
    rhythmTimer = setTimeout(() => {
      if (isPlaying.value) {
        playRhythm()
      }
    }, rhythmDuration * 1000)
  }
  
  // Запускаем все слои одновременно
  playMelody()
  playBass()
  playChords()
  playRhythm()
  
  // Обновляем прогресс
  const progressInterval = setInterval(() => {
    if (!isPlaying.value) {
      clearInterval(progressInterval)
      return
    }
    
    currentTime.value += 1
    progress.value = (currentTime.value / duration.value) * 100
    
    if (currentTime.value >= duration.value) {
      if (isLooping.value) {
        currentTime.value = 0
        progress.value = 0
      } else {
        stopAudio()
      }
    }
  }, 1000)
}

// Остановка аудио (мгновенная)
function stopAudio() {
  isPlaying.value = false
  
  // Очищаем все таймеры мгновенно
  if (melodyTimer) {
    clearTimeout(melodyTimer)
    melodyTimer = null
  }
  if (bassTimer) {
    clearTimeout(bassTimer)
    bassTimer = null
  }
  if (chordTimer) {
    clearTimeout(chordTimer)
    chordTimer = null
  }
  if (rhythmTimer) {
    clearTimeout(rhythmTimer)
    rhythmTimer = null
  }
  
  // Мгновенно останавливаем все звуки через gainNode
  if (gainNode && audioContext) {
    gainNode.gain.setValueAtTime(0, audioContext.currentTime)
    // Восстанавливаем громкость через небольшую задержку
    setTimeout(() => {
      if (gainNode && audioContext) {
        gainNode.gain.setValueAtTime(volume.value / 100, audioContext.currentTime)
      }
    }, 100)
  }
  
  // Сбрасываем индексы
  melodyIndex = 0
  bassIndex = 0
  chordIndex = 0
  rhythmIndex = 0
}

// Переключение воспроизведения
function togglePlay() {
  if (isPlaying.value) {
    stopAudio()
  } else {
    startAudio()
  }
}

// Следующая песня
function nextSong() {
  generateNewSong()
}

// Предыдущая песня
function previousSong() {
  generateNewSong()
}

// Переключение повтора
function toggleLoop() {
  isLooping.value = !isLooping.value
}

// Обновление темпа
function updateTempo() {
  // Если воспроизведение активно, перезапускаем с новым темпом
  if (isPlaying.value) {
    stopAudio()
    setTimeout(() => {
      startAudio()
    }, 100)
  }
}

// Обновление громкости
function updateVolume() {
  if (gainNode) {
    gainNode.gain.setValueAtTime(volume.value / 100, audioContext!.currentTime)
  }
}

// Обновление инструментов
function updateInstruments() {
  // Если воспроизведение активно, перезапускаем с новыми инструментами
  if (isPlaying.value) {
    stopAudio()
    setTimeout(() => {
      startAudio()
    }, 100)
  }
}

// Обновление жанра
function updateGenre() {
  // Генерируем новую музыку независимо от состояния воспроизведения
  generateNewSong()
  
  // Если воспроизведение активно, перезапускаем
  if (isPlaying.value) {
    stopAudio()
    setTimeout(() => {
      startAudio()
    }, 100)
  }
}

// Обновление настроения
function updateMood() {
  // Генерируем новую музыку независимо от состояния воспроизведения
  generateNewSong()
  
  // Если воспроизведение активно, перезапускаем
  if (isPlaying.value) {
    stopAudio()
    setTimeout(() => {
      startAudio()
    }, 100)
  }
}

// Обновление сложности
function updateComplexity() {
  // Генерируем новую музыку независимо от состояния воспроизведения
  generateNewSong()
  
  // Если воспроизведение активно, перезапускаем
  if (isPlaying.value) {
    stopAudio()
    setTimeout(() => {
      startAudio()
    }, 100)
  }
}

// Форматирование времени
function formatTime(seconds: number): string {
  const mins = Math.floor(seconds / 60)
  const secs = seconds % 60
  return `${mins}:${secs.toString().padStart(2, '0')}`
}

// Визуализация
function drawVisualizer() {
  if (!visualizerCanvas.value || !analyser) return
  
  const canvas = visualizerCanvas.value
  const ctx = canvas.getContext('2d')
  if (!ctx) return
  
  const bufferLength = analyser.frequencyBinCount
  const dataArray = new Uint8Array(bufferLength)
  
  function animate() {
    animationId = requestAnimationFrame(animate)
    
    analyser!.getByteFrequencyData(dataArray)
    
    ctx!.fillStyle = 'rgb(20, 20, 20)'
    ctx!.fillRect(0, 0, canvas.width, canvas.height)
    
    const barWidth = (canvas.width / bufferLength) * 2.5
    let barHeight
    let x = 0
    
    for (let i = 0; i < bufferLength; i++) {
      barHeight = dataArray[i] / 2
      
      const gradient = ctx!.createLinearGradient(0, 0, 0, canvas.height)
      gradient.addColorStop(0, '#4CAF50')
      gradient.addColorStop(1, '#2196F3')
      
      ctx!.fillStyle = gradient
      ctx!.fillRect(x, canvas.height - barHeight, barWidth, barHeight)
      
      x += barWidth + 1
    }
  }
  
  animate()
}

// Lifecycle
onMounted(() => {
  initAudio()
  generateNewSong()
  drawVisualizer()
})

onUnmounted(() => {
  if (animationId) {
    cancelAnimationFrame(animationId)
  }
  stopAudio()
  if (audioContext) {
    audioContext.close()
  }
})

// Следим за изменениями
watch(isPlaying, (playing) => {
  if (playing) {
    drawVisualizer()
  }
})
</script>

<style scoped>
.music-player {
  width: 100%;
  height: 100%;
  background: linear-gradient(135deg, #1e3c72 0%, #2a5298 100%);
  color: white;
  padding: 20px;
  display: flex;
  flex-direction: column;
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
  overflow-y: auto;
}

.player-header {
  margin-bottom: 20px;
}

.song-info {
  text-align: center;
  margin-bottom: 15px;
}

.song-title {
  font-size: 24px;
  font-weight: 600;
  margin-bottom: 5px;
}

.song-artist {
  font-size: 16px;
  opacity: 0.8;
  margin-bottom: 3px;
}

.song-genre {
  font-size: 12px;
  opacity: 0.6;
  text-transform: uppercase;
  letter-spacing: 1px;
}

.visualizer {
  margin: 15px 0;
  display: flex;
  justify-content: center;
}

.visualizer canvas {
  border-radius: 8px;
  background: rgba(0, 0, 0, 0.3);
}

.player-controls {
  margin-bottom: 30px;
}

.control-buttons {
  display: flex;
  justify-content: center;
  gap: 20px;
  margin-bottom: 20px;
}

.control-btn {
  width: 50px;
  height: 50px;
  border: none;
  border-radius: 50%;
  background: rgba(255, 255, 255, 0.1);
  color: white;
  font-size: 18px;
  cursor: pointer;
  transition: all 0.3s ease;
  display: flex;
  align-items: center;
  justify-content: center;
}

.control-btn:hover {
  background: rgba(255, 255, 255, 0.2);
  transform: scale(1.05);
}

.control-btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.play-btn {
  width: 60px;
  height: 60px;
  background: #4CAF50;
  font-size: 20px;
}

.play-btn:hover {
  background: #45a049;
}

.progress-container {
  margin-bottom: 20px;
}

.progress-bar {
  width: 100%;
  height: 6px;
  background: rgba(255, 255, 255, 0.2);
  border-radius: 3px;
  overflow: hidden;
  margin-bottom: 10px;
}

.progress-fill {
  height: 100%;
  background: linear-gradient(90deg, #4CAF50, #2196F3);
  border-radius: 3px;
  transition: width 0.3s ease;
}

.time-display {
  text-align: center;
  font-size: 14px;
  opacity: 0.8;
}

.generator-settings {
  flex: 1;
}

.generator-settings h3 {
  margin-bottom: 20px;
  text-align: center;
  font-size: 18px;
}

.settings-grid {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 20px;
  margin-bottom: 20px;
}

.setting-group {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.setting-group label {
  font-size: 14px;
  font-weight: 500;
  opacity: 0.9;
}

.setting-group select,
.setting-group input[type="range"] {
  padding: 8px;
  border: none;
  border-radius: 6px;
  background: rgba(255, 255, 255, 0.1);
  color: white;
  font-size: 14px;
}

.setting-group select option {
  background: #2a5298;
  color: white;
}

.setting-group input[type="range"] {
  -webkit-appearance: none;
  height: 6px;
  background: rgba(255, 255, 255, 0.2);
  border-radius: 3px;
  outline: none;
}

.setting-group input[type="range"]::-webkit-slider-thumb {
  -webkit-appearance: none;
  width: 18px;
  height: 18px;
  background: #4CAF50;
  border-radius: 50%;
  cursor: pointer;
}

.setting-group span {
  font-size: 12px;
  opacity: 0.8;
}

.advanced-settings {
  margin-bottom: 20px;
}

.instrument-toggles {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 10px;
}

.toggle {
  display: flex;
  align-items: center;
  gap: 8px;
  cursor: pointer;
  padding: 8px;
  border-radius: 6px;
  background: rgba(255, 255, 255, 0.05);
  transition: background 0.3s ease;
}

.toggle:hover {
  background: rgba(255, 255, 255, 0.1);
}

.toggle input[type="checkbox"] {
  width: 16px;
  height: 16px;
  accent-color: #4CAF50;
}

.action-buttons {
  display: flex;
  gap: 15px;
  justify-content: center;
}

.action-btn {
  padding: 12px 20px;
  border: none;
  border-radius: 8px;
  background: rgba(255, 255, 255, 0.1);
  color: white;
  font-size: 14px;
  cursor: pointer;
  transition: all 0.3s ease;
}

.action-btn:hover {
  background: rgba(255, 255, 255, 0.2);
  transform: translateY(-2px);
}

.action-btn.active {
  background: #4CAF50;
}

/* Адаптивность */
@media (max-width: 600px) {
  .settings-grid {
    grid-template-columns: 1fr;
  }
  
  .instrument-toggles {
    grid-template-columns: 1fr;
  }
  
  .action-buttons {
    flex-direction: column;
  }
}
</style> 