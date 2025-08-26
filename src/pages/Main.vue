<template>
  <div class="min-h-screen bg-slate-50">
    <main class="container mx-auto px-4 py-6">
      <div class="max-w-4xl mx-auto space-y-6">
        <!-- Welcome Section -->
        <Card class="text-center">
          <div class="space-y-4">
            <div class="flex justify-center">
              <img src="/logo.svg" alt="SwitchShuttle" class="w-16 h-16" />
            </div>
            <div>
              <h1 class="text-2xl font-bold text-slate-900 mb-2">
                Welcome to SwitchShuttle
              </h1>
              <p class="text-slate-600 max-w-md mx-auto">
                Manage terminal configurations and switch between them with ease
              </p>
            </div>
          </div>
        </Card>
      </div>
    </main>
  </div>
</template>

<script setup lang="ts">
import Card from '../components/ui/Card.vue';
</script>
