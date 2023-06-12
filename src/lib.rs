use byteorder::{self, LittleEndian, ReadBytesExt};
use polygon::{Polygon, Header, Triangle};
use std::{fs, env, io, str};
use std::io::{prelude::*, Error};
use std::fs::File;

mod polygon;

pub fn execute_analysis(filename: &str) {

    let file_content = std::fs::read_to_string(&filename);

    match file_content {
        Ok(correct_content) => check_ascii_stl(correct_content),
        Err(_) => {
            let mut data_content = File::open(filename).expect("File cannot be found/opened");
            read_binary_stl(&mut data_content);
        }    
    }
}

//Simple chequeo para ver si el archivo es ASCII o _booleano
pub fn check_ascii_stl(content: String) {
    
    let mut counter: u32 = 0;

    match content.starts_with("solid") {
        true => {
            println!("This is an ASCII STL");
            match content.contains("facet normal") {
                true => {
                    counter = content.matches("facet normal").count() as u32;
                    println!("There are: {} TRIANGLES, {} QUADS", counter, &counter/2);
                },
                false => {},
            }
        },
        false => {}
    }
}

//Al leer el booleano y ver que no es un ascii se activa esto 
pub fn read_binary_stl(file:&mut File) {
    println!("This is a binary STL");

    //Manda una comprobación del archivo (ya no manda como originalmente una referencia de &String)
    let header = check_binary_stl(file);
    
    //En caso de que salga un error, le damos un mensaje predefinido y con el 
    //expect podemos coger el Result sin el Error, pudiendo interaccionar mucho más facilmente
    let polygon = create_triangle_list(file, header).expect("Error creating polygon");
    println!("There are: {} TRIANGLES, {} QUADS", polygon.count_triangles(), polygon.count_quads());
}

//Genérico que usa cualquier struct que tenga el trait ReadBytesExt
//Devuelve un polígono usando el header antes creado y el ahora creado vector de triángulos
pub fn create_triangle_list<T: ReadBytesExt> (input: &mut T, header: Header) -> Result<Polygon, Error> {
    let mut triangles = Vec::new();

    //Recorre todos los triángulos individualmente primero, leyendo los vértices de cada triángulo y luego los triángulos creados con estos
    for _ in 0..header.num_triangles {
        triangles.push(read_triangle(input)?);
    }

    Ok(Polygon::new(header, triangles))
}

pub fn check_binary_stl(file: &mut File) -> Header {    
    //leemos desde el byte 80, 4 bytes;
    //se puede usar además de [4;80] -> [08u; 80];
    let mut header = [4; 80];

    //ABRIMOS EL ARCHIVO
    //LO LEEMOS USANDO PARA REFERENCIA DE QUÉ BYTES COGER
    file.read(&mut header).expect("Error reading file");

    //Ahora Buffer se ha transformado en los datos que hemos cortado del archivo    
    //leemos y el file se convierte en la parte seccionada
    //convertimos la lectura de ese archivo a U32 con un crate externo usando LittleEndian
    let num_triangles = file.read_u32::<LittleEndian>().expect("Error reading number");

    //Mini comprobación de tipo de archivo por curiosidad
    println!("{:?}", file);
    //LO LEE

    //Ya no es necesario que sea tan redundante y así no confunde a nadie
    /*println!("Hay: {} triángulos en esta figura, que serían {} quads", num_triangulos, (&num_triangulos/2));*/

    //Devolvemos el header y el número de triángulos dentro de un struct
    Header::new(header, num_triangles)
}

//Para devolver este sistema, hace falta indicar tanto el resultado, como el posible error
fn read_point <T: ReadBytesExt> (input: &mut T) -> Result <[f32; 3], Error> {
    //La ? del final indica que algo puede devolver un error
    let p1 = input.read_f32::<LittleEndian>()?;
    let p2 = input.read_f32::<LittleEndian>()?;
    let p3 = input.read_f32::<LittleEndian>()?;

    //En caso de que todo salga bien, que mande ese pequeño array de vuelta
    Ok([p1, p2, p3])
}

//Mismo proceso para sacar los triángulos y sus variables asociadas
fn read_triangle <T: ReadBytesExt> (input: &mut T) -> Result <Triangle, Error> {
    let normal = read_point(input)?;
    let v1 = read_point(input)?;
    let v2 = read_point(input)?;
    let v3 = read_point(input)?;
    let attribute = input.read_u16::<LittleEndian>()?;

    //En caso de que todo salga bien, que mande un objeto Triangle de vuelta
    Ok(Triangle::new(normal, v1, v2, v3, attribute))
}