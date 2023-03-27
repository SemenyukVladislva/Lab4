use std::io;

#[derive(Copy, Clone)]
struct Point {
    x: f32,
    y: f32
}

#[derive(Copy, Clone)]
struct Line {
    p1: Point,
    p2: Point
}

fn read_next_geom_line() -> Line {
    let mut input = String::new();

    match io::stdin().read_line( &mut input ) {
        Ok(_) => {},
        Err(err) => println!( "{err}" )
    }
    
    let str_points: Vec<&str> = (&mut input).split_whitespace().collect();
    let str_coords1: Vec<&str> = str_points[0].split( ',' ).collect();
    let str_coords2: Vec<&str> = str_points[1].split( ',' ).collect();

    return Line {
        p1: Point {
            x: str_coords1[ 0 ].parse().unwrap(),
            y: str_coords1[ 1 ].parse().unwrap()
        },
        p2: Point {
            x: str_coords2[ 0 ].parse().unwrap(),
            y: str_coords2[ 1 ].parse().unwrap()
        }
    };
}

fn check_intersect( l1: Line, l2: Line ) -> bool {
    return ( l1.p1.x - l1.p2.x ) * ( l2.p1.y - l2.p2.y ) 
        - ( l1.p1.y - l1.p2.y ) * ( l2.p1.x - l2.p2.x ) != 0.0;
}

fn get_intersect_point( l1: Line, l2: Line ) -> Point {
    let a1 = l1.p1.y - l1.p2.y;
    let b1 = l1.p2.x - l1.p1.x;
    let c1 = l1.p1.x * l1.p2.y - l1.p2.x * l1.p1.y;
    let a2 = l2.p1.y - l2.p2.y;
    let b2 = l2.p2.x - l2.p1.x;
    let c2 = l2.p1.x * l2.p2.y - l2.p2.x * l2.p1.y;
    let det = a1 * b2 - a2 * b1;

    return Point {
        x: ( b1 * c2 - b2 * c1 ) / det,
        y: ( a2 * c1 - a1 * c2 ) / det
    }
}

fn length_points( p1: Point, p2: Point ) -> f32 {
    return ( ( p2.x - p1.x ) * ( p2.x - p1.x ) 
        + ( p2.y - p1.y ) * ( p2.y - p1.y ) ).sqrt();
}

fn lies_on_segment( line: Line, point: Point ) -> bool {
    let len1 = length_points( line.p1, point );
    let len2 = length_points( line.p2, point );
    let segment_length = length_points( line.p1, line.p2 );
    
    return ( len1 <= segment_length ) 
        && ( len2 <= segment_length ); 
}

fn lies_on_ray( ray: Line, point: Point ) -> bool {
    let len_between_ray_points = length_points( ray.p1, ray.p2 );
    let len2 = length_points( point, ray.p2 );
    let len3 = length_points( ray.p1, point );
    
    return !( ( len2 > len_between_ray_points ) 
        && ( len3 <= len_between_ray_points ) );
}

fn main() {
    let mut min_intersection_length = f32::MAX;
    let mut has_intersection: bool = false;
    let mut closest_segment = Line { 
        p1: Point { x: 0.0, y: 0.0 }, 
        p2: Point { x: 0.0, y: 0.0 }
    };
    
    // Считать луч
    let ray = read_next_geom_line();
    println!( "Луч начинается в точке({};{}), и направлен в точку ({};{}).", ray.p1.x, ray.p1.y, ray.p2.x, ray.p2.y );
    
    // В задании не уточнено, но здесь можно ввести только пять отрезков
    for num in 1..=5 {
        // Считать отрезок
        let segment = read_next_geom_line();
		println!( "Координаты отрезка {} - A = ({};{}), B = ({};{}).", num, segment.p1.x, segment.p1.y, segment.p2.x, segment.p2.y );
        
        // 1. Есть пересечение прямых
        if !check_intersect( ray, segment ) {
            println!( "[1]Отрезок и луч не пересекаются!" );
            continue;
        };
        
        // Получить точку пересечения прямых
        let intersect_point = get_intersect_point( ray, segment );
        
        // 2. Точка принадлежит отрезку
        if !lies_on_segment( segment, intersect_point ) {
            println!( "[2]Отрезок и луч не пересекаются!" );
            continue;
        };
        
        // 3. Точка принадлежит лучу
        if !lies_on_ray( ray, intersect_point ) {
            println!( "[3]Отрезок и луч не пересекаются!" );
            continue;
        };
    
        // 4. Сравнить удалённость найденной точки
        let length_ray_to_point = length_points( ray.p1, intersect_point );
        if length_ray_to_point < min_intersection_length {
            min_intersection_length = length_ray_to_point;
            closest_segment = segment;
            has_intersection = true;
        };
        
        if has_intersection {
            println!( "Координаты ближайшего отрезка: A = ({};{}), B = ({};{})", 
                closest_segment.p1.x, closest_segment.p1.y,
                closest_segment.p2.x, closest_segment.p2.y );
        } else {
            println!( "Ближайшего отрезка нет!" );
        }
    }
}
