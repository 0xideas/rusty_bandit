mod simple_bandits {
    pub mod bayesian;
}
use simple_bandits::bayesian::BayesianBandit;
use rand::RngCore;

fn main() {
    let bandit: BayesianBandit = BayesianBandit::new(3);
    let mut rng = rand::thread_rng();
    let actions: Vec<i8> = (0..100).map(|_| -> i8 {bandit.act(&mut rng)}).collect();

    for action in 0..3 {
        let action_i8 = i8::from(action);
        println!("count {}: {}", action_i8, actions.iter().filter(|&n| *n == action_i8).count());
    }

}

