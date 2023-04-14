use rand::Rng;
use std::io;

fn main() {
    let mut numbers: Vec<i64> = Vec::new(); // rastgele üretilen tüm sayılar

    // rastgele 5 adet tek basamaklı sayı üretir
    for _ in 0..5 {
        numbers.push(rand::thread_rng().gen_range(1..10));
    }

    // rastgele çift basamaklı bir sayı üretir
    numbers.push( rand::thread_rng().gen_range(1..10)*10);

    println!("sayılar iste {:?}", numbers);

    // Kullanıcıdan bir sayı ister
    let mut user_input = String::new();

    println!("Lütfen bir sayı giriniz: ");

    io::stdin().read_line(&mut user_input).unwrap();

    let user_input: i64 = user_input.trim().parse().unwrap();

    let mut closest_result: Option<i64> = None;
    let mut closest_difference: Option<i64> = None;


        for num1 in &numbers {
            for num2 in &numbers {
                let result = num1 + num2;
                let difference = (result - user_input).abs();

                if closest_difference.is_none() || difference < closest_difference.unwrap() {
                    closest_result = Some(result);
                    closest_difference = Some(difference);
                    println!("{}+{}={}",num1,num2,closest_result.unwrap());
                }

                let result = num1 - num2;
                let difference = (result - user_input).abs();

                if closest_difference.is_none() || difference < closest_difference.unwrap() {
                    closest_result = Some(result);
                    closest_difference = Some(difference);
                    println!("{}-{}={}",num1,num2,closest_result.unwrap());
                }

                let result = num1 * num2;
                let difference = (result - user_input).abs();

                if closest_difference.is_none() || difference < closest_difference.unwrap() {
                    closest_result = Some(result);
                    closest_difference = Some(difference);
                    println!("{}*{}={}",num1,num2,closest_result.unwrap());
                }

                if *num2 != 0 {
                    let result = num1 / num2;
                    let difference = (result - user_input).abs();

                    if closest_difference.is_none() || difference < closest_difference.unwrap() {
                        closest_result = Some(result);
                        closest_difference = Some(difference);
                        println!("{}/{}={}",num1,num2,closest_result.unwrap());
                    }
                }

            }
        }


    // Sonucu yazdır
    match closest_result {
        Some(res) =>{
            if (closest_result.unwrap()-user_input).abs()<=9{
            println!("En yakın sonuç: {}", res)}
            else { println!("\nUygun aralıkta sonuç bulunamadı.\
            \nBulunan en yakın sonuç:{}",res); }

        },
        None => println!("Hiçbir sonuç bulunamadı"),
    }
}
