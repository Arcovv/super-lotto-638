use itertools::Itertools;
use rand::seq::SliceRandom;

fn main() {
    let mut rng = rand::thread_rng();

    let mut fst_section = vec![
        1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15,
        16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28,
        29, 30, 31, 32, 33, 34, 35, 36, 37, 38,
    ];
    fst_section.shuffle(&mut rng);

    let mut snd_section = vec![1, 2, 3, 4, 5, 6, 7, 8];
    // snd_section.shuffle(&mut rng);

    for _ in 1..=snd_section.len() {
        start_game(&mut fst_section, &mut snd_section);
    }
}

fn start_game(
    fst_section: &mut Vec<i32>,
    snd_section: &mut Vec<i32>,
) {
    let mut rng = rand::thread_rng();

    let fst_chosen: Vec<_> = fst_section
        .choose_multiple(&mut rng, 6)
        .sorted()
        .cloned()
        .collect();

    let snd_chosen =
        snd_section.choose(&mut rng).cloned().unwrap();

    *snd_section = snd_section
        .iter()
        .filter(|x| **x != snd_chosen)
        .cloned()
        .collect_vec();

    println!("===========================");
    println!("fst_chosen: {fst_chosen:?}");
    println!("snd_chosen: {snd_chosen}");
    println!();
}
