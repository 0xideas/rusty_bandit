mod simple_bandits {
    pub mod bayesian;
}
use simple_bandits::bayesian::BayesianBandit;
use rand::Rng;



fn print_action_frequency<R: Rng + ?Sized>(bandit: &BayesianBandit, n: i32, rng: &mut R) -> () {
    let actions: Vec<i8> = (0..n).map(|_| -> i8 {bandit.act(rng)}).collect();
    for action in 0..bandit.n_arms {
        let action_i8 = i8::from(action);
        println!("action {}: {}", action_i8, actions.iter().filter(|&n| *n == action_i8).count());
    }
}

fn main() {
    let mut bandit: BayesianBandit = BayesianBandit::new(3);
    let mut rng = rand::thread_rng();

    print_action_frequency(&bandit, 100, &mut rng);

    for _ in 0..10 {
        bandit = bandit.update(1, true)
    }

    print_action_frequency(&bandit, 100, &mut rng);


}

