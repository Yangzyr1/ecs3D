use amtms_physics::components::*;
#[derive(Clone, Debug)]
pub struct ColorDrillPoint{
    x: f64,
    y: f64,
    z: f64,
    color: String
}
struct ColorDrill(String, Vec<ColorDrillPoint>);
struct TINUnit{
    p1: u64,
    p2: u64,
    p3: u64,
}
struct TIN{
    points: Vec<PhysicalPosition>

}