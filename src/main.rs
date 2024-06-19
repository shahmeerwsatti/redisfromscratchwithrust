use std::io;

fn main()
{
    let mut conversiondirection: String = String::new();
    let mut temp: String = String::new();

    println!("Please input your choice, would you like to convert from Celcius to Fahrenheit or Fahrenheit to Celcius? 1 for Celcius to Fahrenheit, and 2 for Fahrenheit to Celcius: ");
    io::stdin().read_line(&mut conversiondirection).expect("Operation failed.");
    let conversiondirection: u32 = conversiondirection.trim().parse().expect("Enter a number please.");
    if conversiondirection != 1 && conversiondirection != 2
    {
        println!("Error! Unexpected number.");
        std::process::exit(0)
    }

    println!("Input the temperature you would like to convert: ");
    io::stdin().read_line(&mut temp).expect("Operation failed.");
    let temp: f64 = temp.trim().parse().expect("Enter a number please.");

    if conversiondirection == 1
    {
        println!("The temperature in Fahrenheit is {}", celciustofahrenheit(temp));
    }
    else if conversiondirection == 2
    {
        println!("The temperature in Celcius is {}", fahrenheittocelcius(temp));
    }
}

fn celciustofahrenheit(mut x: f64) -> f64
{
    x = ((x * 9.0)/5.0) + 32.0;
    x
}

fn fahrenheittocelcius(mut x: f64) -> f64
{
    x = ((x-32.0) * 5.0)/9.0;
    x
}