from scipy.spatial import Delaunay
import pandas as pd
import glob

from typing import Dict, List, Tuple


class DrillPoint:
    def __init__(self, point_id: int,
                 drill_id: int,
                 terrain_id: int,
                 x: float,
                 y: float,
                 z: float,
                 red: float,
                 green: float,
                 blue: float):
        self.point_id = point_id
        self.drill = drill_id
        self.terrain = terrain_id
        self.x = x
        self.y = y
        self.z = z
        self.red = red
        self.green = green
        self.blue = blue


class GTP:
    def __init__(self, points, stone_type):
        self.points = points
        self.stone_type = stone_type


def transform_hex_to_rgb(hex_color: str) -> Tuple[float, float, float]:
    # 将十六进制的 RGB 值转换为十进制的 RGB 值
    dec_color = int(hex_color, 16)

    # 计算 0-1 之间的 RGB 值
    r = (dec_color >> 16) / 255.0
    g = ((dec_color >> 8) & 0xFF) / 255.0
    b = (dec_color & 0xFF) / 255.0
    return r, g, b


def get_drill_bundle(csv_path: str) -> Tuple[List[DrillPoint], Dict[int, List[DrillPoint]], List[str]]:
    drill = {}
    terrain = []
    drill_list = []
    for filename in glob.glob(csv_path + '/*.csv'):
        terrain.append(filename)
        data = pd.read_csv(filename, dtype={'id': int, 'color': str}).to_numpy().tolist()
        for item in data:
            r, g, b = transform_hex_to_rgb(item[4])
            point = DrillPoint(len(drill_list), item[0], terrain.index(filename), item[1], item[2], item[3], r, g, b)
            drill_list.append(point)
            if drill.get(point.drill) is None:
                drill[point.drill] = [point]
            else:
                drill[point.drill].append(point)
    return drill_list, drill, terrain


def load_drill_point(csv_path: str) -> List[DrillPoint]:
    terrain = []
    drill_list = []
    for filename in glob.glob(csv_path + '/*.csv'):
        terrain.append(filename)
        data = pd.read_csv(filename, dtype={'id': int, 'color': str}).to_numpy().tolist()
        for item in data:
            r, g, b = transform_hex_to_rgb(item[4])
            point = DrillPoint(len(drill_list), item[0], terrain.index(filename), item[1], item[2], item[3], r, g, b)
            drill_list.append(point)
    return drill_list


def generate_surface_tin(points: List[DrillPoint]) -> List[List[int]]:
    tmp = list(filter(lambda point: point.terrain == 0, points))
    tmp.sort(key=lambda item: item.drill)
    surface = list(map(lambda x: [x.x, x.y], tmp))
    # 从0开始
    return Delaunay(surface).simplices.tolist()


def generate_gtp(csv_path: str) -> List[GTP]:
    drill_list, drill, terrain = get_drill_bundle(csv_path)
    tin = generate_surface_tin(drill_list)
    gtp = []
    for drills in tin:
        drill1 = drill[drills[0]]
        drill2 = drill[drills[1]]
        drill3 = drill[drills[2]]
        for i in range(0, min(len(drill1), len(drill2), len(drill3)) - 1):
            points = [drill1[i], drill2[i], drill3[i], drill1[i + 1], drill2[i + 1], drill3[i + 1]]
            gtp.append(GTP(points, str(points[3].terrain)))
    return gtp


if __name__ == '__main__':
    print(generate_gtp("D:/WorkSpace/Project/FormalProject/Drill3D/data/5k")[86599])

#%%
