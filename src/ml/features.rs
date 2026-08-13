use crate::ml::dataset::TrainingSample;
const TYPE_BUCKETS:usize=64;
const INTENT_BUCKETS:usize=128;
const WORD_BUCKETS:usize=512;
const CHAR_BUCKETS:usize=512;
pub const FEATURE_COUNT:usize=
    TYPE_BUCKETS+
    INTENT_BUCKETS+
    WORD_BUCKETS+
    CHAR_BUCKETS+
    23;
fn hash(value:&str)->usize{
    let mut hash:u64=14695981039346656037;
    for byte in value.bytes(){
        hash^=byte as u64;
        hash=hash.wrapping_mul(1099511628211);
    }
    hash as usize
}
fn bucket(value:&str,size:usize)->usize{
    hash(value)%size
}
fn add_hashed(features:&mut[f64],offset:usize,size:usize,value:&str){
    let index=offset+bucket(value,size);
    features[index]+=1.0;
}
fn entity_position(text:&str,value:&str)->(usize,usize){
    if let Some(start)=text.find(value){
        (start,start+value.len())
    }else{
        (0,0)
    }
}
fn shape_features(value:&str)->[f64;20]{
    let chars:Vec<char>=value.chars().collect();
    let len=chars.len()as f64;
    let digits=chars.iter().filter(|c|c.is_ascii_digit()).count();
    let letters=chars.iter().filter(|c|c.is_alphabetic()).count();
    let uppercase=chars.iter().filter(|c|c.is_uppercase()).count();
    let lowercase=chars.iter().filter(|c|c.is_lowercase()).count();
    let whitespace=chars.iter().filter(|c|c.is_whitespace()).count();
    let punctuation=chars.iter().filter(|c|c.is_ascii_punctuation()).count();
    [
        len,
        if len>0.0{1.0}else{0.0},
        digits as f64,
        letters as f64,
        uppercase as f64,
        lowercase as f64,
        whitespace as f64,
        punctuation as f64,
        if value.contains('@'){1.0}else{0.0},
        if value.contains('-'){1.0}else{0.0},
        if value.contains('/'){1.0}else{0.0},
        if value.contains('.'){1.0}else{0.0},
        if value.contains(':'){1.0}else{0.0},
        if value.chars().all(|c|c.is_ascii_digit()){1.0}else{0.0},
        if value.chars().all(|c|c.is_uppercase()||!c.is_alphabetic()){1.0}else{0.0},
        if value.chars().any(|c|c.is_numeric()){1.0}else{0.0},
        if value.chars().any(|c|c.is_alphabetic()){1.0}else{0.0},
        if value.chars().next().is_some_and(|c|c.is_uppercase()){1.0}else{0.0},
        if value.chars().last().is_some_and(|c|c.is_ascii_digit()){1.0}else{0.0},
        if value.contains(' '){1.0}else{0.0},
    ]
}
pub fn extract_features(sample:&TrainingSample)->Vec<f64>{
    let mut features=vec![0.0;FEATURE_COUNT];
    let type_offset=0;
    let intent_offset=type_offset+TYPE_BUCKETS;
    let word_offset=intent_offset+INTENT_BUCKETS;
    let char_offset=word_offset+WORD_BUCKETS;
    let shape_offset=char_offset+CHAR_BUCKETS;
    add_hashed(
        &mut features,
        type_offset,
        TYPE_BUCKETS,
        &sample.entity_type
    );
    add_hashed(
        &mut features,
        intent_offset,
        INTENT_BUCKETS,
        &sample.intent
    );
    let(start,end)=entity_position(&sample.text,&sample.entity_value);
    let text_len=sample.text.len().max(1)as f64;
    features[shape_offset]=start as f64/text_len;
    features[shape_offset+1]=end as f64/text_len;
    features[shape_offset+2]=sample.entity_value.len()as f64/text_len;
    let left=&sample.text[..start];
    let right=&sample.text[end..];
    let context=format!("{} {} {}",left,sample.entity_value,right);
    for word in context.split_whitespace(){
        add_hashed(
            &mut features,
            word_offset,
            WORD_BUCKETS,
            &word.to_lowercase()
        );
    }
    let chars:Vec<char>=context.to_lowercase().chars().collect();
    for n in 2..=4{
        if chars.len()<n{
            continue;
        }
        for window in chars.windows(n){
            let gram:String=window.iter().collect();
            add_hashed(
                &mut features,
                char_offset,
                CHAR_BUCKETS,
                &gram
            );
        }
    }
    let shape=shape_features(&sample.entity_value);
    for(i,value)in shape.iter().enumerate(){
        features[shape_offset+3+i]=*value;
    }
    features
}