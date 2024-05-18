



pub fn main(){
    let option_some = Some(1);
    let option_none:Option<u32> = None;
    let result_ok:Result<i32, ()> = Ok(2);
    let result_err:Result<i32,()> = Err(());
    let handler = |_|{999};
    let var_1 = result_ok.map_err(handler);
    let var_2 = result_err.map_err(|_|{999});
    println!("var_1:{:?} var_2:{:?}",var_1,var_2);

    let var_3 = option_some.and_then(|x|{Some(x)});
    let var_4 = option_none.and_then(|x|{Some(x)});
    println!("var_3:{:?} var_4:{:?}",var_3,var_4);

    let var_5 = option_some.unwrap_or(999);
    let var_6  = option_none.unwrap_or(999);
    println!("var_5:{:?} var_6:{:?}",var_5,var_6);

    let var_7 = option_some.unwrap_or_else(||2*10);
    let var_8  = option_none.unwrap_or_else(||2*10);
    println!("var_7:{:?} var_8:{:?}",var_7,var_8);

    let var_9 = option_some.as_ref();
    let var_10  = option_none.as_ref();
    println!("var_9:{:?} var_10:{:?}",var_9,var_10);

    let var_11 = option_some.or(Some(999));
    let var_12 = option_none.or(Some(999));
    println!("var_11:{:?} var_12:{:?}",var_11,var_12);


    let mut mut_option_some = Some(1);
    let mut mut_option_none:Option<u32> = None;
    let var_13 = mut_option_some.as_mut();
    let var_14  = mut_option_none.as_mut();
    println!("var_13:{:?} var_14:{:?}",var_13,var_14);


    let var_15 = option_some.ok_or(0);
    let var_16 = result_ok.ok();
    println!("var_15:{:?} var_16:{:?}",var_15,var_16);
}