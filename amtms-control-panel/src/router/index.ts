import {createRouter} from "vue-router";
import { createWebHashHistory } from "vue-router"

const router = createRouter({
    history: createWebHashHistory(import.meta.env.BASE_URL),
    routes: [
        {
            path: "/",
            name: "index",
            component: () => import("../views/LoadDataPage.vue"),
        },
    ],
});

export default router;
