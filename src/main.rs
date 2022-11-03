use std::fs;
use kmeans::{Contained, sequential, parallel};

fn read_points_csv<T: Contained>(path: &str, mut container: Vec<T>) -> Vec<T> {
    let contents = fs::read_to_string(path)
        .expect(&format!("A file to exist in the given path: {}", path));

    // parsing CSV line by line,
    // skipping the first line since it contains the header metadata.
    for line in contents.lines().skip(1) {
        let comps: Vec<&str> = line.split(",").collect();

        // skip rows that doesn't have exactly 2 entries
        if comps.len() != 2 {
            continue;
        }

        // if either of x or y fails to be parsed to f64 that line should also be skipped
        let x = match comps.get(0).unwrap().parse::<f64>() {
            Ok(num) => num,
            Err(_) => continue
        };

        let y = match comps.get(1).unwrap().parse::<f64>() {
            Ok(num) => num,
            Err(_) => continue
        };

        container.push(T::new(x, y));
    }

    container
}

fn main() {
    let mut vs = Vec::new();

    vs = read_points_csv("./xclara.csv", vs);

    parallel::kmeans(vs, 3);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn it_can_perform_a_kmeans_iteration() {
        let points = [
            sequential::Point::new(2.0, 10.0),
            sequential::Point::new(2.0, 5.0),
            sequential::Point::new(8.0, 4.0),
            sequential::Point::new(5.0, 8.0),
            sequential::Point::new(7.0, 5.0),
            sequential::Point::new(6.0, 4.0),
            sequential::Point::new(1.0, 2.0),
            sequential::Point::new(4.0, 9.0),
        ];
        
        for point in &points {
            println!("{:?}", point);
        }

        let point_vec = Vec::from(points);

        sequential::kmeans(point_vec, 3);
    }

    #[test]
    pub fn it_does_stuff_in_parallel() {
    }
}
