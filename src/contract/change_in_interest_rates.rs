
pub  async fn change_in_interest_rates(  

    loan_amount :f32,
    interest_rate :f32
       
    )->InterestRateOutput{
    
    
    
    
    if loan_amount>=0.0 && loan_amount<=50.99{
    // 2.0 + (1.0/100.0)*p
    //base fee-> 2.0 + 1.0% service fee->(1.0/100.0)*p
         
    let r_p= interest_rate;
    
         
         
    // Calculating interest rate per month
    // r = lender's interest rate ->R + service fee-> 1.0%  /(12*100)
    
    let delta_rate:f32 = r_p-1.0;
        
    let  loan =InterestRateOutput{  interest_rate:delta_rate};
      
    return loan
    
    }
    
    else if loan_amount>=51.0 && loan_amount<=300.99 {
    
    //2.0 + (1.0/100.0)*p
    //base fee-> 2.0 + 1.0% service fee->(1.0/100.0)*p
             
             
    let r_p:f32= interest_rate;
    
    // Calculating interest rate per month
    // r = lender's interest rate ->R +service fee-> 1.0%  /(12*100)
        
        
    let delta_rate:f32 = r_p-1.0;
        
    
      
       
    let  loan =InterestRateOutput{
    interest_rate:delta_rate};
    
    return loan  
    
        }
    
    
    else if loan_amount>=301.0 && loan_amount<=500.99 {
    //2.0 + (1.0/100.0)*p
    //base fee-> 2.0 + 1.0% service fee->(1.0/100.0)*p
    
    let r_p= interest_rate;
    
    
    // Calculating interest rate per month
    // r = lender's interest rate ->R + service fee-> 1.0% /(12*100)
       
       
    
    let delta_rate:f32= r_p-1.0;
       
      
    let  loan =InterestRateOutput{
    
    interest_rate: delta_rate};
    
    return loan 
    
    }
    
    
    else if loan_amount>=501.0 && loan_amount<=1500.99 {
    
    // 2.0 + (1.25/100.0)*p
    //base fee-> 2.0 + 1.25% service fee->(1.25/100.0)*p
    
    let r_p: f32= interest_rate;
    
       
        
           
    // Calculating interest rate per month
    // r = lender's interest rate ->R + service fee-> 1.25% /(12*100)
       
       
    
    let delta_rate: f32 = r_p-1.25;
       
    let  loan =InterestRateOutput{ 
    
    interest_rate:delta_rate};
    
    return loan  
    
    }
    
        
    else if loan_amount>=1501.0 && loan_amount<=2500.99 {
    //2.0 + (1.25/100.0)*p
    //base fee-> 2.0 + 1.25% service fee->(1.25/100.0)*p
       
    let r_p: f32= interest_rate;
        
           
    // Calculating interest rate per month
    // r = lender's interest rate ->R + service fee-> 1.25%  /(12*100)
       
       
    
    let delta_rate: f32 = r_p-1.25;
       
    
            
                
    
    let  loan =InterestRateOutput{
            
    interest_rate:delta_rate};
        
    return loan  
       
         
    }
    
    else if loan_amount>=2501.0 && loan_amount<=3000.99 {
    //2.0 + (1.25/100.0)*p
    //base fee-> 2.0 + 1.25% service fee->(1.25/100.0)*p
          
    let r_p: f32= interest_rate;
    
    
        
           
    // Calculating interest rate per month
    // r = lender's interest rate ->R + service fee-> 1.25% /(12*100)
       
       
    let delta_rate: f32 = r_p-1.25;
       
    
    
    let  loan =InterestRateOutput{
        
    interest_rate:delta_rate};
        
    return loan  
       
    
    }
        
    else if  loan_amount>3000.99 && loan_amount<=9000.99  {
            
        
    //2.0 + (3.0/100.0)*p
    //base fee-> 2.0 + 3.0% service fee->(3/100.0)*p
    
    let r_p: f32= interest_rate;
    
         
            
    // Calculating interest rate per month
    // r = lender's interest rate ->R + service fee-> 3.0% /(12*100)
        
        
     
    let delta_rate: f32 = r_p-3.0;
        
        
    let  loan =InterestRateOutput{
    
    interest_rate:delta_rate};
        
    return loan  
        
    }  
        
        
    else if  loan_amount>9000.99  {
    //2.0 + (3.0/100.0)*p
    //base fee-> 2.0 + 3.0% service fee->(3/100.0)*p
    
    let r_p: f32= interest_rate;
    
    // Calculating interest rate per month
    // r = lender's interest rate ->R + service fee-> 3.0% /(12*100)
      
    let delta_rate: f32= r_p-3.5;
      
    
    let  loan =InterestRateOutput{
    
    interest_rate:delta_rate};
    
    return loan  
      
    }
    
    
    else{
      
    let interest_rate =0.0;
    
    let err = InterestRateOutput{
    
    interest_rate};
       
    return err;
    
    }
    }
    
    #[derive(Debug)]
    pub struct InterestRateOutput {
    
    pub interest_rate :f32
        
    }