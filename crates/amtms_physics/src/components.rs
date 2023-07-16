use bevy::ecs::component::Component;
enum Coordinate{
    //x,y,z
    CARTESIAN(f64,f64,f64),
    //r,theta
    POLAR(f64,f64),
    //r,theta,z
    CYLINDRICAL(f64,f64,f64),
    //r,theta,fi
    SPHERICAL(f64,f64,f64)
}
#[derive(Component)]
pub struct PhysicalPosition(Coordinate);
