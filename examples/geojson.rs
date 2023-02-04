use knuffel::Decode;

pub enum Geometry {
    Point {
        properties: HashMap<String, >,
        coordinate: Position
    },
    MultiPoint,
    LineString(#[knuffel(children)] Vec<Position>),
    MultiLineString,
    Polygon {
        properties: ,
        coordinates: 
    },
    MultiPolygon,
    GeometryCollection
}

#[derive(Debug, Decode)]
pub struct Feature {
    
}

#[derive(Decode)]
pub struct Position(
    #[knuffel(argument) f32,
    #[knuffel(argument) f32
);

pub struct BBox;

puh enum Pole {
    MinLat,
    MaxLat,
    WestLon,
    EastLon
}
