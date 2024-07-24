import { createRouter, createWebHistory } from "vue-router";

export const router = createRouter({
  history: createWebHistory(),
  routes: [
    {
      path: "/import",
      name: "import",
      component: () => import("@/views/ImportView.vue"),
    },
    {
      path: "/data",
      name: "data",
      component: () => import("@/views/DataView.vue"),
    },
    {
      path: "/about",
      name: "about",
      component: () => import("@/views/AboutView.vue"),
    },
    {
      path: "/",
      redirect: (_to) => {
        const userId = localStorage.getItem("userId");
        if (userId) {
          return "/data";
        } else {
          return "/import";
        }
      },
    },
  ],
});
