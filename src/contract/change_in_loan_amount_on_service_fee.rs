pub async  fn change_in_loan_amount_on_service_fee(  

loan_amount :f32,

   
)->LoanServiceFeeOutput{




if loan_amount>=0.1 && loan_amount<=50.99{

let new_loan_amount  = 6.0/100.0*loan_amount;

let delta_loan_amount :f32 = new_loan_amount;
    
let  loan =LoanServiceFeeOutput{  delta_loan_amount :delta_loan_amount };
  
return loan

}

else if loan_amount>=51.0 && loan_amount<=100.99 {

             
    let new_loan_amount  = 6.0/100.0*loan_amount;
    
    let delta_loan_amount :f32 = new_loan_amount;       

    
       
    let  loan =LoanServiceFeeOutput{
        delta_loan_amount :delta_loan_amount };
    
    return loan  
    
        }
    

else if loan_amount>=101.99 && loan_amount<=300.99 {

//2.0 + (1.0/100.0)*p
//base fee-> 2.0 + 1.0% service fee->(1.0/100.0)*p
         
let new_loan_amount  = 7.5/100.0*loan_amount;

let delta_loan_amount :f32 = new_loan_amount;       


// Calculating interest rate per month
// r = lender's interest rate ->R +service fee-> 1.0%  /(12*100)
    
    
let  loan =LoanServiceFeeOutput{
    delta_loan_amount :delta_loan_amount };

return loan  

    }


else if loan_amount>=301.0 && loan_amount<=500.99 {
//2.0 + (1.0/100.0)*p
//base fee-> 2.0 + 1.0% service fee->(1.0/100.0)*p

let new_loan_amount  = 12.0/100.0*loan_amount;

let delta_loan_amount :f32 = new_loan_amount;  


// Calculating interest rate per month
// r = lender's interest rate ->R + service fee-> 1.0% /(12*100)
    
  
let  loan =LoanServiceFeeOutput{

    delta_loan_amount :delta_loan_amount };

return loan 

}


else if loan_amount>=501.0 && loan_amount<=1500.99 {


let new_loan_amount  = 9.5/100.0*loan_amount;

let delta_loan_amount :f32 = new_loan_amount;  

   
    
let  loan =LoanServiceFeeOutput{ 

    delta_loan_amount :delta_loan_amount };

return loan  

}

    
else if loan_amount>=1501.0 && loan_amount<=2500.99 {

    let new_loan_amount  = 7.0/100.0*loan_amount;

    let delta_loan_amount :f32 = new_loan_amount; 

let  loan =LoanServiceFeeOutput{
        
    delta_loan_amount :delta_loan_amount };
    
return loan  
   
     
}

else if loan_amount>=2501.0 && loan_amount<=3000.99 {


    let new_loan_amount  = 6.0/100.0*loan_amount;

    let delta_loan_amount :f32 = new_loan_amount; 

let  loan =LoanServiceFeeOutput{

    delta_loan_amount :delta_loan_amount };
    
return loan  
   

}
    
else if  loan_amount>3000.99 && loan_amount<=9000.99  {


    let new_loan_amount  = 6.0/100.0*loan_amount;

    let delta_loan_amount :f32 = new_loan_amount; 
    
    
let  loan =LoanServiceFeeOutput{

    delta_loan_amount :delta_loan_amount };
    
return loan  
    
}  
    
    
else if  loan_amount>9000.99  {

    let new_loan_amount  = 3.0/100.0*loan_amount;

    let delta_loan_amount :f32 = new_loan_amount; 
let  loan =LoanServiceFeeOutput{

delta_loan_amount :delta_loan_amount };

return loan  
  
}


else{

    let new_loan_amount  = 0.0/100.0*loan_amount;

    let delta_loan_amount :f32 = new_loan_amount;

let err =  LoanServiceFeeOutput{

    delta_loan_amount };
   
return err;

}
}

#[derive(Debug)]
pub struct LoanServiceFeeOutput {

pub delta_loan_amount :f32
    
}