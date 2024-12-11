use std::collections::HashMap;



fn step(input: Vec<i64>) -> Vec<i64> {
    let mut result: Vec<i64> = Vec::new();
    input.iter().for_each(|k| {
        if *k == 0 { result.push(1); } 
        else if k.to_string().len() % 2 == 0 {
            let k = k.to_string();
            let l: i64 = k[..k.len() / 2].parse().unwrap();
            let r: i64 = k[k.len() / 2 ..].parse().unwrap();
            result.push(l);
            result.push(r);
        } else {
            result.push(k * 2024);
        }
    });
    // println!("result: {:?}", result);
    result
}

fn step_with_cache(input: &mut HashMap<i64, i64>, cache: &mut HashMap<i64, Vec<i64>>) -> HashMap<i64, i64> {  
    input.iter().filter(|kv| !cache.contains_key(kv.0)).map(|k| *k.0).collect::<Vec::<i64>>().into_iter().for_each(|k| {
        let mut result: Vec<i64> = Vec::new(); 
        if k == 0 { result.push(1); } 
        else if k.to_string().len() % 2 == 0 {
            let k = k.to_string();
            let l: i64 = k[..k.len() / 2].parse().unwrap();
            let r: i64 = k[k.len() / 2 ..].parse().unwrap();
            result.push(l);
            result.push(r);
        } else {
            result.push(k * 2024);
        }
        cache.insert(k, result);
    });
    // println!("result: {:?}", result);
    input.iter().fold(HashMap::new(), |mut acc, kv| {
        let maps_to = cache.get(kv.0).unwrap();
        maps_to.iter().for_each(|e| {
            let c = acc.entry(*e).or_insert(0);
            *c += kv.1;
        });
        acc
    })
}

pub fn part_1(input: Vec<i64>, num_steps: usize) -> usize {
    (0..num_steps).into_iter().fold(input, |mut acc, _| {
        acc = step(acc);
        acc
    }).len()
}


pub fn part_2(input: Vec<i64>, num_steps: usize) -> usize {
    let mut keys_and_nums: HashMap<i64, i64> = HashMap::new();
    input.iter().for_each(|k| {
        let count = keys_and_nums.entry(*k).or_insert(0); // Get a mutable reference to the value
        *count += 1;
    });
    let mut cache: HashMap<i64, Vec<i64>> = HashMap::new();
    (0..num_steps).into_iter().for_each(|step_n| {
        keys_and_nums = step_with_cache(& mut keys_and_nums, &mut cache);
        println!("step: {}, length of acc: {}, cache size: {}", step_n, keys_and_nums.len(), cache.len());
    });
    keys_and_nums.iter().map(|kv| *kv.1 as usize).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        // assert_eq!(part_1(vec![125, 17], 6), 22);
        // assert_eq!(part_1(vec![125, 17], 25), 55312);
        // assert_eq!(part_1(vec![0, 44, 175060, 3442, 593, 54398, 9, 8101095], 25), 197157);
        assert_eq!(part_2(vec![0, 44, 175060, 3442, 593, 54398, 9, 8101095], 75), 55312);
    }
}
