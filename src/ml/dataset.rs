use anyhow::{Context,Result};
use serde::Deserialize;
use std::fs;
#[derive(Debug,Deserialize)]
pub struct RawDocument {
    pub input:String,
    pub intent:String,
    pub entities:Vec<RawEntity>,
}
#[derive(Debug,Deserialize,Clone)]
pub struct RawEntity {
    #[serde(rename="type")]
    pub entity_type:String,
    pub value:String,
    pub decision:String,
}
#[derive(Debug,Clone,Copy,PartialEq,Eq)]
pub enum Label {
    Keep,
    Mask,
}
impl Label {
    pub fn from_str(value:&str)->Result<Self>{
        match value {
            "KEEP"=>Ok(Self::Keep),
            "MASK"=>Ok(Self::Mask),
            other=>anyhow::bail!("unknown decision: {other}"),
        }
    }
    pub fn id(&self)->u8{
        match self {
            Self::Keep=>0,
            Self::Mask=>1,
        }
    }
}
#[derive(Debug,Clone)]
pub struct TrainingSample {
    pub document_id:usize,
    pub text:String,
    pub intent:String,
    pub entity_type:String,
    pub entity_value:String,
    pub label:Label,
}
impl TrainingSample {
    pub fn marked_text(&self)->String{
        mark_entity(&self.text,&self.entity_value)
    }
}
pub fn mark_entity(text:&str,entity:&str)->String{
    if let Some(start)=text.find(entity){
        let end=start+entity.len();

        format!(
            "{}[ENT]{}[/ENT]{}",
            &text[..start],
            &text[start..end],
            &text[end..]
        )
    }else{
        text.to_string()
    }
}
pub fn load_dataset(path:&str)->Result<Vec<TrainingSample>>{
    let data=fs::read_to_string(path)
        .with_context(||format!("failed to read dataset: {path}"))?;

    let documents:Vec<RawDocument>=serde_json::from_str(&data)
        .context("failed to parse dataset JSON")?;

    let mut samples=Vec::new();

    for(document_id,document)in documents.into_iter().enumerate(){
        for entity in document.entities{
            samples.push(TrainingSample{
                document_id,
                text:document.input.clone(),
                intent:document.intent.clone(),
                entity_type:entity.entity_type,
                entity_value:entity.value,
                label:Label::from_str(&entity.decision)?,
            });
        }
    }
    Ok(samples)
}