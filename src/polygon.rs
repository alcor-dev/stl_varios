#[derive(Debug, Clone, PartialEq)]
pub struct Triangle {
    normal: [f32; 3],
    v1: [f32; 3],
    v2: [f32; 3],
    v3: [f32; 3],
    attribute: u16,
}

//Implementamos un NEW para que resulte mucho más sencillo crear
//nuevos triánglos
impl Triangle {
    pub fn new(normal: [f32; 3], v1: [f32; 3], v2: [f32; 3], v3: [f32; 3], attribute: u16) -> Self {
        Self {
            normal,
            v1,
            v2,
            v3,
            attribute,
        }
    }
}

//Creamos un header con los datos del header y el número de triángulos
#[derive(Debug, Clone, PartialEq)]
pub struct Header {
    pub header: [u8; 80],
    pub num_triangles: u32,
}

impl Header {
    pub fn new(header: [u8; 80], num_triangles: u32) -> Self {
        Self {
            header,
            num_triangles,
        }
    }
}

//Creamos un struct con otros structs sirviendo de base
#[derive(Debug, Clone, PartialEq)]
pub struct Polygon {
    pub header: Header,
    pub triangles: Vec::<Triangle>,
}

impl Polygon {
    pub fn new(header: Header, triangles: Vec<Triangle>) -> Self {
        Self {
            header,
            triangles,
        }
    }

    pub fn count_triangles(&self) -> usize {
        self.triangles.len()
    }

    pub fn count_quads(&self) -> usize {
        self.triangles.len()/2
    }
}