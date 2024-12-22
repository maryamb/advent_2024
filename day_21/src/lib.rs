use std::collections::HashMap;



#[derive(Debug)]
struct KeyPads {
    numeric_keypad: HashMap<(usize, usize), char>,
    reverse_numeric: HashMap<char, (usize, usize)>,
    directional_keypad: HashMap<(usize, usize), char>,
    reverse_directional: HashMap<char, (usize, usize)>,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
enum Mode {
    Numeric,
    Directional,
}

impl Default for KeyPads {
    fn default() -> Self {
        let mut numeric_kp: HashMap<(usize, usize), char> = HashMap::new();
        numeric_kp.insert((0, 1), '0'); numeric_kp.insert((0, 2), 'A');
        numeric_kp.insert((1, 0), '1'); numeric_kp.insert((1, 1), '2'); numeric_kp.insert((1, 2), '3');
        numeric_kp.insert((2, 0), '4'); numeric_kp.insert((2, 1), '5'); numeric_kp.insert((2, 2), '6');
        numeric_kp.insert((3, 0), '7'); numeric_kp.insert((3, 1), '8'); numeric_kp.insert((3, 2), '9');
        let reverse_numeric: HashMap<char, (usize, usize)> = numeric_kp.iter().map(|(k, v)| (*v, (k.0, k.1))).collect();

        let mut direction_kp: HashMap<(usize, usize), char> = HashMap::new();
        direction_kp.insert((0, 0), '<'); direction_kp.insert((0, 1), 'v'); direction_kp.insert((0, 2), '>');
                                          direction_kp.insert((1, 1), '^'); direction_kp.insert((1, 2), 'A');
        let reverse_directional: HashMap<char, (usize, usize)> = direction_kp.iter().map(|(k, v)| (*v, (k.0, k.1))).collect();
        Self{numeric_keypad: numeric_kp, reverse_numeric: reverse_numeric, directional_keypad: direction_kp, reverse_directional: reverse_directional}
    }
}

fn navigate(kp: &KeyPads, mode: &Mode, start_char: char, end_char: char) -> Vec<char> {
    let rev = if *mode == Mode::Numeric { &kp.reverse_numeric } else { &kp.reverse_directional };
    let s = rev.get(&start_char).unwrap();
    let e = rev.get(&end_char).unwrap();
    let (dx, dy) = (e.0.abs_diff(s.0), e.1.abs_diff(s.1));
    let mut output: Vec<char> = Vec::new();
    let y_dir: char = if e.1 < s.1 {'<'} else {'>'};
    let x_dir: char = if e.0 < s.0 {'v'} else {'^'};
    output.extend(std::iter::repeat(y_dir).take(dy));
    output.extend(std::iter::repeat(x_dir).take(dx));
    output
} 

fn get_cache(kp: &KeyPads) -> HashMap<(char, char), Vec<char>> {
    let cache_navigate_results = |keys: &Vec<&char>, mode: &Mode| -> HashMap<(char, char), Vec<char>> {
        keys.iter()
        .flat_map(|&k1| {
            keys.iter()
                .map(move |&k2| {
                    (
                        (*k1, *k2),
                        navigate(kp, mode, *k1, *k2)
                    ) 
                })
        })
        .collect()
    };
    let mut cache: HashMap<(char, char), Vec<char>> = HashMap::new();
    let directional_keys: Vec<&char> = kp.reverse_directional.keys().collect();
    let numeric_keys: Vec<&char> = kp.reverse_numeric.keys().collect();
   
    cache.extend(cache_navigate_results(&directional_keys, &Mode::Directional));
    cache.extend(cache_navigate_results(&numeric_keys, &Mode::Numeric));
    cache
}

fn first_robot<'a>(passcode: &'a str, cache: &'a HashMap<(char, char), Vec<char>>) -> Vec<&'a Vec<char>> {
    let p: Vec<char> = std::iter::once('A')
        .chain(passcode.chars())
        .collect();
    p.windows(2).into_iter().map(|s| {cache.get(&(s[0], s[1])).unwrap()}).collect::<Vec<&Vec<char>>>()
}

fn second_robot_v2<'a>(passcode: &'a Vec<&Vec<char>>, cache: &'a HashMap<(char, char), Vec<char>>) -> Vec<char> {
    let mut passcode: Vec<Vec<char>> = passcode.iter().map(|&v| v.clone()).collect();
    passcode[0].insert(0, 'A');
    passcode.iter_mut().for_each(|v| {
        v.push('A');
    });
    let all_passcodes = passcode.iter_mut().fold(Vec::<char>::new(), |mut acc, v| {
        acc.extend(v.clone());
        acc
    });
    
    let decoder = |p: &Vec<char>| p
        .windows(2)
        .into_iter()
        .map(|s| {
            let mut a = cache.get(&(s[0], s[1])).unwrap().clone();
            a.push('A');
            a
        })
        .fold(Vec::<char>::new(), |mut acc, v| {
            acc.extend(v);
            acc
        });
    decoder(&all_passcodes)
}

fn second_robot<'a>(passcode: &'a Vec<&Vec<char>>, cache: &'a HashMap<(char, char), Vec<char>>) -> Vec<Vec<char>> {
    let mut passcode: Vec<Vec<char>> = passcode.iter().map(|&v| v.clone()).collect();
    passcode.iter_mut().for_each(|v| {
        let v = v;
        v.insert(0, 'A');
    });
    let mut passcode_with_a: Vec<Vec<char>> = Vec::new();
    passcode.iter().for_each(|v| {
        passcode_with_a.push(v.clone());
        passcode_with_a.push(vec![*v.last().unwrap(), 'A']);
    });
    let decoder = |p: &Vec<char>| p
        .windows(2)
        .into_iter()
        .map(|s| {
            cache.get(&(s[0], s[1])).unwrap()
        })
        .fold(Vec::<char>::new(), |mut acc, v| {
            acc.extend(v);
            acc
        });
    passcode_with_a.iter().map(|v| { let a = decoder(v); println!("v: {:?} \na: {:?}\n\n", v, a); a }).collect()
}

fn pg() {
    let kp =  KeyPads::default();
    let cache = get_cache(&kp);
    let passcode = "029A";
    let first_code = first_robot(&passcode, &cache);
    let second_code = second_robot_v2(&first_code, &cache);
    println!("first code:\n{:?}", first_code);
    println!("second code:\n{:?}", second_code);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let kp =  KeyPads::default();
        // let dirs = navigate(&kp, &Mode::Directional, 'A', '<');
        // println!("{:#?}", get_cache(&kp));
        pg();
    }
}
