<!--<template>-->
<!--  <canvas ref="canvasRef" class="star-sky"></canvas>-->
<!--</template>-->

<!--<script setup lang="ts">-->
<!--import { ref, onMounted, onUnmounted } from 'vue';-->

<!--const canvasRef = ref<HTMLCanvasElement | null>(null);-->

<!--class Star {-->
<!--  orbitRadius: number;-->
<!--  radius: number;-->
<!--  orbitX: number;-->
<!--  orbitY: number;-->
<!--  timePassed: number;-->
<!--  speed: number;-->
<!--  alpha: number;-->
<!--  ctx: CanvasRenderingContext2D;-->
<!--  canvas2: HTMLCanvasElement;-->

<!--  constructor(ctx: CanvasRenderingContext2D, canvas2: HTMLCanvasElement, w: number, h: number, hue: number, count: number, maxStars: number) {-->
<!--    this.orbitRadius = random(maxOrbit(w, h));-->
<!--    this.radius = random(60, this.orbitRadius) / 8;-->
<!--    this.orbitX = w / 2;-->
<!--    this.orbitY = h / 2;-->
<!--    this.timePassed = random(0, maxStars);-->
<!--    this.speed = random(this.orbitRadius) / 50000;-->
<!--    this.alpha = random(2, 10) / 10;-->
<!--    this.ctx = ctx;-->
<!--    this.canvas2 = canvas2;-->
<!--  }-->

<!--  draw() {-->
<!--    const x = Math.sin(this.timePassed) * this.orbitRadius + this.orbitX;-->
<!--    const y = Math.cos(this.timePassed) * this.orbitRadius + this.orbitY;-->
<!--    const twinkle = random(10);-->

<!--    if (twinkle === 1 && this.alpha > 0) {-->
<!--      this.alpha -= 0.05;-->
<!--    } else if (twinkle === 2 && this.alpha < 1) {-->
<!--      this.alpha += 0.05;-->
<!--    }-->

<!--    this.ctx.globalAlpha = this.alpha;-->
<!--    this.ctx.drawImage(this.canvas2, x - this.radius / 2, y - this.radius / 2, this.radius, this.radius);-->
<!--    this.timePassed += this.speed;-->
<!--  }-->
<!--}-->

<!--function random(min: number, max?: number): number {-->
<!--  if (max === undefined) {-->
<!--    max = min;-->
<!--    min = 0;-->
<!--  }-->

<!--  if (min > max) {-->
<!--    const hold = max;-->
<!--    max = min;-->
<!--    min = hold;-->
<!--  }-->

<!--  return Math.floor(Math.random() * (max - min + 1)) + min;-->
<!--}-->

<!--function maxOrbit(x: number, y: number): number {-->
<!--  const max = Math.max(x, y),-->
<!--      diameter = Math.round(Math.sqrt(max * max + max * max));-->
<!--  return diameter / 2;-->
<!--}-->

<!--function initStarSky(canvas: HTMLCanvasElement) {-->
<!--  if (!canvas) return;-->
<!--  const ctx = canvas.getContext('2d');-->
<!--  if (!ctx) return;-->

<!--  const w = canvas.width = window.innerWidth;-->
<!--  const h = canvas.height = window.innerHeight;-->

<!--  const hue = 217;-->
<!--  let stars: Star[] = [];-->
<!--  const maxStars = 1000; // 星星数量-->

<!--  const canvas2 = document.createElement('canvas');-->
<!--  const ctx2 = canvas2.getContext('2d');-->
<!--  if (!ctx2) return;-->
<!--  canvas2.width = 100;-->
<!--  canvas2.height = 100;-->
<!--  const half = canvas2.width / 2;-->
<!--  const gradient2 = ctx2.createRadialGradient(half, half, 0, half, half, half);-->
<!--  gradient2.addColorStop(0.025, '#CCC');-->
<!--  gradient2.addColorStop(0.1, `hsl(${hue}, 61%, 33%)`);-->
<!--  gradient2.addColorStop(0.25, `hsl(${hue}, 64%, 6%)`);-->
<!--  gradient2.addColorStop(1, 'transparent');-->

<!--  ctx2.fillStyle = gradient2;-->
<!--  ctx2.beginPath();-->
<!--  ctx2.arc(half, half, half, 0, Math.PI * 2);-->
<!--  ctx2.fill();-->

<!--  for (let i = 0; i < maxStars; i++) {-->
<!--    stars.push(new Star(ctx, canvas2, w, h, hue, i, maxStars));-->
<!--  }-->

<!--  const animate = () => {-->
<!--    ctx.globalCompositeOperation = 'source-over';-->
<!--    ctx.globalAlpha = 0.5; // 尾巴-->
<!--    ctx.fillStyle = `hsla(${hue}, 64%, 6%, 2)`;-->
<!--    ctx.fillRect(0, 0, w, h);-->

<!--    ctx.globalCompositeOperation = 'lighter';-->
<!--    stars.forEach(star => star.draw());-->

<!--    requestAnimationFrame(animate);-->
<!--  };-->

<!--  animate();-->
<!--}-->

<!--onMounted(() => {-->
<!--  if (canvasRef.value) {-->
<!--    initStarSky(canvasRef.value);-->
<!--  }-->
<!--});-->

<!--onUnmounted(() => {-->
<!--  // 注意: 这里我们不需要取消动画帧，因为组件卸载时，动画自然会停止。-->
<!--});-->
<!--</script>-->

<!--<style scoped>-->
<!--.star-sky {-->
<!--  position: fixed;-->
<!--  top: 0;-->
<!--  left: 0;-->
<!--  width: 100%;-->
<!--  height: 100%;-->
<!--  z-index: -1;-->
<!--}-->
<!--</style>-->
