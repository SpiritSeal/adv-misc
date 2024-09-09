use std::cmp::Ordering;

fn layersort(mut layers: Vec<(f64, f64)>) -> Vec<Vec<(f64, f64)>> {
    layers.sort_by(|a, b| match a.0.partial_cmp(&b.0) {
        Some(Ordering::Equal) => b.1.partial_cmp(&a.1).unwrap(),
        Some(ord) => ord,
        None => panic!("Comparison failed"),
    });

    let mut groups: Vec<Vec<(f64, f64)>> = Vec::new();
    let mut maxlens: Vec<f64> = Vec::new();

    for layer in layers {
        let mut added = false;
        for (idx, group) in groups.iter_mut().enumerate() {
            if group.last().unwrap().1 < layer.0 {
                group.push(layer);
                maxlens[idx] = maxlens[idx].max(layer.1 - layer.0);
                added = true;
                break;
            }
        }
        if !added {
            groups.push(vec![layer]);
            maxlens.push(layer.1 - layer.0);
        }
    }

    let mut group_order: Vec<usize> = (0..groups.len()).collect();
    group_order.sort_by(|&a, &b| maxlens[b].partial_cmp(&maxlens[a]).unwrap());

    let mut out_groups: Vec<Vec<(f64, f64)>> = Vec::new();
    for &i in &group_order {
        out_groups.push(groups[i].clone());
    }

    out_groups
}

fn test() {
    let layers = vec![
        (10.0, 30.0),
        (21.0, 31.0),
        (14.0, 20.0),
        (19.0, 30.0),
        (35.0, 36.0),
    ];
    let out = layersort(layers);
    for group in out {
        println!("{:?}", group);
    }
}

fn main() {
    test();
}
