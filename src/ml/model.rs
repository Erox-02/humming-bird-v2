use anyhow::Result;
use lightgbm3::{Booster,Dataset};
use serde_json::json;
pub struct HbpModel {
    booster:Booster,
}
impl HbpModel {
    pub fn train(features:Vec<Vec<f64>>,labels:Vec<f32>)->Result<Self>{
        let dataset=Dataset::from_vec_of_vec(features,labels,true)?;

        let params=json!({
            "objective":"binary",
            "metric":["binary_logloss","auc"],
            "num_iterations":200,
            "learning_rate":0.05,
            "num_leaves":31,
            "max_depth":-1,
            "min_data_in_leaf":10,
            "feature_fraction":0.9,
            "bagging_fraction":0.9,
            "bagging_freq":1,
            "verbosity":-1
        });
        let booster=Booster::train(dataset,&params)?;
        Ok(Self{booster})
    }
    pub fn save(&self,path:&str)->Result<()>{
        self.booster.save_file(path)?;
        Ok(())
    }
    pub fn load(path:&str)->Result<Self>{
        let booster=Booster::from_file(path)?;

        Ok(Self{booster})
    }
    pub fn predict(&self,features:&[f64])->Result<f64>{
        let predictions=self.booster.predict_with_params(
            features,
            features.len()as i32,
            true,
            "num_threads=1"
        )?;
        Ok(predictions[0])
    }
}