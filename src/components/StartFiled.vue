<template>
  <div class="canvas-box">
    <canvas ref="canvasRef"></canvas>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue';
import {Starts} from "@/types";

const canvasRef = ref<HTMLCanvasElement | null>(null);
let animationFrameId: number | null = null;
let width = 0;
let height = 0;
let stars: Array<Starts> = [];
let c: CanvasRenderingContext2D | null = null;

const initializeCanvas = () => {
  if (!canvasRef.value) return;
  width = window.innerWidth;
  height = window.innerHeight;
  canvasRef.value.width = width;
  canvasRef.value.height = height;
  c = canvasRef.value.getContext('2d');
};

const initializeStars = (numStars: number = 1000) => {
  for (let i = 0; i < numStars; i++) {
    stars.push({
      x: Math.random() * width,
      y: Math.random() * height,
      z: Math.random() * width,
      o: `0.${Math.floor(Math.random() * 99) + 1}`,
    });
  }
};

const moveStars = () => {
  stars.forEach(star => {
    star.z--;
    if (star.z <= 0) {
      star.z = width;
    }
  });
};

const drawStars = () => {
  if (!c) return;
  c.fillStyle = "rgba(0,10,20,1)";
  c.fillRect(0, 0, width, height);
  stars.forEach(star => {
    const pixelX = (star.x - width / 2) * (width / star.z) + width / 2;
    const pixelY = (star.y - height / 2) * (height / star.z) + height / 2;
    const pixelRadius = (width / star.z) * 0.3;
    if (c) {
      c.beginPath();
      c.arc(pixelX, pixelY, pixelRadius, 0, 2 * Math.PI);
      c.fillStyle = `rgba(209, 255, 255, ${star.o})`;
      c.fill();
    }
  });
};

const executeFrame = () => {
  if (animationFrameId) cancelAnimationFrame(animationFrameId);
  moveStars();
  drawStars();
  animationFrameId = requestAnimationFrame(executeFrame);
};

onMounted(() => {
  initializeCanvas();
  initializeStars();
  executeFrame();
  window.addEventListener('resize', () => {
    initializeCanvas();
    initializeStars();
  });

});

onUnmounted(() => {
  if (animationFrameId) cancelAnimationFrame(animationFrameId);
  window.removeEventListener('resize', () => {
    initializeCanvas();
    initializeStars();
  });
});
</script>

<style scoped>
</style>
