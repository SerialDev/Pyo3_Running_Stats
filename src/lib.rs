#![feature(proc_macro, specialization, const_fn)]

extern crate pyo3;
use pyo3::prelude::*;
use std::f64;




#[py::class]
struct RunningStats {
    n: f64,
    old_m: f64,
    new_m: f64,
    old_s: f64,
    new_s: f64,
    min_: f64,
    max_: f64,
    token: PyToken
}

#[py::methods]
impl RunningStats {
    
    #[new]
    fn __new__(obj: &PyRawObject) -> PyResult<()>{
        obj.init(|t| RunningStats {min_: std::f64::NEG_INFINITY,
                               max_: std::f64::INFINITY,
                               n: 0f64,
                               old_m: 0f64,
                               new_m: 0f64,
                               old_s: 0f64,
                               new_s: 0f64,
                               token: t})
    }
    
    fn clear(&mut self) -> PyResult<()>{
        self.n = 0f64;
        Ok(())
    }

    fn push(&mut self, x:f64) -> PyResult<()>{
        self.n += 1f64;

        if self.min_ > x{
            self.min_ = x;
        }

        if self.max_ < x{
            self.max_ = x;
        }

        if self.n == 1f64{
            self.old_m = x;
            self.new_m = x;
            self.old_s = 0f64;
        }
        else{
            self.new_m = self.old_m + ( x - self.old_m) / self.n;
            self.new_s = self.old_s + (x - self.old_m) * (x - self.new_m);

            self.old_m = self.new_m;
            self.old_s = self.new_s;
        }
        Ok(())
    }

    fn mean(&self) -> PyResult<f64> {

        let mut result: f64 = 0.0f64;
        if self.n > 0f64{
            result = self.new_m; 
        }
        
        Ok(result)
        
    }

    fn variance(&self) -> PyResult<f64> {
        let mut result = self.new_s/ (self.n -1f64);
        if self.n > 1f64{
            result = (self.new_s/ (self.n -1f64)) as f64;
        }
        else{
            result = 0.0f64;
        }
        Ok(result)
    }

    fn standard_deviation(&self) ->  PyResult<f64>{
        let mut result = self.new_s/ (self.n -1f64);
        if self.n > 1f64{
            result = (self.new_s/ (self.n -1f64)) as f64;
            result = result.sqrt();
        }
        else{
            result = 0.0f64;
        }
        
        Ok(result)
    }
    
}




#[py::modinit(collectors)]
fn init_mod(py: Python, m: &PyModule) -> PyResult<()> {

    #[pyfn(m, "mean")]
    fn mean(val_array: Vec<f64>)-> PyResult<f64>{
        let mean_val = val_array.iter().fold(0f64,|a, &b| a + b);
        let result: f64 =  mean_val/(val_array.len() as f64);
        Ok(result)
    }

    
    #[pyfn(m, "sum_array")]
    fn sum_array(val_array:Vec<f64>)-> PyResult<f64>{
        let result = val_array.iter().fold(0f64,|a, &b| a + b);
        Ok(result)
    }

    #[pyfn(m, "dub")]
    fn dub(val: i32) -> PyResult<i32>{
        let result: i32 = val * 2;
        Ok(result)
    }

    m.add_class::<RunningStats>()?;
    
    Ok(())
}
