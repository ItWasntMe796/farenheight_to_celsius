#[warn(unused_assignments)]
#[warn(unused_mut)]

struct TempratureCurrent {
    celsius: u64,
    farenheight: u64,
}

impl TempratureCurrent {
    fn get_far(&mut self)
    {
        println!(
            "Farenheight: "
        );
        let mut f: String = String::new();

        std::io::stdin().read_line(&mut f).unwrap();

        let far= f.trim().parse().unwrap();

        self.farenheight = far;
    }

    fn make_cel(&mut self)
    {
        let mut cel: u64 = 0;

        cel = (self.farenheight-32)/(9/5);

        self.celsius = cel;
    }
}

fn main() {
    let mut temp: TempratureCurrent = TempratureCurrent {
        celsius: 0,
        farenheight: 0,
    };

    temp.get_far();

    temp.make_cel();

    println!("");

    println!(
        "{} in Celsius is {}",
        temp.farenheight,
        temp.celsius
    );


}