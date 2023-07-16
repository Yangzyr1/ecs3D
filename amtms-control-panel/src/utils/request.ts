import axios from "axios";
// 基础地址
const BASE_URL = import.meta.env.VITE_BASE_URL;

// 默认配置
const instance = axios.create({
    baseURL: BASE_URL,
    timeout: 10000,
    withCredentials: true,
    headers: {
        post: {
            "Content-Type": "application/json"
        }
    }
})

// 请求拦截器
instance.interceptors.request.use(config => {
    console.log("request " + JSON.stringify({
        url: config.url,
        data: config.data
    }, null, 4))

    return config
})

// 回复拦截器
instance.interceptors.response.use(config => {
    // 获取返回值
    const res = config.data

    console.log("response " + JSON.stringify(res, null, 4))

    // 如果访问出现错误就打印错误信息
    if (res.code === 500) {
        console.error(res.message)
    }

    // 让用户不需要调用.data，直接返回R
    return res
}, error => {
    return Promise.resolve(error.response);
})

export default instance
