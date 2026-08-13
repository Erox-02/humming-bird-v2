use anyhow::Result;
use ml::dataset::load_dataset;
use ml::features::extract_features;
use ml::model::HbpModel;
use std::collections::HashSet;
#[path="../src/ml/mod.rs"]
mod ml;
fn main()->Result<()>{
    let samples=load_dataset("dataset.json")?;
    println!("samples {}",samples.len());
    let mut documents:Vec<usize>=samples
        .iter()
        .map(|s|s.document_id)
        .collect();
    documents.sort_unstable();
    documents.dedup();
    let split=(documents.len()as f64*0.8)as usize;
    let train_docs:HashSet<usize>=documents[..split]
        .iter()
        .copied()
        .collect();
    let mut train_x=Vec::new();
    let mut train_y=Vec::new();
    let mut test_x=Vec::new();
    let mut test_y=Vec::new();
    for sample in &samples{
        let features=extract_features(sample);
        let label=sample.label.id()as f32;
        if train_docs.contains(&sample.document_id){
            train_x.push(features);
            train_y.push(label);
        }else{
            test_x.push(features);
            test_y.push(label);
        }
    }
    println!("train samples {}",train_x.len());
    println!("test samples {}",test_x.len());
    let model=HbpModel::train(train_x,train_y)?;
    println!("done");
    std::fs::create_dir_all("assets")?;
    model.save("assets/hbp100-v3.lgb")?;
    println!("saved");
    let mut tp=0usize;
    let mut tn=0usize;
    let mut fp=0usize;
    let mut fn_=0usize;

    for(features,label)in test_x.iter().zip(test_y.iter()){
        let probability=model.predict(features)?;
        let prediction=if probability>=0.5{1.0}else{0.0};
        match(prediction,*label){
            (1.0,1.0)=>tp+=1,
            (0.0,0.0)=>tn+=1,
            (1.0,0.0)=>fp+=1,
            (0.0,1.0)=>fn_+=1,
            _=>unreachable!(),
        }
    }

    let accuracy=(tp+tn)as f64/(tp+tn+fp+fn_)as f64;
    let precision=if tp+fp>0{
        tp as f64/(tp+fp)as f64
    }else{
        0.0
    };

    let recall=if tp+fn_>0{
        tp as f64/(tp+fn_)as f64
    }else{
        0.0
    };

    let f1=if precision+recall>0.0{
        2.0*precision*recall/(precision+recall)
    }else{
        0.0
    };

    println!();
    println!("accuracy {:.4}",accuracy);
    println!("precision {:.4}",precision);
    println!("recall {:.4}",recall);
    println!("f1 {:.4}",f1);
    Ok(())
}