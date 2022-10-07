import { createRouter, createWebHistory } from "vue-router";

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    { path: '/', component: () => import('../views/Dashboard.vue') },
    { path: '/events', component: () => import('../views/Events.vue') }
  ],
});

export default router;
