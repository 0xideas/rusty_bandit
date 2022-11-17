use statrs::distribution::Beta;

struct BayesianBandit{
    n_arms: i8,
    arms: [Beta]
}

impl BayesianBandit{

    fn new(n_arms: i8) -> BayesianBandit {
        let mut beta_array = [Beta{ a: 1.0, b: 1.0 }.unwrap(); n_arms];
        BayesianBandit{n_arms, arms}
    }

    fn update(&self, arm: i8, success: bool) -> Option<i8> {
        let beta = self.beta_array[arm];
        if success {
            let new_beta = Beta::new(beta.a + 1.0, beta.b).unwrap();
        } else {
            let new_beta = Beta::new(beta.a, beta.b + 1.0).unwrap();
        }
        self.beta_array[arm] = new_beta
    }

    fn act(&self) -> i8 {
        let samples = self.beta_array.iter().map(|beta| -> f64 {beta.sample(rng)});
        let largest = samples.iter().fold(-1.0, |a,b| a.max(*b));
        largest
    }
}