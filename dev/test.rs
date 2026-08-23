use anyhow::Result;

#[path="../src/ml/mod.rs"]
mod ml;
use ml::dataset::load_dataset;
use ml::features::extract_features;
use ml::model::HbpModel;

fn main()->Result<()>{
    let model=HbpModel::load("assets/hbp100-v3.lgb")?;
    let samples=load_dataset("dataset.json")?;
    println!("loaded {} samples",samples.len());
    println!();
    for sample in samples.iter().take(20){
        let features=extract_features(sample);
        let probability=model.predict(&features)?;
        let prediction=if probability>=0.5{
            "MASK"
        }else{
            "KEEP"
        };
        println!(
            "[actual:{:?}] [predicted:{}] [prob:{:.4}] {}",
            sample.label,
            prediction,
            probability,
            sample.entity_value
        );
    }

    Ok(())
}