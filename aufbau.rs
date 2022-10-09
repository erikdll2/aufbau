use std::io;

pub struct Player {
    point: Point,
    priority: usize, 
}

#[derive(Clone)]

pub struct Point {
    l: i32,
    n: i32,
}

fn prioritize (vec: &Vec<Player>) -> Option<usize> {
    
    let mut result = None;
    for index in 0..vec.len() {

        if vec[index].priority == 0 {
            result = Some(index);
            break;
        }
        
    }
    return result;
}

impl Player{
    fn add_player(n: i32) -> Player {
        let player_vec = Player {
            point: { Point { l: 0, n: n+1 } },
            priority: 1,
        };
        player_vec
    }
}

fn decide(mut vec: Vec<Player>,mut magazine: Vec<Point>, priority: usize) -> (Vec<Player>, Vec<Point>) {
    magazine.push(vec[priority].point.clone());

    if vec[priority].point.l == 1 || vec[priority].point.l == 0 && vec[priority].point.n == 0 {       
        vec.push(Player::add_player(vec[priority].point.n));

    }

    vec[priority].priority += 1;

    for i in 0..vec.len() {
        vec[i].priority -= 1;
    }
 
    vec[priority].priority += vec.len()-1;

    if vec[priority].point.l == vec[priority].point.n {

        vec.remove(priority);
        
    } else {
        vec[priority].point.l+=1;
    }
    if vec.len() == 3 {
        if vec[0].priority == 0 && vec[2].priority == 0 {

        vec[0].priority = 1;

        } 
    }
    if vec.len() == 4 {
       if vec[0].priority == 0 && vec[3].priority == 0 {
           vec[0].priority = 1;
    }
}
    return ( vec, magazine );

}

impl Point{
    fn formalize (&self) -> (i32, char) {
        let base = &self.n+1;
        let letter = match &self.l {
            3 => 'f',
            2 => 'd',
            1 => 'p',
            0 => 's',
            _ =>  'n',
        };
        return ( base, letter );
    }
}


fn main() {
    let mut vec: Vec<Player> = Vec::new();
    let mut magazine: Vec<Point> = Vec::new();
    let mut aufbau;
    let mut exponential;

    println!("Aufbau principle");

    println!("\nPlease input the electron number");

    let mut electrons = String::new();

    io::stdin()
        .read_line(&mut electrons)
        .expect("Failed to read line");

    let mut electrons: i32 = electrons.trim().parse().expect("Please type a number!");

    // Player struct contains Point struct and priority usize
    // GITHUB CHANGE
    let player_vec = Player {
        point: { Point { l: 0, n: 0 } },
        priority: 0,
    };

    // Inserts instance of Player into the vector
    vec.push(player_vec);

    // Returns the player with priority 0
    let mut priority = prioritize(&vec).unwrap();

    for i in 0..20 {

        // Returns the modified versions of both list of 'players' and output to be printed
        (vec, magazine) = decide(vec, magazine, priority);

        // Set the priority for the next iteration
        priority = prioritize(&vec).unwrap();

        // Returns the char and i32 values contained in the output vector
        aufbau = magazine[i].formalize().clone();

        // Substruction of electrons value according to the value of l
        electrons = match  aufbau.1 {
            's' => electrons-2,
            'p' => electrons-6,
            'd' => electrons-10,
            'f' => electrons-14,
             _ => 100,
        };
        
        // if the remaining electrons are below 0 terminate the program
        if electrons <= 0 {

            exponential = match  aufbau.1 {
                's' => electrons+2,
                'p' => electrons+6,
                'd' => electrons+10,
                'f' => electrons+14,
                 _ => 100,
            };

            print!("{}{}^{}",&aufbau.0, &aufbau.1, &exponential);
            break;
        }

        exponential = match  aufbau.1 {
            's' => 2,
            'p' => 6,
            'd' => 10,
            'f' => 14,
                 _ => 100,
        };
        print!("{}{}^{} ",&aufbau.0, &aufbau.1,&exponential);
    }

}

// ADD-ON FOR GRAPHICS