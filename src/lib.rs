use std::rand::Rng;
use std::rand;

#[doc = "
RndValues is used as an iterator.
"]

pub struct RndValues {
   max_it: int,
   i: int,
   rnd: rand::TaskRng,
}

#[doc = "
define new trait for RndValues: function 'rnd_value' returns on each call a random value.
"]

pub trait RndTester {
   fn rnd_value<T: rand::Rand>(&mut self) -> T;
   fn rnd_vec<T: rand::Rand>(&mut self) -> Vec<T>;
}

#[doc = "
Implementation of trait RndValues

Function 'rnd_value' returns a random value. 
"]

impl RndTester for rand::TaskRng {
    fn rnd_value<T: rand::Rand>(&mut self) -> T {
        self.gen::<T>()
    }
    
    fn rnd_vec<T: rand::Rand>(&mut self) -> Vec<T> {
		let sz : uint = self.gen::<uint>() % 10000u;
        let mut v: Vec<T> = Vec::with_capacity(sz);
        for i in range(0, sz) {
			v.insert(i,self.gen());
        }
        v
    }
}

#[doc = "
Implementation of iterator for struct RndValues.
"]

impl<T: rand::Rand> Iterator<T> for RndValues {
    fn next(&mut self) -> Option<T> {
        self.i = self.i  + 1;
        if self.i <= self.max_it {
                Some(self.rnd.gen::<T>())
        } else {
                None
        }
    }
}

#[doc = "
factory that returns both an iterator as well as a value generator.

Example usage:

```test_harness

use rndtester::RndTester;

fn f(i: i32, j: u32) -> i32 {
        if j > 10 {
			-i
		} else {
			-i
		}
}


#[test]
fn test_rnd_values() {
   let (mut it, mut v) = rndtester::rnd_values();

   for i in it {
        let r = f(i,v.rnd_value::<u32>());
        assert!(-r == i);
   }
}
```
"]

pub fn rnd_values() -> (RndValues,rand::TaskRng) {
   let os =  match option_env!("RndIterations") {
        Some(s) => s,
        None => ""
   };
   let oi: Option<int> = os.parse();
   let i = match oi {
      Some(n) => n,
      None => 100
   };
   (RndValues { max_it: i, i: 0, rnd: rand::task_rng() } , rand::task_rng())
}


