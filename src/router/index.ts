// src/router/index.ts
import { createRouter, createWebHistory, RouteRecordRaw } from 'vue-router';
import Wedding from "@/views/Wedding.vue"
import Manage from '@/views/Manage.vue'; // 确保路径正确
import Winners from '@/views/Winners.vue'; // 确保路径正确
import ManageUser from '@/views/ManageUser.vue';
import ManagePrize from '@/views/ManagePrize.vue';
import ManageBasic from '@/views/ManageBasic.vue';


const routes: Array<RouteRecordRaw> = [
    {
        path: '/pageWedding',
        name: 'Wedding',
        component: Wedding,
    },
    {
        path: '/pageManage',
        name: 'Manage',
        component: Manage,
        children: [
            {
                path: 'user',
                component: ManageUser
            },
            {
                path: 'prize',
                component: ManagePrize
            },
            {
                path: 'basic',
                component: ManageBasic
            }
        ]
    },
    {
        path: '/pageWinners',
        name: 'Winners',
        component: Winners,
    },
    {
        path: '/',
        redirect: '/pageWedding',
    },
];

const router = createRouter({
    history: createWebHistory(),
    routes,
});

export default router;
