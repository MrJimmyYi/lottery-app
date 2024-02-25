import { createApp } from "vue";
import App from "./App.vue";
import '@/assets/css/main.css';
import router from "@/router";
import {listen} from "@tauri-apps/api/event";

const app = createApp(App);

// 使用路由
app.use(router);

// 示例: 监听后端 Tauri 事件（确保事件名称和负载类型正确）
listen('navigate', (event) => {
    if (typeof event.payload === 'string') {
        // 使用路由导航到不同页面
        console.log(event.payload)
        router.push({ path: event.payload });
    }
}).catch((error) => console.error('Error listening to navigate event', error));

// 挂载 Vue 应用
app.mount('#app');
