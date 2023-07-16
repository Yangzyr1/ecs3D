import request from "../utils/request.ts";
export default class DataLoader {
    static async postGenerateGTP():Promise<boolean> {
        return await request({
            url: "/gtp/generate",
            method: "POST",
        })
    }
    static async postTerrainFileList(list: string[]):Promise<boolean> {
        return await request({
            url: "/gtp/data/terrain",
            method: "POST",
            data: list
        })
    }
}