use std::io;
use std::io::Write;
use rand::Rng;
use rand::seq::SliceRandom;

fn main() {
    print!("[1]Solo? [2]Pair? [3]Group? > ");
    io::stdout().flush().unwrap();
    let mut input_form = String::new();
    io::stdin().read_line(&mut input_form).expect("Failed to read line");
    let form: i32 = input_form.trim().parse().expect("Please enter a valid number");

    print!("Which class? [1]2-1 [2]2-2 [3]2-3 [4]2-4,5 [5]2-6 [6]2-7 [7]2-8 > ");
    io::stdout().flush().unwrap();
    let mut input_class = String::new();
    io::stdin().read_line(&mut input_class).expect("Failed to read line");
    let class: i32 = input_class.trim().parse().expect("Please enter a valid number");

    let mut array: Vec<String> = Vec::new();

    if class == 1 {
        let array_1 = [
            "111 AAA aaa   ",
        ];
        for &item in &array_1 {
            array.push(String::from(item));
        }
        array.shuffle(&mut rand::thread_rng());
        if array_1.len() % 2 == 1 && form == 2 {
            let mut rng = rand::thread_rng();
            let random_index = rng.gen_range(0..array_1.len());
            array.push(String::from(array_1[random_index]) + "*");
        }
    } else if class == 2 {
        let array_2 = [
            "222 BBB bbb   ",
        ];
        for &item in &array_2 {
            array.push(String::from(item));
        }
        array.shuffle(&mut rand::thread_rng());
        if array_2.len() % 2 == 1 && form == 2 {
            let mut rng = rand::thread_rng();
            let random_index = rng.gen_range(0..array_2.len());
            array.push(String::from(array_2[random_index]) + "*");
        }
    } else if class == 3 {
        let array_3 = [
            "333 CCC ccc   ",
        ];
        for &item in &array_3 {
            array.push(String::from(item));
        }
        array.shuffle(&mut rand::thread_rng());
        if array_3.len() % 2 == 1 && form == 2 {
            let mut rng = rand::thread_rng();
            let random_index = rng.gen_range(0..array_3.len());
            array.push(String::from(array_3[random_index]) + "*");
        }
    } else if class == 4 {
        let array_4 = [
            "444 DDD ddd   ",
        ];
        for &item in &array_4 {
            array.push(String::from(item));
        }
        array.shuffle(&mut rand::thread_rng());
        if array_4.len() % 2 == 1 && form == 2 {
            let mut rng = rand::thread_rng();
            let random_index = rng.gen_range(0..array_4.len());
            array.push(String::from(array_4[random_index]) + "*");
        }
    } else if class == 5 {
        let array_5 = [
            "555 EEE eee   ",
        ];
        for &item in &array_5 {
            array.push(String::from(item));
        }
        array.shuffle(&mut rand::thread_rng());
        if array_5.len() % 2 == 1 && form == 2 {
            let mut rng = rand::thread_rng();
            let random_index = rng.gen_range(0..array_5.len());
            array.push(String::from(array_5[random_index]) + "*");
        }
    } else if class == 6 {
        let array_6 = [
            "666 FFF fff   ",
        ];
        for &item in &array_6 {
            array.push(String::from(item));
        }
        array.shuffle(&mut rand::thread_rng());
        if array_6.len() % 2 == 1 && form == 2 {
            let mut rng = rand::thread_rng();
            let random_index = rng.gen_range(0..array_6.len());
            array.push(String::from(array_6[random_index]) + "*");
        }
    } else if class == 7 {
        let array_7 = [
            "777 GGG ggg   ",
        ];
        for &item in &array_7 {
            array.push(String::from(item));
        }
        array.shuffle(&mut rand::thread_rng());
        if array_7.len() % 2 == 1 && form == 2 {
            let mut rng = rand::thread_rng();
            let random_index = rng.gen_range(0..array_7.len());
            array.push(String::from(array_7[random_index]) + "*");
        }
    }

    let mut count: u8 = 1;

    if form == 1 {
        // ソロ
        println!("----------------------------------");
        println!("｜発表順｜No.｜\t　　氏名　　\t｜");
        println!("----------------------------------");
        for i in 0..array.len() {
            println!("｜{0: ^6}｜{1: <3}｜{2}　\t｜", count, &array[i][..3], &array[i][3..]);
            count += 1;
        }
        println!("----------------------------------");
    } else if form == 2 {
        // ペア
        println!("----------------------------------------------------------");
        println!("｜発表順｜No.｜\t　　　A　　\t｜No.｜\t　　　B　　\t｜");
        println!("----------------------------------------------------------");
        for i in (0..array.len()).step_by(2) {
            if i + 1 < array.len() {
                println!("｜{0: ^6}｜{1: <3}｜{2}\t｜{3: <3}｜{4}\t｜", 
                        count, &array[i][..3], &array[i][4..], &array[i + 1][..3], &array[i + 1][4..]);
                count += 1;
            }
        }
        println!("----------------------------------------------------------");
    } else if form == 3 {
        // グループ
        let num_groups = 5;
        let mut group_size = 0;
        if array.len() > 20 {
            group_size = 5;
        } else if array.len() > 15 {
            group_size = 4;
        } else if array.len() > 10 {
            group_size = 3;
        } else if array.len() > 5 {
            group_size = 2;
        }
        println!("--------------------------------------------------------------------------------------------");
        println!("｜{0:^11}｜{1:^11}｜{2:^11}｜{3:^11}｜{4:^11}｜",
        "    Group  A    ", "    Group  B    ", "    Group  C    ", "    Group  D    ", "    Group  E    ");
        println!("--------------------------------------------------------------------------------------------");
        for group in 0..num_groups {
            for i in 0..group_size {
                let index = group * group_size + i;
                if index < array.len() {
                    print!("｜{}", array[index]);
                }
            }
            print!("｜");
        println!("");
        println!("--------------------------------------------------------------------------------------------");
        }
    }
}