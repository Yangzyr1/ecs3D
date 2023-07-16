import {defineStore} from "pinia";
interface BevyBasicConfig{
    bindAddr: string
}
export const useBevyHttpStore = defineStore('BevyHttp', {
    state: (): BevyBasicConfig => {
        return {
            bindAddr: "127.0.0.1:7443"
        }
    },
    getters: {

    },
    actions: {

    }
})