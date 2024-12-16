pub async fn applied_fees(  original_loan_amount :f32,
    loan_amount :f32,
     number_of_months :f32,
     interest_rate :f32,
   )->LoanOutput{



    let  base_fee:f32 = 2.0;

    if loan_amount>=0.0 && loan_amount<=50.99{
       // 2.0 + (1.0/100.0)*p
        //base fee-> 2.0 + 1.0% service fee->(1.0/100.0)*p
     
        let r_p= interest_rate;

        let n=number_of_months;
     
       
    // Calculating interest rate per month
     // r = lender's interest rate ->R + service fee-> 1.0%  /(12*100)

    let r:f32 = (r_p+ 1.0)/(12.0*100.0);
    
    let delta_rate:f32 = r_p+1.0;
    
    //Calculating Equated Monthly Installment(EMI)
    
    let emi: f32 = loan_amount*r/(1.0-(r+1.0).powf(-n));
    
    

        // println!("your loan amount: {:?}",v_i);
        
        // println!("Amount still owe on n months ?");
        // let input: String= owe_on_n_months();
        // let n_mtly: f32=f32::from_str(&input).unwrap();
     
        // println!("Your Number of monthly installments: {:?}", n_mtly);
    
     
        // println!("Monthly payment: {:?}", n_mtly);
        // for i  in 0 ..=n_mtly as i32{
        // let v_n: f32= v_i*(1.0+r).powf( i as f32)-emi*(((1.0+r).powf(i as f32)-1.0)/r);
        // let d_prime: f32 =emi-r*v_n; 
     
        // println!(" Then interest paid : {:.2} /n the Principal reduction {:.2}  Loan balance is : {:.2} ",r*v_n,d_prime,v_n);
            
        // }
        // total_amount = emi*n + base fee
        let total_amount =emi*n+ base_fee; 
        let total_p_interest = emi*n-loan_amount;

    
  
         let number_of_months =n;
         let interest_rate =delta_rate;
         let monthly_payment =emi;
          let total_interest  =total_p_interest;
         let total_principal_interest =total_amount;
        // println!(" The Total amount  : {:.2}  Total interest  {:.2}   ",emi*n+base_fee,emi*n-p);

        let  loan =LoanOutput{original_loan_amount, loan_amount,
            number_of_months,  interest_rate, monthly_payment, total_interest,total_principal_interest};
      return loan
    }

    else if loan_amount>=51.0 && loan_amount<=300.99 {
        //2.0 + (1.0/100.0)*p
         //base fee-> 2.0 + 1.0% service fee->(1.0/100.0)*p
         let r_p:f32= interest_rate;

         let n: f32=number_of_months;
     
        
    // Calculating interest rate per month
 // r = lender's interest rate ->R +service fee-> 1.0%  /(12*100)
    
    

    let r:f32 = (r_p+1.0)/(12.0*100.0);
    
    let delta_rate:f32 = r_p+1.0;
    
    //Calculating Equated Monthly Installment (EMI)
    
    let emi:f32= loan_amount*r/(1.0-(r+1.0).powf(-n));
    
    

     

           // total_amount = emi*n + base fee
           let total_amount =emi*n + base_fee; 
           let total_p_interest = emi*n-loan_amount;
   
       
     
            let number_of_months =n;
            let interest_rate =delta_rate;
            let monthly_payment =emi;
             let total_interest  =total_p_interest;
            let total_principal_interest =total_amount;
           // println!(" The Total amount  : {:.2}  Total interest  {:.2}   ",emi*n+base_fee,emi*n-p);
   
           let  loan =LoanOutput{original_loan_amount, loan_amount,
               number_of_months,  interest_rate, monthly_payment, total_interest,total_principal_interest};
         return loan  
    }


    else if loan_amount>=301.0 && loan_amount<=500.99 {
          //2.0 + (1.0/100.0)*p
       //base fee-> 2.0 + 1.0% service fee->(1.0/100.0)*p

       let r_p= interest_rate;

       let n: f32=number_of_months;

       
   // Calculating interest rate per month
 // r = lender's interest rate ->R + service fee-> 1.0% /(12*100)
   
   

   let r:f32= (r_p+1.0)/(12.0*100.0);
   
   let delta_rate:f32 = r_p+1.0;
   //Calculating Equated Monthly Installment (EMI)
   
   let emi: f32 = loan_amount*r/(1.0-(r+1.0).powf(-n));
   
   
          // total_amount = emi*n + base fee
          let total_amount =emi*n + base_fee; 
          let total_p_interest = emi*n-loan_amount;
  
      
    
           let number_of_months =n;
           let interest_rate =delta_rate;
           let monthly_payment =emi;
            let total_interest  =total_p_interest;
           let total_principal_interest =total_amount;
          // println!(" The Total amount  : {:.2}  Total interest  {:.2}   ",emi*n+base_fee,emi*n-p);
  
          let  loan =LoanOutput{original_loan_amount, loan_amount,
              number_of_months,  interest_rate, monthly_payment, total_interest,total_principal_interest};
        return loan  
    }


    else if loan_amount>=501.0 && loan_amount<=1500.99 {
       // 2.0 + (1.25/100.0)*p
        //base fee-> 2.0 + 1.25% service fee->(1.25/100.0)*p

        let r_p: f32= interest_rate;

        let n: f32 =number_of_months;

    
       
   // Calculating interest rate per month
 // r = lender's interest rate ->R + service fee-> 1.25% /(12*100)
   
   

   let r: f32 = (r_p+1.25)/(12.0*100.0);
   
   let delta_rate:f32 = r_p+1.25;
   

   
   //Calculating Equated Monthly Installment (EMI)
   let emi: f32= loan_amount*r/(1.0-(r+1.0).powf(-n)); 

             // total_amount = emi*n + base fee
             let total_amount =emi*n + base_fee ; 
             let total_p_interest = emi*n-loan_amount;
     
         
       
              let number_of_months =n;
              let interest_rate =delta_rate;
              let monthly_payment =emi;
               let total_interest  =total_p_interest;
              let total_principal_interest =total_amount;
             // println!(" The Total amount  : {:.2}  Total interest  {:.2}   ",emi*n+base_fee,emi*n-p);
     
             let  loan =LoanOutput{original_loan_amount, loan_amount,
                 number_of_months,  interest_rate, monthly_payment, total_interest,total_principal_interest};
           return loan  

    }

    else if loan_amount>=1501.0 && loan_amount<=2500.99 {
          //2.0 + (1.25/100.0)*p
       //base fee-> 2.0 + 1.25% service fee->(1.25/100.0)*p
   
       let r_p: f32= interest_rate;

       
       let n: f32=number_of_months;

    
       
   // Calculating interest rate per month
   // r = lender's interest rate ->R + service fee-> 1.25%  /(12*100)
   
   

   let r: f32 = (r_p+1.25)/(12.0*100.0);
   
   let delta_rate:f32 = r_p+1.25;
   //Calculating Equated Monthly Installment (EMI)
   
   let emi: f32= loan_amount*r/(1.0-(r+1.0).powf(-n)); 


                // total_amount = emi*n + base fee
                let total_amount =emi*n + base_fee ; 
                let total_p_interest = emi*n-loan_amount;
        
            
          
                 let number_of_months =n;
                 let interest_rate =delta_rate;
                 let monthly_payment =emi;
                  let total_interest  =total_p_interest;
                 let total_principal_interest =total_amount;
                // println!(" The Total amount  : {:.2}  Total interest  {:.2}   ",emi*n+base_fee,emi*n-p);
        
                let  loan =LoanOutput{original_loan_amount, loan_amount,
                    number_of_months,  interest_rate, monthly_payment, total_interest,total_principal_interest};
              return loan  
   
     
    }

    else if loan_amount>=2501.0 && loan_amount<=3000.99 {
          //2.0 + (1.25/100.0)*p
       //base fee-> 2.0 + 1.25% service fee->(1.25/100.0)*p
      
       let r_p: f32= interest_rate;

       let n: f32=number_of_months;
    
       
   // Calculating interest rate per month
   // r = lender's interest rate ->R + service fee-> 1.25% /(12*100)
   
   

   let r: f32 = (r_p+1.25)/(12.0*100.0);
   
   let delta_rate:f32 = r_p+1.25;
   
   //Calculating Equated Monthly Installment (EMI)
   
   let emi: f32= loan_amount*r/(1.0-(r+1.0).powf(-n)); 


        // total_amount = emi*n + base fee
        let total_amount =emi*n ; 
        let total_p_interest = emi*n-loan_amount;
         let number_of_months =n;
         let interest_rate =delta_rate;
         let monthly_payment =emi;
          let total_interest  =total_p_interest;
         let total_principal_interest =total_amount;
        // println!(" The Total amount  : {:.2}  Total interest  {:.2}   ",emi*n+base_fee,emi*n-p);

        let  loan =LoanOutput{original_loan_amount, loan_amount,
            number_of_months,  interest_rate, monthly_payment, total_interest,total_principal_interest};
      return loan  
   

    }
    else if  loan_amount>3000.99 && loan_amount<=9000.99  {
        
    
          //2.0 + (3.0/100.0)*p
          //base fee-> 2.0 + 3.0% service fee->(3/100.0)*p

          let r_p: f32= interest_rate;

          let n: f32=number_of_months;
     
        
    // Calculating interest rate per month
 // r = lender's interest rate ->R + service fee-> 3.0% /(12*100)
    
    
 
    let r: f32 = (r_p+3.0)/(12.0*100.0);
    let delta_rate:f32 = r_p+3.0;
    //Calculating Equated Monthly Installment (EMI)
    
    let emi: f32= loan_amount*r/(1.0-(r+1.0).powf(-n)); 


            // total_amount = emi*n + base fee
            let total_amount =emi*n ; 
            let total_p_interest = emi*n-loan_amount;
             let number_of_months =n;
             let interest_rate =delta_rate;
             let monthly_payment =emi;
              let total_interest  =total_p_interest;
             let total_principal_interest =total_amount;
            // println!(" The Total amount  : {:.2}  Total interest  {:.2}   ",emi*n+base_fee,emi*n-p);
    
            let  loan =LoanOutput{original_loan_amount, loan_amount,
                number_of_months,  interest_rate, monthly_payment, total_interest,total_principal_interest};
          return loan  
    
   
    }  
    
    
      else if  loan_amount>9000.99  {
        
    
        //2.0 + (3.0/100.0)*p
        //base fee-> 2.0 + 3.0% service fee->(3/100.0)*p

        let r_p: f32= interest_rate;

        let n: f32=number_of_months;
   
      
  // Calculating interest rate per month
// r = lender's interest rate ->R + service fee-> 3.0% /(12*100)
  
  

  let r: f32= (r_p+3.5)/(12.0*100.0);
  
  let delta_rate:f32 = r_p+3.5;
  //Calculating Equated Monthly Installment (EMI)
  
  let emi: f32 = loan_amount*r/(1.0-(r+1.0).powf(-n)); 


          // total_amount = emi*n + base fee
          let total_amount =emi*n ; 
          let total_p_interest = emi*n-loan_amount;
           let number_of_months =n;
           let interest_rate =delta_rate;
           let monthly_payment =emi;
            let total_interest  =total_p_interest;
           let total_principal_interest =total_amount;
          // println!(" The Total amount  : {:.2}  Total interest  {:.2}   ",emi*n+base_fee,emi*n-p);
  
          let  loan =LoanOutput{original_loan_amount, loan_amount,
              number_of_months,  interest_rate, monthly_payment, total_interest,total_principal_interest};
        return loan  
  
 
  }


else{
    let number_of_months =0.0;
    let interest_rate =0.0;
    let monthly_payment =0.0;
     let total_interest  =0.0;
    let total_principal_interest =0.0;
   let err = LoanOutput{original_loan_amount, loan_amount,
    number_of_months,  interest_rate, monthly_payment, total_interest,total_principal_interest};
   return err;
}
}

#[derive(Debug)]
pub struct LoanOutput {

    pub original_loan_amount :f32,
    pub loan_amount :f32,
    pub number_of_months :f32,
    pub interest_rate :f32,
    pub monthly_payment :f32,
    pub  total_interest  :f32,
    pub  total_principal_interest:f32,

}