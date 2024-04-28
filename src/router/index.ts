// src/router/index.ts
import { createRouter, createWebHistory, RouteRecordRaw } from 'vue-router';
import Wedding from "@/views/model/Wedding.vue"
import Manage from '@/views/manage/Manage.vue'; // 确保路径正确
import Winners from '@/views/winner/Index.vue'; // 确保路径正确
import ManageUser from '@/views/manage/user/Index.vue';
import ManagePrize from '@/views/manage/prize/Index.vue';
import ManageWedding from "@/views/manage/wedding/Index.vue";


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
                path: 'wedding',
                component: ManageWedding
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
