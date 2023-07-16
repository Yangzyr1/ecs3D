pub mod fill_hole;
pub mod algorithm;
pub mod delaunay;

use std::cmp::Ordering;
use std::cmp::Ordering::{Equal, Greater, Less};
use std::collections::{BTreeSet, HashMap, LinkedList};
use std::f32::consts::PI;
use std::ops::Add;
use std::sync::{Arc, Mutex, RwLock};
use bevy::log::{debug, info};
use bevy::math::Vec3;

/// 顶点查找单元
#[derive(Default)]
pub struct PointFindUnit32{
    // 以该点为起点的线
    pub start: BTreeSet<RadiusLine32>,
    // 以该点为终点的线
    pub end: BTreeSet<RadiusLine32>
}
/// 顶点查找集
#[derive(Default, Clone)]
pub struct PointFindSet32{
    // 实际顶点位置数据
    pub origin: Vec<[f32;3]>,
    // 顶点数据索引的映射
    pub mapping: Vec<usize>,
    // 顶点数据的始末线段索引
    pub route: Arc<RwLock<Vec<PointFindUnit32>>>,
}
#[derive(Default, PartialOrd, Clone)]
pub struct RadiusLine32{
    // 分别为起点与终点
    pub origin: [u32;2],
    pub radius: f32,
}

impl Eq for RadiusLine32 {}

impl PartialEq<Self> for RadiusLine32 {
    fn eq(&self, other: &Self) -> bool {
        todo!()
    }
}

impl Ord for RadiusLine32{
    fn cmp(&self, other: &Self) -> Ordering {
        let a = (self.radius*10.0).round();// as i32;
        let b = (other.radius*10.0).round(); // as i32;
        // [a].iter().cmp([b].iter())
        return if a < b {
            Greater
        } else if a > b {
            Less
        } else {
            Equal
        }
    }
}
/// 线段查找集
#[derive(Default)]
pub struct LineFindSet32 {
    // 可用作起始的点
    pub data: Vec<RadiusLine32>,
}

/// 环路条带
#[derive(Default)]
pub struct RingLineStrip32(pub Vec<u32>);
pub fn prepare_point_find_set_32(points: Vec<[f32;3]>, threshold: u32) -> PointFindSet32 {
    let mut temp_hashmap :HashMap<String, usize> = HashMap::new();
    let mut find_set = PointFindSet32::default();
    find_set.origin = points.clone();
    for index in 0..points.len() {
        let point = points[index];
        let mut point_i32: Vec<i32> = Vec::new();
        for index in point {
            point_i32.push((index * threshold as f32).round() as i32);
        }
        let point_hash = point_i32[0].to_string().add(point_i32[1].to_string().as_str()).add(point_i32[2].to_string().as_str());
        if !temp_hashmap.contains_key(point_hash.as_str()) {
            temp_hashmap.insert(point_hash.clone(), temp_hashmap.len());
            find_set.route.write().unwrap().push(PointFindUnit32::default());
        }
        find_set.mapping.push(*temp_hashmap.get(point_hash.as_str()).unwrap());
    }
    return find_set;
}
pub fn prepare_line_find_set_32(lines: Vec<u32>, points: &PointFindSet32, basis: Vec3) -> LineFindSet32{
    let mut line_find_set_32 = LineFindSet32::default();
    for index in (0..lines.len()).step_by(2) {
        let point_a = points.origin[lines[index] as usize];
        let point_b = points.origin[lines[index+1] as usize];
        let vector = [point_b[0]-point_a[0], point_b[1]-point_a[1], point_b[2]-point_a[2]];
        // 反正切函数中的x与y
        // 目前是XY平面
        let x = (vector[0].powi(2) + vector[1].powi(2)).sqrt();
        let y = vector[2];
        // 以度数为单位,范围是(-90,90]
        let mut radius = y.atan2(x) * 180.0 / PI;
        // 对于第二第三象限的向量,需要调整角度为大于90或小于-90度的情况
        if basis.dot(Vec3::from(vector)) < 0.0  {
            if radius < 0.0 {
                radius = -radius - 180.0;
            }
            else {
                radius = 180.0 - radius;
            }
        }
        // 修改范围至0,360
        if radius < 0.0 {
            radius = radius + 360.0;
        }
        let radius_line = RadiusLine32{
            origin: [lines[index], lines[index+1]],
            radius: radius
        };
        // 加入线查找集
        if radius_line.radius < 90.0 && radius_line.radius > 0.0 {
            line_find_set_32.data.push(radius_line.clone());
        }
        // 插入点查找集里的路由树
        info!("{:?}线{:?}点索引是:{:?}和{:?},点实体为{:?},{:?}",
            index,
            lines[index],
            points.mapping[lines[index] as usize],
            points.mapping[lines[index+1] as usize],
            point_a,
            point_b
        );
        points.route.write().unwrap()[points.mapping[lines[index] as usize]].start.insert(radius_line.clone());
        points.route.write().unwrap()[points.mapping[lines[index + 1] as usize]].end.insert(radius_line);
    }
    return line_find_set_32;
}
// 查找环
pub fn search_ring_32(mut lines: LineFindSet32, points: &PointFindSet32 ) -> Vec<RingLineStrip32> {
    // let mut point_clone = points.clone();
    let mut result: Vec<RingLineStrip32> = Vec::new();
    for line in lines.data {
        // info!("点数据是:{:?},{:?},{:?}", line.origin,points.origin[line.origin[0] as usize],points.origin[line.origin[1] as usize]);
        let p = points;
        // let start_index = p.mapping[line.origin[0] as usize];
        // let end_index = p.mapping[line.origin[1] as usize];
        let start_index = line.origin[0] as usize;
        let end_index = line.origin[1] as usize;
        // 以初始线的终点为起点开始搜索，结束条件为初始线的起点
        let o_line_result = points.dfs(end_index, start_index, line.radius, 15);
        if let Some(mut line_result) = o_line_result {
            line_result.0.push(line.origin[0]);
            line_result.0.reverse();
            result.push(line_result);
        }
    }
    return result;
}
/// 将边条带转化为三角形
fn strip_to_triangle(strip: Vec<u32>) -> Vec<u32> {
    debug!("开始条带转换");
    info!("三角形条带数据{:?}", strip);
    let mut triangle_strip = Vec::new();
    // 构建三角形条带的方式构建三角形集
    for i in 1..(strip.len()/2) {
        // 点相邻时的情况
        if strip.len() - 2*i == 1 {
            triangle_strip.push(strip[i]);
            triangle_strip.push(strip[strip.len() - i]);
            triangle_strip.push(strip[strip.len() - i + 1]);
            break;
        }

        // // 点相邻时的情况
        // if strip.len()-1 - 2*i < 1 {
        //     triangle_strip.push(strip[i]);
        //     triangle_strip.push(strip[strip.len() - i + 1]);
        //     triangle_strip.push(strip[strip.len() - i]);
        //     break;
        // }
        // info!("是{:?},{:?}", strip[i - 1],strip[strip.len() - i]);
        // 中线左侧的三角形
        triangle_strip.push(strip[i - 1]);
        triangle_strip.push(strip[i]);
        triangle_strip.push(strip[strip.len() - i]);
        // 中线右侧的三角形
        triangle_strip.push(strip[i]);
        triangle_strip.push(strip[i + 1]);
        triangle_strip.push(strip[strip.len() - i]);
    }
    triangle_strip
}

impl PointFindSet32 {
    /// 深度优先搜索出环，结果集是顺时针的条带，也就是需要经过一次reverse转换成逆时针顺序
    fn dfs(&self,
           index: usize,
           target: usize,
           current_radius: f32,
           tts: u32) -> Option<RingLineStrip32> {

        // 按照顺序遍历以某个点作为起点的线段
        let route = self.route.read().unwrap();
        let route_clone = route[self.mapping[index]].start.iter().clone();
        let mut iter_num = 0;
        for line in route_clone {
            if current_radius + 180.0 > line.radius {
                break;
            }
            iter_num = iter_num + 1;
        }
        let iter = route[self.mapping[index]].start.iter();
        // info!("当前索引是:{:?},目标索引是{:?},迭代器长度为{:?},跳过长度为{:?}", index,target,iter.len(), iter_num);
        if iter_num == iter.len() {
            iter_num = 0;
        }

        for line in iter.skip(iter_num) {
            // info!("哒哒哒:{:?},{:?},{:?},{:?}",line.origin, line.radius,self.mapping[line.origin[0] as usize],self.mapping[line.origin[1] as usize]);
            // 已经达到搜索阈值的情况
            if tts <= 0 {
                // info!("达到搜索阈值");
                break;
            }
            // 已经遇到终点的情况
            if self.mapping[line.origin[1] as usize] == self.mapping[target] {
                let mut ring_line_strip = RingLineStrip32::default();
                ring_line_strip.0.push(line.origin[1]);
                ring_line_strip.0.push(line.origin[0]);
                info!("闭环完成");
                return Some(ring_line_strip);
            }
            // 没遇到终点，往深处搜索
            let result_o = self.dfs(line.origin[1] as usize, target, line.radius, tts - 1);
            if let Some(mut result) = result_o {
                result.0.push(line.origin[0]);
                return Some(result);
            }
        }
        // info!("遇到死节点");
        return None;
    }
}
pub fn regenerate_clip_triangle(points: Vec<[f32;3]>, lines: Vec<u32>, threshold: u32, basis: Vec3) -> Vec<u32> {
    let mut point_set = prepare_point_find_set_32(points, threshold);
    let line_set = prepare_line_find_set_32(lines, &point_set, basis);
    let ring_set = search_ring_32(line_set, &point_set);
    let mut result = LinkedList::new();
    for ring in ring_set {
        result.extend(strip_to_triangle(ring.0));
    }
    return result.into_iter().collect();
}