use std::cmp::Ordering;

fn layersort(mut layers: Vec<(f64, f64)>) -> Vec<Vec<(f64, f64)>> {
    // Sort by length (descending)
    let x = layers.sort_by(|a, b| {
        (b.1 - b.0)
            .partial_cmp(&(a.1 - a.0))
            .unwrap_or(Ordering::Equal);
    });

    // Sort by start time
    layers.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(Ordering::Equal));

    // Group layers
    let mut groups: Vec<Vec<(f64, f64)>> = Vec::new();

    for layer in layers {
        let mut added = false;
        for group in &mut groups {
            if group.last().unwrap().1 < layer.0 {
                group.push(layer);
                added = true;
                break;
            }
        }
        if !added {
            groups.push(vec![layer]);
        }
    }

    groups
}

fn main() {
    let layers = vec![
        (10.0, 30.0),
        (21.0, 31.0),
        (14.0, 20.0),
        (19.0, 22.0),
        (35.0, 36.0),
    ];

    let out = layersort(layers);

    for group in out {
        println!("{:?}", group);
    }
}
