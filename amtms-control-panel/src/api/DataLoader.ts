import request from "../utils/request.ts";
export default class DataLoader {
    static async postTerrainFileList(list: string[]):Promise<boolean> {
        return await request({
            url: "/data/terrain",
            method: "POST",
            data: list
        })
    }
}