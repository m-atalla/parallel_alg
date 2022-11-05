use std::sync::mpsc::{Sender, Receiver, channel};
use std::fmt::Debug;
use rand::seq::SliceRandom;
use std::sync::Arc;
use crate::{Constructed, print_clusters};
use crate::threads::ThreadPool;

#[derive(Debug, PartialEq, Clone)]
pub struct Point {
    x: f64,
    y: f64,
    cluster: Option<Arc<Cluster>> 
}

impl Constructed for Point {
    fn new(x: f64, y: f64) -> Self {
        Point { 
            x,
            y,
            cluster: None
        }
    }
}
impl Point {
    pub fn calc_euclid_dist(&self, other: &Point) -> f64 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
    }
}

#[derive(PartialEq, Clone)]
pub struct Cluster {
    idx: usize,
    centroid: Point,
}

impl Debug for Cluster {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"Centroid #{}: ({:6.2},{:6.2} )", self.idx,self.centroid.x, self.centroid.y)
    }
}

impl Cluster {
    pub fn new(idx: usize, centroid: Point) -> Cluster {
        Cluster {
            idx,
            centroid,
        }
    }
}

pub fn update_points_clusters(mut points: Vec<Point>, clusters: Vec<Arc<Cluster>>) -> Vec<Point>{
    for point in &mut points {
        let mut distances = Vec::new();
        // Distance calc for current centroids
        for cluster in &clusters {
            distances.push((
                cluster,
                point.calc_euclid_dist(&cluster.centroid)
            ));
        }

        let min_distance = distances.iter()
            .min_by(|(_,d1), (_,d2)| d1.total_cmp(d2)) 
            .expect("Distances list to have a minimum");

        point.cluster = Some(Arc::clone(&min_distance.0));
    }

    points
}

pub fn parallel_iteration(
        mut points: Vec<Point>,
        clusters: Vec<Arc<Cluster>>,
        (tx, rx): &(Sender<Vec<Point>>, Receiver<Vec<Point>>),
        pool: &ThreadPool
    ) -> (Vec<Point>, Vec<Arc<Cluster>>) {


    // sync channel to collect thread results
    // let (tx, rx) = mpsc::channel();

    for chunk in points.chunks(250) {
        let chunk = chunk.to_owned();
        let t_clusters = clusters.clone();
        let chan = tx.clone();

        pool.execute(move || {
            let result = update_points_clusters(chunk, t_clusters);
            chan.send(result).unwrap();
        });
    }

    points.clear();

    // collecting thread output
    for _ in 0..pool.size() {
        points.extend(rx.recv().unwrap());
    }

    // Copying to a new cluster container...
    let mut packed_new_clusters: Vec<(Cluster, f64)> = Vec::with_capacity(clusters.len());
    for cluster in clusters {
        packed_new_clusters.push(
            (Cluster::new(cluster.idx, Point::new(0.0, 0.0)), 0.0)
        );
    }


    // Calculating the average
    for point in &mut points {
        let packed_cluster = packed_new_clusters
            .get_mut(point.cluster.as_ref().unwrap().idx)
            .unwrap();

        packed_cluster.0.centroid.x += point.x;

        packed_cluster.0.centroid.y += point.y;

        packed_cluster.1 += 1.0;
    }


    let new_clusters: Vec<_> = packed_new_clusters.iter_mut().map(|(packed_cluster, point_count)| {
        packed_cluster.centroid.x /= *point_count;
        packed_cluster.centroid.y /= *point_count;
        Arc::new(packed_cluster.to_owned())
    }).collect();

    (points, new_clusters)
}

pub fn kmeans(mut points: Vec<Point>, k: usize, max_iter: usize) {
    let pool = ThreadPool::new(12);

    let mut rng = rand::thread_rng();

    let mut iter_count = 0;

    let mut clusters = Vec::with_capacity(k);

    for (idx, point) in points.choose_multiple(&mut rng, k).enumerate() {
        clusters.push(
            Arc::new(Cluster::new(idx, point.clone()))
        );
    }

    let chan = channel();

    while iter_count < max_iter {
        print!("\rCurrent iteration: {}", iter_count);

        (points, clusters) = parallel_iteration(points, clusters, &chan, &pool);

        iter_count += 1;
    }

    print!("\rFinished {} iterations", iter_count);

    print_clusters!(clusters);
}



