use rand::Rng;

pub fn run() {
   let rani = || rand::thread_rng().gen_range(-100..=100);
   let mut working_combos = Vec::new();
   let mut combo: [i64; 3];

   let equation = |x: i64, y: i64, z: i64| x.pow(3) + y.pow(3) + z.pow(3) == 20;

   loop {
      combo = [rani(), rani(), rani()];

      combo.sort();

      if equation(combo[0], combo[1], combo[2]) && !working_combos.contains(&combo) {
         working_combos.push(combo);
         println!("{}: {:?}", working_combos.len(), combo);
      }
   }
}
