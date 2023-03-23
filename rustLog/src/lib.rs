/*Random number generator
*/
use rand::Rng;

const number: [&str; 5] = ["1", "2", "3", "4", "5"];
pub fn random_number() -> String {
    let num = number[rand::thread_rng().gen_range(0..5)];
    //log 
    log::info!("number-info: {}", num);
    log::trace!("number-trace: {}", num);
    log::warn!("number-warn: {}", num);
    num.to_string()
}