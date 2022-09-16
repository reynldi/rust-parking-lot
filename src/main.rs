use std::io;

const MAX_ALLOC_LOT: i8 = 125;

fn main() {
    println!("===PARKING LOT ABC BUILDING===");
    println!("Available Max Parking Lot is: {}", MAX_ALLOC_LOT);

    println!("Enter total available parking lot");
    let mut total_parking_lot = String::new();

    io::stdin()
        .read_line(&mut total_parking_lot)
        .expect("Failed to read line");

    let total_parking_lot: i8 = match total_parking_lot.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please check your input! (Only number)");
            return;
        }
    };

    println!("Total Allocated Parking Lot: {}", total_parking_lot);
    let mut alloc_parking_lot = vec![0; total_parking_lot.try_into().unwrap()]; // allocate memory for parking lot

    loop {
        park_now(&mut alloc_parking_lot);

        // Prompt
        println!("Do you want to park another car? (y/n)");
        let mut answer = String::new();
        io::stdin()
            .read_line(&mut answer)
            .expect("Failed to read line");

        let answer: &str = answer.trim();
        if answer == "n" {
            break;
        }
    }
}

fn park_now(alloc_parking_lot: &mut Vec<u32>) {
    // Input Car Details
    println!("Enter Your Car Number");
    let mut car_number = String::new();

    io::stdin()
        .read_line(&mut car_number)
        .expect("Failed to read line");

    let car_number: i32 = match car_number.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please check your input!");
            return;
        }
    };

    loop {
        // Put your car number into parking lot
        println!(
            "Enter which parking slot you will in for your car: {}",
            car_number
        );
        let mut parking_slot = String::new();

        io::stdin()
            .read_line(&mut parking_slot)
            .expect("Failed to read line");

        let parking_slot: i8 = match parking_slot.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please check your input!");
                return;
            }
        };

        let parking_slot = parking_slot as usize; // convert to usize

        if alloc_parking_lot[parking_slot] > 0 {
            println!("Sorry, this slot is already taken!");
            return;
        }

        alloc_parking_lot[parking_slot] = car_number as u32;

        println!(
            "Your car with car number {} is in parking lot: {}",
            car_number, parking_slot
        );
        println!("Allocated Parking Lot: {:?}", alloc_parking_lot);
        break;
    }
}
