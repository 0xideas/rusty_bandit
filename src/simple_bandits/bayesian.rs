use std::fmt::{Display, Formatter};

use std::fmt;
use statrs::distribution::Beta;
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;
use rand::distributions::Distribution;

pub struct BayesianBandit{
    pub n_arms: i8,
    pub arms: Vec<Beta>,
    rng: StdRng
}

impl BayesianBandit{

    pub fn new(n_arms: i8) -> Self {
        let n_arms_size: usize = n_arms.try_into().unwrap();
        let beta = Beta::new(1.0, 1.0).unwrap();
        let arms: Vec<Beta> =  vec![beta; n_arms_size];
        let rng = StdRng::seed_from_u64(101);
        BayesianBandit{ n_arms, arms, rng}
    }

    pub fn update(mut self, arm: i8, success: bool) -> Self {
        let arm_size: usize = arm.try_into().unwrap();
        let beta = self.arms[arm_size];
        if success {
            let new_beta = Beta::new(beta.shape_a() + 1.0, beta.shape_b()).unwrap();
            self.arms[arm_size] = new_beta;
        } else {
            let new_beta = Beta::new(beta.shape_a(), beta.shape_b()+ 1.0).unwrap();
            self.arms[arm_size] = new_beta;
        }
        self
    }

    pub fn act<R: Rng + ?Sized>(&self, rng: &mut R) -> i8 {
        let samples_float: Vec<f64> =  self.arms.iter().map(|beta| -> f64 {beta.sample(rng)}).collect();
        //println!("{:?}", samples_float);
        let samples: Vec<u64> = samples_float.iter().map(|v| -> u64 {v.to_bits()}).collect();
        let max_value: u64 = samples.iter().fold(0, |max: u64, val: &u64| if val > &max{ *val } else{ max });
        let largest = samples.iter().position(|r: &u64| r == &max_value).unwrap();
        let largest_int: i8 = largest.try_into().unwrap();
        largest_int
    }
}


impl fmt::Display for BayesianBandit{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let arm_reprs: Vec<String> = self.arms.iter().enumerate().map(|(i,arm)| -> String {format!("arm {}: (a: {} b: {})", i, arm.shape_a(), arm.shape_b() )}).collect();
        write!(f, "---BayesianBandit---\n{}", arm_reprs.join("\n"))
    }
}

