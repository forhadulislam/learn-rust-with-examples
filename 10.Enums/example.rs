#[derive(Debug)]
enum Status{
    Started,
    Running,
    Stopped,
    Crashed
}

#[derive(Debug)]
enum AgeCategory {
    Child,
    Youth,
    Adult,
    Senior
 }

 fn get_age_category(age_cat:AgeCategory) {
    match age_cat {
        AgeCategory::Child => {
          println!("You are a child and your age is less than 15");
       },
       AgeCategory::Youth => {
        println!("You are a Youth and your age is between 15 and 24");
       },
       AgeCategory::Adult =>{
        println!("You are an Adult and your age is between 25 and 64!");
       },
       AgeCategory::Senior =>{
        println!("You are a senior and your are over 64 years of age!");
       }
    }
 }

// embedding enum in a struct
#[derive(Debug)]
struct Server {
    name: String,
    status: Status
 }

fn main() {
    // creating some variables from enum
    let started_ = Status::Started;
    let success_ = Status::Running;

    println!("Enum value is : {:?}", started_);
    println!("Another Enum value is : {:?}", success_);

    // creating from a struct
    let server_germany = Server {
        name: String::from("Munich_Germany"),
        status: Status::Running
     };

     println!("information of server: {:?}", server_germany);

     get_age_category(AgeCategory::Adult);

     // TODO: Match with Option
}
