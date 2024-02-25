// src/router/index.ts
import { createRouter, createWebHistory, RouteRecordRaw } from 'vue-router';
import Weeding from "@/views/Weeding.vue"
import Manage from '@/views/Manage.vue'; // 确保路径正确
import LuckUsers from '@/views/LuckUsers.vue'; // 确保路径正确

const routes: Array<RouteRecordRaw> = [
    {
        path: '/pageWeeding',
        name: 'Weeding',
        component: Weeding,
    },
    {
        path: '/pageManage',
        name: 'Manage',
        component: Manage,
    },
    {
        path: '/pageLuckUsers',
        name: 'LuckUsers',
        component: LuckUsers,
    },
    {
        path: '/',
        redirect: '/pageHome',
    },
];

const router = createRouter({
    history: createWebHistory(),
    routes,
});

export default router;
